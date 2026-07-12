#![recursion_limit = "512"]

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::sync::Mutex;

use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand};
use jsonschema::{Draft, JSONSchema};
use serde::Deserialize;
use serde_json::{Map, Value};
use url::Url;
use xmlwriter::{Options as XmlOptions, XmlWriter};

fn main() -> process::ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(outcome) => emit_outcome(&outcome).unwrap_or_else(report_output_error),
        Err(error) => {
            eprintln!("{error:#}");
            process::ExitCode::from(2)
        }
    }
}

fn emit_outcome(outcome: &CommandOutcome) -> io::Result<process::ExitCode> {
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();

    write_outcome(&mut stdout, &mut stderr, outcome)
}

fn write_outcome(
    stdout: &mut impl Write,
    stderr: &mut impl Write,
    outcome: &CommandOutcome,
) -> io::Result<process::ExitCode> {
    if !outcome.stdout.is_empty() {
        stdout.write_all(outcome.stdout.as_bytes())?;
        stdout.flush()?;
    }
    if !outcome.stderr.is_empty() {
        stderr.write_all(outcome.stderr.as_bytes())?;
        stderr.flush()?;
    }

    Ok(normalize_exit_code(outcome.exit_code))
}

fn report_output_error(error: io::Error) -> process::ExitCode {
    eprintln!("failed to write command output: {error}");
    process::ExitCode::from(2)
}

fn normalize_exit_code(exit_code: i32) -> process::ExitCode {
    let normalized = match u8::try_from(exit_code) {
        Ok(code) => code,
        Err(_) if exit_code < 0 => 1,
        Err(_) => u8::MAX,
    };
    process::ExitCode::from(normalized)
}

fn run(cli: Cli) -> Result<CommandOutcome> {
    match cli.command {
        Commands::Validate { input } => run_validate(&input),
        Commands::Generate {
            command: GenerateCommands::Svd { input, output },
        } => run_generate_svd(&input, output.as_deref()),
        Commands::Generate {
            command: GenerateCommands::Embassy { input, output_dir },
        } => run_generate_embassy(&input, &output_dir),
        Commands::Generate {
            command: GenerateCommands::EmbassyHost { input, output_dir },
        } => run_generate_embassy_host(&input, &output_dir),
        Commands::Diff { left, right } => run_diff(&left, &right),
    }
}

#[derive(Parser)]
#[command(name = "hair")]
#[command(about = "HAIR document tooling")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Validate {
        input: PathBuf,
    },
    Generate {
        #[command(subcommand)]
        command: GenerateCommands,
    },
    Diff {
        left: String,
        right: String,
    },
}

#[derive(Subcommand)]
enum GenerateCommands {
    Svd {
        input: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Embassy {
        input: PathBuf,
        #[arg(long = "output-dir")]
        output_dir: PathBuf,
    },
    EmbassyHost {
        input: PathBuf,
        #[arg(long = "output-dir")]
        output_dir: PathBuf,
    },
}

struct CommandOutcome {
    exit_code: i32,
    stdout: String,
    stderr: String,
}

fn run_validate(input: &Path) -> Result<CommandOutcome> {
    let schema_root = find_schema_root(&std::env::current_dir()?)?;
    let instance = load_json_file(input)?;
    let validator = build_validator(&schema_root)?;
    match validator.validate(&instance) {
        Ok(()) => Ok(CommandOutcome {
            exit_code: 0,
            stdout: format!("Schema validation passed for {}", input.display()),
            stderr: String::new(),
        }),
        Err(errors) => {
            let messages = errors
                .map(|error| format_validation_error(&error))
                .collect::<Vec<_>>();
            Ok(CommandOutcome {
                exit_code: 1,
                stdout: String::new(),
                stderr: messages.join("\n"),
            })
        }
    }
}

fn run_generate_svd(input: &Path, output: Option<&Path>) -> Result<CommandOutcome> {
    let schema_root = find_schema_root(&std::env::current_dir()?)?;
    let document = load_validated_hair_document(input, &schema_root)?;
    let svd = generate_svd(&document)?;

    if let Some(output_path) = output {
        if let Some(parent) = output_path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating output directory {}", parent.display()))?;
        }
        fs::write(output_path, &svd)
            .with_context(|| format!("writing {}", output_path.display()))?;
        Ok(CommandOutcome {
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
        })
    } else {
        Ok(CommandOutcome {
            exit_code: 0,
            stdout: svd,
            stderr: String::new(),
        })
    }
}

fn run_generate_embassy(input: &Path, output_dir: &Path) -> Result<CommandOutcome> {
    let schema_root = find_schema_root(&std::env::current_dir()?)?;
    let document = load_validated_hair_document(input, &schema_root)?;
    generate_embassy_crate(&document, output_dir)?;
    Ok(CommandOutcome {
        exit_code: 0,
        stdout: String::new(),
        stderr: String::new(),
    })
}

fn run_generate_embassy_host(input: &Path, output_dir: &Path) -> Result<CommandOutcome> {
    let schema_root = find_schema_root(&std::env::current_dir()?)?;
    let document = load_validated_hair_document(input, &schema_root)?;
    generate_embassy_host_crate(&document, output_dir)?;
    Ok(CommandOutcome {
        exit_code: 0,
        stdout: String::new(),
        stderr: String::new(),
    })
}

fn generate_embassy_crate(document: &HairDocument, output_dir: &Path) -> Result<()> {
    let model = EmbassyGenerationModel::from_document(document)?;
    let src_dir = output_dir.join("src");
    fs::create_dir_all(&src_dir)
        .with_context(|| format!("creating output directory {}", src_dir.display()))?;

    write_generated_text(
        &output_dir.join("Cargo.toml"),
        &render_embassy_cargo_toml(&model),
    )?;
    write_generated_text(&src_dir.join("lib.rs"), &render_embassy_lib_rs(&model))?;
    write_generated_text(
        &src_dir.join("metadata.rs"),
        &render_embassy_metadata_rs(&model),
    )?;

    for (module_name, driver_group) in model.driver_groups() {
        let rendered = if module_name == "interrupt" {
            render_interrupt_module(&model, &driver_group)?
        } else {
            render_driver_module(&model, &module_name, &driver_group)?
        };
        write_generated_text(&src_dir.join(format!("{module_name}.rs")), &rendered)?;
    }

    Ok(())
}

fn generate_embassy_host_crate(document: &HairDocument, output_dir: &Path) -> Result<()> {
    let model = EmbassyGenerationModel::from_document(document)?;
    validate_reserved_generated_module_names(
        &model,
        &[("host", "reserved generated host module name host")],
    )?;
    validate_host_accessor_name_collisions(&model.drivers)?;
    let src_dir = output_dir.join("src");
    fs::create_dir_all(&src_dir)
        .with_context(|| format!("creating output directory {}", src_dir.display()))?;

    write_generated_text(
        &output_dir.join("Cargo.toml"),
        &render_embassy_host_cargo_toml(&model),
    )?;
    write_generated_text(&src_dir.join("lib.rs"), &render_embassy_host_lib_rs(&model))?;
    write_generated_text(
        &src_dir.join("metadata.rs"),
        &render_embassy_host_metadata_rs(&model)?,
    )?;
    write_generated_text(
        &src_dir.join("host.rs"),
        &render_embassy_host_root_rs(&model),
    )?;

    for (module_name, driver_group) in model.driver_groups() {
        let rendered = render_embassy_host_driver_module(&model, &module_name, &driver_group)?;
        write_generated_text(&src_dir.join(format!("{module_name}.rs")), &rendered)?;
    }

    Ok(())
}

fn write_generated_text(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating output directory {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("writing {}", path.display()))
}

fn run_diff(left: &str, right: &str) -> Result<CommandOutcome> {
    let current_dir = std::env::current_dir()?;
    let repo_root = if is_git_selector(left) || is_git_selector(right) {
        find_git_repo_root(&current_dir)?
    } else {
        current_dir
    };
    let left_value = load_diff_operand(&repo_root, left)?;
    let right_value = load_diff_operand(&repo_root, right)?;

    let differences = diff_values(&left_value, &right_value, "");
    if differences.is_empty() {
        return Ok(CommandOutcome {
            exit_code: 0,
            stdout: "No differences found.".to_string(),
            stderr: String::new(),
        });
    }

    let mut lines = Vec::with_capacity(differences.len() * 2);
    for difference in differences {
        lines.push(difference.summary());
    }

    Ok(CommandOutcome {
        exit_code: 1,
        stdout: lines.join("\n"),
        stderr: String::new(),
    })
}

fn build_validator(repo_root: &Path) -> Result<JSONSchema> {
    let schema_root = repo_root.join("schema");
    let bundle = HairSchemaBundle::new(schema_root);
    let documents = bundle.load_all_documents()?;
    let root_id = "https://hair.dev/schema/hair.json";
    let normalized_root = documents
        .get(root_id)
        .cloned()
        .ok_or_else(|| anyhow!("root schema {root_id} was not loaded"))?;

    let mut options = JSONSchema::options();
    options.with_draft(Draft::Draft202012);
    for (id, document) in &documents {
        if id != root_id {
            options.with_document(id.clone(), document.clone());
        }
    }

    options
        .compile(&normalized_root)
        .map_err(|error| anyhow!("compiling HAIR schema failed: {error}"))
}

struct HairSchemaBundle {
    schema_root: PathBuf,
    cache: Mutex<HashMap<String, Value>>,
}

impl HairSchemaBundle {
    fn new(schema_root: PathBuf) -> Self {
        Self {
            schema_root,
            cache: Mutex::new(HashMap::new()),
        }
    }

    fn load_all_documents(&self) -> Result<HashMap<String, Value>> {
        let mut documents = HashMap::new();
        for path in collect_json_files(&self.schema_root)? {
            let text =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            let raw: Value = serde_json::from_str(&text)
                .with_context(|| format!("parsing {}", path.display()))?;
            let id = raw
                .get("$id")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow!("schema {} is missing $id", path.display()))?;
            let normalized = self.normalize_schema(&raw, id)?;
            documents.insert(id.to_string(), normalized);
        }
        Ok(documents)
    }

    fn normalize_schema(&self, schema: &Value, base_uri: &str) -> Result<Value> {
        match schema {
            Value::Array(items) => {
                let mut normalized = Vec::with_capacity(items.len());
                for item in items {
                    normalized.push(self.normalize_schema(item, base_uri)?);
                }
                Ok(Value::Array(normalized))
            }
            Value::Object(map) => {
                if let Some(all_of) = map.get("allOf").and_then(Value::as_array) {
                    let mut merged = Map::new();
                    let mut saw_additional_properties_false = false;

                    let mut base_map = map.clone();
                    base_map.remove("allOf");
                    let base_value = self.normalize_non_allof_object(base_map, base_uri)?;
                    self.merge_schema_object(
                        &mut merged,
                        base_value
                            .as_object()
                            .ok_or_else(|| anyhow!("normalized schema object was not an object"))?,
                        &mut saw_additional_properties_false,
                    )?;

                    for item in all_of {
                        let expanded = self.expand_all_of_item(item, base_uri)?;
                        let object = expanded.as_object().ok_or_else(|| {
                            anyhow!("allOf item could not be normalized as an object")
                        })?;
                        self.merge_schema_object(
                            &mut merged,
                            object,
                            &mut saw_additional_properties_false,
                        )?;
                    }

                    if saw_additional_properties_false {
                        merged.insert("additionalProperties".to_string(), Value::Bool(false));
                    }

                    self.normalize_schema(&Value::Object(merged), base_uri)
                } else {
                    self.normalize_non_allof_object(map.clone(), base_uri)
                }
            }
            _ => Ok(schema.clone()),
        }
    }

    fn normalize_non_allof_object(
        &self,
        mut map: Map<String, Value>,
        base_uri: &str,
    ) -> Result<Value> {
        let current_base = map
            .get("$id")
            .and_then(Value::as_str)
            .map(|id| resolve_reference_uri(id, Some(base_uri)))
            .transpose()?
            .unwrap_or_else(|| base_uri.to_string());

        if let Some(reference) = map.get("$ref").and_then(Value::as_str) {
            let absolute = resolve_reference_uri(reference, Some(&current_base))?;
            map.insert("$ref".to_string(), Value::String(absolute));
        }

        let keys = map.keys().cloned().collect::<Vec<_>>();
        for key in keys {
            if key == "$ref" {
                continue;
            }
            if let Some(value) = map.get(&key).cloned() {
                let normalized = self.normalize_schema(&value, &current_base)?;
                map.insert(key, normalized);
            }
        }
        Ok(Value::Object(map))
    }

    fn expand_all_of_item(&self, item: &Value, base_uri: &str) -> Result<Value> {
        if let Some(reference) = item.get("$ref").and_then(Value::as_str) {
            let resolved_uri = resolve_reference_uri(reference, Some(base_uri))?;
            let document = self.load_document(&resolved_uri)?;
            let sub_schema = resolve_fragment(&document, &resolved_uri)?;
            return self.normalize_schema(&sub_schema, strip_fragment(&resolved_uri));
        }
        self.normalize_schema(item, base_uri)
    }

    fn load_document(&self, document_uri: &str) -> Result<Value> {
        let cache_key = strip_fragment(document_uri).to_string();
        {
            let cache = self
                .cache
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some(value) = cache.get(&cache_key).cloned() {
                return Ok(value);
            }
        }

        let path = self.uri_to_path(&cache_key)?;
        let text =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let value: Value =
            serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
        self.cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(cache_key, value.clone());
        Ok(value)
    }

    fn uri_to_path(&self, uri: &str) -> Result<PathBuf> {
        let parsed = Url::parse(strip_fragment(uri))
            .with_context(|| format!("parsing schema URI {}", strip_fragment(uri)))?;

        match (parsed.scheme(), parsed.host_str()) {
            ("https", Some("hair.dev")) => {
                let path = parsed.path().trim_start_matches('/');
                let relative = path
                    .strip_prefix("schema/")
                    .ok_or_else(|| anyhow!("unsupported schema path {}", parsed.path()))?;
                Ok(self.schema_root.join(relative))
            }
            ("file", _) => parsed
                .to_file_path()
                .map_err(|_| anyhow!("could not convert {uri} to a file path")),
            _ => bail!("unsupported schema URI {uri}"),
        }
    }

    fn merge_schema_object(
        &self,
        merged: &mut Map<String, Value>,
        next: &Map<String, Value>,
        saw_additional_properties_false: &mut bool,
    ) -> Result<()> {
        for (key, value) in next {
            match key.as_str() {
                "additionalProperties" if value == &Value::Bool(false) => {
                    *saw_additional_properties_false = true;
                }
                "properties" | "$defs" => {
                    let target = merged
                        .entry(key.clone())
                        .or_insert_with(|| Value::Object(Map::new()));
                    merge_object_values(target, value)
                        .with_context(|| format!("merging schema object key {key}"))?;
                }
                "required" => {
                    let target = merged
                        .entry(key.clone())
                        .or_insert_with(|| Value::Array(Vec::new()));
                    merge_unique_arrays(target, value)
                        .with_context(|| format!("merging schema requirements for {key}"))?;
                }
                _ => {
                    if let Some(existing) = merged.get(key) {
                        if existing != value {
                            bail!("conflicting schema values for key {key}");
                        }
                    } else {
                        merged.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        Ok(())
    }
}

fn merge_object_values(target: &mut Value, source: &Value) -> Result<()> {
    let target_object = target
        .as_object_mut()
        .ok_or_else(|| anyhow!("merge target was not an object"))?;
    let source_object = source
        .as_object()
        .ok_or_else(|| anyhow!("merge source was not an object"))?;

    for (key, value) in source_object {
        if let Some(existing) = target_object.get(key) {
            if existing != value {
                bail!("conflicting object values for key {key}");
            }
        } else {
            target_object.insert(key.clone(), value.clone());
        }
    }
    Ok(())
}

fn merge_unique_arrays(target: &mut Value, source: &Value) -> Result<()> {
    let target_array = target
        .as_array_mut()
        .ok_or_else(|| anyhow!("merge target was not an array"))?;
    let source_array = source
        .as_array()
        .ok_or_else(|| anyhow!("merge source was not an array"))?;

    let mut seen = target_array
        .iter()
        .map(render_json)
        .collect::<BTreeSet<_>>();
    for value in source_array {
        if seen.insert(render_json(value)) {
            target_array.push(value.clone());
        }
    }
    Ok(())
}

fn resolve_reference_uri(reference: &str, base_uri: Option<&str>) -> Result<String> {
    if let Ok(url) = Url::parse(reference) {
        return Ok(url.into());
    }

    if let Some(base_uri) = base_uri {
        let base = Url::parse(base_uri).with_context(|| format!("parsing base URI {base_uri}"))?;
        return Ok(base.join(reference)?.into());
    }

    bail!("cannot resolve schema reference {reference} without a base URI")
}

fn resolve_fragment(document: &Value, uri: &str) -> Result<Value> {
    let parsed = Url::parse(uri)?;
    match parsed.fragment() {
        Some(fragment) if !fragment.is_empty() => {
            let pointer = if let Some(stripped) = fragment.strip_prefix('/') {
                format!("/{stripped}")
            } else {
                fragment.to_string()
            };
            document
                .pointer(&pointer)
                .cloned()
                .ok_or_else(|| anyhow!("schema fragment #{fragment} not found"))
        }
        _ => Ok(document.clone()),
    }
}

fn strip_fragment(uri: &str) -> &str {
    uri.split_once('#').map(|(head, _)| head).unwrap_or(uri)
}

fn is_git_selector(operand: &str) -> bool {
    operand.starts_with("git:")
}

fn load_json_file(path: &Path) -> Result<Value> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

fn load_validated_hair_document(path: &Path, repo_root: &Path) -> Result<HairDocument> {
    let instance = load_json_file(path)?;
    let validator = build_validator(repo_root)?;
    if let Err(errors) = validator.validate(&instance) {
        let messages = errors
            .map(|error| format_validation_error(&error))
            .collect::<Vec<_>>();
        bail!(
            "input {} does not conform to the HAIR schema:\n{}",
            path.display(),
            messages.join("\n")
        );
    }

    serde_json::from_value(instance).with_context(|| format!("parsing {}", path.display()))
}

fn find_schema_root(start: &Path) -> Result<PathBuf> {
    for candidate in start.ancestors() {
        if candidate.join("schema").join("hair.json").exists() {
            return Ok(candidate.to_path_buf());
        }
    }
    bail!("could not find schema root containing schema/hair.json")
}

fn find_git_repo_root(start: &Path) -> Result<PathBuf> {
    for candidate in start.ancestors() {
        if candidate.join(".git").exists() {
            return Ok(candidate.to_path_buf());
        }
    }
    bail!("could not find git repository root containing .git")
}

fn collect_json_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root).with_context(|| format!("reading {}", root.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_json_files(&path)?);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            files.push(path);
        }
    }
    Ok(files)
}

fn format_validation_error(error: &jsonschema::ValidationError) -> String {
    let instance_path = if error.instance_path.to_string().is_empty() {
        "/".to_string()
    } else {
        error.instance_path.to_string()
    };
    format!("{instance_path}: {error}")
}

fn load_diff_operand(repo_root: &Path, operand: &str) -> Result<Value> {
    if let Some(selector) = operand.strip_prefix("git:") {
        let (rev, repo_path) = selector
            .split_once(':')
            .ok_or_else(|| anyhow!("git selector must have the form git:<rev>:<repo-path>"))?;
        let output = Command::new("git")
            .arg("--no-pager")
            .arg("show")
            .arg(format!("{rev}:{repo_path}"))
            .current_dir(repo_root)
            .output()
            .with_context(|| format!("running git show for {operand}"))?;
        if !output.status.success() {
            bail!(
                "git show failed for {operand}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        serde_json::from_slice(&output.stdout)
            .with_context(|| format!("parsing JSON from git selector {operand}"))
    } else {
        let path = Path::new(operand);
        let resolved = if path.is_absolute() {
            path.to_path_buf()
        } else {
            repo_root.join(path)
        };
        load_json_file(&resolved)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DifferenceKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Difference {
    path: String,
    kind: DifferenceKind,
    left: Option<String>,
    right: Option<String>,
}

impl Difference {
    fn summary(&self) -> String {
        match self.kind {
            DifferenceKind::Added => {
                format!(
                    "+ {} = {}",
                    self.path,
                    self.right.as_deref().unwrap_or("null")
                )
            }
            DifferenceKind::Removed => {
                format!(
                    "- {} = {}",
                    self.path,
                    self.left.as_deref().unwrap_or("null")
                )
            }
            DifferenceKind::Changed => format!(
                "~ {}: {} -> {}",
                self.path,
                self.left.as_deref().unwrap_or("null"),
                self.right.as_deref().unwrap_or("null")
            ),
        }
    }
}

fn diff_values(left: &Value, right: &Value, path: &str) -> Vec<Difference> {
    match (left, right) {
        (Value::Object(left_map), Value::Object(right_map)) => {
            let mut differences = Vec::new();
            let keys = left_map
                .keys()
                .chain(right_map.keys())
                .cloned()
                .collect::<BTreeSet<_>>();
            for key in keys {
                let next_path = join_path(path, &escape_pointer(&key));
                match (left_map.get(&key), right_map.get(&key)) {
                    (Some(left_value), Some(right_value)) => {
                        differences.extend(diff_values(left_value, right_value, &next_path));
                    }
                    (Some(left_value), None) => differences.push(Difference {
                        path: next_path,
                        kind: DifferenceKind::Removed,
                        left: Some(render_json(left_value)),
                        right: None,
                    }),
                    (None, Some(right_value)) => differences.push(Difference {
                        path: next_path,
                        kind: DifferenceKind::Added,
                        left: None,
                        right: Some(render_json(right_value)),
                    }),
                    (None, None) => {}
                }
            }
            differences
        }
        (Value::Array(left_items), Value::Array(right_items)) => {
            let mut differences = Vec::new();
            let max_len = left_items.len().max(right_items.len());
            for index in 0..max_len {
                let next_path = join_path(path, &index.to_string());
                match (left_items.get(index), right_items.get(index)) {
                    (Some(left_value), Some(right_value)) => {
                        differences.extend(diff_values(left_value, right_value, &next_path));
                    }
                    (Some(left_value), None) => differences.push(Difference {
                        path: next_path,
                        kind: DifferenceKind::Removed,
                        left: Some(render_json(left_value)),
                        right: None,
                    }),
                    (None, Some(right_value)) => differences.push(Difference {
                        path: next_path,
                        kind: DifferenceKind::Added,
                        left: None,
                        right: Some(render_json(right_value)),
                    }),
                    (None, None) => {}
                }
            }
            differences
        }
        _ if left == right => Vec::new(),
        _ => vec![Difference {
            path: normalized_path(path),
            kind: DifferenceKind::Changed,
            left: Some(render_json(left)),
            right: Some(render_json(right)),
        }],
    }
}

fn render_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<unrenderable JSON>".to_string())
}

fn normalized_path(path: &str) -> String {
    if path.is_empty() {
        "/".to_string()
    } else {
        path.to_string()
    }
}

fn join_path(base: &str, segment: &str) -> String {
    if base.is_empty() {
        format!("/{segment}")
    } else {
        format!("{base}/{segment}")
    }
}

fn escape_pointer(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairDocument {
    metadata: DocumentMetadata,
    #[serde(default)]
    provenance: HairProvenance,
    structure: HairStructure,
    #[serde(default)]
    semantics: HairSemantics,
    #[serde(default)]
    physical: HairPhysical,
    #[serde(default)]
    normalization: HairNormalization,
    #[serde(default)]
    profiles: HairProfiles,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentMetadata {
    title: String,
    version: String,
    description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairProvenance {
    #[serde(default)]
    sources: Vec<HairProvenanceSource>,
    #[serde(default)]
    evidence: Vec<HairProvenanceEvidence>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairProvenanceSource {
    id: String,
    name: String,
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairProvenanceEvidence {
    id: String,
    name: String,
    source_ref: String,
    #[serde(default)]
    normalized_claim: Option<String>,
    #[serde(default)]
    extraction_method: Option<String>,
    #[serde(default)]
    confidence: Option<f64>,
    #[serde(default)]
    locator: Option<HairProvenanceLocator>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairProvenanceLocator {
    #[serde(default)]
    fragment: Option<String>,
    #[serde(default)]
    section: Option<String>,
    #[serde(default)]
    line_start: Option<u32>,
    #[serde(default)]
    line_end: Option<u32>,
    #[serde(default)]
    page_start: Option<u32>,
    #[serde(default)]
    page_end: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairStructure {
    #[serde(default)]
    architectures: Vec<ArchitectureProfile>,
    device: Device,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArchitectureProfile {
    id: String,
    #[serde(default)]
    address_unit_bits: Option<u32>,
    #[serde(default)]
    data_bus_width_bits: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Device {
    name: String,
    vendor: String,
    #[serde(default)]
    architecture_ref: Option<String>,
    cpu: CpuModel,
    #[serde(default)]
    interrupts: Vec<Interrupt>,
    peripherals: Vec<Peripheral>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CpuModel {
    name: String,
    revision: String,
    endianness: String,
    interrupt_priority_bits: u32,
    feature_flags: CpuFeatureFlags,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CpuFeatureFlags {
    mpu_present: bool,
    fpu_present: bool,
    vendor_system_timer_config: bool,
    #[serde(default)]
    fpu_double_precision: Option<bool>,
    #[serde(default)]
    dsp_present: Option<bool>,
    #[serde(default)]
    icache_present: Option<bool>,
    #[serde(default)]
    dcache_present: Option<bool>,
    #[serde(default)]
    itcm_present: Option<bool>,
    #[serde(default)]
    dtcm_present: Option<bool>,
    #[serde(default)]
    vtor_present: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Interrupt {
    id: String,
    name: String,
    number: i32,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Peripheral {
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "type")]
    peripheral_type: String,
    #[serde(default)]
    base_address: Option<u64>,
    #[serde(default)]
    derived_from_ref: Option<String>,
    #[serde(default)]
    address_blocks: Vec<AddressBlock>,
    #[serde(default)]
    interrupt_refs: Vec<String>,
    #[serde(default)]
    registers: Vec<RegisterBlockMember>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddressBlock {
    #[serde(default)]
    name: Option<String>,
    offset_bytes: u64,
    size_bytes: u64,
    usage: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum RegisterBlockMember {
    Register(Register),
    Cluster(RegisterCluster),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Register {
    id: String,
    name: String,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    offset_bytes: u64,
    width_bits: u32,
    #[serde(default)]
    access: Option<String>,
    #[serde(default)]
    reset_value: Option<Value>,
    #[serde(default)]
    reset_mask: Option<Value>,
    #[serde(default)]
    array: Option<ArrayShape>,
    #[serde(default)]
    alternate_of_ref: Option<String>,
    #[serde(default)]
    fields: Vec<Field>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterCluster {
    name: String,
    #[serde(default)]
    description: Option<String>,
    offset_bytes: u64,
    #[serde(default)]
    array: Option<ArrayShape>,
    members: Vec<RegisterBlockMember>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArrayShape {
    axes: Vec<ArrayAxis>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArrayAxis {
    count: u32,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    stride_bytes: Option<u64>,
}

#[derive(Debug)]
struct RegisterBinding {
    svd_name: String,
    offset_bytes: u64,
    width_bits: u32,
    access: Option<String>,
    reset_value: Option<Value>,
    reset_mask: Option<Value>,
    array: Option<ArrayShape>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Field {
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    bit_range: BitRange,
    #[serde(default)]
    access: Option<String>,
    #[serde(default)]
    array: Option<ArrayShape>,
    #[serde(default)]
    enumerated_sets: Vec<EnumeratedSet>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BitRange {
    lsb: u32,
    msb: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnumeratedSet {
    #[serde(default)]
    usage: Option<String>,
    values: Vec<EnumeratedValue>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnumeratedValue {
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    value: Option<Value>,
    #[serde(default)]
    is_default: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairSemantics {
    #[serde(default)]
    operations: Vec<SemanticOperation>,
    #[serde(default)]
    state_machines: Vec<SemanticStateMachine>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticOperation {
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    target_refs: Vec<String>,
    #[serde(default)]
    steps: Vec<SemanticOperationStep>,
    #[serde(default)]
    preconditions: Vec<SemanticPredicate>,
    #[serde(default)]
    postconditions: Vec<SemanticPredicate>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticStateMachine {
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    target_refs: Vec<String>,
    #[serde(default)]
    initial_state: Option<String>,
    #[serde(default)]
    states: Vec<SemanticState>,
    #[serde(default)]
    transitions: Vec<SemanticTransition>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticPredicate {
    kind: String,
    #[serde(default)]
    target_ref: Option<String>,
    #[serde(default)]
    expression: Option<SemanticExpression>,
    #[serde(default)]
    expected_value: Option<Value>,
    #[serde(default)]
    description: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticOperationStep {
    index: u32,
    action: String,
    #[serde(default)]
    target_ref: Option<String>,
    #[serde(default)]
    expression: Option<SemanticExpression>,
    #[serde(default)]
    value: Option<Value>,
    #[serde(default)]
    description: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticExpression {
    #[serde(default)]
    language: Option<String>,
    text: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticState {
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    invariants: Vec<SemanticPredicate>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticTransition {
    from: String,
    to: String,
    #[allow(dead_code)]
    #[serde(default)]
    trigger: Option<String>,
    #[serde(default)]
    conditions: Vec<SemanticPredicate>,
    #[serde(default)]
    effects: Vec<SemanticSideEffect>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticSideEffect {
    kind: String,
    #[serde(default)]
    target_ref: Option<String>,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairPhysical {
    #[serde(default)]
    pins: Vec<PhysicalPin>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairNormalization {
    #[serde(default)]
    canonical_terms: Vec<CanonicalTerm>,
    #[serde(default)]
    mappings: Vec<NormalizedMapping>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CanonicalTerm {
    id: String,
    name: String,
    scope: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NormalizedMapping {
    id: String,
    name: String,
    target_ref: String,
    #[serde(default)]
    canonical_term_refs: Vec<String>,
    #[serde(default)]
    vendor_names: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PhysicalPin {
    id: String,
    name: String,
    #[serde(default)]
    port: Option<String>,
    #[serde(default)]
    index: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairProfiles {
    #[serde(default)]
    mcu_soc: Option<McuSocProfile>,
    #[serde(default)]
    embassy_hal: Option<EmbassyHalProfile>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuSocProfile {
    #[serde(default)]
    canonical_blocks: Vec<McuCanonicalBlock>,
    #[serde(default)]
    dma_topology: Option<McuDmaTopology>,
    #[serde(default)]
    interrupt_topology: Option<McuInterruptTopology>,
    #[serde(default)]
    clock_reset_topology: Option<McuClockResetTopology>,
    #[serde(default)]
    pin_topology: Option<McuPinTopology>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuCanonicalBlock {
    id: String,
    name: String,
    target_ref: String,
    block_class: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuClockResetTopology {
    #[serde(default)]
    clock_bindings: Vec<McuClockBinding>,
    #[serde(default)]
    reset_bindings: Vec<McuResetBinding>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuClockBinding {
    id: String,
    name: String,
    consumer_ref: String,
    clock_ref: String,
    #[serde(default)]
    controller_ref: Option<String>,
    binding_kind: String,
    #[serde(default)]
    control_refs: Vec<String>,
    #[serde(default)]
    enable_operation_refs: Vec<String>,
    #[serde(default)]
    disable_operation_refs: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuResetBinding {
    id: String,
    name: String,
    target_ref: String,
    #[serde(default)]
    controller_ref: Option<String>,
    #[serde(default)]
    reset_domain_ref: Option<String>,
    binding_kind: String,
    #[serde(default)]
    control_refs: Vec<String>,
    #[serde(default)]
    assert_operation_refs: Vec<String>,
    #[serde(default)]
    release_operation_refs: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuInterruptTopology {
    #[serde(default)]
    sources: Vec<McuInterruptSource>,
    #[serde(default)]
    routes: Vec<McuInterruptRoute>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuInterruptSource {
    id: String,
    name: String,
    source_ref: String,
    #[serde(default)]
    producer_ref: Option<String>,
    kind: String,
    #[serde(default)]
    flag_refs: Vec<String>,
    #[serde(default)]
    clear_operation_refs: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuInterruptRoute {
    id: String,
    name: String,
    source_ref: String,
    interrupt_ref: String,
    controller_ref: String,
    #[serde(default)]
    cpu_target_ref: Option<String>,
    #[serde(default)]
    line_index: Option<u32>,
    route_type: String,
    #[serde(default)]
    control_refs: Vec<String>,
    #[serde(default)]
    acknowledge_operation_refs: Vec<String>,
    #[serde(default)]
    shared_group: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuDmaTopology {
    #[serde(default)]
    channels: Vec<McuDmaChannel>,
    #[serde(default)]
    routes: Vec<McuDmaRoute>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuDmaChannel {
    id: String,
    name: String,
    controller_ref: String,
    #[serde(default)]
    target_ref: Option<String>,
    channel_index: u32,
    #[serde(default)]
    capabilities: Vec<String>,
    #[serde(default)]
    priority_levels: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuDmaRoute {
    id: String,
    name: String,
    peripheral_ref: String,
    #[serde(default)]
    signal: Option<String>,
    channel_ref: String,
    direction: String,
    #[serde(default)]
    control_refs: Vec<String>,
    #[serde(default)]
    shared_channel_group_ref: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuPinTopology {
    #[serde(default)]
    routes: Vec<McuPinRoute>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuPinRoute {
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    pin_ref: String,
    peripheral_ref: String,
    signal: String,
    route_type: String,
    #[serde(default)]
    control_refs: Vec<String>,
    #[serde(default)]
    electrical_constraint_refs: Vec<String>,
    #[serde(default)]
    conflict_refs: Vec<String>,
    #[serde(default)]
    default_after_reset: Option<bool>,
}

#[derive(Debug, Clone)]
struct ResolvedRegister {
    id: String,
    name: String,
    peripheral_ref: String,
    absolute_address: u64,
    width_bits: u32,
    fields: Vec<ResolvedField>,
}

#[derive(Debug, Clone)]
struct ResolvedField {
    id: String,
    name: String,
    description: Option<String>,
    lsb: u32,
    msb: u32,
}

#[derive(Debug, Clone)]
struct ResolvedFieldTarget {
    peripheral_ref: String,
    register_id: String,
    field: ResolvedField,
}

#[derive(Debug)]
struct GeneratedMethod {
    name: String,
    code: String,
}

#[derive(Debug, Clone)]
struct ResolvedIndexedFieldGpioPinLowering {
    role_index: usize,
    pin_name: String,
    accessor_name: String,
    moder_clear_mask: u32,
    moder_output_mask: u32,
    pupdr_clear_mask: u32,
    pupdr_up_mask: u32,
    pupdr_down_mask: u32,
    idr_mask: u32,
    odr_mask: u32,
    bsrr_set_mask: u32,
    bsrr_reset_mask: u32,
}

#[derive(Debug, Clone)]
struct ResolvedWchCfgGpioPinLowering {
    role_index: usize,
    pin_name: String,
    accessor_name: String,
    cfg_addr: u64,
    cfg_clear_mask: u32,
    cfg_input_float_mask: u32,
    cfg_input_pull_mask: u32,
    cfg_output_mask: u32,
    idr_mask: u32,
    odr_mask: u32,
}

#[derive(Debug, Clone)]
struct ResolvedBitmaskGpioPinLowering {
    role_index: usize,
    pin_name: String,
    accessor_name: String,
    bit_mask: u32,
    data_alias_address: u64,
}

#[derive(Debug, Clone)]
struct ResolvedEspGpioPinLowering {
    role_index: usize,
    pin_name: String,
    accessor_name: String,
    bit_mask: u32,
    out_sel_cfg_addr: u64,
    out_sel_clear_mask: u32,
    out_sel_gpio_mask: u32,
    inv_sel_mask: u32,
    oen_sel_mask: u32,
    oen_inv_sel_mask: u32,
    io_mux_addr: u64,
    mcu_sel_mask: u32,
    fun_wpd_mask: u32,
    fun_wpu_mask: u32,
    fun_ie_mask: u32,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
enum ResolvedGpioPortLowering {
    IndexedFields {
        moder: ResolvedRegister,
        pupdr: ResolvedRegister,
        idr: ResolvedRegister,
        odr: ResolvedRegister,
        bsrr: ResolvedRegister,
        pins: Vec<ResolvedIndexedFieldGpioPinLowering>,
    },
    WchCfgRegisters {
        idr: ResolvedRegister,
        odr: ResolvedRegister,
        pins: Vec<ResolvedWchCfgGpioPinLowering>,
    },
    BitmaskRegisters {
        dir: ResolvedRegister,
        afsel: ResolvedRegister,
        den: ResolvedRegister,
        pur: ResolvedRegister,
        pdr: ResolvedRegister,
        pins: Vec<ResolvedBitmaskGpioPinLowering>,
    },
    EspRegisters {
        out: ResolvedRegister,
        out_w1ts: ResolvedRegister,
        out_w1tc: ResolvedRegister,
        enable: ResolvedRegister,
        enable_w1ts: ResolvedRegister,
        enable_w1tc: ResolvedRegister,
        input: ResolvedRegister,
        pins: Vec<ResolvedEspGpioPinLowering>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmbassyHalProfile {
    #[serde(rename = "crate")]
    crate_info: EmbassyCrateMetadata,
    driver_instances: Vec<EmbassyDriverInstance>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmbassyCrateMetadata {
    package_name: String,
    crate_name: String,
    #[serde(default)]
    target_architecture: Option<String>,
    #[serde(default)]
    feature_flags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmbassyTimeDriverBindings {
    counter_ref: String,
    alarm_ref: String,
    #[serde(default)]
    alarm_apply_operation_refs: Vec<String>,
    interrupt_enable_ref: String,
    interrupt_pending_ref: String,
    interrupt_clear_operation_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmbassyDriverInstance {
    id: String,
    name: String,
    target_ref: String,
    driver_kind: String,
    module_path: String,
    #[serde(default)]
    lowering_pattern: Option<String>,
    #[serde(default)]
    time_driver_source: Option<String>,
    #[serde(default)]
    time_driver_tick_hz: Option<u64>,
    #[serde(default)]
    time_driver_bindings: Option<EmbassyTimeDriverBindings>,
    #[serde(default)]
    clock_binding_refs: Vec<String>,
    #[serde(default)]
    reset_binding_refs: Vec<String>,
    #[serde(default)]
    interrupt_route_refs: Vec<String>,
    #[serde(default)]
    dma_route_refs: Vec<String>,
    #[serde(default)]
    pin_roles: Vec<EmbassyPinRole>,
    #[serde(default)]
    init_operation_refs: Vec<String>,
    #[serde(default)]
    state_machine_refs: Vec<String>,
    #[serde(default)]
    capability_tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmbassyPinRole {
    role: String,
    signal: String,
    route_refs: Vec<String>,
    requirement: ResourceRequirement,
}

#[derive(Debug, Clone)]
struct ResolvedEmbassyPinRole {
    role: String,
    signal: String,
    requirement: ResourceRequirement,
    routes: Vec<McuPinRoute>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ResourceRequirement {
    Required,
    Optional,
    MutuallyExclusive,
}

#[derive(Debug, Clone)]
struct ResolvedDriverInstance {
    id: String,
    name: String,
    type_name: String,
    driver_kind: String,
    module_name: String,
    lowering_pattern: Option<String>,
    time_driver_source: Option<String>,
    time_driver_tick_hz: Option<u64>,
    time_driver_bindings: Option<EmbassyTimeDriverBindings>,
    target: McuCanonicalBlock,
    clock_bindings: Vec<McuClockBinding>,
    reset_bindings: Vec<McuResetBinding>,
    interrupt_sources: Vec<McuInterruptSource>,
    interrupt_routes: Vec<McuInterruptRoute>,
    dma_channels: Vec<McuDmaChannel>,
    dma_routes: Vec<McuDmaRoute>,
    pin_roles: Vec<ResolvedEmbassyPinRole>,
    init_operations: Vec<SemanticOperation>,
    state_machines: Vec<SemanticStateMachine>,
    capability_tags: Vec<String>,
}

struct DriverRequirementInputs<'a> {
    clock_bindings: &'a [McuClockBinding],
    reset_bindings: &'a [McuResetBinding],
    interrupt_routes: &'a [McuInterruptRoute],
    dma_routes: &'a [McuDmaRoute],
    pin_roles: &'a [ResolvedEmbassyPinRole],
    init_operations: &'a [SemanticOperation],
    state_machines: &'a [SemanticStateMachine],
}

struct DriverResourceScopeInputs<'a> {
    clock_bindings: &'a [McuClockBinding],
    reset_bindings: &'a [McuResetBinding],
    interrupt_routes: &'a [McuInterruptRoute],
    interrupt_sources: &'a HashMap<&'a str, McuInterruptSource>,
    dma_routes: &'a [McuDmaRoute],
    dma_channels: &'a HashMap<&'a str, McuDmaChannel>,
    pin_roles: &'a [ResolvedEmbassyPinRole],
}

const EMBASSY_TIME_DRIVER_TAG: &str = "embassy-time-driver";
const EMBASSY_TIME_DRIVER_SOURCE_SYSTICK: &str = "systick";
const EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER: &str = "hardware-timer";
const TIMER_LOWERING_COUNTER_COMPARE: &str = "counter-compare-timer";
const USB_LOWERING_SERIAL_JTAG_PRESERVE_LINK: &str = "serial-jtag-preserve-link";
const USB_PRESERVE_LINK_OPERATION_ID: &str = "op.usb_device.preserve_serial_jtag_link";

fn has_time_driver_tag(tags: &[String]) -> bool {
    tags.iter().any(|tag| tag == EMBASSY_TIME_DRIVER_TAG)
}

fn time_driver_source(driver: &ResolvedDriverInstance) -> Option<&str> {
    driver.time_driver_source.as_deref()
}

fn time_driver_tick_hz(driver: &ResolvedDriverInstance) -> Option<u64> {
    driver.time_driver_tick_hz
}

fn time_driver_bindings(driver: &ResolvedDriverInstance) -> Option<&EmbassyTimeDriverBindings> {
    driver.time_driver_bindings.as_ref()
}

fn has_usb_preserve_link_lowering(pattern: Option<&str>) -> bool {
    matches!(pattern, Some(USB_LOWERING_SERIAL_JTAG_PRESERVE_LINK))
}

fn has_counter_compare_timer_lowering(pattern: Option<&str>) -> bool {
    matches!(pattern, Some(TIMER_LOWERING_COUNTER_COMPARE))
}

fn driver_uses_usb_preserve_link_lowering(driver: &ResolvedDriverInstance) -> bool {
    has_usb_preserve_link_lowering(driver.lowering_pattern.as_deref())
}

fn driver_uses_counter_compare_timer_lowering(driver: &ResolvedDriverInstance) -> bool {
    has_counter_compare_timer_lowering(driver.lowering_pattern.as_deref())
}

fn is_systick_interrupt_ref(interrupt_ref: &str) -> bool {
    interrupt_ref.eq_ignore_ascii_case("int.systick")
        || interrupt_ref.eq_ignore_ascii_case("irq.systick")
        || interrupt_ref
            .rsplit('.')
            .next()
            .is_some_and(|segment| segment.eq_ignore_ascii_case("systick"))
}

#[allow(dead_code)]
#[derive(Debug)]
struct EmbassyGenerationModel {
    crate_info: EmbassyCrateMetadata,
    document_title: String,
    document_version: String,
    device_name: String,
    target_architecture: Option<String>,
    feature_flags: Vec<String>,
    interrupts: Vec<Interrupt>,
    pins: Vec<PhysicalPin>,
    drivers: Vec<ResolvedDriverInstance>,
    operations: HashMap<String, SemanticOperation>,
    peripherals: HashMap<String, Peripheral>,
    peripheral_names: HashMap<String, String>,
    register_owners: HashMap<String, Vec<String>>,
    registers: HashMap<String, ResolvedRegister>,
    fields: HashMap<String, Vec<ResolvedFieldTarget>>,
    canonical_terms_by_target: HashMap<String, BTreeSet<String>>,
    provenance: HairProvenance,
}

impl EmbassyGenerationModel {
    fn from_document(document: &HairDocument) -> Result<Self> {
        let profiles = &document.profiles;
        let embassy = profiles
            .embassy_hal
            .as_ref()
            .ok_or_else(|| anyhow!("input document is missing profiles.embassyHal"))?;
        validate_embassy_crate_metadata(&embassy.crate_info)?;
        let mcu = profiles
            .mcu_soc
            .as_ref()
            .ok_or_else(|| anyhow!("input document is missing profiles.mcuSoc"))?;

        let canonical_blocks =
            collect_unique_map(&mcu.canonical_blocks, "canonical block", |item| {
                item.id.as_str()
            })?;
        let clock_bindings = mcu
            .clock_reset_topology
            .as_ref()
            .map(|topology| {
                collect_unique_map(&topology.clock_bindings, "clock binding", |item| {
                    item.id.as_str()
                })
            })
            .transpose()?
            .unwrap_or_default();
        let reset_bindings = mcu
            .clock_reset_topology
            .as_ref()
            .map(|topology| {
                collect_unique_map(&topology.reset_bindings, "reset binding", |item| {
                    item.id.as_str()
                })
            })
            .transpose()?
            .unwrap_or_default();
        let interrupt_routes = mcu
            .interrupt_topology
            .as_ref()
            .map(|topology| {
                collect_unique_map(&topology.routes, "interrupt route", |item| item.id.as_str())
            })
            .transpose()?
            .unwrap_or_default();
        let interrupt_sources = mcu
            .interrupt_topology
            .as_ref()
            .map(|topology| {
                collect_unique_map(&topology.sources, "interrupt source", |item| {
                    item.id.as_str()
                })
            })
            .transpose()?
            .unwrap_or_default();
        let dma_routes = mcu
            .dma_topology
            .as_ref()
            .map(|topology| {
                collect_unique_map(&topology.routes, "DMA route", |item| item.id.as_str())
            })
            .transpose()?
            .unwrap_or_default();
        let dma_channels = mcu
            .dma_topology
            .as_ref()
            .map(|topology| {
                collect_unique_map(&topology.channels, "DMA channel", |item| item.id.as_str())
            })
            .transpose()?
            .unwrap_or_default();
        let pin_routes = mcu
            .pin_topology
            .as_ref()
            .map(|topology| {
                collect_unique_map(&topology.routes, "pin route", |item| item.id.as_str())
            })
            .transpose()?
            .unwrap_or_default();
        let operations = document.semantics.operations.as_slice();
        let operations = collect_unique_map(operations, "operation", |item| item.id.as_str())?;
        let state_machines = document.semantics.state_machines.as_slice();
        let state_machines =
            collect_unique_map(state_machines, "state machine", |item| item.id.as_str())?;
        let device_interrupts = document.structure.device.interrupts.as_slice();
        let device_interrupts =
            collect_unique_map(device_interrupts, "device interrupt", |item| {
                item.id.as_str()
            })?;
        let peripheral_names = document
            .structure
            .device
            .peripherals
            .iter()
            .map(|peripheral| (peripheral.id.clone(), peripheral.name.clone()))
            .collect::<HashMap<_, _>>();
        let peripherals = document
            .structure
            .device
            .peripherals
            .iter()
            .map(|peripheral| (peripheral.id.clone(), peripheral.clone()))
            .collect::<HashMap<_, _>>();
        let register_owners = collect_register_owner_map(&document.structure.device.peripherals)?;
        let register_scope = EmbassyRegisterScopeInputs {
            clock_bindings: &clock_bindings,
            reset_bindings: &reset_bindings,
            interrupt_routes: &interrupt_routes,
            dma_routes: &dma_routes,
            pin_routes: &pin_routes,
            operations: &operations,
            state_machines: &state_machines,
            register_owners: &register_owners,
        };
        let required_peripherals =
            collect_embassy_register_scope(&embassy.driver_instances, &register_scope)?;
        let required_fields =
            collect_embassy_field_scope(&embassy.driver_instances, &operations, &state_machines)?;
        let registers = collect_register_map(
            &document.structure.device.peripherals,
            &required_peripherals,
        )?;
        let fields = collect_field_map(&registers, &required_fields)?;
        let mut canonical_terms_by_target = HashMap::<String, BTreeSet<String>>::new();
        for mapping in &document.normalization.mappings {
            if mapping.canonical_term_refs.is_empty() {
                continue;
            }
            canonical_terms_by_target
                .entry(mapping.target_ref.clone())
                .or_default()
                .extend(mapping.canonical_term_refs.iter().cloned());
        }

        let mut drivers = Vec::with_capacity(embassy.driver_instances.len());
        for driver in &embassy.driver_instances {
            validate_supported_driver_kind(&driver.driver_kind)?;
            let module_name = validate_module_name(&driver.module_path)?;
            validate_driver_module_scope(driver, &module_name)?;
            let target = canonical_blocks
                .get(driver.target_ref.as_str())
                .cloned()
                .ok_or_else(|| {
                    anyhow!(
                        "driver {} references unknown canonical block {}",
                        driver.id,
                        driver.target_ref
                    )
                })?;
            let clock_bindings = resolve_ref_list(
                &driver.clock_binding_refs,
                &clock_bindings,
                "clock binding",
                &driver.id,
            )?;
            let reset_bindings = resolve_ref_list(
                &driver.reset_binding_refs,
                &reset_bindings,
                "reset binding",
                &driver.id,
            )?;
            let interrupt_routes = resolve_ref_list(
                &driver.interrupt_route_refs,
                &interrupt_routes,
                "interrupt route",
                &driver.id,
            )?;
            let driver_interrupt_sources = resolve_related_ref_list(
                &interrupt_routes,
                |route| route.source_ref.as_str(),
                &interrupt_sources,
                "interrupt source",
                &driver.id,
            )?;
            let dma_routes =
                resolve_ref_list(&driver.dma_route_refs, &dma_routes, "DMA route", &driver.id)?;
            let driver_dma_channels = resolve_related_ref_list(
                &dma_routes,
                |route| route.channel_ref.as_str(),
                &dma_channels,
                "DMA channel",
                &driver.id,
            )?;
            validate_interrupt_route_targets(driver, &interrupt_routes, &device_interrupts)?;
            validate_dma_route_channels(driver, &dma_routes, &dma_channels)?;
            let mut pin_roles = Vec::with_capacity(driver.pin_roles.len());
            for pin_role in &driver.pin_roles {
                let routes =
                    resolve_ref_list(&pin_role.route_refs, &pin_routes, "pin route", &driver.id)?;
                for route in &routes {
                    if route.signal != pin_role.signal {
                        bail!(
                            "driver {} pin role {} expects signal {} but route {} uses {}",
                            driver.id,
                            pin_role.role,
                            pin_role.signal,
                            route.id,
                            route.signal
                        );
                    }
                }
                pin_roles.push(ResolvedEmbassyPinRole {
                    role: pin_role.role.clone(),
                    signal: pin_role.signal.clone(),
                    requirement: pin_role.requirement.clone(),
                    routes,
                });
            }
            let init_operations = resolve_ref_list(
                &driver.init_operation_refs,
                &operations,
                "operation",
                &driver.id,
            )?;
            let state_machines = resolve_ref_list(
                &driver.state_machine_refs,
                &state_machines,
                "state machine",
                &driver.id,
            )?;
            let resource_scope = DriverResourceScopeInputs {
                clock_bindings: &clock_bindings,
                reset_bindings: &reset_bindings,
                interrupt_routes: &interrupt_routes,
                interrupt_sources: &interrupt_sources,
                dma_routes: &dma_routes,
                dma_channels: &dma_channels,
                pin_roles: &pin_roles,
            };
            validate_driver_resource_scope(driver, &target, &resource_scope)?;
            validate_driver_semantic_scope(
                driver,
                &target,
                &init_operations,
                &state_machines,
                &registers,
                &fields,
            )?;

            let requirements = DriverRequirementInputs {
                clock_bindings: &clock_bindings,
                reset_bindings: &reset_bindings,
                interrupt_routes: &interrupt_routes,
                dma_routes: &dma_routes,
                pin_roles: &pin_roles,
                init_operations: &init_operations,
                state_machines: &state_machines,
            };
            validate_driver_requirements(driver, &requirements)?;
            validate_driver_lowering_pattern(driver, &init_operations)?;

            drivers.push(ResolvedDriverInstance {
                id: driver.id.clone(),
                name: driver.name.clone(),
                type_name: to_rust_type_name(&driver.name),
                driver_kind: driver.driver_kind.clone(),
                module_name,
                lowering_pattern: driver.lowering_pattern.clone(),
                time_driver_source: driver.time_driver_source.clone(),
                time_driver_tick_hz: driver.time_driver_tick_hz,
                time_driver_bindings: driver.time_driver_bindings.clone(),
                target,
                clock_bindings,
                reset_bindings,
                interrupt_sources: driver_interrupt_sources,
                interrupt_routes,
                dma_channels: driver_dma_channels,
                dma_routes,
                pin_roles,
                init_operations,
                state_machines,
                capability_tags: driver.capability_tags.clone(),
            });
        }

        validate_driver_name_collisions(&drivers)?;
        validate_capability_tag_contracts(&drivers)?;
        validate_interrupt_name_collisions(&document.structure.device.interrupts)?;

        Ok(Self {
            crate_info: embassy.crate_info.clone(),
            document_title: document.metadata.title.clone(),
            document_version: document.metadata.version.clone(),
            device_name: document.structure.device.name.clone(),
            target_architecture: embassy.crate_info.target_architecture.clone(),
            feature_flags: embassy.crate_info.feature_flags.clone(),
            interrupts: document.structure.device.interrupts.clone(),
            pins: document.physical.pins.clone(),
            drivers,
            operations: operations
                .iter()
                .map(|(id, operation)| ((*id).to_string(), operation.clone()))
                .collect(),
            peripherals,
            peripheral_names,
            register_owners,
            registers,
            fields,
            canonical_terms_by_target,
            provenance: document.provenance.clone(),
        })
    }

    fn module_names(&self) -> Vec<String> {
        let mut names = self
            .drivers
            .iter()
            .map(|driver| driver.module_name.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let has_interrupt_driver = self
            .drivers
            .iter()
            .any(|driver| driver.driver_kind == "interrupt");
        if (has_interrupt_driver || !self.interrupts.is_empty())
            && !names.iter().any(|name| name == "interrupt")
        {
            names.push("interrupt".to_string());
        }
        names.insert(0, "metadata".to_string());
        names
    }

    fn driver_groups(&self) -> Vec<(String, Vec<&ResolvedDriverInstance>)> {
        let mut groups = BTreeSet::<String>::new();
        for driver in &self.drivers {
            groups.insert(driver.module_name.clone());
        }
        if self
            .drivers
            .iter()
            .any(|driver| driver.driver_kind == "interrupt")
            || !self.interrupts.is_empty()
        {
            groups.insert("interrupt".to_string());
        }

        groups
            .into_iter()
            .map(|module_name| {
                let members = self
                    .drivers
                    .iter()
                    .filter(|driver| driver.module_name == module_name)
                    .collect::<Vec<_>>();
                (module_name, members)
            })
            .collect()
    }
}

fn validate_supported_driver_kind(driver_kind: &str) -> Result<()> {
    match driver_kind {
        "rcc" | "gpio-port" | "uart" | "usart" | "spi" | "i2c" | "timer" | "pwm" | "adc"
        | "dma" | "interrupt" | "usb-device" => Ok(()),
        "custom" => bail!("driver kind custom is outside the supported first-cut Embassy subset"),
        other => bail!("driver kind {other} is not supported by the first-cut Embassy generator"),
    }
}

fn validate_module_name(module_path: &str) -> Result<String> {
    if module_path.is_empty() {
        bail!("modulePath must not be empty");
    }
    if module_path.contains("::") || module_path.contains('/') || module_path.contains('\\') {
        bail!(
            "first-cut Embassy generation only supports simple modulePath values, got {module_path}"
        );
    }

    let normalized = module_path
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    if normalized
        .chars()
        .next()
        .map(|ch| ch.is_ascii_alphabetic() || ch == '_')
        != Some(true)
    {
        bail!("modulePath {module_path} does not normalize to a valid Rust module name");
    }
    if is_rust_keyword(&normalized) {
        bail!("modulePath {module_path} normalizes to reserved Rust keyword {normalized}");
    }
    if normalized == "metadata" {
        bail!("modulePath {module_path} normalizes to reserved generated module name metadata");
    }
    Ok(normalized)
}

fn validate_reserved_generated_module_names(
    model: &EmbassyGenerationModel,
    reserved: &[(&str, &str)],
) -> Result<()> {
    for module_name in model.module_names() {
        if let Some((_, message)) = reserved
            .iter()
            .find(|(reserved_name, _)| module_name == *reserved_name)
        {
            bail!("modulePath {module_name} normalizes to {message}");
        }
    }
    Ok(())
}

fn render_module_provenance_const(module_name: &str) -> String {
    format!(
        "pub const MODULE_PROVENANCE: metadata::ModuleProvenance = metadata::ModuleProvenance {{\n    module_name: {},\n    document_title: metadata::GENERATED_METADATA.document_title,\n    document_version: metadata::GENERATED_METADATA.document_version,\n    source_ids: metadata::GENERATED_PROVENANCE_SOURCE_IDS,\n    evidence_ids: metadata::GENERATED_PROVENANCE_EVIDENCE_IDS,\n}};\n\n",
        render_rust_string(module_name)
    )
}

fn validate_embassy_crate_metadata(crate_info: &EmbassyCrateMetadata) -> Result<()> {
    validate_cargo_package_name(&crate_info.package_name)?;
    validate_rust_identifier(&crate_info.crate_name, "crateName")?;
    Ok(())
}

fn validate_cargo_package_name(package_name: &str) -> Result<()> {
    if package_name.is_empty() {
        bail!("packageName must not be empty");
    }
    if !package_name
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    {
        bail!("packageName {package_name} is not a valid first-cut Cargo package name");
    }
    Ok(())
}

fn validate_rust_identifier(identifier: &str, field_name: &str) -> Result<()> {
    if identifier.is_empty() {
        bail!("{field_name} must not be empty");
    }
    let mut chars = identifier.chars();
    let first = chars
        .next()
        .ok_or_else(|| anyhow!("{field_name} must not be empty"))?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        bail!("{field_name} {identifier} is not a valid Rust identifier");
    }
    if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
        bail!("{field_name} {identifier} is not a valid Rust identifier");
    }
    if is_rust_keyword(identifier) {
        bail!("{field_name} {identifier} is a reserved Rust keyword");
    }
    Ok(())
}

fn is_rust_keyword(identifier: &str) -> bool {
    matches!(
        identifier,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "gen"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "union"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

fn validate_driver_name_collisions(drivers: &[ResolvedDriverInstance]) -> Result<()> {
    let mut type_names = HashMap::<(&str, &str), &str>::new();
    let mut const_names = HashMap::<(&str, String), &str>::new();
    for driver in drivers {
        validate_generated_type_name(
            &driver.type_name,
            "driver name",
            &driver.name,
            &driver.id,
            "driver type",
        )?;
        let type_key = (driver.module_name.as_str(), driver.type_name.as_str());
        if let Some(previous) = type_names.insert(type_key, driver.id.as_str()) {
            bail!(
                "drivers {previous} and {} normalize to the same Rust type {} in module {}",
                driver.id,
                driver.type_name,
                driver.module_name
            );
        }

        let const_name = to_rust_const_name(&driver.id);
        let const_key = (driver.module_name.as_str(), const_name.clone());
        if let Some(previous) = const_names.insert(const_key, driver.id.as_str()) {
            bail!(
                "drivers {previous} and {} normalize to the same Rust constant {} in module {}",
                driver.id,
                const_name,
                driver.module_name
            );
        }
    }
    Ok(())
}

fn validate_host_accessor_name_collisions(drivers: &[ResolvedDriverInstance]) -> Result<()> {
    let mut method_names = HashMap::<String, String>::from([
        (
            "new".to_string(),
            "built-in HostEmulator method new".to_string(),
        ),
        (
            "activate".to_string(),
            "built-in HostEmulator method activate".to_string(),
        ),
        (
            "activate_scoped".to_string(),
            "built-in HostEmulator method activate_scoped".to_string(),
        ),
        (
            "clear_active".to_string(),
            "built-in HostEmulator method clear_active".to_string(),
        ),
        (
            "state".to_string(),
            "built-in HostEmulator method state".to_string(),
        ),
    ]);

    for driver in drivers {
        let accessor = to_rust_method_name(&driver.name);
        if is_rust_keyword(&accessor) {
            bail!(
                "driver {} normalizes to reserved Rust keyword {} as HostEmulator method",
                driver.id,
                accessor
            );
        }
        let accessor_owner = format!("driver {} accessor", driver.id);
        if let Some(previous) = method_names.insert(accessor.clone(), accessor_owner) {
            bail!(
                "{} collides with {} as HostEmulator method {}",
                driver.id,
                previous,
                accessor
            );
        }

        let emulator_accessor = format!("{accessor}_emulator");
        let emulator_owner = format!("driver {} emulator accessor", driver.id);
        if let Some(previous) = method_names.insert(emulator_accessor.clone(), emulator_owner) {
            bail!(
                "{} collides with {} as HostEmulator method {}",
                driver.id,
                previous,
                emulator_accessor
            );
        }
    }

    Ok(())
}

fn validate_interrupt_name_collisions(interrupts: &[Interrupt]) -> Result<()> {
    let mut variants = HashMap::<String, &str>::new();
    for interrupt in interrupts {
        let variant = to_rust_type_name(&interrupt.name);
        validate_generated_type_name(
            &variant,
            "interrupt name",
            &interrupt.name,
            &interrupt.id,
            "interrupt variant",
        )?;
        if let Some(previous) = variants.insert(variant.clone(), interrupt.id.as_str()) {
            bail!(
                "interrupts {previous} and {} normalize to the same Rust enum variant {}",
                interrupt.id,
                variant
            );
        }
    }
    Ok(())
}

fn validate_driver_module_scope(driver: &EmbassyDriverInstance, module_name: &str) -> Result<()> {
    if module_name == "interrupt" && driver.driver_kind != "interrupt" {
        bail!(
            "driver {} uses reserved interrupt modulePath {} but has driver kind {}",
            driver.id,
            driver.module_path,
            driver.driver_kind
        );
    }
    if module_name == "time" && !has_time_driver_tag(&driver.capability_tags) {
        bail!(
            "driver {} uses reserved time modulePath {} without capability tag {}",
            driver.id,
            driver.module_path,
            EMBASSY_TIME_DRIVER_TAG
        );
    }
    if driver.time_driver_source.is_some() && !has_time_driver_tag(&driver.capability_tags) {
        bail!(
            "driver {} declares timeDriverSource without capability tag {}",
            driver.id,
            EMBASSY_TIME_DRIVER_TAG
        );
    }
    if driver.time_driver_tick_hz.is_some() && !has_time_driver_tag(&driver.capability_tags) {
        bail!(
            "driver {} declares timeDriverTickHz without capability tag {}",
            driver.id,
            EMBASSY_TIME_DRIVER_TAG
        );
    }
    if driver.time_driver_bindings.is_some() && !has_time_driver_tag(&driver.capability_tags) {
        bail!(
            "driver {} declares timeDriverBindings without capability tag {}",
            driver.id,
            EMBASSY_TIME_DRIVER_TAG
        );
    }
    Ok(())
}

fn validate_capability_tag_contracts(drivers: &[ResolvedDriverInstance]) -> Result<()> {
    let time_drivers = drivers
        .iter()
        .filter(|driver| has_time_driver_tag(&driver.capability_tags))
        .collect::<Vec<_>>();
    if time_drivers.len() > 1 {
        let ids = time_drivers
            .iter()
            .map(|driver| driver.id.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "drivers {} all claim capability tag {}; exactly one generated async timing provider is allowed",
            ids,
            EMBASSY_TIME_DRIVER_TAG
        );
    }
    if let Some(driver) = time_drivers.first() {
        let source = time_driver_source(driver).ok_or_else(|| {
            anyhow!(
                "driver {} claims capability tag {} but omits required timeDriverSource",
                driver.id,
                EMBASSY_TIME_DRIVER_TAG
            )
        })?;
        match source {
            EMBASSY_TIME_DRIVER_SOURCE_SYSTICK => {
                if driver.time_driver_tick_hz.is_some() {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} and must not declare timeDriverTickHz; SysTick timing defines its own tick rate",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source
                    );
                }
                if driver.time_driver_bindings.is_some() {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} and must not declare timeDriverBindings",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source
                    );
                }
                if driver.driver_kind != "interrupt" {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but has unsupported driver kind {}; SysTick timing requires interrupt lowering",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source,
                        driver.driver_kind
                    );
                }
                if driver.target.block_class != "interrupt-controller" {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {}, but target block {} is not an interrupt-controller",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source,
                        driver.target.id
                    );
                }
                if driver.interrupt_routes.len() != 1 {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but must reference exactly one SysTick interrupt route",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source
                    );
                }
                let route = &driver.interrupt_routes[0];
                if !is_systick_interrupt_ref(&route.interrupt_ref) {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but route {} targets {}; SysTick timing requires the SysTick exception",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source,
                        route.id,
                        route.interrupt_ref
                    );
                }
            }
            EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER => {
                let tick_hz = driver.time_driver_tick_hz.ok_or_else(|| {
                    anyhow!(
                        "driver {} claims capability tag {} with timeDriverSource {} but omits required timeDriverTickHz",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source
                    )
                })?;
                if tick_hz == 0 {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but timeDriverTickHz must be greater than zero",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source
                    );
                }
                if driver.driver_kind != "timer" {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but has unsupported driver kind {}; hardware-timer timing requires a timer driver",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source,
                        driver.driver_kind
                    );
                }
                if driver.target.block_class != "timer-general" {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but target block {} is not timer-general",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source,
                        driver.target.id
                    );
                }
                if driver.interrupt_routes.len() != 1 || driver.interrupt_sources.len() != 1 {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but must reference exactly one timer interrupt route/source pair",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source
                    );
                }
                let interrupt_source = &driver.interrupt_sources[0];
                if interrupt_source.kind != "timer" {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but interrupt source {} is kind {} instead of timer",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source,
                        interrupt_source.id,
                        interrupt_source.kind
                    );
                }
                if interrupt_source.clear_operation_refs.len() != 1 {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but interrupt source {} must expose exactly one clear operation",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source,
                        interrupt_source.id
                    );
                }
                if driver.clock_bindings.len() > 1 {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but first-cut hardware-timer timing supports at most one clock binding",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source
                    );
                }
                if driver.reset_bindings.len() > 1 {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} but first-cut hardware-timer timing supports at most one reset binding",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source
                    );
                }
                if driver_uses_counter_compare_timer_lowering(driver)
                    && time_driver_bindings(driver).is_none()
                {
                    bail!(
                        "driver {} claims capability tag {} with timeDriverSource {} and loweringPattern {} but omits required timeDriverBindings",
                        driver.id,
                        EMBASSY_TIME_DRIVER_TAG,
                        source,
                        TIMER_LOWERING_COUNTER_COMPARE
                    );
                }
            }
            other => {
                bail!(
                    "driver {} claims capability tag {} with unsupported timeDriverSource {}",
                    driver.id,
                    EMBASSY_TIME_DRIVER_TAG,
                    other
                );
            }
        }
    }
    Ok(())
}

fn validate_generated_type_name(
    generated_name: &str,
    source_kind: &str,
    source_name: &str,
    owner_id: &str,
    generated_kind: &str,
) -> Result<()> {
    if is_rust_keyword(generated_name) {
        bail!(
            "{} {} on {} normalizes to reserved Rust keyword {} for {}",
            source_kind,
            source_name,
            owner_id,
            generated_name,
            generated_kind
        );
    }
    Ok(())
}

fn validate_interrupt_route_targets(
    driver: &EmbassyDriverInstance,
    interrupt_routes: &[McuInterruptRoute],
    device_interrupts: &HashMap<&str, Interrupt>,
) -> Result<()> {
    for route in interrupt_routes {
        if !device_interrupts.contains_key(route.interrupt_ref.as_str()) {
            bail!(
                "driver {} interrupt route {} references unknown device interrupt {}",
                driver.id,
                route.id,
                route.interrupt_ref
            );
        }
    }
    Ok(())
}

fn validate_dma_route_channels(
    driver: &EmbassyDriverInstance,
    dma_routes: &[McuDmaRoute],
    dma_channels: &HashMap<&str, McuDmaChannel>,
) -> Result<()> {
    for route in dma_routes {
        if !dma_channels.contains_key(route.channel_ref.as_str()) {
            bail!(
                "driver {} DMA route {} references unknown channel {}",
                driver.id,
                route.id,
                route.channel_ref
            );
        }
    }
    Ok(())
}

fn validate_driver_resource_scope(
    driver: &EmbassyDriverInstance,
    target: &McuCanonicalBlock,
    resources: &DriverResourceScopeInputs<'_>,
) -> Result<()> {
    match driver.driver_kind.as_str() {
        "uart" | "usart" | "spi" | "i2c" | "timer" | "pwm" | "adc" | "gpio-port" | "usb-device" => {
            let target_ref = target.target_ref.as_str();
            for binding in resources.clock_bindings {
                if binding.consumer_ref != target_ref {
                    bail!(
                        "driver {} references clock binding {} for {} instead of {}",
                        driver.id,
                        binding.id,
                        binding.consumer_ref,
                        target_ref
                    );
                }
            }
            for binding in resources.reset_bindings {
                if binding.target_ref != target_ref {
                    bail!(
                        "driver {} references reset binding {} for {} instead of {}",
                        driver.id,
                        binding.id,
                        binding.target_ref,
                        target_ref
                    );
                }
            }
            for route in resources.interrupt_routes {
                let source = resources
                    .interrupt_sources
                    .get(route.source_ref.as_str())
                    .ok_or_else(|| {
                        anyhow!(
                            "driver {} interrupt route {} references unknown source {}",
                            driver.id,
                            route.id,
                            route.source_ref
                        )
                    })?;
                if source.source_ref != target_ref {
                    bail!(
                        "driver {} references interrupt route {} for {} instead of {}",
                        driver.id,
                        route.id,
                        source.source_ref,
                        target_ref
                    );
                }
            }
            for route in resources.dma_routes {
                if route.peripheral_ref != target_ref {
                    bail!(
                        "driver {} references DMA route {} for {} instead of {}",
                        driver.id,
                        route.id,
                        route.peripheral_ref,
                        target_ref
                    );
                }
            }
            for pin_role in resources.pin_roles {
                for route in &pin_role.routes {
                    if route.peripheral_ref != target_ref {
                        bail!(
                            "driver {} pin role {} references pin route {} for {} instead of {}",
                            driver.id,
                            pin_role.role,
                            route.id,
                            route.peripheral_ref,
                            target_ref
                        );
                    }
                }
            }
        }
        "interrupt" => {
            let target_ref = driver.target_ref.as_str();
            for route in resources.interrupt_routes {
                if route.controller_ref != target_ref {
                    bail!(
                        "driver {} references interrupt route {} for controller {} instead of {}",
                        driver.id,
                        route.id,
                        route.controller_ref,
                        target_ref
                    );
                }
            }
        }
        "dma" => {
            let target_ref = driver.target_ref.as_str();
            for route in resources.dma_routes {
                let channel = resources
                    .dma_channels
                    .get(route.channel_ref.as_str())
                    .ok_or_else(|| {
                        anyhow!(
                            "driver {} DMA route {} references unknown channel {}",
                            driver.id,
                            route.id,
                            route.channel_ref
                        )
                    })?;
                if channel.controller_ref != target_ref {
                    bail!(
                        "driver {} references DMA route {} through controller {} instead of {}",
                        driver.id,
                        route.id,
                        channel.controller_ref,
                        target_ref
                    );
                }
            }
        }
        "rcc" => {}
        _ => {}
    }

    Ok(())
}

fn validate_driver_semantic_scope(
    driver: &EmbassyDriverInstance,
    target: &McuCanonicalBlock,
    init_operations: &[SemanticOperation],
    state_machines: &[SemanticStateMachine],
    registers: &HashMap<String, ResolvedRegister>,
    fields: &HashMap<String, Vec<ResolvedFieldTarget>>,
) -> Result<()> {
    let target_ref = target.target_ref.as_str();

    for operation in init_operations {
        validate_operation_scope(driver, target_ref, operation, registers, fields)?;
    }
    for state_machine in state_machines {
        validate_state_machine_scope(driver, target_ref, state_machine, registers, fields)?;
    }

    Ok(())
}

fn validate_operation_scope(
    driver: &EmbassyDriverInstance,
    target_ref: &str,
    operation: &SemanticOperation,
    registers: &HashMap<String, ResolvedRegister>,
    fields: &HashMap<String, Vec<ResolvedFieldTarget>>,
) -> Result<()> {
    for operation_target in &operation.target_refs {
        if operation_target != target_ref {
            bail!(
                "driver {} references operation {} for {} instead of {}",
                driver.id,
                operation.id,
                operation_target,
                target_ref
            );
        }
    }

    for predicate in operation
        .preconditions
        .iter()
        .chain(operation.postconditions.iter())
    {
        validate_predicate_scope(
            &format!("operation {}", operation.id),
            driver.id.as_str(),
            target_ref,
            predicate,
            registers,
            fields,
        )?;
    }

    for step in &operation.steps {
        let Some(step_target_ref) = step.target_ref.as_deref() else {
            continue;
        };
        let register = registers.get(step_target_ref).ok_or_else(|| {
            anyhow!(
                "operation {} step {} references unknown register {}",
                operation.id,
                step.index,
                step_target_ref
            )
        })?;
        if register.peripheral_ref != target_ref {
            bail!(
                "driver {} operation {} step {} references register {} on {} instead of {}",
                driver.id,
                operation.id,
                step.index,
                step_target_ref,
                register.peripheral_ref,
                target_ref
            );
        }
    }

    Ok(())
}

fn validate_state_machine_scope(
    driver: &EmbassyDriverInstance,
    target_ref: &str,
    state_machine: &SemanticStateMachine,
    registers: &HashMap<String, ResolvedRegister>,
    fields: &HashMap<String, Vec<ResolvedFieldTarget>>,
) -> Result<()> {
    for state_machine_target in &state_machine.target_refs {
        if state_machine_target != target_ref {
            bail!(
                "driver {} references state machine {} for {} instead of {}",
                driver.id,
                state_machine.id,
                state_machine_target,
                target_ref
            );
        }
    }

    for state in &state_machine.states {
        for predicate in &state.invariants {
            validate_predicate_scope(
                &format!("state machine {} state {}", state_machine.id, state.name),
                driver.id.as_str(),
                target_ref,
                predicate,
                registers,
                fields,
            )?;
        }
    }

    for transition in &state_machine.transitions {
        for predicate in &transition.conditions {
            validate_predicate_scope(
                &format!(
                    "state machine {} transition {} -> {}",
                    state_machine.id, transition.from, transition.to
                ),
                driver.id.as_str(),
                target_ref,
                predicate,
                registers,
                fields,
            )?;
        }
        for effect in &transition.effects {
            let Some(effect_target_ref) = effect.target_ref.as_deref() else {
                continue;
            };
            if let Some(field_target) =
                try_resolve_field_target(fields, effect_target_ref, Some(target_ref))?
            {
                if field_target.peripheral_ref != target_ref {
                    bail!(
                        "driver {} state machine {} transition {} -> {} references field {} on {} instead of {}",
                        driver.id,
                        state_machine.id,
                        transition.from,
                        transition.to,
                        effect_target_ref,
                        field_target.peripheral_ref,
                        target_ref
                    );
                }
                continue;
            }
            let register = registers.get(effect_target_ref).ok_or_else(|| {
                anyhow!(
                    "state machine {} transition {} -> {} references unknown effect target {}",
                    state_machine.id,
                    transition.from,
                    transition.to,
                    effect_target_ref
                )
            })?;
            if register.peripheral_ref != target_ref {
                bail!(
                    "driver {} state machine {} transition {} -> {} references register {} on {} instead of {}",
                    driver.id,
                    state_machine.id,
                    transition.from,
                    transition.to,
                    effect_target_ref,
                    register.peripheral_ref,
                    target_ref
                );
            }
        }
    }

    Ok(())
}

fn validate_predicate_scope(
    context: &str,
    driver_id: &str,
    target_ref: &str,
    predicate: &SemanticPredicate,
    registers: &HashMap<String, ResolvedRegister>,
    fields: &HashMap<String, Vec<ResolvedFieldTarget>>,
) -> Result<()> {
    let Some(predicate_target_ref) = predicate.target_ref.as_deref() else {
        return Ok(());
    };
    if !predicate_target_ref.starts_with("reg.") && !predicate_target_ref.starts_with("field.") {
        return Ok(());
    }
    if let Some(field_target) =
        try_resolve_field_target(fields, predicate_target_ref, Some(target_ref))?
    {
        if field_target.peripheral_ref != target_ref {
            bail!(
                "driver {} {} predicate {} references field {} on {} instead of {}",
                driver_id,
                context,
                predicate.kind,
                predicate_target_ref,
                field_target.peripheral_ref,
                target_ref
            );
        }
        return Ok(());
    }
    if let Some(register) = registers.get(predicate_target_ref) {
        if register.peripheral_ref != target_ref {
            bail!(
                "driver {} {} predicate {} references register {} on {} instead of {}",
                driver_id,
                context,
                predicate.kind,
                predicate_target_ref,
                register.peripheral_ref,
                target_ref
            );
        }
        return Ok(());
    }
    bail!(
        "driver {} {} predicate {} references unknown target {}",
        driver_id,
        context,
        predicate.kind,
        predicate_target_ref
    );
}

fn collect_unique_map<'a, T: Clone, F>(
    items: &'a [T],
    kind: &str,
    id_fn: F,
) -> Result<HashMap<&'a str, T>>
where
    F: Fn(&'a T) -> &'a str,
{
    let mut map = HashMap::new();
    for item in items {
        let id = id_fn(item);
        if map.insert(id, item.clone()).is_some() {
            bail!("duplicate {kind} id {id} is not supported");
        }
    }
    Ok(map)
}

fn resolve_ref_list<T: Clone>(
    refs: &[String],
    map: &HashMap<&str, T>,
    kind: &str,
    owner_id: &str,
) -> Result<Vec<T>> {
    refs.iter()
        .map(|reference| {
            map.get(reference.as_str()).cloned().ok_or_else(|| {
                anyhow!("{kind} reference {reference} on driver {owner_id} could not be resolved")
            })
        })
        .collect()
}

fn resolve_related_ref_list<TSource, TTarget: Clone, F>(
    sources: &[TSource],
    related_ref: F,
    map: &HashMap<&str, TTarget>,
    kind: &str,
    owner_id: &str,
) -> Result<Vec<TTarget>>
where
    F: Fn(&TSource) -> &str,
{
    let mut resolved = Vec::new();
    let mut seen = HashSet::new();
    for source in sources {
        let reference = related_ref(source);
        if !seen.insert(reference) {
            continue;
        }
        resolved.push(map.get(reference).cloned().ok_or_else(|| {
            anyhow!("{kind} reference {reference} on driver {owner_id} could not be resolved")
        })?);
    }
    Ok(resolved)
}

fn validate_driver_requirements(
    driver: &EmbassyDriverInstance,
    requirements: &DriverRequirementInputs<'_>,
) -> Result<()> {
    match driver.driver_kind.as_str() {
        "rcc" => {
            if requirements.clock_bindings.is_empty()
                && requirements.reset_bindings.is_empty()
                && requirements.init_operations.is_empty()
            {
                bail!(
                    "rcc driver {} requires clock bindings, reset bindings, or init operations",
                    driver.id
                );
            }
        }
        "gpio-port" => {
            if requirements.pin_roles.is_empty() {
                bail!(
                    "gpio-port driver {} requires at least one pin role",
                    driver.id
                );
            }
        }
        "uart" | "usart" => {
            if requirements.pin_roles.is_empty() {
                bail!(
                    "{} driver {} requires pin roles",
                    driver.driver_kind,
                    driver.id
                );
            }
            if !requirements.dma_routes.is_empty() && requirements.interrupt_routes.is_empty() {
                bail!(
                    "{} driver {} requires interrupt routes for async DMA-backed operation",
                    driver.driver_kind,
                    driver.id
                );
            }
        }
        "spi" | "i2c" => {
            if requirements.pin_roles.is_empty() {
                bail!(
                    "{} driver {} requires pin roles",
                    driver.driver_kind,
                    driver.id
                );
            }
            if !requirements.dma_routes.is_empty() && requirements.interrupt_routes.is_empty() {
                bail!(
                    "{} driver {} requires interrupt routes for async DMA-backed operation",
                    driver.driver_kind,
                    driver.id
                );
            }
        }
        "adc" => {
            if !requirements.dma_routes.is_empty() && requirements.interrupt_routes.is_empty() {
                bail!(
                    "{} driver {} requires interrupt routes for async DMA-backed operation",
                    driver.driver_kind,
                    driver.id
                );
            }
            if requirements.init_operations.is_empty() {
                bail!(
                    "adc driver {} requires at least one init operation",
                    driver.id
                );
            }
        }
        "timer" | "pwm" => {
            let is_time_driver = has_time_driver_tag(&driver.capability_tags);
            if requirements.pin_roles.is_empty() && !is_time_driver {
                bail!(
                    "{} driver {} requires pin roles",
                    driver.driver_kind,
                    driver.id
                );
            }
            if requirements.state_machines.is_empty() {
                bail!(
                    "{} driver {} requires at least one state machine",
                    driver.driver_kind,
                    driver.id
                );
            }
            if requirements.init_operations.is_empty() {
                bail!(
                    "{} driver {} requires at least one operation",
                    driver.driver_kind,
                    driver.id
                );
            }
        }
        "usb-device" => {
            let has_dp = requirements.pin_roles.iter().any(|role| {
                role.role.eq_ignore_ascii_case("dp")
                    || role.signal.eq_ignore_ascii_case("dp")
                    || role.signal.eq_ignore_ascii_case("usb_d+")
            });
            let has_dm = requirements.pin_roles.iter().any(|role| {
                role.role.eq_ignore_ascii_case("dm")
                    || role.signal.eq_ignore_ascii_case("dm")
                    || role.signal.eq_ignore_ascii_case("usb_d-")
            });
            if !has_dp || !has_dm {
                bail!(
                    "usb-device driver {} requires both DP and DM pin roles",
                    driver.id
                );
            }
            if requirements.interrupt_routes.is_empty() {
                bail!("usb-device driver {} requires interrupt routes", driver.id);
            }
            if requirements.clock_bindings.is_empty()
                && requirements.reset_bindings.is_empty()
                && requirements.init_operations.is_empty()
                && requirements.state_machines.is_empty()
            {
                bail!(
                    "usb-device driver {} requires clock/reset bindings or semantic init/state-machine evidence",
                    driver.id
                );
            }
        }
        "dma" => {
            if requirements.dma_routes.is_empty() {
                bail!("dma driver {} requires at least one DMA route", driver.id);
            }
        }
        "interrupt" if requirements.interrupt_routes.is_empty() => {
            bail!("interrupt driver {} requires interrupt routes", driver.id);
        }
        "interrupt" => {}
        _ => {}
    }
    Ok(())
}

fn validate_driver_lowering_pattern(
    driver: &EmbassyDriverInstance,
    init_operations: &[SemanticOperation],
) -> Result<()> {
    let Some(pattern) = driver.lowering_pattern.as_deref() else {
        return Ok(());
    };

    match pattern {
        USB_LOWERING_SERIAL_JTAG_PRESERVE_LINK => {
            if driver.driver_kind != "usb-device" {
                bail!(
                    "driver {} uses loweringPattern {} but that lowering is only supported on usb-device drivers",
                    driver.id,
                    pattern
                );
            }
            validate_usb_preserve_link_lowering(driver, init_operations)?
        }
        TIMER_LOWERING_COUNTER_COMPARE => validate_counter_compare_timer_lowering(driver)?,
        other => bail!(
            "driver {} uses unsupported loweringPattern {}",
            driver.id,
            other
        ),
    }
    Ok(())
}

fn validate_counter_compare_timer_lowering(driver: &EmbassyDriverInstance) -> Result<()> {
    if driver.driver_kind != "timer" {
        bail!(
            "driver {} uses loweringPattern {} but that lowering is only supported on timer drivers",
            driver.id,
            TIMER_LOWERING_COUNTER_COMPARE
        );
    }
    if driver.time_driver_source.as_deref() != Some(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER) {
        bail!(
            "driver {} uses loweringPattern {} but must also declare timeDriverSource {}",
            driver.id,
            TIMER_LOWERING_COUNTER_COMPARE,
            EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER
        );
    }
    if driver.time_driver_bindings.is_none() {
        bail!(
            "driver {} uses loweringPattern {} but omits required timeDriverBindings",
            driver.id,
            TIMER_LOWERING_COUNTER_COMPARE
        );
    }
    Ok(())
}

fn validate_usb_preserve_link_lowering(
    driver: &EmbassyDriverInstance,
    init_operations: &[SemanticOperation],
) -> Result<()> {
    let operation = init_operations
        .iter()
        .find(|operation| operation.id == USB_PRESERVE_LINK_OPERATION_ID)
        .ok_or_else(|| {
            anyhow!(
                "usb-device driver {} loweringPattern {} requires init operation {}",
                driver.id,
                USB_LOWERING_SERIAL_JTAG_PRESERVE_LINK,
                USB_PRESERVE_LINK_OPERATION_ID
            )
        })?;

    if operation.steps.is_empty() {
        bail!(
            "operation {} on driver {} must contain explicit preserve-link steps",
            operation.id,
            driver.id
        );
    }

    for step in &operation.steps {
        if step.action != "write" {
            bail!(
                "operation {} on driver {} uses unsupported action {} for loweringPattern {}",
                operation.id,
                driver.id,
                step.action,
                USB_LOWERING_SERIAL_JTAG_PRESERVE_LINK
            );
        }
        let target_ref = step.target_ref.as_deref().ok_or_else(|| {
            anyhow!(
                "operation {} step {} on driver {} is missing targetRef",
                operation.id,
                step.index,
                driver.id
            )
        })?;
        let parsed = parse_field_write_expression(
            step.expression.as_ref(),
            step.value.as_ref(),
            &operation.id,
            step.index,
        )?;
        let register_name = normalize_search_text(last_ref_segment(target_ref));
        let field_name = normalize_search_text(&parsed.field_name);
        let supported = matches!(
            (register_name.as_str(), field_name.as_str(), parsed.value),
            ("INTENA", "SERIALINEMPTYINTENA", 0)
                | ("INTENA", "SERIALOUTRECVPKTINTENA", 0)
                | ("INTCLR", "SERIALINEMPTYINTCLR", 1)
                | ("INTCLR", "SERIALOUTRECVPKTINTCLR", 1)
        );
        if !supported {
            bail!(
                "operation {} step {} on driver {} is not compatible with loweringPattern {}: {} -> {} = {}",
                operation.id,
                step.index,
                driver.id,
                USB_LOWERING_SERIAL_JTAG_PRESERVE_LINK,
                register_name,
                field_name,
                parsed.value
            );
        }
    }

    Ok(())
}

fn to_rust_type_name(text: &str) -> String {
    let mut output = String::new();
    let mut capitalize = true;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            if capitalize {
                output.push(ch.to_ascii_uppercase());
                capitalize = false;
            } else {
                output.push(ch);
            }
        } else {
            capitalize = true;
        }
    }
    if output.is_empty() || !output.chars().next().unwrap_or('A').is_ascii_alphabetic() {
        format!("Generated{}", output)
    } else {
        output
    }
}

fn to_rust_method_name(text: &str) -> String {
    let mut output = String::new();
    let mut last_was_separator = false;
    let chars = text.chars().collect::<Vec<_>>();
    for (index, ch) in chars.iter().copied().enumerate() {
        if ch.is_ascii_alphanumeric() {
            let prev = chars.get(index.wrapping_sub(1)).copied();
            let next = chars.get(index + 1).copied();
            let should_insert_separator = ch.is_ascii_uppercase()
                && !output.is_empty()
                && !last_was_separator
                && prev.is_some_and(|prev| {
                    prev.is_ascii_lowercase()
                        || (prev.is_ascii_uppercase()
                            && next.is_some_and(|next| next.is_ascii_lowercase()))
                });
            if should_insert_separator {
                output.push('_');
            }
            output.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !output.is_empty() && !last_was_separator {
            output.push('_');
            last_was_separator = true;
        }
    }
    while output.ends_with('_') {
        output.pop();
    }
    if output.is_empty() || !output.chars().next().unwrap_or('a').is_ascii_alphabetic() {
        format!("generated_{output}")
    } else {
        output
    }
}

fn render_embassy_cargo_toml(model: &EmbassyGenerationModel) -> String {
    let mut out = format!(
        "[package]\nname = {}\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\nname = {}\npath = \"src/lib.rs\"\n",
        render_rust_string(&model.crate_info.package_name),
        render_rust_string(&model.crate_info.crate_name)
    );
    if model
        .drivers
        .iter()
        .any(|driver| has_time_driver_tag(&driver.capability_tags))
    {
        out.push_str("\n[dependencies]\ncritical-section = \"1.2\"\n");
        out.push_str(&format!(
            "{}\n",
            render_embassy_time_driver_dependency(&model.drivers)
        ));
        out.push_str("embassy-time-queue-utils = { version = \"0.3.2\", features = [\"generic-queue-8\"] }\n");
    }
    out
}

fn render_tick_hz_feature(hz: u64) -> String {
    let digits = hz.to_string();
    let mut output = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, ch) in digits.chars().enumerate() {
        if index != 0 && (digits.len() - index).is_multiple_of(3) {
            output.push('_');
        }
        output.push(ch);
    }
    format!("tick-hz-{output}")
}

fn render_embassy_time_driver_dependency(drivers: &[ResolvedDriverInstance]) -> String {
    let time_driver = drivers
        .iter()
        .find(|driver| has_time_driver_tag(&driver.capability_tags));
    if let Some(driver) = time_driver
        && let Some(tick_hz) = time_driver_tick_hz(driver)
    {
        return format!(
            "embassy-time-driver = {{ version = \"0.2.2\", features = [{}] }}",
            render_rust_string(&render_tick_hz_feature(tick_hz))
        );
    }
    "embassy-time-driver = \"0.2.2\"".to_string()
}

fn render_embassy_lib_rs(model: &EmbassyGenerationModel) -> String {
    let mut out = String::from("#![no_std]\n\n");
    for module_name in model.module_names() {
        out.push_str(&format!("pub mod {module_name};\n"));
    }
    out
}

fn render_embassy_metadata_rs(model: &EmbassyGenerationModel) -> String {
    let doc_comment_title = render_comment_text(&model.document_title);
    format!(
        "//! Generated Embassy-style HAL metadata for {}.\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum ResourceRequirement {{\n    Required,\n    Optional,\n    MutuallyExclusive,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Error {{\n    Unsupported(&'static str),\n    InvalidReference(&'static str),\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub enum ValueLiteral {{\n    Integer(i64),\n    Unsigned(u64),\n    Number(f64),\n    String(&'static str),\n    Bool(bool),\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct SemanticExpression {{\n    pub language: Option<&'static str>,\n    pub text: &'static str,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct Predicate {{\n    pub kind: &'static str,\n    pub target_ref: Option<&'static str>,\n    pub expression: Option<SemanticExpression>,\n    pub expected_value: Option<ValueLiteral>,\n    pub description: Option<&'static str>,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct SemanticOperationStep {{\n    pub index: u32,\n    pub action: &'static str,\n    pub target_ref: Option<&'static str>,\n    pub expression: Option<SemanticExpression>,\n    pub value: Option<ValueLiteral>,\n    pub description: Option<&'static str>,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct SemanticOperation {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub description: Option<&'static str>,\n    pub kind: Option<&'static str>,\n    pub target_refs: &'static [&'static str],\n    pub steps: &'static [SemanticOperationStep],\n    pub preconditions: &'static [Predicate],\n    pub postconditions: &'static [Predicate],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct SemanticState {{\n    pub name: &'static str,\n    pub description: Option<&'static str>,\n    pub invariants: &'static [Predicate],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct SemanticSideEffect {{\n    pub kind: &'static str,\n    pub target_ref: Option<&'static str>,\n    pub description: Option<&'static str>,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct SemanticTransition {{\n    pub from: &'static str,\n    pub to: &'static str,\n    pub trigger: Option<&'static str>,\n    pub conditions: &'static [Predicate],\n    pub effects: &'static [SemanticSideEffect],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct SemanticStateMachine {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub description: Option<&'static str>,\n    pub target_refs: &'static [&'static str],\n    pub initial_state: Option<&'static str>,\n    pub states: &'static [SemanticState],\n    pub transitions: &'static [SemanticTransition],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct ClockBinding {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub consumer_ref: &'static str,\n    pub clock_ref: &'static str,\n    pub controller_ref: Option<&'static str>,\n    pub binding_kind: &'static str,\n    pub control_refs: &'static [&'static str],\n    pub enable_operation_refs: &'static [&'static str],\n    pub disable_operation_refs: &'static [&'static str],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct ResetBinding {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub target_ref: &'static str,\n    pub controller_ref: Option<&'static str>,\n    pub reset_domain_ref: Option<&'static str>,\n    pub binding_kind: &'static str,\n    pub control_refs: &'static [&'static str],\n    pub assert_operation_refs: &'static [&'static str],\n    pub release_operation_refs: &'static [&'static str],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct InterruptSource {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub source_ref: &'static str,\n    pub producer_ref: Option<&'static str>,\n    pub kind: &'static str,\n    pub flag_refs: &'static [&'static str],\n    pub clear_operation_refs: &'static [&'static str],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct InterruptRoute {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub source_ref: &'static str,\n    pub interrupt_ref: &'static str,\n    pub controller_ref: &'static str,\n    pub cpu_target_ref: Option<&'static str>,\n    pub line_index: Option<u32>,\n    pub route_type: &'static str,\n    pub control_refs: &'static [&'static str],\n    pub acknowledge_operation_refs: &'static [&'static str],\n    pub shared_group: Option<&'static str>,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct DmaChannel {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub controller_ref: &'static str,\n    pub target_ref: Option<&'static str>,\n    pub channel_index: u32,\n    pub capabilities: &'static [&'static str],\n    pub priority_levels: &'static [&'static str],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct DmaRoute {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub peripheral_ref: &'static str,\n    pub signal: Option<&'static str>,\n    pub channel_ref: &'static str,\n    pub direction: &'static str,\n    pub control_refs: &'static [&'static str],\n    pub shared_channel_group_ref: Option<&'static str>,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct PinRoute {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub pin_ref: &'static str,\n    pub peripheral_ref: &'static str,\n    pub signal: &'static str,\n    pub route_type: &'static str,\n    pub control_refs: &'static [&'static str],\n    pub electrical_constraint_refs: &'static [&'static str],\n    pub conflict_refs: &'static [&'static str],\n    pub default_after_reset: Option<bool>,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct PinRole {{\n    pub role: &'static str,\n    pub signal: &'static str,\n    pub routes: &'static [PinRoute],\n    pub requirement: ResourceRequirement,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct ProvenanceSource {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub kind: Option<&'static str>,\n    pub path: Option<&'static str>,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct ProvenanceEvidence {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub source_ref: &'static str,\n    pub normalized_claim: Option<&'static str>,\n    pub extraction_method: Option<&'static str>,\n    pub confidence: Option<f64>,\n    pub locator: Option<&'static str>,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct ModuleProvenance {{\n    pub module_name: &'static str,\n    pub document_title: &'static str,\n    pub document_version: &'static str,\n    pub source_ids: &'static [&'static str],\n    pub evidence_ids: &'static [&'static str],\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq)]\npub struct GeneratedMetadata {{\n    pub document_title: &'static str,\n    pub document_version: &'static str,\n    pub device_name: &'static str,\n    pub target_architecture: Option<&'static str>,\n    pub feature_flags: &'static [&'static str],\n    pub provenance_sources: &'static [ProvenanceSource],\n    pub provenance_evidence: &'static [ProvenanceEvidence],\n}}\n\npub const GENERATED_PROVENANCE_SOURCE_IDS: &[&str] = &{};\npub const GENERATED_PROVENANCE_EVIDENCE_IDS: &[&str] = &{};\n\npub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata {{\n    document_title: {},\n    document_version: {},\n    device_name: {},\n    target_architecture: {},\n    feature_flags: &{},\n    provenance_sources: &{},\n    provenance_evidence: &{},\n}};\n",
        doc_comment_title,
        render_named_entity_slice(
            &model
                .provenance
                .sources
                .iter()
                .map(|item| item.id.clone())
                .collect::<Vec<_>>()
        ),
        render_named_entity_slice(
            &model
                .provenance
                .evidence
                .iter()
                .map(|item| item.id.clone())
                .collect::<Vec<_>>()
        ),
        render_rust_string(&model.document_title),
        render_rust_string(&model.document_version),
        render_rust_string(&model.device_name),
        render_optional_rust_string(model.target_architecture.as_deref()),
        render_string_slice(&model.feature_flags),
        render_provenance_source_slice(&model.provenance.sources),
        render_provenance_evidence_slice(&model.provenance.evidence),
    )
}

fn render_interrupt_module(
    model: &EmbassyGenerationModel,
    drivers: &[&ResolvedDriverInstance],
) -> Result<String> {
    let mut out = format!(
        "//! Generated Embassy-style interrupt module for {}.\n\nuse crate::metadata;\nuse core::ptr::{{read_volatile, write_volatile}};\n\n{}\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Irq {{\n",
        render_comment_text(&model.device_name),
        render_mmio_helpers()
    );
    for interrupt in &model.interrupts {
        out.push_str(&format!(
            "    {} = {},\n",
            to_rust_type_name(&interrupt.name),
            interrupt.number
        ));
    }
    out.push_str("}\n\n");
    out.push_str(&render_module_provenance_const("interrupt"));
    for driver in drivers {
        out.push_str(&render_driver_instance(model, driver)?);
    }
    Ok(out)
}

fn render_driver_module(
    model: &EmbassyGenerationModel,
    module_name: &str,
    drivers: &[&ResolvedDriverInstance],
) -> Result<String> {
    let mut out = format!(
        "//! Generated Embassy-style {module_name} module for {}.\n\nuse crate::metadata;\nuse core::ptr::{{read_volatile, write_volatile}};\n\n{}\n",
        render_comment_text(&model.device_name),
        render_mmio_helpers()
    );
    if drivers
        .iter()
        .any(|driver| driver.driver_kind == "gpio-port")
    {
        out.push_str(render_gpio_module_prelude());
    }
    out.push_str(&render_module_provenance_const(module_name));
    for driver in drivers {
        out.push_str(&render_driver_instance(model, driver)?);
    }
    Ok(out)
}

fn render_driver_instance(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let const_prefix = to_rust_const_name(&driver.id);
    let support_items = render_driver_support_items(model, driver)?;
    let mut out = String::new();
    out.push_str(&format!(
        "// Driver instance: {} ({}) from canonical block {} -> {}\n",
        render_comment_text(&driver.name),
        render_comment_text(&driver.driver_kind),
        render_comment_text(&driver.target.id),
        render_comment_text(&driver.target.block_class)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_CLOCK_BINDINGS: &[metadata::ClockBinding] = &{};\n",
        render_clock_binding_slice(&driver.clock_bindings)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_RESET_BINDINGS: &[metadata::ResetBinding] = &{};\n",
        render_reset_binding_slice(&driver.reset_bindings)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &{};\n",
        render_interrupt_source_slice(&driver.interrupt_sources)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &{};\n",
        render_interrupt_route_slice(&driver.interrupt_routes)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_DMA_CHANNELS: &[metadata::DmaChannel] = &{};\n",
        render_dma_channel_slice(&driver.dma_channels)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_DMA_ROUTES: &[metadata::DmaRoute] = &{};\n",
        render_dma_route_slice(&driver.dma_routes)
    ));
    for (index, pin_role) in driver.pin_roles.iter().enumerate() {
        out.push_str(&format!(
            "pub const {const_prefix}_PIN_ROLE_{index}_ROUTES: &[metadata::PinRoute] = &{};\n",
            render_pin_route_slice(&pin_role.routes)
        ));
    }
    out.push_str(&format!(
        "pub const {const_prefix}_PIN_ROLES: &[metadata::PinRole] = &{};\n",
        render_pin_role_slice(const_prefix.as_str(), &driver.pin_roles)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_INIT_OPERATIONS: &[metadata::SemanticOperation] = &{};\n",
        render_semantic_operation_slice(&driver.init_operations)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_STATE_MACHINES: &[metadata::SemanticStateMachine] = &{};\n",
        render_semantic_state_machine_slice(&driver.state_machines)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_CAPABILITY_TAGS: &[&str] = &{};\n\n",
        render_named_entity_slice(&driver.capability_tags)
    ));
    out.push_str(&format!(
        "#[derive(Debug, Clone, Copy)]\npub struct {type_name}Resources {{\n    pub clocks: &'static [metadata::ClockBinding],\n    pub resets: &'static [metadata::ResetBinding],\n    pub interrupt_sources: &'static [metadata::InterruptSource],\n    pub interrupts: &'static [metadata::InterruptRoute],\n    pub dma_channels: &'static [metadata::DmaChannel],\n    pub dma: &'static [metadata::DmaRoute],\n    pub pins: &'static [metadata::PinRole],\n    pub init_operations: &'static [metadata::SemanticOperation],\n    pub state_machines: &'static [metadata::SemanticStateMachine],\n    pub lowering_pattern: Option<&'static str>,\n    pub time_driver_source: Option<&'static str>,\n    pub capability_tags: &'static [&'static str],\n}}\n\npub const {const_prefix}_RESOURCES: {type_name}Resources = {type_name}Resources {{\n    clocks: {const_prefix}_CLOCK_BINDINGS,\n    resets: {const_prefix}_RESET_BINDINGS,\n    interrupt_sources: {const_prefix}_INTERRUPT_SOURCES,\n    interrupts: {const_prefix}_INTERRUPT_ROUTES,\n    dma_channels: {const_prefix}_DMA_CHANNELS,\n    dma: {const_prefix}_DMA_ROUTES,\n    pins: {const_prefix}_PIN_ROLES,\n    init_operations: {const_prefix}_INIT_OPERATIONS,\n    state_machines: {const_prefix}_STATE_MACHINES,\n    lowering_pattern: {lowering_pattern},\n    time_driver_source: {time_driver_source},\n    capability_tags: {const_prefix}_CAPABILITY_TAGS,\n}};\n\n#[derive(Debug, Clone, Copy)]\npub struct {type_name} {{\n    resources: {type_name}Resources,\n}}\n\nimpl {type_name} {{\n    pub fn new(resources: {type_name}Resources) -> Result<Self, metadata::Error> {{\n        Ok(Self {{ resources }})\n    }}\n\n    pub fn resources(&self) -> {type_name}Resources {{\n        self.resources\n    }}\n{methods}\n}}\n\n{support_items}",
        type_name = driver.type_name,
        const_prefix = const_prefix,
        lowering_pattern = render_optional_rust_string(driver.lowering_pattern.as_deref()),
        time_driver_source = render_optional_rust_string(driver.time_driver_source.as_deref()),
        methods = render_driver_methods(model, driver)?,
        support_items = support_items
    ));
    Ok(out)
}

fn render_driver_support_items(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let mut out = String::new();
    if driver_uses_pfic_runtime_support(driver) {
        out.push_str(render_pfic_interrupt_support_items());
    }
    if driver.driver_kind == "gpio-port" {
        out.push_str(&render_gpio_support_items(model, driver)?);
    }
    if has_time_driver_tag(&driver.capability_tags) {
        out.push_str(&render_time_driver_support_items(model, driver)?);
    }
    Ok(out)
}

fn render_driver_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let mut methods = Vec::new();
    if driver.driver_kind == "interrupt" {
        methods.push(GeneratedMethod {
            name: "bind".to_string(),
            code: "    pub fn bind(&self) -> &'static [metadata::InterruptRoute] {\n        self.resources.interrupts\n    }\n"
                .to_string(),
        });
        if driver_uses_pfic_runtime_support(driver) {
            methods.push(GeneratedMethod {
                name: "enable_irq".to_string(),
                code: "    pub fn enable_irq(&self, irq: Irq) -> Result<(), metadata::Error> {\n        let irq_index = pfic_irq_index(irq)?;\n        let register = pfic_register_address(PFIC_IENR_BASE_ADDRESS, irq_index);\n        write_u32(register, pfic_irq_bit(irq_index))\n    }\n"
                    .to_string(),
            });
            methods.push(GeneratedMethod {
                name: "disable_irq".to_string(),
                code: "    pub fn disable_irq(&self, irq: Irq) -> Result<(), metadata::Error> {\n        let irq_index = pfic_irq_index(irq)?;\n        let register = pfic_register_address(PFIC_IRER_BASE_ADDRESS, irq_index);\n        write_u32(register, pfic_irq_bit(irq_index))\n    }\n"
                    .to_string(),
            });
            methods.push(GeneratedMethod {
                name: "is_irq_active".to_string(),
                code: "    pub fn is_irq_active(&self, irq: Irq) -> Result<bool, metadata::Error> {\n        let irq_index = pfic_irq_index(irq)?;\n        let register = pfic_register_address(PFIC_IACTR_BASE_ADDRESS, irq_index);\n        Ok((read_u32(register)? & pfic_irq_bit(irq_index)) != 0)\n    }\n"
                    .to_string(),
            });
        }
    }
    methods.extend(render_clock_binding_methods(model, driver)?);
    methods.extend(render_reset_binding_methods(model, driver)?);
    methods.extend(render_gpio_methods(model, driver)?);
    methods.extend(render_pin_route_methods(model, driver)?);
    methods.extend(render_usart_methods(model, driver)?);
    methods.extend(render_spi_methods(model, driver)?);
    methods.extend(render_adc_methods(model, driver)?);
    methods.extend(render_usb_device_methods(model, driver)?);
    methods.extend(render_operation_methods(model, driver)?);
    methods.extend(render_state_machine_methods(model, driver)?);
    methods.extend(render_time_driver_methods(driver)?);

    if methods.is_empty() {
        bail!(
            "driver {} has no executable lowering inputs for {}",
            driver.id,
            driver.driver_kind
        );
    }

    let mut seen = BTreeSet::new();
    let mut rendered = String::new();
    for method in methods {
        if !seen.insert(method.name.clone()) {
            bail!(
                "driver {} produced duplicate method {}",
                driver.id,
                method.name
            );
        }
        rendered.push_str(&method.code);
        rendered.push('\n');
    }
    Ok(rendered)
}

fn driver_uses_pfic_runtime_support(driver: &ResolvedDriverInstance) -> bool {
    driver.driver_kind == "interrupt"
        && driver.module_name == "interrupt"
        && driver.target.block_class == "interrupt-controller"
        && (driver.target.id == "block.pfic" || driver.target.name.eq_ignore_ascii_case("PFIC"))
}

fn render_pfic_interrupt_support_items() -> &'static str {
    // WCH's PFIC helper macros index IENR/IRER/IPSR/IACTR by the raw external IRQn
    // value (for example TIM4_IRQn = 46 -> IENR[1] bit 14), not by IRQn - 16.
    "\nconst PFIC_EXTERNAL_IRQ_OFFSET: u32 = 16;\nconst PFIC_IENR_BASE_ADDRESS: u64 = 0xE000_E100;\nconst PFIC_IRER_BASE_ADDRESS: u64 = 0xE000_E180;\nconst PFIC_IACTR_BASE_ADDRESS: u64 = 0xE000_E300;\n\nfn pfic_irq_index(irq: Irq) -> Result<u32, metadata::Error> {\n    let irq_index = irq as u32;\n    if irq_index < PFIC_EXTERNAL_IRQ_OFFSET {\n        return Err(metadata::Error::Unsupported(\n            \"PFIC runtime helpers only support external interrupts\",\n        ));\n    }\n    Ok(irq_index)\n}\n\nfn pfic_register_address(base: u64, irq_index: u32) -> u64 {\n    base + u64::from((irq_index / 32) * 4)\n}\n\nfn pfic_irq_bit(irq_index: u32) -> u32 {\n    1u32 << (irq_index % 32)\n}\n"
}

fn render_time_driver_methods(driver: &ResolvedDriverInstance) -> Result<Vec<GeneratedMethod>> {
    if !has_time_driver_tag(&driver.capability_tags) {
        return Ok(Vec::new());
    }

    let driver_prefix = to_rust_const_name(&driver.id).to_lowercase();
    match time_driver_source(driver) {
        Some(EMBASSY_TIME_DRIVER_SOURCE_SYSTICK) => Ok(vec![GeneratedMethod {
            name: "init_time_driver".to_string(),
            code: format!(
                "    pub fn init_time_driver(&self) -> Result<(), metadata::Error> {{\n        initialize_{driver_prefix}_time_driver()\n    }}\n",
            ),
        }]),
        Some(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER) => {
            let mut init_code =
                "    pub fn init_time_driver(&self) -> Result<(), metadata::Error> {\n".to_string();
            if !driver.clock_bindings.is_empty() {
                init_code.push_str("        self.enable_clock()?;\n");
            }
            if !driver.reset_bindings.is_empty() {
                init_code.push_str("        self.release_reset()?;\n");
            }
            for operation in &driver.init_operations {
                init_code.push_str(&format!(
                    "        self.apply_{}()?;\n",
                    operation_method_slug(operation)
                ));
            }
            init_code.push_str(&format!(
                "        initialize_{driver_prefix}_time_driver()\n    }}\n"
            ));
            Ok(vec![
                GeneratedMethod {
                    name: "init_time_driver".to_string(),
                    code: init_code,
                },
                GeneratedMethod {
                    name: "now_ticks".to_string(),
                    code: format!(
                        "    pub fn now_ticks(&self) -> u64 {{\n        generated_{driver_prefix}_time_driver_now()\n    }}\n"
                    ),
                },
                GeneratedMethod {
                    name: "delay_ticks".to_string(),
                    code: format!(
                        "    pub fn delay_ticks(&self, ticks: u64) -> Result<(), metadata::Error> {{\n        generated_{driver_prefix}_time_driver_delay_ticks(ticks)\n    }}\n"
                    ),
                },
                GeneratedMethod {
                    name: "on_time_driver_interrupt".to_string(),
                    code: format!(
                        "    pub fn on_time_driver_interrupt(&self) {{\n        generated_{driver_prefix}_time_driver_interrupt();\n    }}\n"
                    ),
                },
            ])
        }
        Some(other) => bail!(
            "driver {} uses unsupported timeDriverSource {} for generated time-driver methods",
            driver.id,
            other
        ),
        None => bail!(
            "driver {} claims capability tag {} but omits required timeDriverSource",
            driver.id,
            EMBASSY_TIME_DRIVER_TAG
        ),
    }
}

fn render_mmio_helpers() -> &'static str {
    "#[allow(dead_code)]\nfn checked_address(address: u64, align: usize) -> Result<usize, metadata::Error> {\n    let address = usize::try_from(address)\n        .map_err(|_| metadata::Error::Unsupported(\"MMIO address does not fit usize on this target\"))?;\n    if address % align != 0 {\n        return Err(metadata::Error::Unsupported(\"MMIO address is not naturally aligned for the target register width\"));\n    }\n    Ok(address)\n}\n\n#[allow(dead_code)]\nfn modify_u8(address: u64, clear_mask: u8, set_mask: u8) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u8>())?;\n    unsafe {\n        let current = read_volatile(address as *const u8);\n        write_volatile(address as *mut u8, (current & !clear_mask) | set_mask);\n    }\n    Ok(())\n}\n\n#[allow(dead_code)]\nfn modify_u16(address: u64, clear_mask: u16, set_mask: u16) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u16>())?;\n    unsafe {\n        let current = read_volatile(address as *const u16);\n        write_volatile(address as *mut u16, (current & !clear_mask) | set_mask);\n    }\n    Ok(())\n}\n\n#[allow(dead_code)]\nfn modify_u32(address: u64, clear_mask: u32, set_mask: u32) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u32>())?;\n    unsafe {\n        let current = read_volatile(address as *const u32);\n        write_volatile(address as *mut u32, (current & !clear_mask) | set_mask);\n    }\n    Ok(())\n}\n\n#[allow(dead_code)]\nfn read_u8(address: u64) -> Result<u8, metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u8>())?;\n    unsafe { Ok(read_volatile(address as *const u8)) }\n}\n\n#[allow(dead_code)]\nfn read_u16(address: u64) -> Result<u16, metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u16>())?;\n    unsafe { Ok(read_volatile(address as *const u16)) }\n}\n\n#[allow(dead_code)]\nfn read_u32(address: u64) -> Result<u32, metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u32>())?;\n    unsafe { Ok(read_volatile(address as *const u32)) }\n}\n\n#[allow(dead_code)]\nfn write_u8(address: u64, value: u8) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u8>())?;\n    unsafe {\n        write_volatile(address as *mut u8, value);\n    }\n    Ok(())\n}\n\n#[allow(dead_code)]\nfn write_u16(address: u64, value: u16) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u16>())?;\n    unsafe {\n        write_volatile(address as *mut u16, value);\n    }\n    Ok(())\n}\n\n#[allow(dead_code)]\nfn write_u32(address: u64, value: u32) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u32>())?;\n    unsafe {\n        write_volatile(address as *mut u32, value);\n    }\n    Ok(())\n}\n"
}

fn render_time_driver_support_items(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let driver_prefix = to_rust_const_name(&driver.id).to_lowercase();
    let init_fn = format!("initialize_{driver_prefix}_time_driver");
    match time_driver_source(driver) {
        Some(EMBASSY_TIME_DRIVER_SOURCE_SYSTICK) => Ok(format!(
            "\nuse core::cell::{{Cell, RefCell}};\nuse critical_section::Mutex as CriticalSectionMutex;\nuse embassy_time_driver::Driver as EmbassyTimeDriver;\nuse embassy_time_queue_utils::Queue as EmbassyTimeQueue;\n\nconst SYST_CSR_ADDRESS: u64 = 0xE000_E010;\nconst SYST_RVR_ADDRESS: u64 = 0xE000_E014;\nconst SYST_CVR_ADDRESS: u64 = 0xE000_E018;\nconst SYST_CSR_ENABLE: u32 = 1 << 0;\nconst SYST_CSR_TICKINT: u32 = 1 << 1;\nconst SYST_CSR_CLKSOURCE: u32 = 1 << 2;\nconst SYST_RELOAD_VALUE: u32 = 15;\n\nstruct GeneratedSystickTimeDriver {{\n    initialized: CriticalSectionMutex<Cell<bool>>,\n    ticks: CriticalSectionMutex<Cell<u64>>,\n    queue: CriticalSectionMutex<RefCell<EmbassyTimeQueue>>,\n}}\n\nimpl GeneratedSystickTimeDriver {{\n    const fn new() -> Self {{\n        Self {{\n            initialized: CriticalSectionMutex::new(Cell::new(false)),\n            ticks: CriticalSectionMutex::new(Cell::new(0)),\n            queue: CriticalSectionMutex::new(RefCell::new(EmbassyTimeQueue::new())),\n        }}\n    }}\n\n    fn init(&self) -> Result<(), metadata::Error> {{\n        critical_section::with(|cs| {{\n            if self.initialized.borrow(cs).get() {{\n                return Ok(());\n            }}\n            self.ticks.borrow(cs).set(0);\n            write_u32(SYST_CSR_ADDRESS, 0)?;\n            write_u32(SYST_RVR_ADDRESS, SYST_RELOAD_VALUE)?;\n            write_u32(SYST_CVR_ADDRESS, 0)?;\n            write_u32(\n                SYST_CSR_ADDRESS,\n                SYST_CSR_ENABLE | SYST_CSR_TICKINT | SYST_CSR_CLKSOURCE,\n            )?;\n            self.initialized.borrow(cs).set(true);\n            Ok(())\n        }})\n    }}\n\n    fn on_systick(&self) {{\n        critical_section::with(|cs| {{\n            if !self.initialized.borrow(cs).get() {{\n                return;\n            }}\n            let now = self.ticks.borrow(cs).get().wrapping_add(1);\n            self.ticks.borrow(cs).set(now);\n            let _ = self.queue.borrow(cs).borrow_mut().next_expiration(now);\n        }});\n    }}\n}}\n\nimpl EmbassyTimeDriver for GeneratedSystickTimeDriver {{\n    fn now(&self) -> u64 {{\n        critical_section::with(|cs| self.ticks.borrow(cs).get())\n    }}\n\n    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {{\n        critical_section::with(|cs| {{\n            let now = self.ticks.borrow(cs).get();\n            let mut queue = self.queue.borrow(cs).borrow_mut();\n            let _ = queue.schedule_wake(at, waker);\n            let _ = queue.next_expiration(now);\n        }});\n    }}\n}}\n\nembassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedSystickTimeDriver = GeneratedSystickTimeDriver::new());\n\n#[allow(dead_code)]\n#[allow(non_snake_case)]\n#[unsafe(no_mangle)]\nextern \"C\" fn SysTick() {{\n    GENERATED_TIME_DRIVER.on_systick();\n}}\n\nfn {init_fn}() -> Result<(), metadata::Error> {{\n    GENERATED_TIME_DRIVER.init()\n}}\n"
        )),
        Some(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER) => {
            if driver_uses_counter_compare_timer_lowering(driver) {
                let layout = resolve_counter_compare_timer_time_driver_layout(model, driver)?;
                return render_counter_compare_time_driver_support_items(
                    &driver_prefix,
                    &init_fn,
                    &layout,
                );
            }
            let layout = resolve_hardware_timer_time_driver_layout(model, driver)?;
            let clear_must_statement = layout
                .clear_statement
                .replace("modify_u8(", "must_modify_u8(")
                .replace("modify_u16(", "must_modify_u16(")
                .replace("modify_u32(", "must_modify_u32(")
                .replace("write_u32(", "must_write_u32(")
                .replace("?;\n", ";\n");
            let disarm_alarm = "    fn disarm_alarm(&self) {\n        must_modify_u32(GENERATED_TIME_INT_ENA_ADDRESS, GENERATED_TIME_INT_ENA_TARGET0_MASK, 0u32);\n    }\n";
            let acknowledge_interrupt =
                format!("    fn acknowledge_interrupt(&self) {{\n{clear_must_statement}    }}\n");
            let schedule_wake_impl = "    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {\n        let should_rearm = critical_section::with(|cs| {\n            self.queue.borrow(cs).borrow_mut().schedule_wake(at, waker)\n        });\n        if !should_rearm {\n            return;\n        }\n        let now = self.read_now();\n        critical_section::with(|cs| {\n            let next = self.queue.borrow(cs).borrow_mut().next_expiration(now);\n            if next == u64::MAX {\n                self.disarm_alarm();\n            } else {\n                self.arm_alarm(next);\n            }\n        });\n    }\n";
            Ok(format!(
                "\nuse core::cell::{{Cell, RefCell}};\nuse critical_section::Mutex as CriticalSectionMutex;\nuse embassy_time_driver::Driver as EmbassyTimeDriver;\nuse embassy_time_queue_utils::Queue as EmbassyTimeQueue;\n\nconst GENERATED_TIME_CONF_ADDRESS: u64 = 0x{conf_addr:X}u64;\nconst GENERATED_TIME_CONF_CLK_EN_MASK: u32 = 0x{conf_clk_en_mask:08X}u32;\nconst GENERATED_TIME_CONF_TARGET0_WORK_EN_MASK: u32 = 0x{conf_target0_work_en_mask:08X}u32;\nconst GENERATED_TIME_CONF_UNIT0_WORK_EN_MASK: u32 = 0x{conf_unit0_work_en_mask:08X}u32;\nconst GENERATED_TIME_UNIT0_OP_ADDRESS: u64 = 0x{unit0_op_addr:X}u64;\nconst GENERATED_TIME_UNIT0_OP_VALUE_VALID_MASK: u32 = 0x{unit0_value_valid_mask:08X}u32;\nconst GENERATED_TIME_UNIT0_OP_UPDATE_MASK: u32 = 0x{unit0_update_mask:08X}u32;\nconst GENERATED_TIME_UNIT0_VALUE_HI_ADDRESS: u64 = 0x{unit0_value_hi_addr:X}u64;\nconst GENERATED_TIME_UNIT0_VALUE_HI_MASK: u32 = 0x{unit0_value_hi_mask:08X}u32;\nconst GENERATED_TIME_UNIT0_VALUE_HI_SHIFT: u32 = {unit0_value_hi_shift};\nconst GENERATED_TIME_UNIT0_VALUE_LO_ADDRESS: u64 = 0x{unit0_value_lo_addr:X}u64;\nconst GENERATED_TIME_UNIT0_VALUE_LO_MASK: u32 = 0x{unit0_value_lo_mask:08X}u32;\nconst GENERATED_TIME_UNIT0_VALUE_LO_SHIFT: u32 = {unit0_value_lo_shift};\nconst GENERATED_TIME_TARGET0_HI_ADDRESS: u64 = 0x{target0_hi_addr:X}u64;\nconst GENERATED_TIME_TARGET0_HI_MASK: u32 = 0x{target0_hi_mask:08X}u32;\nconst GENERATED_TIME_TARGET0_HI_SHIFT: u32 = {target0_hi_shift};\nconst GENERATED_TIME_TARGET0_LO_ADDRESS: u64 = 0x{target0_lo_addr:X}u64;\nconst GENERATED_TIME_TARGET0_LO_MASK: u32 = 0x{target0_lo_mask:08X}u32;\nconst GENERATED_TIME_TARGET0_LO_SHIFT: u32 = {target0_lo_shift};\nconst GENERATED_TIME_TARGET0_CONF_ADDRESS: u64 = 0x{target0_conf_addr:X}u64;\nconst GENERATED_TIME_TARGET0_PERIOD_MASK: u32 = 0x{target0_period_mask:08X}u32;\nconst GENERATED_TIME_TARGET0_PERIOD_MODE_MASK: u32 = 0x{target0_period_mode_mask:08X}u32;\nconst GENERATED_TIME_TARGET0_UNIT_SEL_MASK: u32 = 0x{target0_unit_sel_mask:08X}u32;\nconst GENERATED_TIME_COMP0_LOAD_ADDRESS: u64 = 0x{comp0_load_addr:X}u64;\nconst GENERATED_TIME_COMP0_LOAD_MASK: u32 = 0x{comp0_load_mask:08X}u32;\nconst GENERATED_TIME_COMP0_LOAD_SHIFT: u32 = {comp0_load_shift};\nconst GENERATED_TIME_INT_ENA_ADDRESS: u64 = 0x{int_ena_addr:X}u64;\nconst GENERATED_TIME_INT_ENA_TARGET0_MASK: u32 = 0x{int_ena_mask:08X}u32;\n\nstruct GeneratedHardwareTimerTimeDriver {{\n    initialized: CriticalSectionMutex<Cell<bool>>,\n    queue: CriticalSectionMutex<RefCell<EmbassyTimeQueue>>,\n}}\n\nimpl GeneratedHardwareTimerTimeDriver {{\n    const fn new() -> Self {{\n        Self {{\n            initialized: CriticalSectionMutex::new(Cell::new(false)),\n            queue: CriticalSectionMutex::new(RefCell::new(EmbassyTimeQueue::new())),\n        }}\n    }}\n\n    fn init(&self) -> Result<(), metadata::Error> {{\n        critical_section::with(|cs| {{\n            if self.initialized.borrow(cs).get() {{\n                return Ok(());\n            }}\n            modify_u32(\n                GENERATED_TIME_CONF_ADDRESS,\n                0u32,\n                GENERATED_TIME_CONF_CLK_EN_MASK\n                    | GENERATED_TIME_CONF_TARGET0_WORK_EN_MASK\n                    | GENERATED_TIME_CONF_UNIT0_WORK_EN_MASK,\n            )?;\n            modify_u32(\n                GENERATED_TIME_TARGET0_CONF_ADDRESS,\n                GENERATED_TIME_TARGET0_PERIOD_MASK\n                    | GENERATED_TIME_TARGET0_PERIOD_MODE_MASK\n                    | GENERATED_TIME_TARGET0_UNIT_SEL_MASK,\n                0u32,\n            )?;\n            modify_u32(GENERATED_TIME_INT_ENA_ADDRESS, GENERATED_TIME_INT_ENA_TARGET0_MASK, 0u32)?;\n{clear_statement}            self.initialized.borrow(cs).set(true);\n            Ok(())\n        }})\n    }}\n\n    fn read_now(&self) -> u64 {{\n        must_modify_u32(\n            GENERATED_TIME_UNIT0_OP_ADDRESS,\n            GENERATED_TIME_UNIT0_OP_UPDATE_MASK,\n            GENERATED_TIME_UNIT0_OP_UPDATE_MASK,\n        );\n        loop {{\n            if (must_read_u32(GENERATED_TIME_UNIT0_OP_ADDRESS) & GENERATED_TIME_UNIT0_OP_VALUE_VALID_MASK) != 0 {{\n                let high = ((must_read_u32(GENERATED_TIME_UNIT0_VALUE_HI_ADDRESS) & GENERATED_TIME_UNIT0_VALUE_HI_MASK) >> GENERATED_TIME_UNIT0_VALUE_HI_SHIFT) as u64;\n                let low = ((must_read_u32(GENERATED_TIME_UNIT0_VALUE_LO_ADDRESS) & GENERATED_TIME_UNIT0_VALUE_LO_MASK) >> GENERATED_TIME_UNIT0_VALUE_LO_SHIFT) as u64;\n                return (high << 32) | low;\n            }}\n        }}\n    }}\n\n    fn arm_alarm(&self, at: u64) {{\n        let high = ((at >> 32) as u32) << GENERATED_TIME_TARGET0_HI_SHIFT;\n        let low = (at as u32) << GENERATED_TIME_TARGET0_LO_SHIFT;\n        must_modify_u32(GENERATED_TIME_TARGET0_HI_ADDRESS, GENERATED_TIME_TARGET0_HI_MASK, high & GENERATED_TIME_TARGET0_HI_MASK);\n        must_modify_u32(GENERATED_TIME_TARGET0_LO_ADDRESS, GENERATED_TIME_TARGET0_LO_MASK, low & GENERATED_TIME_TARGET0_LO_MASK);\n        must_modify_u32(\n            GENERATED_TIME_TARGET0_CONF_ADDRESS,\n            GENERATED_TIME_TARGET0_PERIOD_MASK | GENERATED_TIME_TARGET0_PERIOD_MODE_MASK | GENERATED_TIME_TARGET0_UNIT_SEL_MASK,\n            0u32,\n        );\n        must_modify_u32(\n            GENERATED_TIME_COMP0_LOAD_ADDRESS,\n            GENERATED_TIME_COMP0_LOAD_MASK,\n            (1u32 << GENERATED_TIME_COMP0_LOAD_SHIFT) & GENERATED_TIME_COMP0_LOAD_MASK,\n        );\n        must_modify_u32(\n            GENERATED_TIME_INT_ENA_ADDRESS,\n            GENERATED_TIME_INT_ENA_TARGET0_MASK,\n            GENERATED_TIME_INT_ENA_TARGET0_MASK,\n        );\n    }}\n\n{disarm_alarm}\n{acknowledge_interrupt}\n    fn on_interrupt(&self) {{\n        critical_section::with(|cs| {{\n            if !self.initialized.borrow(cs).get() {{\n                return;\n            }}\n            self.acknowledge_interrupt();\n            let now = self.read_now();\n            let mut queue = self.queue.borrow(cs).borrow_mut();\n            let next = queue.next_expiration(now);\n            if next == u64::MAX {{\n                self.disarm_alarm();\n            }} else {{\n                self.arm_alarm(next);\n            }}\n        }});\n    }}\n\n    fn delay_ticks(&self, ticks: u64) -> Result<(), metadata::Error> {{\n        let start = self.read_now();\n        let deadline = start.wrapping_add(ticks);\n        loop {{\n            let now = self.read_now();\n            if now.wrapping_sub(deadline) < (1u64 << 63) {{\n                break;\n            }}\n            core::hint::spin_loop();\n        }}\n        Ok(())\n    }}\n}}\n\nimpl EmbassyTimeDriver for GeneratedHardwareTimerTimeDriver {{\n    fn now(&self) -> u64 {{\n        self.read_now()\n    }}\n\n{schedule_wake_impl}}}\n\n#[allow(dead_code)]\nfn must_read_u32(address: u64) -> u32 {{\n    read_u32(address).expect(\"generated time-driver MMIO read\")\n}}\n\n#[allow(dead_code)]\nfn must_modify_u8(address: u64, clear_mask: u8, set_mask: u8) {{\n    modify_u8(address, clear_mask, set_mask).expect(\"generated time-driver MMIO write\")\n}}\n\n#[allow(dead_code)]\nfn must_modify_u16(address: u64, clear_mask: u16, set_mask: u16) {{\n    modify_u16(address, clear_mask, set_mask).expect(\"generated time-driver MMIO write\")\n}}\n\n#[allow(dead_code)]\nfn must_modify_u32(address: u64, clear_mask: u32, set_mask: u32) {{\n    modify_u32(address, clear_mask, set_mask).expect(\"generated time-driver MMIO write\")\n}}\n\n#[allow(dead_code)]\nfn must_write_u32(address: u64, value: u32) {{\n    write_u32(address, value).expect(\"generated time-driver MMIO write\")\n}}\n\nembassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedHardwareTimerTimeDriver = GeneratedHardwareTimerTimeDriver::new());\n\n#[allow(dead_code)]\npub fn generated_{driver_prefix}_time_driver_interrupt() {{\n    GENERATED_TIME_DRIVER.on_interrupt();\n}}\n\nfn {init_fn}() -> Result<(), metadata::Error> {{\n    GENERATED_TIME_DRIVER.init()\n}}\n\nfn generated_{driver_prefix}_time_driver_now() -> u64 {{\n    GENERATED_TIME_DRIVER.now()\n}}\n\nfn generated_{driver_prefix}_time_driver_delay_ticks(ticks: u64) -> Result<(), metadata::Error> {{\n    GENERATED_TIME_DRIVER.delay_ticks(ticks)\n}}\n",
                conf_addr = layout.conf_clk_en.register.absolute_address,
                conf_clk_en_mask = layout.conf_clk_en.mask,
                conf_target0_work_en_mask = layout.conf_target0_work_en.mask,
                conf_unit0_work_en_mask = layout.conf_unit0_work_en.mask,
                unit0_op_addr = layout.unit0_op_update.register.absolute_address,
                unit0_value_valid_mask = layout.unit0_op_value_valid.mask,
                unit0_update_mask = layout.unit0_op_update.mask,
                unit0_value_hi_addr = layout.unit0_value_hi.register.absolute_address,
                unit0_value_hi_mask = layout.unit0_value_hi.mask,
                unit0_value_hi_shift = layout.unit0_value_hi.lsb,
                unit0_value_lo_addr = layout.unit0_value_lo.register.absolute_address,
                unit0_value_lo_mask = layout.unit0_value_lo.mask,
                unit0_value_lo_shift = layout.unit0_value_lo.lsb,
                target0_hi_addr = layout.target0_hi.register.absolute_address,
                target0_hi_mask = layout.target0_hi.mask,
                target0_hi_shift = layout.target0_hi.lsb,
                target0_lo_addr = layout.target0_lo.register.absolute_address,
                target0_lo_mask = layout.target0_lo.mask,
                target0_lo_shift = layout.target0_lo.lsb,
                target0_conf_addr = layout.target0_conf_period.register.absolute_address,
                target0_period_mask = layout.target0_conf_period.mask,
                target0_period_mode_mask = layout.target0_conf_period_mode.mask,
                target0_unit_sel_mask = layout.target0_conf_unit_sel.mask,
                comp0_load_addr = layout.comp0_load.register.absolute_address,
                comp0_load_mask = layout.comp0_load.mask,
                comp0_load_shift = layout.comp0_load.lsb,
                int_ena_addr = layout.int_ena_target0.register.absolute_address,
                int_ena_mask = layout.int_ena_target0.mask,
                clear_statement = layout.clear_statement,
                disarm_alarm = disarm_alarm,
                acknowledge_interrupt = acknowledge_interrupt,
                schedule_wake_impl = schedule_wake_impl,
            ))
        }
        Some(other) => bail!(
            "driver {} uses unsupported timeDriverSource {} for generated time-driver support",
            driver.id,
            other
        ),
        None => bail!(
            "driver {} claims capability tag {} but omits required timeDriverSource",
            driver.id,
            EMBASSY_TIME_DRIVER_TAG
        ),
    }
}

fn render_counter_compare_time_driver_support_items(
    driver_prefix: &str,
    init_fn: &str,
    layout: &CounterCompareTimerTimeDriverLayout,
) -> Result<String> {
    let clear_must_statement = layout
        .clear_statement
        .replace("modify_u8(", "must_modify_u8(")
        .replace("modify_u16(", "must_modify_u16(")
        .replace("modify_u32(", "must_modify_u32(")
        .replace("write_u8(", "must_write_u8(")
        .replace("write_u16(", "must_write_u16(")
        .replace("write_u32(", "must_write_u32(")
        .replace("?;\n", ";\n");
    let alarm_apply_must_statement = layout
        .alarm_apply_statement
        .replace("modify_u8(", "must_modify_u8(")
        .replace("modify_u16(", "must_modify_u16(")
        .replace("modify_u32(", "must_modify_u32(")
        .replace("write_u8(", "must_write_u8(")
        .replace("write_u16(", "must_write_u16(")
        .replace("write_u32(", "must_write_u32(")
        .replace("?;\n", ";\n");

    let read_counter_impl = match layout.counter.register.width_bits {
        8 => "        let raw = must_read_u8(GENERATED_TIME_COUNTER_ADDRESS) as u32;\n        ((raw & GENERATED_TIME_COUNTER_MASK) >> GENERATED_TIME_COUNTER_SHIFT) as u32\n".to_string(),
        16 => "        let raw = must_read_u16(GENERATED_TIME_COUNTER_ADDRESS) as u32;\n        ((raw & GENERATED_TIME_COUNTER_MASK) >> GENERATED_TIME_COUNTER_SHIFT) as u32\n".to_string(),
        32 => "        let raw = must_read_u32(GENERATED_TIME_COUNTER_ADDRESS);\n        ((raw & GENERATED_TIME_COUNTER_MASK) >> GENERATED_TIME_COUNTER_SHIFT) as u32\n".to_string(),
        other => bail!(
            "counter-compare time driver uses unsupported counter register width {}",
            other
        ),
    };

    let alarm_write_impl = match layout.compare.register.width_bits {
        8 => {
            "        let raw = (((at as u32) << GENERATED_TIME_COMPARE_SHIFT) & GENERATED_TIME_COMPARE_MASK) as u8;\n        must_modify_u8(GENERATED_TIME_COMPARE_ADDRESS, GENERATED_TIME_COMPARE_MASK as u8, raw);\n".to_string()
        }
        16 => {
            "        let raw = (((at as u32) << GENERATED_TIME_COMPARE_SHIFT) & GENERATED_TIME_COMPARE_MASK) as u16;\n        must_modify_u16(GENERATED_TIME_COMPARE_ADDRESS, GENERATED_TIME_COMPARE_MASK as u16, raw);\n".to_string()
        }
        32 => {
            "        let raw = (((at as u32) << GENERATED_TIME_COMPARE_SHIFT) & GENERATED_TIME_COMPARE_MASK) as u32;\n        must_modify_u32(GENERATED_TIME_COMPARE_ADDRESS, GENERATED_TIME_COMPARE_MASK, raw);\n".to_string()
        }
        other => bail!(
            "counter-compare time driver uses unsupported compare register width {}",
            other
        ),
    };

    let set_alarm_enable_impl = match layout.interrupt_enable.register.width_bits {
        8 => {
            "        let set_mask = enabled.then_some(GENERATED_TIME_INTERRUPT_ENABLE_MASK as u8).unwrap_or(0u8);\n        must_modify_u8(GENERATED_TIME_INTERRUPT_ENABLE_ADDRESS, GENERATED_TIME_INTERRUPT_ENABLE_MASK as u8, set_mask);\n".to_string()
        }
        16 => {
            "        let set_mask = enabled.then_some(GENERATED_TIME_INTERRUPT_ENABLE_MASK as u16).unwrap_or(0u16);\n        must_modify_u16(GENERATED_TIME_INTERRUPT_ENABLE_ADDRESS, GENERATED_TIME_INTERRUPT_ENABLE_MASK as u16, set_mask);\n".to_string()
        }
        32 => {
            "        let set_mask = enabled.then_some(GENERATED_TIME_INTERRUPT_ENABLE_MASK).unwrap_or(0u32);\n        must_modify_u32(GENERATED_TIME_INTERRUPT_ENABLE_ADDRESS, GENERATED_TIME_INTERRUPT_ENABLE_MASK, set_mask);\n".to_string()
        }
        other => bail!(
            "counter-compare time driver uses unsupported interrupt-enable register width {}",
            other
        ),
    };

    let alarm_pending_impl = match layout.interrupt_pending.register.width_bits {
        8 => {
            "        (must_read_u8(GENERATED_TIME_INTERRUPT_PENDING_ADDRESS) & (GENERATED_TIME_INTERRUPT_PENDING_MASK as u8)) != 0\n".to_string()
        }
        16 => {
            "        (must_read_u16(GENERATED_TIME_INTERRUPT_PENDING_ADDRESS) & (GENERATED_TIME_INTERRUPT_PENDING_MASK as u16)) != 0\n".to_string()
        }
        32 => {
            "        (must_read_u32(GENERATED_TIME_INTERRUPT_PENDING_ADDRESS) & GENERATED_TIME_INTERRUPT_PENDING_MASK) != 0\n".to_string()
        }
        other => bail!(
            "counter-compare time driver uses unsupported interrupt-pending register width {}",
            other
        ),
    };

    Ok(format!(
        "\nuse core::cell::{{Cell, RefCell}};\nuse critical_section::Mutex as CriticalSectionMutex;\nuse embassy_time_driver::Driver as EmbassyTimeDriver;\nuse embassy_time_queue_utils::Queue as EmbassyTimeQueue;\n\nconst GENERATED_TIME_COUNTER_ADDRESS: u64 = 0x{counter_addr:X}u64;\nconst GENERATED_TIME_COUNTER_MASK: u32 = 0x{counter_mask:08X}u32;\nconst GENERATED_TIME_COUNTER_SHIFT: u32 = {counter_shift};\nconst GENERATED_TIME_COUNTER_BITS: u32 = {counter_bits};\nconst GENERATED_TIME_COMPARE_ADDRESS: u64 = 0x{compare_addr:X}u64;\nconst GENERATED_TIME_COMPARE_MASK: u32 = 0x{compare_mask:08X}u32;\nconst GENERATED_TIME_COMPARE_SHIFT: u32 = {compare_shift};\nconst GENERATED_TIME_INTERRUPT_ENABLE_ADDRESS: u64 = 0x{enable_addr:X}u64;\nconst GENERATED_TIME_INTERRUPT_ENABLE_MASK: u32 = 0x{enable_mask:08X}u32;\nconst GENERATED_TIME_INTERRUPT_PENDING_ADDRESS: u64 = 0x{pending_addr:X}u64;\nconst GENERATED_TIME_INTERRUPT_PENDING_MASK: u32 = 0x{pending_mask:08X}u32;\n\nstruct GeneratedCounterCompareTimeDriver {{\n    initialized: CriticalSectionMutex<Cell<bool>>,\n    wraps: CriticalSectionMutex<Cell<u64>>,\n    last_raw: CriticalSectionMutex<Cell<u32>>,\n    queue: CriticalSectionMutex<RefCell<EmbassyTimeQueue>>,\n}}\n\nimpl GeneratedCounterCompareTimeDriver {{\n    const fn new() -> Self {{\n        Self {{\n            initialized: CriticalSectionMutex::new(Cell::new(false)),\n            wraps: CriticalSectionMutex::new(Cell::new(0)),\n            last_raw: CriticalSectionMutex::new(Cell::new(0)),\n            queue: CriticalSectionMutex::new(RefCell::new(EmbassyTimeQueue::new())),\n        }}\n    }}\n\n    fn init(&self) -> Result<(), metadata::Error> {{\n        critical_section::with(|cs| {{\n            if self.initialized.borrow(cs).get() {{\n                return Ok(());\n            }}\n            self.set_alarm_enabled(false);\n{clear_statement}            self.wraps.borrow(cs).set(0);\n            self.last_raw.borrow(cs).set(self.read_raw_counter());\n            self.initialized.borrow(cs).set(true);\n            Ok(())\n        }})\n    }}\n\n    fn read_raw_counter(&self) -> u32 {{\n{read_counter_impl}    }}\n\n    fn read_now(&self) -> u64 {{\n        critical_section::with(|cs| {{\n            let raw = self.read_raw_counter();\n            let last = self.last_raw.borrow(cs).get();\n            let mut wraps = self.wraps.borrow(cs).get();\n            if raw < last {{\n                wraps = wraps.wrapping_add(1);\n                self.wraps.borrow(cs).set(wraps);\n            }}\n            self.last_raw.borrow(cs).set(raw);\n            (wraps << GENERATED_TIME_COUNTER_BITS) | u64::from(raw)\n        }})\n    }}\n\n    fn arm_alarm(&self, at: u64) {{\n{alarm_write_impl}{alarm_apply_impl}        self.set_alarm_enabled(true);\n    }}\n\n    fn set_alarm_enabled(&self, enabled: bool) {{\n{set_alarm_enable_impl}    }}\n\n    fn acknowledge_interrupt(&self) {{\n{clear_statement}    }}\n\n    fn is_alarm_pending(&self) -> bool {{\n{alarm_pending_impl}    }}\n        \n    fn on_interrupt(&self) {{\n        if !critical_section::with(|cs| self.initialized.borrow(cs).get()) || !self.is_alarm_pending() {{\n            return;\n        }}\n        self.acknowledge_interrupt();\n        let now = self.read_now();\n        critical_section::with(|cs| {{\n            let mut queue = self.queue.borrow(cs).borrow_mut();\n            let next = queue.next_expiration(now);\n            if next == u64::MAX {{\n                self.set_alarm_enabled(false);\n            }} else {{\n                self.arm_alarm(next);\n            }}\n        }});\n    }}\n\n    fn delay_ticks(&self, ticks: u64) -> Result<(), metadata::Error> {{\n        let start = self.read_now();\n        let deadline = start.wrapping_add(ticks);\n        loop {{\n            let now = self.read_now();\n            if now.wrapping_sub(deadline) < (1u64 << 63) {{\n                break;\n            }}\n            core::hint::spin_loop();\n        }}\n        Ok(())\n    }}\n}}\n\nimpl EmbassyTimeDriver for GeneratedCounterCompareTimeDriver {{\n    fn now(&self) -> u64 {{\n        self.read_now()\n    }}\n\n    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {{\n        let should_rearm = critical_section::with(|cs| {{\n            self.queue.borrow(cs).borrow_mut().schedule_wake(at, waker)\n        }});\n        if !should_rearm {{\n            return;\n        }}\n        let now = self.read_now();\n        critical_section::with(|cs| {{\n            let next = self.queue.borrow(cs).borrow_mut().next_expiration(now);\n            if next == u64::MAX {{\n                self.set_alarm_enabled(false);\n            }} else {{\n                self.arm_alarm(next);\n            }}\n        }});\n    }}\n}}\n\n#[allow(dead_code)]\nfn must_read_u8(address: u64) -> u8 {{\n    read_u8(address).expect(\"generated time-driver MMIO read\")\n}}\n\n#[allow(dead_code)]\nfn must_read_u16(address: u64) -> u16 {{\n    read_u16(address).expect(\"generated time-driver MMIO read\")\n}}\n\n#[allow(dead_code)]\nfn must_read_u32(address: u64) -> u32 {{\n    read_u32(address).expect(\"generated time-driver MMIO read\")\n}}\n\n#[allow(dead_code)]\nfn must_modify_u8(address: u64, clear_mask: u8, set_mask: u8) {{\n    modify_u8(address, clear_mask, set_mask).expect(\"generated time-driver MMIO write\")\n}}\n\n#[allow(dead_code)]\nfn must_modify_u16(address: u64, clear_mask: u16, set_mask: u16) {{\n    modify_u16(address, clear_mask, set_mask).expect(\"generated time-driver MMIO write\")\n}}\n\n#[allow(dead_code)]\nfn must_modify_u32(address: u64, clear_mask: u32, set_mask: u32) {{\n    modify_u32(address, clear_mask, set_mask).expect(\"generated time-driver MMIO write\")\n}}\n\n#[allow(dead_code)]\nfn must_write_u8(address: u64, value: u8) {{\n    write_u8(address, value).expect(\"generated time-driver MMIO write\")\n}}\n\n#[allow(dead_code)]\nfn must_write_u16(address: u64, value: u16) {{\n    write_u16(address, value).expect(\"generated time-driver MMIO write\")\n}}\n\n#[allow(dead_code)]\nfn must_write_u32(address: u64, value: u32) {{\n    write_u32(address, value).expect(\"generated time-driver MMIO write\")\n}}\n\nembassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedCounterCompareTimeDriver = GeneratedCounterCompareTimeDriver::new());\n\n#[allow(dead_code)]\npub fn generated_{driver_prefix}_time_driver_interrupt() {{\n    GENERATED_TIME_DRIVER.on_interrupt();\n}}\n\nfn {init_fn}() -> Result<(), metadata::Error> {{\n    GENERATED_TIME_DRIVER.init()\n}}\n\nfn generated_{driver_prefix}_time_driver_now() -> u64 {{\n    GENERATED_TIME_DRIVER.now()\n}}\n\nfn generated_{driver_prefix}_time_driver_delay_ticks(ticks: u64) -> Result<(), metadata::Error> {{\n    GENERATED_TIME_DRIVER.delay_ticks(ticks)\n}}\n",
        counter_addr = layout.counter.register.absolute_address,
        counter_mask = layout.counter.mask,
        counter_shift = layout.counter.lsb,
        counter_bits = layout.counter.register.width_bits.min(32),
        compare_addr = layout.compare.register.absolute_address,
        compare_mask = layout.compare.mask,
        compare_shift = layout.compare.lsb,
        enable_addr = layout.interrupt_enable.register.absolute_address,
        enable_mask = layout.interrupt_enable.mask,
        pending_addr = layout.interrupt_pending.register.absolute_address,
        pending_mask = layout.interrupt_pending.mask,
        clear_statement = clear_must_statement,
        read_counter_impl = read_counter_impl,
        alarm_write_impl = alarm_write_impl,
        alarm_apply_impl = alarm_apply_must_statement,
        set_alarm_enable_impl = set_alarm_enable_impl,
        alarm_pending_impl = alarm_pending_impl,
    ))
}

fn render_gpio_module_prelude() -> &'static str {
    "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Level {\n    Low,\n    High,\n}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Pull {\n    None,\n    Up,\n    Down,\n}\n\n"
}

fn render_clock_binding_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if driver_uses_usb_preserve_link_lowering(driver) {
        return Ok(Vec::new());
    }
    let multi_binding_names = driver.clock_bindings.len() > 1 || driver.driver_kind == "rcc";
    let mut subject_counts = BTreeMap::<String, usize>::new();
    for binding in &driver.clock_bindings {
        let subject = model
            .peripheral_names
            .get(&binding.consumer_ref)
            .cloned()
            .unwrap_or_else(|| last_ref_segment(&binding.consumer_ref).to_string());
        *subject_counts
            .entry(to_rust_method_name(&subject))
            .or_default() += 1;
    }
    driver
        .clock_bindings
        .iter()
        .map(|binding| {
            let subject = model
                .peripheral_names
                .get(&binding.consumer_ref)
                .cloned()
                .unwrap_or_else(|| last_ref_segment(&binding.consumer_ref).to_string());
            let (register, field) = resolve_control_target(
                model,
                &binding.control_refs,
                "clock binding",
                &binding.id,
                &[
                    binding.name.clone(),
                    subject.clone(),
                    last_ref_segment(&binding.consumer_ref).to_string(),
                ],
            )?;
            let subject_slug = to_rust_method_name(&subject);
            let duplicate_subject = subject_counts
                .get(&subject_slug)
                .copied()
                .unwrap_or_default()
                > 1;
            let binding_suffix = duplicate_subject.then(|| to_rust_method_name(&binding.id));
            let enable_name = if multi_binding_names {
                binding_method_name("enable", &subject_slug, binding_suffix.as_deref(), "clock")
            } else {
                "enable_clock".to_string()
            };
            let disable_name = if multi_binding_names {
                binding_method_name("disable", &subject_slug, binding_suffix.as_deref(), "clock")
            } else {
                "disable_clock".to_string()
            };

            Ok(vec![
                render_register_write_method(
                    enable_name,
                    register,
                    &field,
                    1,
                    &format!("Enable the {} clock gate.", subject),
                )?,
                render_register_write_method(
                    disable_name,
                    register,
                    &field,
                    0,
                    &format!("Disable the {} clock gate.", subject),
                )?,
            ])
        })
        .collect::<Result<Vec<_>>>()
        .map(|nested| nested.into_iter().flatten().collect())
}

fn render_reset_binding_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if driver_uses_usb_preserve_link_lowering(driver) {
        return Ok(Vec::new());
    }
    let multi_binding_names = driver.reset_bindings.len() > 1 || driver.driver_kind == "rcc";
    let mut subject_counts = BTreeMap::<String, usize>::new();
    for binding in &driver.reset_bindings {
        let subject = model
            .peripheral_names
            .get(&binding.target_ref)
            .cloned()
            .unwrap_or_else(|| last_ref_segment(&binding.target_ref).to_string());
        *subject_counts
            .entry(to_rust_method_name(&subject))
            .or_default() += 1;
    }
    driver
        .reset_bindings
        .iter()
        .map(|binding| {
            let subject = model
                .peripheral_names
                .get(&binding.target_ref)
                .cloned()
                .unwrap_or_else(|| last_ref_segment(&binding.target_ref).to_string());
            let (register, field) = resolve_control_target(
                model,
                &binding.control_refs,
                "reset binding",
                &binding.id,
                &[
                    binding.name.clone(),
                    subject.clone(),
                    last_ref_segment(&binding.target_ref).to_string(),
                ],
            )?;
            let subject_slug = to_rust_method_name(&subject);
            let duplicate_subject = subject_counts
                .get(&subject_slug)
                .copied()
                .unwrap_or_default()
                > 1;
            let binding_suffix = duplicate_subject.then(|| to_rust_method_name(&binding.id));
            let assert_name = if multi_binding_names {
                binding_method_name("assert", &subject_slug, binding_suffix.as_deref(), "reset")
            } else {
                "assert_reset".to_string()
            };
            let release_name = if multi_binding_names {
                binding_method_name("release", &subject_slug, binding_suffix.as_deref(), "reset")
            } else {
                "release_reset".to_string()
            };

            Ok(vec![
                render_register_write_method(
                    assert_name,
                    register,
                    &field,
                    1,
                    &format!("Assert reset for {}.", subject),
                )?,
                render_register_write_method(
                    release_name,
                    register,
                    &field,
                    0,
                    &format!("Release reset for {}.", subject),
                )?,
            ])
        })
        .collect::<Result<Vec<_>>>()
        .map(|nested| nested.into_iter().flatten().collect())
}

fn binding_method_name(prefix: &str, subject: &str, suffix: Option<&str>, noun: &str) -> String {
    match suffix {
        Some(suffix) => format!("{prefix}_{subject}_{suffix}_{noun}"),
        None => format!("{prefix}_{subject}_{noun}"),
    }
}

fn render_pin_route_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if !matches!(driver.driver_kind.as_str(), "uart" | "usart") {
        return Ok(Vec::new());
    }

    let mut methods = Vec::new();
    for pin_role in &driver.pin_roles {
        for route in &pin_role.routes {
            if let Some(method) = render_muxed_pin_route_method(model, driver, pin_role, route)? {
                methods.push(method);
            }
        }
    }
    Ok(methods)
}

fn render_muxed_pin_route_method(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
    pin_role: &ResolvedEmbassyPinRole,
    route: &McuPinRoute,
) -> Result<Option<GeneratedMethod>> {
    if route.route_type != "muxed" {
        return Ok(None);
    }

    let Some(description) = route.description.as_deref() else {
        return Ok(None);
    };
    let Some(alternate_function) = extract_alternate_function_index(description) else {
        return Ok(None);
    };

    let Some(pin) = model.pins.iter().find(|pin| pin.id == route.pin_ref) else {
        return Ok(None);
    };
    let Some(pin_index) = pin.index else {
        return Ok(None);
    };

    let mut moder_register = None;
    let mut afr_register = None;
    for control_ref in &route.control_refs {
        let Some(register) = try_resolve_register_by_ref(model, control_ref)? else {
            continue;
        };
        match normalize_search_text(&register.name).as_str() {
            "MODER" => moder_register = Some(register),
            "AFRL" | "AFRH" => afr_register = Some(register),
            _ => {}
        }
    }

    let (Some(moder_register), Some(afr_register)) = (moder_register, afr_register) else {
        return Ok(None);
    };
    let Some(moder_field) =
        try_resolve_register_field(&moder_register, &format!("MODER{pin_index}"))
    else {
        return Ok(None);
    };
    let afr_field_name = if pin_index < 8 {
        format!("AFRL{pin_index}")
    } else {
        format!("AFRH{pin_index}")
    };
    let Some(afr_field) = try_resolve_register_field(&afr_register, &afr_field_name) else {
        return Ok(None);
    };

    let mut code = format!(
        "    /// Configure the {} {} route on {}.\n",
        render_comment_text(&driver.name),
        render_comment_text(&route.signal),
        render_comment_text(&pin.name),
    );
    let method_name = format!(
        "configure_{}_route",
        to_rust_method_name(&format!("{}_{}", pin_role.role, pin.name))
    );
    code.push_str(&format!(
        "    pub fn {method_name}(&self) -> Result<(), metadata::Error> {{\n"
    ));
    code.push_str(&render_register_write_statement(
        &moder_register,
        &moder_field,
        0b10,
        "        ",
    )?);
    code.push_str(&render_register_write_statement(
        &afr_register,
        &afr_field,
        alternate_function,
        "        ",
    )?);
    code.push_str("        Ok(())\n    }\n");
    Ok(Some(GeneratedMethod {
        name: method_name,
        code,
    }))
}

fn render_gpio_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if driver.driver_kind != "gpio-port" {
        return Ok(Vec::new());
    }

    let lowering = resolve_gpio_port_lowering(model, driver)?;
    let flex_type = format!("{}Flex", driver.type_name);
    let mut methods = Vec::new();
    match &lowering {
        ResolvedGpioPortLowering::IndexedFields {
            moder,
            pupdr,
            idr,
            odr,
            bsrr,
            pins,
        } => {
            for pin in pins {
                let mut code = format!(
                    "    /// Access the {} pin on {}.\n",
                    render_comment_text(&pin.pin_name),
                    render_comment_text(&driver.name)
                );
                code.push_str(&format!(
                    "    pub fn {}(&self) -> {} {{\n",
                    pin.accessor_name, flex_type
                ));
                code.push_str(&format!(
                    "        {} {{\n            resources: self.resources,\n            role: &self.resources.pins[{}],\n            pin_name: {},\n            moder_addr: 0x{:X}u64,\n            moder_clear_mask: 0x{:08X}u32,\n            moder_output_mask: 0x{:08X}u32,\n            pupdr_addr: 0x{:X}u64,\n            pupdr_clear_mask: 0x{:08X}u32,\n            pupdr_up_mask: 0x{:08X}u32,\n            pupdr_down_mask: 0x{:08X}u32,\n            idr_addr: 0x{:X}u64,\n            idr_mask: 0x{:08X}u32,\n            odr_addr: 0x{:X}u64,\n            odr_mask: 0x{:08X}u32,\n            bsrr_addr: 0x{:X}u64,\n            bsrr_set_mask: 0x{:08X}u32,\n            bsrr_reset_mask: 0x{:08X}u32,\n        }}\n",
                    flex_type,
                    pin.role_index,
                    render_rust_string(&pin.pin_name),
                    moder.absolute_address,
                    pin.moder_clear_mask,
                    pin.moder_output_mask,
                    pupdr.absolute_address,
                    pin.pupdr_clear_mask,
                    pin.pupdr_up_mask,
                    pin.pupdr_down_mask,
                    idr.absolute_address,
                    pin.idr_mask,
                    odr.absolute_address,
                    pin.odr_mask,
                    bsrr.absolute_address,
                    pin.bsrr_set_mask,
                    pin.bsrr_reset_mask,
                ));
                code.push_str("    }\n");
                methods.push(GeneratedMethod {
                    name: pin.accessor_name.clone(),
                    code,
                });
            }
        }
        ResolvedGpioPortLowering::WchCfgRegisters { idr, odr, pins } => {
            for pin in pins {
                let mut code = format!(
                    "    /// Access the {} pin on {}.\n",
                    render_comment_text(&pin.pin_name),
                    render_comment_text(&driver.name)
                );
                code.push_str(&format!(
                    "    pub fn {}(&self) -> {} {{\n",
                    pin.accessor_name, flex_type
                ));
                code.push_str(&format!(
                    "        {} {{\n            resources: self.resources,\n            role: &self.resources.pins[{}],\n            pin_name: {},\n            cfg_addr: 0x{:X}u64,\n            cfg_clear_mask: 0x{:08X}u32,\n            cfg_input_float_mask: 0x{:08X}u32,\n            cfg_input_pull_mask: 0x{:08X}u32,\n            cfg_output_mask: 0x{:08X}u32,\n            idr_addr: 0x{:X}u64,\n            idr_mask: 0x{:08X}u32,\n            odr_addr: 0x{:X}u64,\n            odr_mask: 0x{:08X}u32,\n        }}\n",
                    flex_type,
                    pin.role_index,
                    render_rust_string(&pin.pin_name),
                    pin.cfg_addr,
                    pin.cfg_clear_mask,
                    pin.cfg_input_float_mask,
                    pin.cfg_input_pull_mask,
                    pin.cfg_output_mask,
                    idr.absolute_address,
                    pin.idr_mask,
                    odr.absolute_address,
                    pin.odr_mask,
                ));
                code.push_str("    }\n");
                methods.push(GeneratedMethod {
                    name: pin.accessor_name.clone(),
                    code,
                });
            }
        }
        ResolvedGpioPortLowering::BitmaskRegisters {
            dir,
            afsel,
            den,
            pur,
            pdr,
            pins,
        } => {
            for pin in pins {
                let mut code = format!(
                    "    /// Access the {} pin on {}.\n",
                    render_comment_text(&pin.pin_name),
                    render_comment_text(&driver.name)
                );
                code.push_str(&format!(
                    "    pub fn {}(&self) -> {} {{\n",
                    pin.accessor_name, flex_type
                ));
                code.push_str(&format!(
                    "        {} {{\n            resources: self.resources,\n            role: &self.resources.pins[{}],\n            pin_name: {},\n            dir_addr: 0x{:X}u64,\n            afsel_addr: 0x{:X}u64,\n            den_addr: 0x{:X}u64,\n            pur_addr: 0x{:X}u64,\n            pdr_addr: 0x{:X}u64,\n            data_alias_addr: 0x{:X}u64,\n            bit_mask: 0x{:08X}u32,\n        }}\n",
                    flex_type,
                    pin.role_index,
                    render_rust_string(&pin.pin_name),
                    dir.absolute_address,
                    afsel.absolute_address,
                    den.absolute_address,
                    pur.absolute_address,
                    pdr.absolute_address,
                    pin.data_alias_address,
                    pin.bit_mask,
                ));
                code.push_str("    }\n");
                methods.push(GeneratedMethod {
                    name: pin.accessor_name.clone(),
                    code,
                });
            }
        }
        ResolvedGpioPortLowering::EspRegisters {
            out,
            out_w1ts,
            out_w1tc,
            enable,
            enable_w1ts,
            enable_w1tc,
            input,
            pins,
        } => {
            for pin in pins {
                let mut code = format!(
                    "    /// Access the {} pin on {}.\n",
                    render_comment_text(&pin.pin_name),
                    render_comment_text(&driver.name)
                );
                code.push_str(&format!(
                    "    pub fn {}(&self) -> {} {{\n",
                    pin.accessor_name, flex_type
                ));
                code.push_str(&format!(
                    "        {} {{\n            resources: self.resources,\n            role: &self.resources.pins[{}],\n            pin_name: {},\n            out_addr: 0x{:X}u64,\n            out_w1ts_addr: 0x{:X}u64,\n            out_w1tc_addr: 0x{:X}u64,\n            enable_addr: 0x{:X}u64,\n            enable_w1ts_addr: 0x{:X}u64,\n            enable_w1tc_addr: 0x{:X}u64,\n            input_addr: 0x{:X}u64,\n            out_sel_cfg_addr: 0x{:X}u64,\n            out_sel_clear_mask: 0x{:08X}u32,\n            out_sel_gpio_mask: 0x{:08X}u32,\n            inv_sel_mask: 0x{:08X}u32,\n            oen_sel_mask: 0x{:08X}u32,\n            oen_inv_sel_mask: 0x{:08X}u32,\n            io_mux_addr: 0x{:X}u64,\n            mcu_sel_mask: 0x{:08X}u32,\n            bit_mask: 0x{:08X}u32,\n            fun_wpd_mask: 0x{:08X}u32,\n            fun_wpu_mask: 0x{:08X}u32,\n            fun_ie_mask: 0x{:08X}u32,\n        }}\n",
                    flex_type,
                    pin.role_index,
                    render_rust_string(&pin.pin_name),
                    out.absolute_address,
                    out_w1ts.absolute_address,
                    out_w1tc.absolute_address,
                    enable.absolute_address,
                    enable_w1ts.absolute_address,
                    enable_w1tc.absolute_address,
                    input.absolute_address,
                    pin.out_sel_cfg_addr,
                    pin.out_sel_clear_mask,
                    pin.out_sel_gpio_mask,
                    pin.inv_sel_mask,
                    pin.oen_sel_mask,
                    pin.oen_inv_sel_mask,
                    pin.io_mux_addr,
                    pin.mcu_sel_mask,
                    pin.bit_mask,
                    pin.fun_wpd_mask,
                    pin.fun_wpu_mask,
                    pin.fun_ie_mask,
                ));
                code.push_str("    }\n");
                methods.push(GeneratedMethod {
                    name: pin.accessor_name.clone(),
                    code,
                });
            }
        }
    }
    Ok(methods)
}

fn render_gpio_support_items(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    if driver.driver_kind != "gpio-port" {
        return Ok(String::new());
    }

    let lowering = resolve_gpio_port_lowering(model, driver)?;
    let flex_type = format!("{}Flex", driver.type_name);
    let input_type = format!("{}Input", driver.type_name);
    let output_type = format!("{}Output", driver.type_name);
    let mut out = String::new();

    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str(&format!("pub struct {flex_type} {{\n"));
    out.push_str(&format!("    resources: {}Resources,\n", driver.type_name));
    out.push_str("    role: &'static metadata::PinRole,\n");
    out.push_str("    pin_name: &'static str,\n");
    match &lowering {
        ResolvedGpioPortLowering::IndexedFields { .. } => {
            out.push_str("    moder_addr: u64,\n");
            out.push_str("    moder_clear_mask: u32,\n");
            out.push_str("    moder_output_mask: u32,\n");
            out.push_str("    pupdr_addr: u64,\n");
            out.push_str("    pupdr_clear_mask: u32,\n");
            out.push_str("    pupdr_up_mask: u32,\n");
            out.push_str("    pupdr_down_mask: u32,\n");
            out.push_str("    idr_addr: u64,\n");
            out.push_str("    idr_mask: u32,\n");
            out.push_str("    odr_addr: u64,\n");
            out.push_str("    odr_mask: u32,\n");
            out.push_str("    bsrr_addr: u64,\n");
            out.push_str("    bsrr_set_mask: u32,\n");
            out.push_str("    bsrr_reset_mask: u32,\n");
        }
        ResolvedGpioPortLowering::WchCfgRegisters { .. } => {
            out.push_str("    cfg_addr: u64,\n");
            out.push_str("    cfg_clear_mask: u32,\n");
            out.push_str("    cfg_input_float_mask: u32,\n");
            out.push_str("    cfg_input_pull_mask: u32,\n");
            out.push_str("    cfg_output_mask: u32,\n");
            out.push_str("    idr_addr: u64,\n");
            out.push_str("    idr_mask: u32,\n");
            out.push_str("    odr_addr: u64,\n");
            out.push_str("    odr_mask: u32,\n");
        }
        ResolvedGpioPortLowering::BitmaskRegisters { .. } => {
            out.push_str("    dir_addr: u64,\n");
            out.push_str("    afsel_addr: u64,\n");
            out.push_str("    den_addr: u64,\n");
            out.push_str("    pur_addr: u64,\n");
            out.push_str("    pdr_addr: u64,\n");
            out.push_str("    data_alias_addr: u64,\n");
            out.push_str("    bit_mask: u32,\n");
        }
        ResolvedGpioPortLowering::EspRegisters { .. } => {
            out.push_str("    out_addr: u64,\n");
            out.push_str("    out_w1ts_addr: u64,\n");
            out.push_str("    out_w1tc_addr: u64,\n");
            out.push_str("    enable_addr: u64,\n");
            out.push_str("    enable_w1ts_addr: u64,\n");
            out.push_str("    enable_w1tc_addr: u64,\n");
            out.push_str("    input_addr: u64,\n");
            out.push_str("    out_sel_cfg_addr: u64,\n");
            out.push_str("    out_sel_clear_mask: u32,\n");
            out.push_str("    out_sel_gpio_mask: u32,\n");
            out.push_str("    inv_sel_mask: u32,\n");
            out.push_str("    oen_sel_mask: u32,\n");
            out.push_str("    oen_inv_sel_mask: u32,\n");
            out.push_str("    io_mux_addr: u64,\n");
            out.push_str("    mcu_sel_mask: u32,\n");
            out.push_str("    bit_mask: u32,\n");
            out.push_str("    fun_wpd_mask: u32,\n");
            out.push_str("    fun_wpu_mask: u32,\n");
            out.push_str("    fun_ie_mask: u32,\n");
        }
    }
    out.push_str("}\n\n");

    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str(&format!("pub struct {input_type} {{\n"));
    out.push_str(&format!("    pin: {flex_type},\n"));
    out.push_str("}\n\n");

    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str(&format!("pub struct {output_type} {{\n"));
    out.push_str(&format!("    pin: {flex_type},\n"));
    out.push_str("}\n\n");

    out.push_str(&format!("impl {flex_type} {{\n"));
    out.push_str(&format!(
        "    pub fn resources(&self) -> {}Resources {{\n",
        driver.type_name
    ));
    out.push_str("        self.resources\n    }\n\n");
    out.push_str("    pub fn role(&self) -> &'static metadata::PinRole {\n");
    out.push_str("        self.role\n    }\n\n");
    out.push_str("    pub fn pin_name(&self) -> &'static str {\n");
    out.push_str("        self.pin_name\n    }\n\n");
    out.push_str(&format!(
        "    pub fn into_input(self, pull: Pull) -> Result<{input_type}, metadata::Error> {{\n"
    ));
    out.push_str("        self.set_as_input(pull)?;\n");
    out.push_str(&format!("        Ok({input_type} {{ pin: self }})\n"));
    out.push_str("    }\n\n");
    out.push_str(&format!("    pub fn into_output(self, initial_level: Level) -> Result<{output_type}, metadata::Error> {{\n"));
    out.push_str("        self.set_as_output(initial_level)?;\n");
    out.push_str(&format!("        Ok({output_type} {{ pin: self }})\n"));
    out.push_str("    }\n\n");
    match &lowering {
        ResolvedGpioPortLowering::IndexedFields { .. } => {
            out.push_str(
                "    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        self.set_pull(pull)?;\n");
            out.push_str(
                "        modify_u32(self.moder_addr, self.moder_clear_mask, 0x00000000u32)?;\n",
            );
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        self.set_level(initial_level)?;\n");
            out.push_str(
                "        modify_u32(self.moder_addr, self.moder_clear_mask, self.moder_output_mask)?;\n",
            );
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        let set_mask = match pull {\n");
            out.push_str("            Pull::None => 0x00000000u32,\n");
            out.push_str("            Pull::Up => self.pupdr_up_mask,\n");
            out.push_str("            Pull::Down => self.pupdr_down_mask,\n");
            out.push_str("        };\n");
            out.push_str(
                "        modify_u32(self.pupdr_addr, self.pupdr_clear_mask, set_mask)?;\n",
            );
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_high(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok((read_u32(self.idr_addr)? & self.idr_mask) != 0)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_low(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok(!self.is_high()?)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn get_level(&self) -> Result<Level, metadata::Error> {\n");
            out.push_str("        Ok(if self.is_high()? { Level::High } else { Level::Low })\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok((read_u32(self.odr_addr)? & self.odr_mask) != 0)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok(!self.is_set_high()?)\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {\n",
            );
            out.push_str(
                "        Ok(if self.is_set_high()? { Level::High } else { Level::Low })\n",
            );
            out.push_str("    }\n\n");
            out.push_str("    pub fn set_high(&self) -> Result<(), metadata::Error> {\n");
            out.push_str("        write_u32(self.bsrr_addr, self.bsrr_set_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn set_low(&self) -> Result<(), metadata::Error> {\n");
            out.push_str("        write_u32(self.bsrr_addr, self.bsrr_reset_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
        }
        ResolvedGpioPortLowering::WchCfgRegisters { .. } => {
            out.push_str(
                "    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        self.set_pull(pull)\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        self.set_level(initial_level)?;\n");
            out.push_str(
                "        modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_output_mask)?;\n",
            );
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        match pull {\n");
            out.push_str("            Pull::None => {\n");
            out.push_str(
                "                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_float_mask)?;\n",
            );
            out.push_str("            }\n");
            out.push_str("            Pull::Up => {\n");
            out.push_str(
                "                modify_u32(self.odr_addr, self.odr_mask, self.odr_mask)?;\n",
            );
            out.push_str(
                "                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_pull_mask)?;\n",
            );
            out.push_str("            }\n");
            out.push_str("            Pull::Down => {\n");
            out.push_str(
                "                modify_u32(self.odr_addr, self.odr_mask, 0x00000000u32)?;\n",
            );
            out.push_str(
                "                modify_u32(self.cfg_addr, self.cfg_clear_mask, self.cfg_input_pull_mask)?;\n",
            );
            out.push_str("            }\n");
            out.push_str("        }\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_high(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok((read_u32(self.idr_addr)? & self.idr_mask) != 0)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_low(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok(!self.is_high()?)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn get_level(&self) -> Result<Level, metadata::Error> {\n");
            out.push_str("        Ok(if self.is_high()? { Level::High } else { Level::Low })\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok((read_u32(self.odr_addr)? & self.odr_mask) != 0)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok(!self.is_set_high()?)\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {\n",
            );
            out.push_str(
                "        Ok(if self.is_set_high()? { Level::High } else { Level::Low })\n",
            );
            out.push_str("    }\n\n");
            out.push_str("    pub fn set_high(&self) -> Result<(), metadata::Error> {\n");
            out.push_str("        modify_u32(self.odr_addr, self.odr_mask, self.odr_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn set_low(&self) -> Result<(), metadata::Error> {\n");
            out.push_str("        modify_u32(self.odr_addr, self.odr_mask, 0x00000000u32)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
        }
        ResolvedGpioPortLowering::BitmaskRegisters { .. } => {
            out.push_str(
                "    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        self.set_pull(pull)?;\n");
            out.push_str("        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;\n");
            out.push_str("        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;\n");
            out.push_str("        modify_u32(self.dir_addr, self.bit_mask, 0x00000000u32)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        modify_u32(self.afsel_addr, self.bit_mask, 0x00000000u32)?;\n");
            out.push_str("        modify_u32(self.den_addr, self.bit_mask, self.bit_mask)?;\n");
            out.push_str("        self.set_level(initial_level)?;\n");
            out.push_str("        modify_u32(self.dir_addr, self.bit_mask, self.bit_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        match pull {\n");
            out.push_str("            Pull::None => {\n");
            out.push_str(
                "                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;\n",
            );
            out.push_str(
                "                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;\n",
            );
            out.push_str("            }\n");
            out.push_str("            Pull::Up => {\n");
            out.push_str(
                "                modify_u32(self.pdr_addr, self.bit_mask, 0x00000000u32)?;\n",
            );
            out.push_str(
                "                modify_u32(self.pur_addr, self.bit_mask, self.bit_mask)?;\n",
            );
            out.push_str("            }\n");
            out.push_str("            Pull::Down => {\n");
            out.push_str(
                "                modify_u32(self.pur_addr, self.bit_mask, 0x00000000u32)?;\n",
            );
            out.push_str(
                "                modify_u32(self.pdr_addr, self.bit_mask, self.bit_mask)?;\n",
            );
            out.push_str("            }\n");
            out.push_str("        }\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_high(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok(read_u32(self.data_alias_addr)? != 0)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_low(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok(!self.is_high()?)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn get_level(&self) -> Result<Level, metadata::Error> {\n");
            out.push_str("        Ok(if self.is_high()? { Level::High } else { Level::Low })\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        self.is_high()\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        self.is_low()\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {\n",
            );
            out.push_str("        self.get_level()\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn set_high(&self) -> Result<(), metadata::Error> {\n");
            out.push_str("        write_u32(self.data_alias_addr, self.bit_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn set_low(&self) -> Result<(), metadata::Error> {\n");
            out.push_str("        write_u32(self.data_alias_addr, 0x00000000u32)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
        }
        ResolvedGpioPortLowering::EspRegisters { .. } => {
            out.push_str(
                "    pub fn set_as_input(&self, pull: Pull) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        self.set_pull(pull)?;\n");
            out.push_str(
                "        modify_u32(self.io_mux_addr, self.mcu_sel_mask | self.fun_ie_mask, self.fun_ie_mask)?;\n",
            );
            out.push_str("        write_u32(self.enable_w1tc_addr, self.bit_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn set_as_output(&self, initial_level: Level) -> Result<(), metadata::Error> {\n",
            );
            out.push_str(
                "        modify_u32(self.out_sel_cfg_addr, self.out_sel_clear_mask | self.inv_sel_mask | self.oen_sel_mask | self.oen_inv_sel_mask, self.out_sel_gpio_mask)?;\n",
            );
            out.push_str("        self.set_level(initial_level)?;\n");
            out.push_str(
                "        modify_u32(self.io_mux_addr, self.mcu_sel_mask | self.fun_ie_mask, 0x00000000u32)?;\n",
            );
            out.push_str("        write_u32(self.enable_w1ts_addr, self.bit_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {\n",
            );
            out.push_str("        let set_mask = match pull {\n");
            out.push_str("            Pull::None => 0x00000000u32,\n");
            out.push_str("            Pull::Up => self.fun_wpu_mask,\n");
            out.push_str("            Pull::Down => self.fun_wpd_mask,\n");
            out.push_str("        };\n");
            out.push_str(
                "        modify_u32(self.io_mux_addr, self.fun_wpu_mask | self.fun_wpd_mask, set_mask)?;\n",
            );
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_high(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok((read_u32(self.input_addr)? & self.bit_mask) != 0)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_low(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok(!self.is_high()?)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn get_level(&self) -> Result<Level, metadata::Error> {\n");
            out.push_str("        Ok(if self.is_high()? { Level::High } else { Level::Low })\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok((read_u32(self.out_addr)? & self.bit_mask) != 0)\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {\n");
            out.push_str("        Ok(!self.is_set_high()?)\n");
            out.push_str("    }\n\n");
            out.push_str(
                "    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {\n",
            );
            out.push_str(
                "        Ok(if self.is_set_high()? { Level::High } else { Level::Low })\n",
            );
            out.push_str("    }\n\n");
            out.push_str("    pub fn set_high(&self) -> Result<(), metadata::Error> {\n");
            out.push_str("        write_u32(self.out_w1ts_addr, self.bit_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn set_low(&self) -> Result<(), metadata::Error> {\n");
            out.push_str("        write_u32(self.out_w1tc_addr, self.bit_mask)?;\n");
            out.push_str("        Ok(())\n");
            out.push_str("    }\n\n");
        }
    }
    out.push_str("    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {\n");
    out.push_str("        match level {\n");
    out.push_str("            Level::Low => self.set_low(),\n");
    out.push_str("            Level::High => self.set_high(),\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str(&format!("impl {input_type} {{\n"));
    out.push_str(&format!("    pub fn into_flex(self) -> {flex_type} {{\n"));
    out.push_str("        self.pin\n    }\n\n");
    out.push_str("    pub fn pin_name(&self) -> &'static str {\n");
    out.push_str("        self.pin.pin_name()\n    }\n\n");
    out.push_str("    pub fn role(&self) -> &'static metadata::PinRole {\n");
    out.push_str("        self.pin.role()\n    }\n\n");
    out.push_str("    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {\n");
    out.push_str("        self.pin.set_pull(pull)\n    }\n\n");
    out.push_str("    pub fn is_high(&self) -> Result<bool, metadata::Error> {\n");
    out.push_str("        self.pin.is_high()\n    }\n\n");
    out.push_str("    pub fn is_low(&self) -> Result<bool, metadata::Error> {\n");
    out.push_str("        self.pin.is_low()\n    }\n\n");
    out.push_str("    pub fn get_level(&self) -> Result<Level, metadata::Error> {\n");
    out.push_str("        self.pin.get_level()\n    }\n");
    out.push_str("}\n\n");

    out.push_str(&format!("impl {output_type} {{\n"));
    out.push_str(&format!("    pub fn into_flex(self) -> {flex_type} {{\n"));
    out.push_str("        self.pin\n    }\n\n");
    out.push_str("    pub fn pin_name(&self) -> &'static str {\n");
    out.push_str("        self.pin.pin_name()\n    }\n\n");
    out.push_str("    pub fn role(&self) -> &'static metadata::PinRole {\n");
    out.push_str("        self.pin.role()\n    }\n\n");
    out.push_str("    pub fn set_pull(&self, pull: Pull) -> Result<(), metadata::Error> {\n");
    out.push_str("        self.pin.set_pull(pull)\n    }\n\n");
    out.push_str("    pub fn set_high(&self) -> Result<(), metadata::Error> {\n");
    out.push_str("        self.pin.set_high()\n    }\n\n");
    out.push_str("    pub fn set_low(&self) -> Result<(), metadata::Error> {\n");
    out.push_str("        self.pin.set_low()\n    }\n\n");
    out.push_str("    pub fn set_level(&self, level: Level) -> Result<(), metadata::Error> {\n");
    out.push_str("        self.pin.set_level(level)\n    }\n\n");
    out.push_str("    pub fn is_set_high(&self) -> Result<bool, metadata::Error> {\n");
    out.push_str("        self.pin.is_set_high()\n    }\n\n");
    out.push_str("    pub fn is_set_low(&self) -> Result<bool, metadata::Error> {\n");
    out.push_str("        self.pin.is_set_low()\n    }\n\n");
    out.push_str("    pub fn get_output_level(&self) -> Result<Level, metadata::Error> {\n");
    out.push_str("        self.pin.get_output_level()\n    }\n");
    out.push_str("}\n\n");

    Ok(out)
}

fn resolve_gpio_port_lowering(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<ResolvedGpioPortLowering> {
    let target_ref = driver.target.target_ref.as_str();
    if supports_gpio_register_layout(model, target_ref, &["MODER", "PUPDR", "IDR", "ODR", "BSRR"])?
    {
        return resolve_indexed_field_gpio_port_lowering(model, driver);
    }
    if supports_gpio_register_layout(model, target_ref, &["CFGLR", "CFGHR", "INDR", "OUTDR"])? {
        return resolve_wch_cfg_gpio_port_lowering(model, driver);
    }
    if supports_gpio_register_layout(
        model,
        target_ref,
        &["DIR", "AFSEL", "DEN", "PUR", "PDR", "DATA"],
    )? {
        return resolve_bitmask_gpio_port_lowering(model, driver);
    }
    if supports_gpio_register_layout(
        model,
        target_ref,
        &[
            "OUT",
            "OUT_W1TS",
            "OUT_W1TC",
            "ENABLE",
            "ENABLE_W1TS",
            "ENABLE_W1TC",
            "IN",
        ],
    )? {
        return resolve_esp_gpio_port_lowering(model, driver);
    }
    bail!(
        "gpio-port driver {} target {} does not expose a supported GPIO register layout; expected either indexed-field registers [MODER, PUPDR, IDR, ODR, BSRR], CH32 CFG registers [CFGLR, CFGHR, INDR, OUTDR], bitmask registers [DIR, AFSEL, DEN, PUR, PDR, DATA], or ESP-style registers [OUT, OUT_W1TS, OUT_W1TC, ENABLE, ENABLE_W1TS, ENABLE_W1TC, IN]",
        driver.id,
        target_ref
    );
}

fn supports_gpio_register_layout(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    register_names: &[&str],
) -> Result<bool> {
    for register_name in register_names {
        if try_resolve_target_register_by_name(model, target_ref, register_name)?.is_none() {
            return Ok(false);
        }
    }
    Ok(true)
}

fn resolve_indexed_field_gpio_port_lowering(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<ResolvedGpioPortLowering> {
    let target_ref = driver.target.target_ref.as_str();
    let moder = resolve_gpio_register32(model, target_ref, "MODER", driver)?;
    let pupdr = resolve_gpio_register32(model, target_ref, "PUPDR", driver)?;
    let idr = resolve_gpio_register32(model, target_ref, "IDR", driver)?;
    let odr = resolve_gpio_register32(model, target_ref, "ODR", driver)?;
    let bsrr = resolve_gpio_register32(model, target_ref, "BSRR", driver)?;
    let mut pins = Vec::with_capacity(driver.pin_roles.len());
    let mut seen_accessors = BTreeSet::new();

    for (role_index, pin_role) in driver.pin_roles.iter().enumerate() {
        let [route] = pin_role.routes.as_slice() else {
            bail!(
                "gpio-port driver {} pin role {} requires exactly one route for first-cut per-pin lowering, found {}",
                driver.id,
                pin_role.role,
                pin_role.routes.len()
            );
        };
        if route.peripheral_ref != target_ref {
            bail!(
                "gpio-port driver {} pin role {} references pin route {} for {} instead of {}",
                driver.id,
                pin_role.role,
                route.id,
                route.peripheral_ref,
                target_ref
            );
        }
        let pin = model
            .pins
            .iter()
            .find(|candidate| candidate.id == route.pin_ref)
            .ok_or_else(|| {
                anyhow!(
                    "gpio-port driver {} route {} references unknown physical pin {}",
                    driver.id,
                    route.id,
                    route.pin_ref
                )
            })?;
        let pin_index = pin.index.ok_or_else(|| {
            anyhow!(
                "gpio-port driver {} route {} references pin {} without an index required for lowering",
                driver.id,
                route.id,
                pin.id
            )
        })?;
        let accessor_name = to_rust_method_name(&pin.name);
        if !seen_accessors.insert(accessor_name.clone()) {
            bail!(
                "gpio-port driver {} would generate duplicate pin accessor {}",
                driver.id,
                accessor_name
            );
        }

        let moder_field = resolve_register_field(&moder, &format!("MODER{pin_index}"))?;
        validate_field_width(&moder_field, &moder, 2)?;
        let pupdr_field = resolve_register_field(&pupdr, &format!("PUPDR{pin_index}"))?;
        validate_field_width(&pupdr_field, &pupdr, 2)?;
        let idr_field = resolve_register_field(&idr, &format!("IDR{pin_index}"))?;
        let odr_field = resolve_register_field(&odr, &format!("ODR{pin_index}"))?;
        let bs_field = resolve_register_field(&bsrr, &format!("BS{pin_index}"))?;
        let br_field = resolve_register_field(&bsrr, &format!("BR{pin_index}"))?;
        let moder_shift = moder_field.lsb;
        let pupdr_shift = pupdr_field.lsb;

        pins.push(ResolvedIndexedFieldGpioPinLowering {
            role_index,
            pin_name: pin.name.clone(),
            accessor_name,
            moder_clear_mask: shifted_field_mask(&moder_field, &moder)?,
            moder_output_mask: 1u32 << moder_shift,
            pupdr_clear_mask: shifted_field_mask(&pupdr_field, &pupdr)?,
            pupdr_up_mask: 1u32 << pupdr_shift,
            pupdr_down_mask: 2u32 << pupdr_shift,
            idr_mask: field_bit_mask(&idr_field, &idr)?,
            odr_mask: field_bit_mask(&odr_field, &odr)?,
            bsrr_set_mask: field_bit_mask(&bs_field, &bsrr)?,
            bsrr_reset_mask: field_bit_mask(&br_field, &bsrr)?,
        });
    }

    Ok(ResolvedGpioPortLowering::IndexedFields {
        moder,
        pupdr,
        idr,
        odr,
        bsrr,
        pins,
    })
}

fn resolve_bitmask_gpio_port_lowering(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<ResolvedGpioPortLowering> {
    let target_ref = driver.target.target_ref.as_str();
    let dir = resolve_gpio_register32(model, target_ref, "DIR", driver)?;
    let afsel = resolve_gpio_register32(model, target_ref, "AFSEL", driver)?;
    let den = resolve_gpio_register32(model, target_ref, "DEN", driver)?;
    let pur = resolve_gpio_register32(model, target_ref, "PUR", driver)?;
    let pdr = resolve_gpio_register32(model, target_ref, "PDR", driver)?;
    let data = resolve_gpio_register32(model, target_ref, "DATA", driver)?;
    let mut pins = Vec::with_capacity(driver.pin_roles.len());
    let mut seen_accessors = BTreeSet::new();

    for (role_index, pin_role) in driver.pin_roles.iter().enumerate() {
        let [route] = pin_role.routes.as_slice() else {
            bail!(
                "gpio-port driver {} pin role {} requires exactly one route for first-cut per-pin lowering, found {}",
                driver.id,
                pin_role.role,
                pin_role.routes.len()
            );
        };
        if route.peripheral_ref != target_ref {
            bail!(
                "gpio-port driver {} pin role {} references pin route {} for {} instead of {}",
                driver.id,
                pin_role.role,
                route.id,
                route.peripheral_ref,
                target_ref
            );
        }
        let pin = model
            .pins
            .iter()
            .find(|candidate| candidate.id == route.pin_ref)
            .ok_or_else(|| {
                anyhow!(
                    "gpio-port driver {} route {} references unknown physical pin {}",
                    driver.id,
                    route.id,
                    route.pin_ref
                )
            })?;
        let pin_index = pin.index.ok_or_else(|| {
            anyhow!(
                "gpio-port driver {} route {} references pin {} without an index required for lowering",
                driver.id,
                route.id,
                pin.id
            )
        })?;
        let accessor_name = to_rust_method_name(&pin.name);
        if !seen_accessors.insert(accessor_name.clone()) {
            bail!(
                "gpio-port driver {} would generate duplicate pin accessor {}",
                driver.id,
                accessor_name
            );
        }

        let bit_mask = resolve_bitmask_gpio_bit_mask(&dir, pin_index, driver, "DIR")?;
        let gpio_block_base = gpio_register_block_base(&dir);
        for (register, name) in [
            (&afsel, "AFSEL"),
            (&den, "DEN"),
            (&pur, "PUR"),
            (&pdr, "PDR"),
            (&data, "DATA"),
        ] {
            let register_mask = resolve_bitmask_gpio_bit_mask(register, pin_index, driver, name)?;
            if register_mask != bit_mask {
                bail!(
                    "gpio-port driver {} pin index {} uses inconsistent bit masks across bitmask GPIO registers: DIR=0x{:X}, {}=0x{:X}",
                    driver.id,
                    pin_index,
                    bit_mask,
                    name,
                    register_mask
                );
            }
            let register_block_base = gpio_register_block_base(register);
            if register_block_base != gpio_block_base {
                bail!(
                    "gpio-port driver {} register {} is in GPIO block 0x{:X}, expected 0x{:X} to match DIR",
                    driver.id,
                    register.id,
                    register_block_base,
                    gpio_block_base
                );
            }
        }
        let data_alias_address = resolve_bitmask_gpio_data_alias_address(&data, bit_mask, driver)?;

        pins.push(ResolvedBitmaskGpioPinLowering {
            role_index,
            pin_name: pin.name.clone(),
            accessor_name,
            bit_mask,
            data_alias_address,
        });
    }

    Ok(ResolvedGpioPortLowering::BitmaskRegisters {
        dir,
        afsel,
        den,
        pur,
        pdr,
        pins,
    })
}

fn resolve_wch_cfg_gpio_port_lowering(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<ResolvedGpioPortLowering> {
    let target_ref = driver.target.target_ref.as_str();
    let cfglr = resolve_gpio_register32(model, target_ref, "CFGLR", driver)?;
    let cfghr = resolve_gpio_register32(model, target_ref, "CFGHR", driver)?;
    let idr = resolve_gpio_register32(model, target_ref, "INDR", driver)?;
    let odr = resolve_gpio_register32(model, target_ref, "OUTDR", driver)?;
    let mut pins = Vec::with_capacity(driver.pin_roles.len());
    let mut seen_accessors = BTreeSet::new();

    for (role_index, pin_role) in driver.pin_roles.iter().enumerate() {
        let [route] = pin_role.routes.as_slice() else {
            bail!(
                "gpio-port driver {} pin role {} requires exactly one route for first-cut per-pin lowering, found {}",
                driver.id,
                pin_role.role,
                pin_role.routes.len()
            );
        };
        if route.peripheral_ref != target_ref {
            bail!(
                "gpio-port driver {} pin role {} references pin route {} for {} instead of {}",
                driver.id,
                pin_role.role,
                route.id,
                route.peripheral_ref,
                target_ref
            );
        }
        let pin = model
            .pins
            .iter()
            .find(|candidate| candidate.id == route.pin_ref)
            .ok_or_else(|| {
                anyhow!(
                    "gpio-port driver {} route {} references unknown physical pin {}",
                    driver.id,
                    route.id,
                    route.pin_ref
                )
            })?;
        let pin_index = pin.index.ok_or_else(|| {
            anyhow!(
                "gpio-port driver {} route {} references pin {} without an index required for lowering",
                driver.id,
                route.id,
                pin.id
            )
        })?;
        let accessor_name = to_rust_method_name(&pin.name);
        if !seen_accessors.insert(accessor_name.clone()) {
            bail!(
                "gpio-port driver {} would generate duplicate pin accessor {}",
                driver.id,
                accessor_name
            );
        }

        let cfg_register = if pin_index < 8 { &cfglr } else { &cfghr };
        let cfg_shift = (pin_index % 8) * 4;
        let cfg_clear_mask = 0xFu32 << cfg_shift;
        let idr_field = resolve_register_field(&idr, &format!("IDR{pin_index}"))?;
        let odr_field = resolve_register_field(&odr, &format!("ODR{pin_index}"))?;

        pins.push(ResolvedWchCfgGpioPinLowering {
            role_index,
            pin_name: pin.name.clone(),
            accessor_name,
            cfg_addr: cfg_register.absolute_address,
            cfg_clear_mask,
            cfg_input_float_mask: 0x4u32 << cfg_shift,
            cfg_input_pull_mask: 0x8u32 << cfg_shift,
            cfg_output_mask: 0x3u32 << cfg_shift,
            idr_mask: field_bit_mask(&idr_field, &idr)?,
            odr_mask: field_bit_mask(&odr_field, &odr)?,
        });
    }

    Ok(ResolvedGpioPortLowering::WchCfgRegisters { idr, odr, pins })
}

fn resolve_esp_gpio_port_lowering(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<ResolvedGpioPortLowering> {
    let target_ref = driver.target.target_ref.as_str();
    let out = resolve_gpio_register32(model, target_ref, "OUT", driver)?;
    let out_w1ts = resolve_gpio_register32(model, target_ref, "OUT_W1TS", driver)?;
    let out_w1tc = resolve_gpio_register32(model, target_ref, "OUT_W1TC", driver)?;
    let enable = resolve_gpio_register32(model, target_ref, "ENABLE", driver)?;
    let enable_w1ts = resolve_gpio_register32(model, target_ref, "ENABLE_W1TS", driver)?;
    let enable_w1tc = resolve_gpio_register32(model, target_ref, "ENABLE_W1TC", driver)?;
    let input = resolve_gpio_register32(model, target_ref, "IN", driver)?;
    const ESP_SIG_GPIO_OUT_IDX: u32 = 128;
    let mut pins = Vec::with_capacity(driver.pin_roles.len());
    let mut seen_accessors = BTreeSet::new();

    for (role_index, pin_role) in driver.pin_roles.iter().enumerate() {
        let [route] = pin_role.routes.as_slice() else {
            bail!(
                "gpio-port driver {} pin role {} requires exactly one route for first-cut per-pin lowering, found {}",
                driver.id,
                pin_role.role,
                pin_role.routes.len()
            );
        };
        if route.peripheral_ref != target_ref {
            bail!(
                "gpio-port driver {} pin role {} references pin route {} for {} instead of {}",
                driver.id,
                pin_role.role,
                route.id,
                route.peripheral_ref,
                target_ref
            );
        }
        let pin = model
            .pins
            .iter()
            .find(|candidate| candidate.id == route.pin_ref)
            .ok_or_else(|| {
                anyhow!(
                    "gpio-port driver {} route {} references unknown physical pin {}",
                    driver.id,
                    route.id,
                    route.pin_ref
                )
            })?;
        let pin_index = pin.index.ok_or_else(|| {
            anyhow!(
                "gpio-port driver {} route {} references pin {} without an index required for lowering",
                driver.id,
                route.id,
                pin.id
            )
        })?;
        let accessor_name = to_rust_method_name(&pin.name);
        if !seen_accessors.insert(accessor_name.clone()) {
            bail!(
                "gpio-port driver {} would generate duplicate pin accessor {}",
                driver.id,
                accessor_name
            );
        }

        let bit_mask = resolve_esp_gpio_bit_mask(&out, pin_index, driver, "OUT", "DATA_ORIG")?;
        for (register, register_name, field_name) in [
            (&out_w1ts, "OUT_W1TS", "OUT_W1TS"),
            (&out_w1tc, "OUT_W1TC", "OUT_W1TC"),
            (&enable, "ENABLE", "DATA"),
            (&enable_w1ts, "ENABLE_W1TS", "ENABLE_W1TS"),
            (&enable_w1tc, "ENABLE_W1TC", "ENABLE_W1TC"),
            (&input, "IN", "DATA_NEXT"),
        ] {
            let register_mask =
                resolve_esp_gpio_bit_mask(register, pin_index, driver, register_name, field_name)?;
            if register_mask != bit_mask {
                bail!(
                    "gpio-port driver {} pin index {} uses inconsistent bit masks across ESP GPIO registers: OUT=0x{:X}, {}=0x{:X}",
                    driver.id,
                    pin_index,
                    bit_mask,
                    register_name,
                    register_mask
                );
            }
        }

        let io_mux_register =
            resolve_gpio_register32(model, "per.io_mux", &format!("GPIO{pin_index}"), driver)?;
        let out_sel_cfg_register = resolve_gpio_register32(
            model,
            target_ref,
            &format!("FUNC{pin_index}_OUT_SEL_CFG"),
            driver,
        )?;
        let out_sel = resolve_register_field(&out_sel_cfg_register, "OUT_SEL")?;
        let inv_sel = resolve_register_field(&out_sel_cfg_register, "INV_SEL")?;
        let oen_sel = resolve_register_field(&out_sel_cfg_register, "OEN_SEL")?;
        let oen_inv_sel = resolve_register_field(&out_sel_cfg_register, "OEN_INV_SEL")?;
        let mcu_sel = resolve_register_field(&io_mux_register, "MCU_SEL")?;
        let fun_wpd = resolve_register_field(&io_mux_register, "FUN_WPD")?;
        let fun_wpu = resolve_register_field(&io_mux_register, "FUN_WPU")?;
        let fun_ie = resolve_register_field(&io_mux_register, "FUN_IE")?;
        let out_sel_value_mask = field_value_mask(&out_sel)?;
        if u64::from(ESP_SIG_GPIO_OUT_IDX) > out_sel_value_mask {
            bail!(
                "gpio-port driver {} pin index {} cannot encode SIG_GPIO_OUT_IDX={} into {}.OUT_SEL width {}",
                driver.id,
                pin_index,
                ESP_SIG_GPIO_OUT_IDX,
                out_sel_cfg_register.name,
                out_sel.msb - out_sel.lsb + 1
            );
        }

        pins.push(ResolvedEspGpioPinLowering {
            role_index,
            pin_name: pin.name.clone(),
            accessor_name,
            bit_mask,
            out_sel_cfg_addr: out_sel_cfg_register.absolute_address,
            out_sel_clear_mask: shifted_field_mask(&out_sel, &out_sel_cfg_register)?,
            out_sel_gpio_mask: ESP_SIG_GPIO_OUT_IDX << out_sel.lsb,
            inv_sel_mask: field_bit_mask(&inv_sel, &out_sel_cfg_register)?,
            oen_sel_mask: field_bit_mask(&oen_sel, &out_sel_cfg_register)?,
            oen_inv_sel_mask: field_bit_mask(&oen_inv_sel, &out_sel_cfg_register)?,
            io_mux_addr: io_mux_register.absolute_address,
            mcu_sel_mask: shifted_field_mask(&mcu_sel, &io_mux_register)?,
            fun_wpd_mask: field_bit_mask(&fun_wpd, &io_mux_register)?,
            fun_wpu_mask: field_bit_mask(&fun_wpu, &io_mux_register)?,
            fun_ie_mask: field_bit_mask(&fun_ie, &io_mux_register)?,
        });
    }

    Ok(ResolvedGpioPortLowering::EspRegisters {
        out,
        out_w1ts,
        out_w1tc,
        enable,
        enable_w1ts,
        enable_w1tc,
        input,
        pins,
    })
}

fn resolve_gpio_register32(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    register_name: &str,
    driver: &ResolvedDriverInstance,
) -> Result<ResolvedRegister> {
    let register = try_resolve_target_register_by_name(model, target_ref, register_name)?
        .ok_or_else(|| {
            anyhow!(
                "gpio-port driver {} requires {} on target {} for first-cut lowering",
                driver.id,
                register_name,
                target_ref
            )
        })?;
    if register.width_bits != 32 {
        bail!(
            "gpio-port driver {} requires 32-bit {} but {} uses width {}",
            driver.id,
            register_name,
            register.id,
            register.width_bits
        );
    }
    Ok(register)
}

fn resolve_bitmask_gpio_bit_mask(
    register: &ResolvedRegister,
    pin_index: u32,
    driver: &ResolvedDriverInstance,
    register_name: &str,
) -> Result<u32> {
    let field = resolve_register_field(register, "GPIO")?;
    if field.lsb != 0 {
        bail!(
            "gpio-port driver {} requires {} GPIO field {} to start at bit 0 for bitmask lowering, found lsb {}",
            driver.id,
            register_name,
            field.name,
            field.lsb
        );
    }
    let bit = field.lsb.checked_add(pin_index).ok_or_else(|| {
        anyhow!(
            "gpio-port driver {} pin index {} overflowed {} field {}",
            driver.id,
            pin_index,
            register_name,
            field.name
        )
    })?;
    if bit > field.msb {
        bail!(
            "gpio-port driver {} pin index {} is outside {} field {} bit range {}..={}",
            driver.id,
            pin_index,
            register_name,
            field.name,
            field.lsb,
            field.msb
        );
    }
    1u32.checked_shl(bit).ok_or_else(|| {
        anyhow!(
            "gpio-port driver {} pin index {} produced an invalid bit position {} for {}",
            driver.id,
            pin_index,
            bit,
            register_name
        )
    })
}

fn resolve_esp_gpio_bit_mask(
    register: &ResolvedRegister,
    pin_index: u32,
    driver: &ResolvedDriverInstance,
    register_name: &str,
    field_name: &str,
) -> Result<u32> {
    let field = resolve_register_field(register, field_name)?;
    if field.lsb != 0 {
        bail!(
            "gpio-port driver {} requires {} field {} to start at bit 0 for ESP GPIO lowering, found lsb {}",
            driver.id,
            register_name,
            field.name,
            field.lsb
        );
    }
    let bit = field.lsb.checked_add(pin_index).ok_or_else(|| {
        anyhow!(
            "gpio-port driver {} pin index {} overflowed {} field {}",
            driver.id,
            pin_index,
            register_name,
            field.name
        )
    })?;
    if bit > field.msb {
        bail!(
            "gpio-port driver {} pin index {} is outside {} field {} bit range {}..={}",
            driver.id,
            pin_index,
            register_name,
            field.name,
            field.lsb,
            field.msb
        );
    }
    1u32.checked_shl(bit).ok_or_else(|| {
        anyhow!(
            "gpio-port driver {} pin index {} produced an invalid bit position {} for {}",
            driver.id,
            pin_index,
            bit,
            register_name
        )
    })
}

fn gpio_register_block_base(register: &ResolvedRegister) -> u64 {
    register.absolute_address & !0xFFFu64
}

fn resolve_bitmask_gpio_data_alias_address(
    data_register: &ResolvedRegister,
    bit_mask: u32,
    driver: &ResolvedDriverInstance,
) -> Result<u64> {
    if bit_mask > 0xFF {
        bail!(
            "gpio-port driver {} data register {} produced unsupported bit mask 0x{:X}; expected the Stellaris/Tiva masked DATA alias window to use only bits 0..7",
            driver.id,
            data_register.id,
            bit_mask
        );
    }
    let base_address = data_register.absolute_address.checked_sub(0x3FC).ok_or_else(|| {
        anyhow!(
            "gpio-port driver {} data register {} does not use the expected bitmask-alias layout",
            driver.id,
            data_register.id
        )
    })?;
    if base_address % 0x1000 != 0 {
        bail!(
            "gpio-port driver {} data register {} implies an unaligned GPIO base address 0x{:X}; expected DATA at base+0x3FC within a 4KiB GPIO block",
            driver.id,
            data_register.id,
            base_address
        );
    }
    checked_offset_add(
        base_address,
        (bit_mask as u64) << 2,
        "bitmask GPIO DATA alias address",
    )
    .map_err(|error| anyhow!("gpio-port driver {}: {}", driver.id, error))
}

fn validate_field_width(
    field: &ResolvedField,
    register: &ResolvedRegister,
    expected_width: u32,
) -> Result<()> {
    let actual_width = field
        .msb
        .checked_sub(field.lsb)
        .and_then(|delta| delta.checked_add(1))
        .ok_or_else(|| anyhow!("field {} has an invalid bit range", field.name))?;
    if actual_width != expected_width {
        bail!(
            "field {} on register {} uses width {} but first-cut lowering requires {}",
            field.name,
            register.id,
            actual_width,
            expected_width
        );
    }
    Ok(())
}

fn render_usart_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if !matches!(driver.driver_kind.as_str(), "uart" | "usart") {
        return Ok(Vec::new());
    }

    let target_ref = driver.target.target_ref.as_str();
    let has_tx = driver.pin_roles.iter().any(|role| {
        role.role.eq_ignore_ascii_case("tx")
            || role.signal.eq_ignore_ascii_case("tx")
            || role.signal.ends_with("TXD")
    });
    let has_rx = driver.pin_roles.iter().any(|role| {
        role.role.eq_ignore_ascii_case("rx")
            || role.signal.eq_ignore_ascii_case("rx")
            || role.signal.ends_with("RXD")
    });
    let has_irq = !driver.interrupt_routes.is_empty();
    let has_tx_dma = driver.dma_routes.iter().any(|route| {
        route
            .signal
            .as_deref()
            .is_some_and(|signal| signal.eq_ignore_ascii_case("tx"))
    });
    let has_rx_dma = driver.dma_routes.iter().any(|route| {
        route
            .signal
            .as_deref()
            .is_some_and(|signal| signal.eq_ignore_ascii_case("rx"))
    });

    let cr1 = try_resolve_target_register_by_names(model, target_ref, &["CR1", "CTLR1"])?;
    let cr2 = try_resolve_target_register_by_names(model, target_ref, &["CR2", "CTLR2"])?;
    if let (Some(cr1), Some(cr2)) = (cr1, cr2) {
        let brr = try_resolve_target_register_by_name_or_canonical_term(
            model,
            target_ref,
            "BRR",
            "term.register.baud-rate",
        )?;
        if let Some(brr) = brr {
            return render_stm32_usart_methods(
                model, driver, target_ref, has_tx, has_rx, has_irq, has_tx_dma, has_rx_dma, cr1,
                brr, cr2,
            );
        }
    }
    let conf0 = try_resolve_target_register_by_name(model, target_ref, "CONF0")?;
    let clkdiv = try_resolve_target_register_by_name(model, target_ref, "CLKDIV")?;
    let fifo = try_resolve_target_register_by_name(model, target_ref, "FIFO")?;
    let status = try_resolve_target_register_by_name(model, target_ref, "STATUS")?;
    let int_ena = try_resolve_target_register_by_name(model, target_ref, "INT_ENA")?;
    if let (Some(conf0), Some(clkdiv), Some(fifo), Some(status), Some(int_ena)) =
        (conf0, clkdiv, fifo, status, int_ena)
    {
        return render_esp_uart_methods(
            model, driver, has_tx, has_rx, has_irq, has_tx_dma, has_rx_dma, conf0, clkdiv, fifo,
            status, int_ena,
        );
    }
    Ok(Vec::new())
}

fn render_spi_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if driver.driver_kind != "spi" {
        return Ok(Vec::new());
    }

    let has_tx_dma = driver
        .dma_routes
        .iter()
        .any(|route| route.direction == "memory-to-peripheral");
    let has_rx_dma = driver
        .dma_routes
        .iter()
        .any(|route| route.direction == "peripheral-to-memory");
    let has_irq = !driver.interrupt_routes.is_empty();
    if !has_tx_dma && !has_rx_dma && !has_irq {
        return Ok(Vec::new());
    }

    let target_ref = driver.target.target_ref.as_str();
    let target = model.peripherals.get(target_ref).ok_or_else(|| {
        anyhow!(
            "spi driver {} references unknown target peripheral {}",
            driver.id,
            target_ref
        )
    })?;
    if target.base_address.is_none() {
        bail!(
            "spi driver {} target peripheral {} is missing baseAddress required for executable lowering",
            driver.id,
            target_ref
        );
    }
    let dma_conf = match try_resolve_target_register_by_name(model, target_ref, "DMA_CONF")? {
        Some(register) => register,
        None => return Ok(Vec::new()),
    };

    let mut methods = Vec::new();
    if has_tx_dma {
        let dma_tx_ena = resolve_register_field(&dma_conf, "DMA_TX_ENA")?;
        methods.push(render_register_write_method(
            "enable_tx_dma".to_string(),
            &dma_conf,
            &dma_tx_ena,
            1,
            &format!("Enable the {} TX DMA path.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_tx_dma".to_string(),
            &dma_conf,
            &dma_tx_ena,
            0,
            &format!("Disable the {} TX DMA path.", driver.name),
        )?);
    }
    if has_rx_dma {
        let dma_rx_ena = resolve_register_field(&dma_conf, "DMA_RX_ENA")?;
        methods.push(render_register_write_method(
            "enable_rx_dma".to_string(),
            &dma_conf,
            &dma_rx_ena,
            1,
            &format!("Enable the {} RX DMA path.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_rx_dma".to_string(),
            &dma_conf,
            &dma_rx_ena,
            0,
            &format!("Disable the {} RX DMA path.", driver.name),
        )?);
    }
    if has_irq
        && let Some(dma_int_ena) =
            try_resolve_target_register_by_name(model, target_ref, "DMA_INT_ENA")?
        && let Ok(done_ena) = resolve_register_field(&dma_int_ena, "DMA_SEG_TRANS_DONE_INT_ENA")
    {
        methods.push(render_register_write_method(
            "enable_dma_segment_done_interrupt".to_string(),
            &dma_int_ena,
            &done_ena,
            1,
            &format!("Enable the {} DMA segment-done interrupt.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_dma_segment_done_interrupt".to_string(),
            &dma_int_ena,
            &done_ena,
            0,
            &format!("Disable the {} DMA segment-done interrupt.", driver.name),
        )?);
    }

    Ok(methods)
}

fn render_adc_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if driver.driver_kind != "adc" {
        return Ok(Vec::new());
    }
    if driver.dma_routes.is_empty() && driver.interrupt_routes.is_empty() {
        return Ok(Vec::new());
    }

    let target_ref = driver.target.target_ref.as_str();
    let mut methods = Vec::new();

    if !driver.dma_routes.is_empty()
        && let Some(dma_conf) = try_resolve_target_register_by_name(model, target_ref, "DMA_CONF")?
    {
        let apb_adc_trans = resolve_register_field(&dma_conf, "APB_ADC_TRANS")?;
        methods.push(render_register_write_method(
            "enable_dma".to_string(),
            &dma_conf,
            &apb_adc_trans,
            1,
            &format!("Enable the {} DMA path.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_dma".to_string(),
            &dma_conf,
            &apb_adc_trans,
            0,
            &format!("Disable the {} DMA path.", driver.name),
        )?);

        let reset_fsm = resolve_register_field(&dma_conf, "APB_ADC_RESET_FSM")?;
        let mut code =
            "    pub fn reset_dma_fsm(&self) -> Result<(), metadata::Error> {\n".to_string();
        code.push_str(&render_register_write_statement(
            &dma_conf, &reset_fsm, 1, "        ",
        )?);
        code.push_str(&render_register_write_statement(
            &dma_conf, &reset_fsm, 0, "        ",
        )?);
        code.push_str("        Ok(())\n    }\n");
        methods.push(GeneratedMethod {
            name: "reset_dma_fsm".to_string(),
            code,
        });
    }

    if !driver.interrupt_routes.is_empty()
        && let Some(int_ena) = try_resolve_target_register_by_name(model, target_ref, "INT_ENA")?
        && let Ok(done_ena) = resolve_register_field(&int_ena, "APB_SARADC1_DONE_INT_ENA")
    {
        methods.push(render_register_write_method(
            "enable_done_interrupt".to_string(),
            &int_ena,
            &done_ena,
            1,
            &format!("Enable the {} conversion-done interrupt.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_done_interrupt".to_string(),
            &int_ena,
            &done_ena,
            0,
            &format!("Disable the {} conversion-done interrupt.", driver.name),
        )?);
    }

    Ok(methods)
}

fn render_usb_device_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if driver.driver_kind != "usb-device" {
        return Ok(Vec::new());
    }

    let target_ref = driver.target.target_ref.as_str();
    let Some(ep1) = try_resolve_target_register_by_name(model, target_ref, "EP1")? else {
        return Ok(Vec::new());
    };
    let Some(ep1_conf) = try_resolve_target_register_by_name(model, target_ref, "EP1_CONF")? else {
        return Ok(Vec::new());
    };

    let rdwr_byte = resolve_register_field(&ep1, "RDWR_BYTE")?;
    validate_field_width(&rdwr_byte, &ep1, 8)?;
    let wr_done = resolve_register_field(&ep1_conf, "WR_DONE")?;
    let serial_in_free = resolve_register_field(&ep1_conf, "SERIAL_IN_EP_DATA_FREE")?;
    let serial_out_avail = resolve_register_field(&ep1_conf, "SERIAL_OUT_EP_DATA_AVAIL")?;
    let rdwr_byte_mask = field_bit_mask_or_range_mask(&rdwr_byte, &ep1)?;
    let rdwr_byte_lsb = rdwr_byte.lsb;
    let serial_in_free_mask = field_bit_mask(&serial_in_free, &ep1_conf)?;
    let serial_out_avail_mask = field_bit_mask(&serial_out_avail, &ep1_conf)?;

    let mut methods = Vec::new();

    if !driver_uses_usb_preserve_link_lowering(driver)
        && let Some(conf0) = try_resolve_target_register_by_name(model, target_ref, "CONF0")?
    {
        if let Some(usb_pad_enable) = try_resolve_register_field(&conf0, "USB_PAD_ENABLE") {
            methods.push(render_register_write_method(
                "enable_usb_pad".to_string(),
                &conf0,
                &usb_pad_enable,
                1,
                &format!("Enable the {} USB pad drivers.", driver.name),
            )?);
            methods.push(render_register_write_method(
                "disable_usb_pad".to_string(),
                &conf0,
                &usb_pad_enable,
                0,
                &format!("Disable the {} USB pad drivers.", driver.name),
            )?);
        }
        if let Some(dp_pullup) = try_resolve_register_field(&conf0, "DP_PULLUP") {
            methods.push(render_register_write_method(
                "enable_dp_pullup".to_string(),
                &conf0,
                &dp_pullup,
                1,
                &format!("Enable the {} D+ pull-up.", driver.name),
            )?);
            methods.push(render_register_write_method(
                "disable_dp_pullup".to_string(),
                &conf0,
                &dp_pullup,
                0,
                &format!("Disable the {} D+ pull-up.", driver.name),
            )?);
        }
        if let Some(pad_pull_override) = try_resolve_register_field(&conf0, "PAD_PULL_OVERRIDE") {
            methods.push(render_register_write_method(
                "enable_pad_pull_override".to_string(),
                &conf0,
                &pad_pull_override,
                1,
                &format!(
                    "Let {} pad pull control follow software override bits.",
                    driver.name
                ),
            )?);
            methods.push(render_register_write_method(
                "disable_pad_pull_override".to_string(),
                &conf0,
                &pad_pull_override,
                0,
                &format!(
                    "Release {} pad pull control back to hardware defaults.",
                    driver.name
                ),
            )?);
        }
    }

    methods.push(render_register_flag_read_method(
        "serial_in_ready".to_string(),
        &ep1_conf,
        &serial_in_free,
        &format!(
            "Return whether the {} Serial IN endpoint can accept another byte.",
            driver.name
        ),
    )?);
    methods.push(render_register_flag_read_method(
        "serial_out_data_available".to_string(),
        &ep1_conf,
        &serial_out_avail,
        &format!(
            "Return whether the {} Serial OUT endpoint currently holds unread data.",
            driver.name
        ),
    )?);

    let mut write_byte = format!(
        "    /// Queue one byte for the {} Serial IN endpoint.\n",
        render_comment_text(&driver.name)
    );
    write_byte.push_str(
        "    pub fn write_serial_byte(&self, byte: u8) -> Result<(), metadata::Error> {\n",
    );
    write_byte.push_str("        while !self.serial_in_ready()? {}\n");
    write_byte.push_str(&format!(
        "        write_u32(0x{address:X}u64, u32::from(byte) << {rdwr_byte_lsb})?;\n",
        address = ep1.absolute_address,
        rdwr_byte_lsb = rdwr_byte_lsb,
    ));
    write_byte.push_str("        Ok(())\n    }\n");
    methods.push(GeneratedMethod {
        name: "write_serial_byte".to_string(),
        code: write_byte,
    });

    let mut write_bytes = format!(
        "    /// Queue a byte slice for the {} Serial IN endpoint.\n",
        render_comment_text(&driver.name)
    );
    write_bytes.push_str(
        "    pub fn write_serial_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {\n",
    );
    write_bytes.push_str("        for &byte in bytes {\n");
    write_bytes.push_str("            self.write_serial_byte(byte)?;\n");
    write_bytes.push_str("        }\n");
    write_bytes.push_str("        Ok(())\n    }\n");
    methods.push(GeneratedMethod {
        name: "write_serial_bytes".to_string(),
        code: write_bytes,
    });

    let mut read_byte = format!(
        "    /// Read one byte from the {} Serial OUT endpoint.\n",
        render_comment_text(&driver.name)
    );
    read_byte.push_str("    pub fn read_serial_byte(&self) -> Result<u8, metadata::Error> {\n");
    read_byte.push_str("        while !self.serial_out_data_available()? {}\n");
    read_byte.push_str(&format!(
        "        Ok(((read_u32(0x{address:X}u64)? & 0x{rdwr_byte_mask:08X}u32) >> {rdwr_byte_lsb}) as u8)\n",
        address = ep1.absolute_address,
        rdwr_byte_mask = rdwr_byte_mask,
        rdwr_byte_lsb = rdwr_byte_lsb,
    ));
    read_byte.push_str("    }\n");
    methods.push(GeneratedMethod {
        name: "read_serial_byte".to_string(),
        code: read_byte,
    });

    let mut read_bytes = format!(
        "    /// Read bytes from the {} Serial OUT endpoint into the supplied buffer.\n",
        render_comment_text(&driver.name)
    );
    read_bytes.push_str(
        "    pub fn read_serial_bytes(&self, buffer: &mut [u8]) -> Result<(), metadata::Error> {\n",
    );
    read_bytes.push_str("        for slot in buffer {\n");
    read_bytes.push_str("            *slot = self.read_serial_byte()?;\n");
    read_bytes.push_str("        }\n");
    read_bytes.push_str("        Ok(())\n    }\n");
    methods.push(GeneratedMethod {
        name: "read_serial_bytes".to_string(),
        code: read_bytes,
    });

    methods.push(render_register_write_method(
        "complete_write_packet".to_string(),
        &ep1_conf,
        &wr_done,
        1,
        &format!(
            "Commit the currently queued {} Serial IN packet to the host-facing endpoint.",
            driver.name
        ),
    )?);

    let mut write_packet = format!(
        "    /// Queue a packet and commit it on the {} Serial IN endpoint.\n",
        render_comment_text(&driver.name)
    );
    write_packet.push_str(
        "    pub fn write_serial_packet(&self, bytes: &[u8]) -> Result<(), metadata::Error> {\n",
    );
    write_packet.push_str("        self.write_serial_bytes(bytes)?;\n");
    write_packet.push_str("        self.complete_write_packet()\n");
    write_packet.push_str("    }\n");
    methods.push(GeneratedMethod {
        name: "write_serial_packet".to_string(),
        code: write_packet,
    });

    if let Some(int_ena) = try_resolve_target_register_by_name(model, target_ref, "INT_ENA")? {
        if let Some(serial_out_recv_pkt) =
            try_resolve_register_field(&int_ena, "SERIAL_OUT_RECV_PKT_INT_ENA")
        {
            methods.push(render_register_write_method(
                "enable_serial_out_receive_interrupt".to_string(),
                &int_ena,
                &serial_out_recv_pkt,
                1,
                &format!(
                    "Enable the {} Serial OUT packet-received interrupt.",
                    driver.name
                ),
            )?);
            methods.push(render_register_write_method(
                "disable_serial_out_receive_interrupt".to_string(),
                &int_ena,
                &serial_out_recv_pkt,
                0,
                &format!(
                    "Disable the {} Serial OUT packet-received interrupt.",
                    driver.name
                ),
            )?);
        }
        if let Some(serial_in_empty) =
            try_resolve_register_field(&int_ena, "SERIAL_IN_EMPTY_INT_ENA")
        {
            methods.push(render_register_write_method(
                "enable_serial_in_empty_interrupt".to_string(),
                &int_ena,
                &serial_in_empty,
                1,
                &format!("Enable the {} Serial IN empty interrupt.", driver.name),
            )?);
            methods.push(render_register_write_method(
                "disable_serial_in_empty_interrupt".to_string(),
                &int_ena,
                &serial_in_empty,
                0,
                &format!("Disable the {} Serial IN empty interrupt.", driver.name),
            )?);
        }
        if let Some(usb_bus_reset) = try_resolve_register_field(&int_ena, "USB_BUS_RESET_INT_ENA") {
            methods.push(render_register_write_method(
                "enable_usb_bus_reset_interrupt".to_string(),
                &int_ena,
                &usb_bus_reset,
                1,
                &format!("Enable the {} USB bus-reset interrupt.", driver.name),
            )?);
            methods.push(render_register_write_method(
                "disable_usb_bus_reset_interrupt".to_string(),
                &int_ena,
                &usb_bus_reset,
                0,
                &format!("Disable the {} USB bus-reset interrupt.", driver.name),
            )?);
        }
    }

    if let Some(int_clr) = try_resolve_target_register_by_name(model, target_ref, "INT_CLR")? {
        if let Some(serial_out_recv_pkt) =
            try_resolve_register_field(&int_clr, "SERIAL_OUT_RECV_PKT_INT_CLR")
        {
            methods.push(render_register_write_method(
                "clear_serial_out_receive_interrupt".to_string(),
                &int_clr,
                &serial_out_recv_pkt,
                1,
                &format!(
                    "Clear the {} Serial OUT packet-received interrupt.",
                    driver.name
                ),
            )?);
        }
        if let Some(serial_in_empty) =
            try_resolve_register_field(&int_clr, "SERIAL_IN_EMPTY_INT_CLR")
        {
            methods.push(render_register_write_method(
                "clear_serial_in_empty_interrupt".to_string(),
                &int_clr,
                &serial_in_empty,
                1,
                &format!("Clear the {} Serial IN empty interrupt.", driver.name),
            )?);
        }
        if let Some(usb_bus_reset) = try_resolve_register_field(&int_clr, "USB_BUS_RESET_INT_CLR") {
            methods.push(render_register_write_method(
                "clear_usb_bus_reset_interrupt".to_string(),
                &int_clr,
                &usb_bus_reset,
                1,
                &format!("Clear the {} USB bus-reset interrupt.", driver.name),
            )?);
        }
    }

    if let Some(int_st) = try_resolve_target_register_by_name(model, target_ref, "INT_ST")?
        && let Some(usb_bus_reset) = try_resolve_register_field(&int_st, "USB_BUS_RESET_INT_ST")
    {
        methods.push(render_register_flag_read_method(
            "is_usb_bus_reset_pending".to_string(),
            &int_st,
            &usb_bus_reset,
            &format!(
                "Return whether the {} USB bus-reset interrupt is currently pending.",
                driver.name
            ),
        )?);
    }

    // Ensure the compiler keeps the directly used status masks above tied to the resolved model.
    let _ = (serial_in_free_mask, serial_out_avail_mask);

    Ok(methods)
}

#[allow(clippy::too_many_arguments)]
fn render_stm32_usart_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
    target_ref: &str,
    has_tx: bool,
    has_rx: bool,
    has_irq: bool,
    has_tx_dma: bool,
    has_rx_dma: bool,
    cr1: ResolvedRegister,
    brr: ResolvedRegister,
    cr2: ResolvedRegister,
) -> Result<Vec<GeneratedMethod>> {
    let div_mantissa = resolve_register_field(&brr, "DIV_Mantissa")?;
    let div_fraction = resolve_register_field(&brr, "DIV_Fraction")?;
    let ue = resolve_register_field_or_canonical_term(model, &cr1, "UE", "term.field.enable")?;
    let te = resolve_register_field(&cr1, "TE")?;
    let re = resolve_register_field(&cr1, "RE")?;
    let over8 = resolve_register_field(&cr1, "OVER8")?;
    let word_length = resolve_register_field(&cr1, "M")?;
    let parity_enable = try_resolve_register_field(&cr1, "PCE");
    let parity_select = try_resolve_register_field(&cr1, "PS");
    let stop = resolve_register_field(&cr2, "STOP")?;
    let mantissa_value_mask = field_value_mask(&div_mantissa)?;
    let fraction_value_mask = field_value_mask(&div_fraction)?;
    let mantissa_clear_mask = shifted_field_mask(&div_mantissa, &brr)?;
    let fraction_clear_mask = shifted_field_mask(&div_fraction, &brr)?;

    let mut methods = Vec::new();
    methods.push(render_register_write_method(
        "enable".to_string(),
        &cr1,
        &ue,
        1,
        &format!("Enable {}.", driver.name),
    )?);
    methods.push(render_register_write_method(
        "disable".to_string(),
        &cr1,
        &ue,
        0,
        &format!("Disable {}.", driver.name),
    )?);
    methods.push(render_register_write_method(
        "enable_transmitter".to_string(),
        &cr1,
        &te,
        1,
        &format!("Enable the {} transmitter.", driver.name),
    )?);
    methods.push(render_register_write_method(
        "disable_transmitter".to_string(),
        &cr1,
        &te,
        0,
        &format!("Disable the {} transmitter.", driver.name),
    )?);
    methods.push(render_register_write_method(
        "enable_receiver".to_string(),
        &cr1,
        &re,
        1,
        &format!("Enable the {} receiver.", driver.name),
    )?);
    methods.push(render_register_write_method(
        "disable_receiver".to_string(),
        &cr1,
        &re,
        0,
        &format!("Disable the {} receiver.", driver.name),
    )?);

    let mut configure_code =
        "    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {\n".to_string();
    configure_code.push_str(&render_register_write_statement(&cr1, &ue, 0, "        ")?);
    configure_code.push_str(&render_register_write_statement(&cr1, &te, 0, "        ")?);
    configure_code.push_str(&render_register_write_statement(&cr1, &re, 0, "        ")?);
    configure_code.push_str(&render_register_write_statement(
        &cr1, &over8, 0, "        ",
    )?);
    configure_code.push_str(&render_register_write_statement(
        &cr1,
        &word_length,
        0,
        "        ",
    )?);
    if let Some(parity_enable) = parity_enable {
        configure_code.push_str(&render_register_write_statement(
            &cr1,
            &parity_enable,
            0,
            "        ",
        )?);
    }
    if let Some(parity_select) = parity_select {
        configure_code.push_str(&render_register_write_statement(
            &cr1,
            &parity_select,
            0,
            "        ",
        )?);
    }
    configure_code.push_str(&render_register_write_statement(
        &cr2, &stop, 0, "        ",
    )?);
    configure_code.push_str("        Ok(())\n    }\n");
    methods.push(GeneratedMethod {
        name: "configure_8n1".to_string(),
        code: configure_code,
    });

    methods.push(GeneratedMethod {
        name: "set_baud_divider".to_string(),
        code: format!(
            "    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {{\n        if u32::from(mantissa) > 0x{mantissa_value_mask:X}u32 {{\n            return Err(metadata::Error::Unsupported(\"USART baud mantissa exceeds modeled field width\"));\n        }}\n        if u32::from(fraction) > 0x{fraction_value_mask:X}u32 {{\n            return Err(metadata::Error::Unsupported(\"USART baud fraction exceeds modeled field width\"));\n        }}\n        modify_u32(0x{brr_address:X}u64, 0x{combined_clear_mask:08X}u32, ((u32::from(mantissa) & 0x{mantissa_value_mask:X}u32) << {mantissa_lsb}) | ((u32::from(fraction) & 0x{fraction_value_mask:X}u32) << {fraction_lsb}))?;\n        Ok(())\n    }}\n",
            brr_address = brr.absolute_address,
            combined_clear_mask = mantissa_clear_mask | fraction_clear_mask,
            mantissa_lsb = div_mantissa.lsb,
            fraction_lsb = div_fraction.lsb,
        ),
    });

    if has_tx {
        let sr = resolve_target_register_by_names(model, target_ref, &["SR", "STATR"])?;
        let dr = resolve_target_register_by_names(model, target_ref, &["DR", "DATAR"])?;
        let txe = resolve_register_field(&sr, "TXE")?;
        let tc = resolve_register_field(&sr, "TC")?;
        let txe_mask = field_bit_mask(&txe, &sr)?;
        let tc_mask = field_bit_mask(&tc, &sr)?;

        methods.push(GeneratedMethod {
            name: "write_byte".to_string(),
            code: format!(
                "    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {{\n        while (read_u32(0x{sr_address:X}u64)? & 0x{txe_mask:08X}u32) == 0 {{}}\n        write_u32(0x{dr_address:X}u64, u32::from(byte))?;\n        Ok(())\n    }}\n",
                sr_address = sr.absolute_address,
                dr_address = dr.absolute_address,
            ),
        });
        methods.push(GeneratedMethod {
            name: "write_bytes".to_string(),
            code: "    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {\n        for &byte in bytes {\n            self.write_byte(byte)?;\n        }\n        Ok(())\n    }\n"
                .to_string(),
        });
        methods.push(GeneratedMethod {
            name: "flush".to_string(),
            code: format!(
                "    pub fn flush(&self) -> Result<(), metadata::Error> {{\n        while (read_u32(0x{sr_address:X}u64)? & 0x{tc_mask:08X}u32) == 0 {{}}\n        Ok(())\n    }}\n",
                sr_address = sr.absolute_address,
            ),
        });
    }

    if has_rx {
        let sr = resolve_target_register_by_names(model, target_ref, &["SR", "STATR"])?;
        let dr = resolve_target_register_by_names(model, target_ref, &["DR", "DATAR"])?;
        let rxne = resolve_register_field(&sr, "RXNE")?;
        let rxne_mask = field_bit_mask(&rxne, &sr)?;

        methods.push(GeneratedMethod {
            name: "read_byte".to_string(),
            code: format!(
                "    pub fn read_byte(&self) -> Result<u8, metadata::Error> {{\n        while (read_u32(0x{sr_address:X}u64)? & 0x{rxne_mask:08X}u32) == 0 {{}}\n        Ok((read_u32(0x{dr_address:X}u64)? & 0xFFu32) as u8)\n    }}\n",
                sr_address = sr.absolute_address,
                dr_address = dr.absolute_address,
            ),
        });
    }

    if has_irq {
        let txeie = resolve_register_field(&cr1, "TXEIE")?;
        let rxneie = resolve_register_field(&cr1, "RXNEIE")?;
        methods.push(render_register_write_method(
            "enable_txe_interrupt".to_string(),
            &cr1,
            &txeie,
            1,
            &format!("Enable the {} TXE interrupt.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_txe_interrupt".to_string(),
            &cr1,
            &txeie,
            0,
            &format!("Disable the {} TXE interrupt.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "enable_rxne_interrupt".to_string(),
            &cr1,
            &rxneie,
            1,
            &format!("Enable the {} RXNE interrupt.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_rxne_interrupt".to_string(),
            &cr1,
            &rxneie,
            0,
            &format!("Disable the {} RXNE interrupt.", driver.name),
        )?);
    }

    if has_tx_dma {
        let cr3 = resolve_target_register_by_names(model, target_ref, &["CR3", "CTLR3"])?;
        let dmat = resolve_register_field(&cr3, "DMAT")?;
        methods.push(render_register_write_method(
            "enable_tx_dma".to_string(),
            &cr3,
            &dmat,
            1,
            &format!("Enable the {} TX DMA path.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_tx_dma".to_string(),
            &cr3,
            &dmat,
            0,
            &format!("Disable the {} TX DMA path.", driver.name),
        )?);
    }

    if has_rx_dma {
        let cr3 = resolve_target_register_by_names(model, target_ref, &["CR3", "CTLR3"])?;
        let dmar = resolve_register_field(&cr3, "DMAR")?;
        methods.push(render_register_write_method(
            "enable_rx_dma".to_string(),
            &cr3,
            &dmar,
            1,
            &format!("Enable the {} RX DMA path.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_rx_dma".to_string(),
            &cr3,
            &dmar,
            0,
            &format!("Disable the {} RX DMA path.", driver.name),
        )?);
    }

    Ok(methods)
}

#[allow(clippy::too_many_arguments)]
fn render_esp_uart_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
    has_tx: bool,
    has_rx: bool,
    has_irq: bool,
    _has_tx_dma: bool,
    _has_rx_dma: bool,
    conf0: ResolvedRegister,
    clkdiv: ResolvedRegister,
    fifo: ResolvedRegister,
    status: ResolvedRegister,
    int_ena: ResolvedRegister,
) -> Result<Vec<GeneratedMethod>> {
    let clk_en = resolve_register_field_or_canonical_term(
        model,
        &conf0,
        "CLK_EN",
        "term.field.clock-enable",
    )?;
    let mem_clk_en = try_resolve_register_field(&conf0, "MEM_CLK_EN");
    let parity_en = resolve_register_field(&conf0, "PARITY_EN")?;
    let parity = resolve_register_field(&conf0, "PARITY")?;
    let bit_num = resolve_register_field(&conf0, "BIT_NUM")?;
    let stop_bit_num = resolve_register_field(&conf0, "STOP_BIT_NUM")?;
    let clkdiv_field = resolve_register_field(&clkdiv, "CLKDIV")?;
    let frag_field = resolve_register_field(&clkdiv, "FRAG")?;
    let rxfifo_cnt = resolve_register_field(&status, "RXFIFO_CNT")?;
    let txfifo_cnt = resolve_register_field(&status, "TXFIFO_CNT")?;
    let clkdiv_value_mask = field_value_mask(&clkdiv_field)?;
    let frag_value_mask = field_value_mask(&frag_field)?;
    let clkdiv_clear_mask = shifted_field_mask(&clkdiv_field, &clkdiv)?;
    let frag_clear_mask = shifted_field_mask(&frag_field, &clkdiv)?;
    let rxfifo_cnt_mask = field_bit_mask_or_range_mask(&rxfifo_cnt, &status)?;
    let txfifo_cnt_mask = field_bit_mask_or_range_mask(&txfifo_cnt, &status)?;

    let mut methods = Vec::new();
    methods.push(render_register_write_method(
        "enable".to_string(),
        &conf0,
        &clk_en,
        1,
        &format!("Enable {}.", driver.name),
    )?);
    methods.push(render_register_write_method(
        "disable".to_string(),
        &conf0,
        &clk_en,
        0,
        &format!("Disable {}.", driver.name),
    )?);

    let mut configure_code =
        "    pub fn configure_8n1(&self) -> Result<(), metadata::Error> {\n".to_string();
    configure_code.push_str(&render_register_write_statement(
        &conf0, &parity_en, 0, "        ",
    )?);
    configure_code.push_str(&render_register_write_statement(
        &conf0, &parity, 0, "        ",
    )?);
    configure_code.push_str(&render_register_write_statement(
        &conf0, &bit_num, 3, "        ",
    )?);
    configure_code.push_str(&render_register_write_statement(
        &conf0,
        &stop_bit_num,
        1,
        "        ",
    )?);
    if let Some(mem_clk_en) = mem_clk_en {
        configure_code.push_str(&render_register_write_statement(
            &conf0,
            &mem_clk_en,
            1,
            "        ",
        )?);
    }
    configure_code.push_str(&render_register_write_statement(
        &conf0, &clk_en, 1, "        ",
    )?);
    configure_code.push_str("        Ok(())\n    }\n");
    methods.push(GeneratedMethod {
        name: "configure_8n1".to_string(),
        code: configure_code,
    });

    methods.push(GeneratedMethod {
        name: "set_baud_divider".to_string(),
        code: format!(
            "    pub fn set_baud_divider(&self, divider: u16, fraction: u8) -> Result<(), metadata::Error> {{\n        if u32::from(divider) > 0x{clkdiv_value_mask:X}u32 {{\n            return Err(metadata::Error::Unsupported(\"UART baud divider exceeds modeled field width\"));\n        }}\n        if u32::from(fraction) > 0x{frag_value_mask:X}u32 {{\n            return Err(metadata::Error::Unsupported(\"UART baud fraction exceeds modeled field width\"));\n        }}\n        modify_u32(0x{clkdiv_address:X}u64, 0x{combined_clear_mask:08X}u32, ((u32::from(divider) & 0x{clkdiv_value_mask:X}u32) << {clkdiv_lsb}) | ((u32::from(fraction) & 0x{frag_value_mask:X}u32) << {frag_lsb}))?;\n        Ok(())\n    }}\n",
            clkdiv_address = clkdiv.absolute_address,
            combined_clear_mask = clkdiv_clear_mask | frag_clear_mask,
            clkdiv_lsb = clkdiv_field.lsb,
            frag_lsb = frag_field.lsb,
        ),
    });

    if has_tx {
        methods.push(GeneratedMethod {
            name: "write_byte".to_string(),
            code: format!(
                "    pub fn write_byte(&self, byte: u8) -> Result<(), metadata::Error> {{\n        write_u32(0x{fifo_address:X}u64, u32::from(byte))?;\n        Ok(())\n    }}\n",
                fifo_address = fifo.absolute_address,
            ),
        });
        methods.push(GeneratedMethod {
            name: "write_bytes".to_string(),
            code: "    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {\n        for &byte in bytes {\n            self.write_byte(byte)?;\n        }\n        Ok(())\n    }\n"
                .to_string(),
        });
        methods.push(GeneratedMethod {
            name: "flush".to_string(),
            code: format!(
                "    pub fn flush(&self) -> Result<(), metadata::Error> {{\n        while ((read_u32(0x{status_address:X}u64)? & 0x{txfifo_cnt_mask:08X}u32) >> {txfifo_cnt_lsb}) != 0 {{}}\n        Ok(())\n    }}\n",
                status_address = status.absolute_address,
                txfifo_cnt_lsb = txfifo_cnt.lsb,
            ),
        });
    }

    if has_rx {
        methods.push(GeneratedMethod {
            name: "read_byte".to_string(),
            code: format!(
                "    pub fn read_byte(&self) -> Result<u8, metadata::Error> {{\n        while ((read_u32(0x{status_address:X}u64)? & 0x{rxfifo_cnt_mask:08X}u32) >> {rxfifo_cnt_lsb}) == 0 {{}}\n        Ok((read_u32(0x{fifo_address:X}u64)? & 0xFFu32) as u8)\n    }}\n",
                status_address = status.absolute_address,
                fifo_address = fifo.absolute_address,
                rxfifo_cnt_lsb = rxfifo_cnt.lsb,
            ),
        });
    }

    if has_irq {
        let txfifo_empty_int_ena = resolve_register_field(&int_ena, "TXFIFO_EMPTY_INT_ENA")?;
        let rxfifo_full_int_ena = resolve_register_field(&int_ena, "RXFIFO_FULL_INT_ENA")?;
        methods.push(render_register_write_method(
            "enable_txe_interrupt".to_string(),
            &int_ena,
            &txfifo_empty_int_ena,
            1,
            &format!("Enable the {} TXFIFO-empty interrupt.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_txe_interrupt".to_string(),
            &int_ena,
            &txfifo_empty_int_ena,
            0,
            &format!("Disable the {} TXFIFO-empty interrupt.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "enable_rxne_interrupt".to_string(),
            &int_ena,
            &rxfifo_full_int_ena,
            1,
            &format!("Enable the {} RXFIFO-full interrupt.", driver.name),
        )?);
        methods.push(render_register_write_method(
            "disable_rxne_interrupt".to_string(),
            &int_ena,
            &rxfifo_full_int_ena,
            0,
            &format!("Disable the {} RXFIFO-full interrupt.", driver.name),
        )?);
    }

    Ok(methods)
}

fn render_operation_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    driver
        .init_operations
        .iter()
        .map(|operation| {
            if operation.steps.is_empty() {
                bail!(
                    "operation {} on driver {} has no steps",
                    operation.id,
                    driver.id
                );
            }

            let mut steps = operation.steps.clone();
            let mut seen_indices = HashSet::new();
            for step in &steps {
                if !seen_indices.insert(step.index) {
                    bail!(
                        "operation {} on driver {} reuses step index {}",
                        operation.id,
                        driver.id,
                        step.index
                    );
                }
            }
            steps.sort_by_key(|step| step.index);
            let method_name = format!("apply_{}", operation_method_slug(operation));
            let mut code =
                format!("    pub fn {method_name}(&self) -> Result<(), metadata::Error> {{\n");
            for step in &steps {
                if step.action != "write" {
                    bail!(
                        "operation {} on driver {} uses unsupported action {}",
                        operation.id,
                        driver.id,
                        step.action
                    );
                }
                let target_ref = step.target_ref.as_deref().ok_or_else(|| {
                    anyhow!(
                        "operation {} step {} is missing targetRef",
                        operation.id,
                        step.index
                    )
                })?;
                let register = model.registers.get(target_ref).ok_or_else(|| {
                    anyhow!(
                        "operation {} step {} references unknown register {}",
                        operation.id,
                        step.index,
                        target_ref
                    )
                })?;
                let parsed = parse_field_write_expression(
                    step.expression.as_ref(),
                    step.value.as_ref(),
                    &operation.id,
                    step.index,
                )?;
                let field = resolve_register_field(register, &parsed.field_name)?;
                code.push_str(&render_register_write_statement(
                    register,
                    &field,
                    parsed.value,
                    "        ",
                )?);
            }
            code.push_str("        Ok(())\n    }\n");
            Ok(GeneratedMethod {
                name: method_name,
                code,
            })
        })
        .collect()
}

fn render_state_machine_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    let mut transition_pair_counts = BTreeMap::<(String, String), usize>::new();
    for state_machine in &driver.state_machines {
        for transition in &state_machine.transitions {
            *transition_pair_counts
                .entry((transition.from.clone(), transition.to.clone()))
                .or_default() += 1;
        }
    }
    let mut methods = Vec::new();
    for state_machine in &driver.state_machines {
        for transition in &state_machine.transitions {
            let (register, field, value) =
                resolve_transition_effect_write(model, driver, state_machine, transition)?;
            let from_slug = to_rust_method_name(&transition.from);
            let to_slug = to_rust_method_name(&transition.to);
            let method_name = if transition_pair_counts
                .get(&(transition.from.clone(), transition.to.clone()))
                .copied()
                .unwrap_or_default()
                > 1
            {
                format!(
                    "transition_{}_{}_to_{}",
                    state_machine_method_slug(state_machine),
                    from_slug,
                    to_slug
                )
            } else {
                format!("transition_{}_to_{}", from_slug, to_slug)
            };
            let mut code =
                format!("    pub fn {method_name}(&self) -> Result<(), metadata::Error> {{\n");
            code.push_str(&render_register_write_statement(
                register, &field, value, "        ",
            )?);
            code.push_str("        Ok(())\n    }\n");
            methods.push(GeneratedMethod {
                name: method_name,
                code,
            });
        }
    }
    Ok(methods)
}

fn resolve_target_register_by_name(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    register_name: &str,
) -> Result<ResolvedRegister> {
    let target = model.peripherals.get(target_ref).ok_or_else(|| {
        anyhow!(
            "driver target peripheral {} could not be resolved for register lookup",
            target_ref
        )
    })?;
    let target_base = target.base_address.ok_or_else(|| {
        anyhow!(
            "peripheral {} is missing baseAddress required for derived register lookup",
            target.id
        )
    })?;
    let template = find_register_template_peripheral(model, target)?;
    try_resolve_target_register_from_template(target, target_base, template, register_name)?.ok_or_else(
        || {
            anyhow!(
                "peripheral {} does not expose register {} through its explicit structural template",
                target.id,
                register_name
            )
        },
    )
}

fn resolve_target_register_by_names(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    register_names: &[&str],
) -> Result<ResolvedRegister> {
    try_resolve_target_register_by_names(model, target_ref, register_names)?.ok_or_else(|| {
        anyhow!(
            "peripheral {} does not expose any register in [{}] through its explicit structural template",
            target_ref,
            register_names.join(", ")
        )
    })
}

#[derive(Debug, Clone)]
struct TimeDriverFieldAccess {
    register: ResolvedRegister,
    mask: u32,
    lsb: u32,
}

struct HardwareTimerTimeDriverLayout {
    conf_clk_en: TimeDriverFieldAccess,
    conf_target0_work_en: TimeDriverFieldAccess,
    conf_unit0_work_en: TimeDriverFieldAccess,
    unit0_op_value_valid: TimeDriverFieldAccess,
    unit0_op_update: TimeDriverFieldAccess,
    unit0_value_hi: TimeDriverFieldAccess,
    unit0_value_lo: TimeDriverFieldAccess,
    target0_hi: TimeDriverFieldAccess,
    target0_lo: TimeDriverFieldAccess,
    target0_conf_period: TimeDriverFieldAccess,
    target0_conf_period_mode: TimeDriverFieldAccess,
    target0_conf_unit_sel: TimeDriverFieldAccess,
    comp0_load: TimeDriverFieldAccess,
    int_ena_target0: TimeDriverFieldAccess,
    clear_statement: String,
}

struct CounterCompareTimerTimeDriverLayout {
    counter: TimeDriverFieldAccess,
    compare: TimeDriverFieldAccess,
    alarm_apply_statement: String,
    interrupt_enable: TimeDriverFieldAccess,
    interrupt_pending: TimeDriverFieldAccess,
    clear_statement: String,
}

fn register_width_mask(register: &ResolvedRegister) -> Result<u32> {
    match register.width_bits {
        0 => bail!("register {} has invalid width 0", register.id),
        1..=31 => Ok((1u32 << register.width_bits) - 1),
        32 => Ok(u32::MAX),
        other => bail!(
            "register {} has unsupported width {} for generated time-driver access",
            register.id,
            other
        ),
    }
}

fn resolve_time_driver_access_ref(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
    entity_ref: &str,
    usage: &str,
) -> Result<TimeDriverFieldAccess> {
    if let Some(field_target) = try_resolve_field_target(
        &model.fields,
        entity_ref,
        Some(driver.target.target_ref.as_str()),
    )? {
        if field_target.peripheral_ref != driver.target.target_ref {
            bail!(
                "driver {} {} {} targets {} instead of {}",
                driver.id,
                usage,
                entity_ref,
                field_target.peripheral_ref,
                driver.target.target_ref
            );
        }
        let register = model
            .registers
            .get(field_target.register_id.as_str())
            .ok_or_else(|| {
                anyhow!(
                    "driver {} {} {} references register {} that is unavailable in the generated scope",
                    driver.id,
                    usage,
                    entity_ref,
                    field_target.register_id
                )
            })?
            .clone();
        return Ok(TimeDriverFieldAccess {
            mask: field_bit_mask_or_range_mask(&field_target.field, &register)?,
            lsb: field_target.field.lsb,
            register,
        });
    }
    let register = model.registers.get(entity_ref).ok_or_else(|| {
        anyhow!(
            "driver {} {} {} must resolve to a register or field",
            driver.id,
            usage,
            entity_ref
        )
    })?;
    if register.peripheral_ref != driver.target.target_ref {
        bail!(
            "driver {} {} {} targets {} instead of {}",
            driver.id,
            usage,
            entity_ref,
            register.peripheral_ref,
            driver.target.target_ref
        );
    }
    Ok(TimeDriverFieldAccess {
        mask: register_width_mask(register)?,
        lsb: 0,
        register: register.clone(),
    })
}

fn resolve_time_driver_field_access(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
    register_name: &str,
    field_name: &str,
) -> Result<TimeDriverFieldAccess> {
    let register =
        resolve_target_register_by_name(model, &driver.target.target_ref, register_name)?;
    let field = resolve_register_field(&register, field_name)?;
    Ok(TimeDriverFieldAccess {
        mask: field_bit_mask_or_range_mask(&field, &register)?,
        lsb: field.lsb,
        register,
    })
}

fn render_time_driver_operation_statements(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
    operation_ref: &str,
    usage: &str,
) -> Result<String> {
    let driver_target_ref = driver.target.target_ref.as_str();
    let operation = model.operations.get(operation_ref).ok_or_else(|| {
        anyhow!(
            "driver {} {} references unknown operation {}",
            driver.id,
            usage,
            operation_ref
        )
    })?;
    for operation_target in &operation.target_refs {
        if operation_target != driver_target_ref {
            bail!(
                "driver {} {} {} targets {} instead of {}",
                driver.id,
                usage,
                operation.id,
                operation_target,
                driver_target_ref
            );
        }
    }
    let mut rendered = String::new();
    for step in &operation.steps {
        if step.action != "write" {
            bail!(
                "driver {} {} {} uses unsupported action {}",
                driver.id,
                usage,
                operation.id,
                step.action
            );
        }
        let step_target_ref = step.target_ref.as_deref().ok_or_else(|| {
            anyhow!(
                "driver {} {} {} step {} is missing targetRef",
                driver.id,
                usage,
                operation.id,
                step.index
            )
        })?;
        let register = model.registers.get(step_target_ref).ok_or_else(|| {
            anyhow!(
                "driver {} {} {} step {} references unknown register {}",
                driver.id,
                usage,
                operation.id,
                step.index,
                step_target_ref
            )
        })?;
        if register.peripheral_ref != driver_target_ref {
            bail!(
                "driver {} {} {} step {} references register {} on {} instead of {}",
                driver.id,
                usage,
                operation.id,
                step.index,
                step_target_ref,
                register.peripheral_ref,
                driver_target_ref
            );
        }
        let parsed = parse_field_write_expression(
            step.expression.as_ref(),
            step.value.as_ref(),
            &operation.id,
            step.index,
        )?;
        let field = resolve_register_field(register, &parsed.field_name)?;
        rendered.push_str(&render_register_write_statement(
            register,
            &field,
            parsed.value,
            "        ",
        )?);
    }
    Ok(rendered)
}

fn render_time_driver_clear_statement(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let [interrupt_source] = driver.interrupt_sources.as_slice() else {
        bail!(
            "driver {} must resolve exactly one interrupt source for generated time-driver clearing",
            driver.id
        );
    };
    let clear_ref = time_driver_bindings(driver)
        .map(|bindings| bindings.interrupt_clear_operation_ref.as_str())
        .or_else(|| interrupt_source.clear_operation_refs.first().map(String::as_str))
        .ok_or_else(|| {
            anyhow!(
                "driver {} interrupt source {} must expose a clear operation for generated time-driver clearing",
                driver.id,
                interrupt_source.id
            )
        })?;
    if !interrupt_source.clear_operation_refs.is_empty()
        && !interrupt_source
            .clear_operation_refs
            .iter()
            .any(|candidate| candidate == clear_ref)
    {
        bail!(
            "driver {} time-driver clear operation {} is not listed on interrupt source {}",
            driver.id,
            clear_ref,
            interrupt_source.id
        );
    }
    render_time_driver_operation_statements(model, driver, clear_ref, "time-driver clear operation")
}

fn resolve_hardware_timer_time_driver_layout(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<HardwareTimerTimeDriverLayout> {
    Ok(HardwareTimerTimeDriverLayout {
        conf_clk_en: resolve_time_driver_field_access(model, driver, "CONF", "CLK_EN")?,
        conf_target0_work_en: resolve_time_driver_field_access(
            model,
            driver,
            "CONF",
            "TARGET0_WORK_EN",
        )?,
        conf_unit0_work_en: resolve_time_driver_field_access(
            model,
            driver,
            "CONF",
            "TIMER_UNIT0_WORK_EN",
        )?,
        unit0_op_value_valid: resolve_time_driver_field_access(
            model,
            driver,
            "UNIT0_OP",
            "TIMER_UNIT0_VALUE_VALID",
        )?,
        unit0_op_update: resolve_time_driver_field_access(
            model,
            driver,
            "UNIT0_OP",
            "TIMER_UNIT0_UPDATE",
        )?,
        unit0_value_hi: resolve_time_driver_field_access(
            model,
            driver,
            "UNIT0_VALUE_HI",
            "TIMER_UNIT0_VALUE_HI",
        )?,
        unit0_value_lo: resolve_time_driver_field_access(
            model,
            driver,
            "UNIT0_VALUE_LO",
            "TIMER_UNIT0_VALUE_LO",
        )?,
        target0_hi: resolve_time_driver_field_access(
            model,
            driver,
            "TARGET0_HI",
            "TIMER_TARGET0_HI",
        )?,
        target0_lo: resolve_time_driver_field_access(
            model,
            driver,
            "TARGET0_LO",
            "TIMER_TARGET0_LO",
        )?,
        target0_conf_period: resolve_time_driver_field_access(
            model,
            driver,
            "TARGET0_CONF",
            "TARGET0_PERIOD",
        )?,
        target0_conf_period_mode: resolve_time_driver_field_access(
            model,
            driver,
            "TARGET0_CONF",
            "TARGET0_PERIOD_MODE",
        )?,
        target0_conf_unit_sel: resolve_time_driver_field_access(
            model,
            driver,
            "TARGET0_CONF",
            "TARGET0_TIMER_UNIT_SEL",
        )?,
        comp0_load: resolve_time_driver_field_access(
            model,
            driver,
            "COMP0_LOAD",
            "TIMER_COMP0_LOAD",
        )?,
        int_ena_target0: resolve_time_driver_field_access(
            model,
            driver,
            "INT_ENA",
            "TARGET0_INT_ENA",
        )?,
        clear_statement: render_time_driver_clear_statement(model, driver)?,
    })
}

fn resolve_counter_compare_timer_time_driver_layout(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<CounterCompareTimerTimeDriverLayout> {
    let bindings = time_driver_bindings(driver).ok_or_else(|| {
        anyhow!(
            "driver {} uses loweringPattern {} but omits required timeDriverBindings",
            driver.id,
            TIMER_LOWERING_COUNTER_COMPARE
        )
    })?;
    let mut alarm_apply_statement = String::new();
    for operation_ref in &bindings.alarm_apply_operation_refs {
        alarm_apply_statement.push_str(&render_time_driver_operation_statements(
            model,
            driver,
            operation_ref,
            "time-driver alarm apply operation",
        )?);
    }
    Ok(CounterCompareTimerTimeDriverLayout {
        counter: resolve_time_driver_access_ref(
            model,
            driver,
            &bindings.counter_ref,
            "time-driver counter binding",
        )?,
        compare: resolve_time_driver_access_ref(
            model,
            driver,
            &bindings.alarm_ref,
            "time-driver alarm binding",
        )?,
        alarm_apply_statement,
        interrupt_enable: resolve_time_driver_access_ref(
            model,
            driver,
            &bindings.interrupt_enable_ref,
            "time-driver interrupt-enable binding",
        )?,
        interrupt_pending: resolve_time_driver_access_ref(
            model,
            driver,
            &bindings.interrupt_pending_ref,
            "time-driver interrupt-pending binding",
        )?,
        clear_statement: render_time_driver_clear_statement(model, driver)?,
    })
}

fn try_resolve_target_register_by_name(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    register_name: &str,
) -> Result<Option<ResolvedRegister>> {
    let target = model.peripherals.get(target_ref).ok_or_else(|| {
        anyhow!(
            "driver target peripheral {} could not be resolved for register lookup",
            target_ref
        )
    })?;
    let target_base = target.base_address.ok_or_else(|| {
        anyhow!(
            "peripheral {} is missing baseAddress required for derived register lookup",
            target.id
        )
    })?;
    let Some(template) = try_find_register_template_peripheral(model, target)? else {
        return Ok(None);
    };
    try_resolve_target_register_from_template(target, target_base, template, register_name)
}

fn try_resolve_target_register_by_names(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    register_names: &[&str],
) -> Result<Option<ResolvedRegister>> {
    for register_name in register_names {
        if let Some(register) =
            try_resolve_target_register_by_name(model, target_ref, register_name)?
        {
            return Ok(Some(register));
        }
    }
    Ok(None)
}

fn try_resolve_target_register_by_name_or_canonical_term(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    register_name: &str,
    canonical_term_ref: &str,
) -> Result<Option<ResolvedRegister>> {
    if let Some(register) = try_resolve_target_register_by_name(model, target_ref, register_name)? {
        return Ok(Some(register));
    }
    try_resolve_target_register_by_canonical_term(model, target_ref, canonical_term_ref)
}

fn try_resolve_target_register_by_canonical_term(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    canonical_term_ref: &str,
) -> Result<Option<ResolvedRegister>> {
    let target = model.peripherals.get(target_ref).ok_or_else(|| {
        anyhow!(
            "driver target peripheral {} could not be resolved for canonical register lookup",
            target_ref
        )
    })?;
    let target_base = target.base_address.ok_or_else(|| {
        anyhow!(
            "peripheral {} is missing baseAddress required for canonical derived register lookup",
            target.id
        )
    })?;
    let Some(template) = try_find_register_template_peripheral(model, target)? else {
        return Ok(None);
    };
    try_resolve_target_register_from_template_by_canonical_term(
        model,
        target,
        target_base,
        template,
        canonical_term_ref,
    )
}

fn try_resolve_register_by_ref(
    model: &EmbassyGenerationModel,
    register_ref: &str,
) -> Result<Option<ResolvedRegister>> {
    let Some(owners) = model.register_owners.get(register_ref) else {
        return Ok(None);
    };
    for owner_ref in owners {
        let Some(peripheral) = model.peripherals.get(owner_ref) else {
            continue;
        };
        let Some(target_base) = peripheral.base_address else {
            continue;
        };
        let Some(template) = try_find_register_template_peripheral(model, peripheral)? else {
            continue;
        };
        if let Some(register) = try_resolve_target_register_from_template_by_ref(
            peripheral,
            target_base,
            template,
            register_ref,
        )? {
            return Ok(Some(register));
        }
    }
    Ok(None)
}

fn try_resolve_target_register_from_template(
    target: &Peripheral,
    target_base: u64,
    template: &Peripheral,
    register_name: &str,
) -> Result<Option<ResolvedRegister>> {
    let template_base = template.base_address.ok_or_else(|| {
        anyhow!(
            "template peripheral {} is missing baseAddress required for register lookup",
            template.id
        )
    })?;

    let mut registers = HashMap::new();
    collect_register_members(
        &mut registers,
        template.id.as_str(),
        template_base,
        0,
        &template.registers,
    )?;
    let mut matches = registers
        .into_values()
        .filter(|register| register.name.eq_ignore_ascii_case(register_name))
        .collect::<Vec<_>>();
    if matches.is_empty() {
        return Ok(None);
    }
    if matches.len() > 1 {
        bail!(
            "peripheral {} exposes multiple registers named {} through its structural template",
            target.id,
            register_name
        );
    }

    let mut resolved = matches.pop().expect("one matching register");
    let offset = resolved
        .absolute_address
        .checked_sub(template_base)
        .ok_or_else(|| {
            anyhow!(
                "register {} absolute address underflowed relative to template peripheral {}",
                resolved.id,
                template.id
            )
        })?;
    resolved.peripheral_ref = target.id.clone();
    resolved.absolute_address = checked_offset_add(
        target_base,
        offset,
        &format!("derived register {} on {}", register_name, target.id),
    )?;
    Ok(Some(resolved))
}

fn try_resolve_target_register_from_template_by_ref(
    target: &Peripheral,
    target_base: u64,
    template: &Peripheral,
    register_ref: &str,
) -> Result<Option<ResolvedRegister>> {
    let template_base = template.base_address.ok_or_else(|| {
        anyhow!(
            "template peripheral {} is missing baseAddress required for register lookup",
            template.id
        )
    })?;

    let mut registers = HashMap::new();
    collect_register_members(
        &mut registers,
        template.id.as_str(),
        template_base,
        0,
        &template.registers,
    )?;
    let mut matches = registers
        .into_values()
        .filter(|register| register.id == register_ref)
        .collect::<Vec<_>>();
    if matches.is_empty() {
        return Ok(None);
    }
    if matches.len() > 1 {
        bail!(
            "peripheral {} exposes multiple registers with id {} through its structural template",
            target.id,
            register_ref
        );
    }

    let mut resolved = matches.pop().expect("one matching register");
    let offset = resolved
        .absolute_address
        .checked_sub(template_base)
        .ok_or_else(|| {
            anyhow!(
                "register {} absolute address underflowed relative to template peripheral {}",
                resolved.id,
                template.id
            )
        })?;
    resolved.peripheral_ref = target.id.clone();
    resolved.absolute_address = checked_offset_add(
        target_base,
        offset,
        &format!("derived register {register_ref} on {}", target.id),
    )?;
    Ok(Some(resolved))
}

fn try_resolve_target_register_from_template_by_canonical_term(
    model: &EmbassyGenerationModel,
    target: &Peripheral,
    target_base: u64,
    template: &Peripheral,
    canonical_term_ref: &str,
) -> Result<Option<ResolvedRegister>> {
    let template_base = template.base_address.ok_or_else(|| {
        anyhow!(
            "template peripheral {} is missing baseAddress required for canonical register lookup",
            template.id
        )
    })?;

    let mut registers = HashMap::new();
    collect_register_members(
        &mut registers,
        template.id.as_str(),
        template_base,
        0,
        &template.registers,
    )?;
    let mut matches = registers
        .into_values()
        .filter(|register| {
            target_has_canonical_term(model, register.id.as_str(), canonical_term_ref)
        })
        .collect::<Vec<_>>();
    if matches.is_empty() {
        return Ok(None);
    }
    if matches.len() > 1 {
        let ids = matches
            .iter()
            .map(|register| register.id.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "peripheral {} exposes multiple registers mapped to canonical term {} through its structural template: {}",
            target.id,
            canonical_term_ref,
            ids
        );
    }

    let mut resolved = matches.pop().expect("one matching register");
    let offset = resolved
        .absolute_address
        .checked_sub(template_base)
        .ok_or_else(|| {
            anyhow!(
                "register {} absolute address underflowed relative to template peripheral {}",
                resolved.id,
                template.id
            )
        })?;
    resolved.peripheral_ref = target.id.clone();
    resolved.absolute_address = checked_offset_add(
        target_base,
        offset,
        &format!(
            "derived canonical register {} on {}",
            canonical_term_ref, target.id
        ),
    )?;
    Ok(Some(resolved))
}

fn target_has_canonical_term(
    model: &EmbassyGenerationModel,
    target_ref: &str,
    canonical_term_ref: &str,
) -> bool {
    model
        .canonical_terms_by_target
        .get(target_ref)
        .is_some_and(|terms| terms.contains(canonical_term_ref))
}

fn try_find_register_template_peripheral<'a>(
    model: &'a EmbassyGenerationModel,
    target: &'a Peripheral,
) -> Result<Option<&'a Peripheral>> {
    let mut current = target;
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(current.id.clone()) {
            bail!(
                "peripheral {} has a derivedFromRef cycle while resolving structural template",
                target.id
            );
        }
        if !current.registers.is_empty() {
            return Ok(Some(current));
        }
        let Some(derived_from) = current.derived_from_ref.as_deref() else {
            return Ok(None);
        };
        current = model.peripherals.get(derived_from).ok_or_else(|| {
            anyhow!(
                "peripheral {} references unknown derivedFromRef {}",
                current.id,
                derived_from
            )
        })?;
    }
}

fn find_register_template_peripheral<'a>(
    model: &'a EmbassyGenerationModel,
    target: &'a Peripheral,
) -> Result<&'a Peripheral> {
    try_find_register_template_peripheral(model, target)?.ok_or_else(|| {
        anyhow!(
            "peripheral {} has no explicit registers and no derivedFromRef template",
            target.id
        )
    })
}

fn resolve_transition_effect_write<'a>(
    model: &'a EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
    state_machine: &SemanticStateMachine,
    transition: &SemanticTransition,
) -> Result<(&'a ResolvedRegister, ResolvedField, u64)> {
    let [effect] = transition.effects.as_slice() else {
        bail!(
            "state machine {} transition {} -> {} on driver {} requires exactly one effect for first-cut lowering, found {}",
            state_machine.id,
            transition.from,
            transition.to,
            driver.id,
            transition.effects.len()
        );
    };
    let effect_target_ref = effect.target_ref.as_deref().ok_or_else(|| {
        anyhow!(
            "state machine {} transition {} -> {} on driver {} is missing effect targetRef",
            state_machine.id,
            transition.from,
            transition.to,
            driver.id
        )
    })?;
    let field_target = resolve_field_target(
        &model.fields,
        effect_target_ref,
        Some(driver.target.target_ref.as_str()),
        || {
            anyhow!(
                "state machine {} transition {} -> {} on driver {} requires effect targetRef {} to resolve to a field for first-cut lowering",
                state_machine.id,
                transition.from,
                transition.to,
                driver.id,
                effect_target_ref
            )
        },
    )?;
    let register = model
        .registers
        .get(field_target.register_id.as_str())
        .ok_or_else(|| {
            anyhow!(
                "field {} on state machine {} transition {} -> {} resolved to unknown register {}",
                effect_target_ref,
                state_machine.id,
                transition.from,
                transition.to,
                field_target.register_id
            )
        })?;
    let value = transition_effect_value(effect, state_machine, transition, &driver.id)?;
    Ok((register, field_target.field.clone(), value))
}

fn resolve_control_target<'a>(
    model: &'a EmbassyGenerationModel,
    control_refs: &[String],
    kind: &str,
    owner_id: &str,
    subjects: &[String],
) -> Result<(&'a ResolvedRegister, ResolvedField)> {
    let control_ref = control_refs.first().ok_or_else(|| {
        anyhow!("{kind} {owner_id} is missing controlRefs required for executable lowering")
    })?;
    if control_refs.len() > 1 {
        bail!("{kind} {owner_id} references multiple controlRefs, which is not yet supported");
    }
    if let Some(register) = model.registers.get(control_ref) {
        return Ok((register, resolve_control_field(register, subjects)?));
    }
    if let Some(field_target) = try_resolve_field_target(&model.fields, control_ref, None)? {
        let register = model
            .registers
            .get(field_target.register_id.as_str())
            .ok_or_else(|| {
                anyhow!(
                    "{kind} {owner_id} field controlRef {} resolved to unknown register {}",
                    control_ref,
                    field_target.register_id
                )
            })?;
        return Ok((register, field_target.field.clone()));
    }
    Err(anyhow!(
        "{kind} {owner_id} references unknown control target {control_ref}; expected either a register id or a field id"
    ))
}

fn resolve_control_field(
    register: &ResolvedRegister,
    subjects: &[String],
) -> Result<ResolvedField> {
    let subject_aliases = subjects
        .iter()
        .flat_map(|subject| subject_aliases(subject))
        .filter(|subject| subject.len() >= 3)
        .collect::<Vec<_>>();

    let mut matches = Vec::<(usize, ResolvedField)>::new();
    for field in &register.fields {
        let haystack = normalize_search_text(&format!(
            "{} {}",
            field.name,
            field.description.as_deref().unwrap_or_default()
        ));
        let mut score = 0usize;
        for subject in &subject_aliases {
            let normalized_subject = normalize_search_text(subject);
            if !normalized_subject.is_empty() && haystack.contains(&normalized_subject) {
                score = score.max(normalized_subject.len());
            }
        }
        if score > 0 {
            matches.push((score, field.clone()));
        }
    }

    matches.sort_by_key(|right| std::cmp::Reverse(right.0));
    let (best_score, best_field) = matches.first().cloned().ok_or_else(|| {
        anyhow!(
            "register {} does not expose a uniquely matchable control field",
            register.id
        )
    })?;
    if matches
        .iter()
        .filter(|(score, _)| *score == best_score)
        .nth(1)
        .is_some()
    {
        bail!(
            "register {} has ambiguous control-field matches for the requested subject",
            register.id
        );
    }
    Ok(best_field)
}

fn resolve_register_field(register: &ResolvedRegister, field_name: &str) -> Result<ResolvedField> {
    let normalized_name = normalize_search_text(field_name);
    register
        .fields
        .iter()
        .find(|field| normalize_search_text(&field.name) == normalized_name)
        .cloned()
        .ok_or_else(|| {
            anyhow!(
                "register {} does not contain field {}",
                register.id,
                field_name
            )
        })
}

fn resolve_register_field_or_canonical_term(
    model: &EmbassyGenerationModel,
    register: &ResolvedRegister,
    field_name: &str,
    canonical_term_ref: &str,
) -> Result<ResolvedField> {
    if let Some(field) = try_resolve_register_field(register, field_name) {
        return Ok(field);
    }
    try_resolve_register_field_by_canonical_term(model, register, canonical_term_ref)?.ok_or_else(
        || {
            anyhow!(
                "register {} does not contain field {} or any field mapped to canonical term {}",
                register.id,
                field_name,
                canonical_term_ref
            )
        },
    )
}

fn try_resolve_register_field(
    register: &ResolvedRegister,
    field_name: &str,
) -> Option<ResolvedField> {
    let normalized_name = normalize_search_text(field_name);
    register
        .fields
        .iter()
        .find(|field| normalize_search_text(&field.name) == normalized_name)
        .cloned()
}

fn try_resolve_register_field_by_canonical_term(
    model: &EmbassyGenerationModel,
    register: &ResolvedRegister,
    canonical_term_ref: &str,
) -> Result<Option<ResolvedField>> {
    let mut matches = register
        .fields
        .iter()
        .filter(|field| target_has_canonical_term(model, field.id.as_str(), canonical_term_ref))
        .cloned()
        .collect::<Vec<_>>();
    if matches.is_empty() {
        return Ok(None);
    }
    if matches.len() > 1 {
        let ids = matches
            .iter()
            .map(|field| field.id.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "register {} exposes multiple fields mapped to canonical term {}: {}",
            register.id,
            canonical_term_ref,
            ids
        );
    }
    Ok(matches.pop())
}

fn render_register_write_method(
    method_name: String,
    register: &ResolvedRegister,
    field: &ResolvedField,
    value: u64,
    description: &str,
) -> Result<GeneratedMethod> {
    let mut code = format!("    /// {}\n", render_comment_text(description));
    code.push_str(&format!(
        "    pub fn {method_name}(&self) -> Result<(), metadata::Error> {{\n"
    ));
    code.push_str(&render_register_write_statement(
        register, field, value, "        ",
    )?);
    code.push_str("        Ok(())\n    }\n");
    Ok(GeneratedMethod {
        name: method_name,
        code,
    })
}

fn render_register_flag_read_method(
    method_name: String,
    register: &ResolvedRegister,
    field: &ResolvedField,
    description: &str,
) -> Result<GeneratedMethod> {
    let mask = field_bit_mask(field, register)?;
    let code = format!(
        "    /// {}\n    pub fn {method_name}(&self) -> Result<bool, metadata::Error> {{\n        Ok((read_u32(0x{:X}u64)? & 0x{:08X}u32) != 0)\n    }}\n",
        render_comment_text(description),
        register.absolute_address,
        mask,
    );
    Ok(GeneratedMethod {
        name: method_name,
        code,
    })
}

fn render_register_write_statement(
    register: &ResolvedRegister,
    field: &ResolvedField,
    value: u64,
    indent: &str,
) -> Result<String> {
    match register.width_bits {
        8 | 16 | 32 => {}
        other => {
            bail!(
                "register {} uses unsupported width {} for Embassy code generation",
                register.id,
                other
            );
        }
    }
    if field.msb >= register.width_bits || field.lsb >= register.width_bits {
        bail!(
            "field {} bit range {}..={} exceeds register {} width {}",
            field.name,
            field.lsb,
            field.msb,
            register.id,
            register.width_bits
        );
    }
    let field_width = field
        .msb
        .checked_sub(field.lsb)
        .and_then(|delta| delta.checked_add(1))
        .ok_or_else(|| anyhow!("field {} has an invalid bit range", field.name))?;
    let raw_mask = if field_width == 64 {
        u64::MAX
    } else {
        ((1u128 << field_width) - 1) as u64
    };
    if value > raw_mask {
        bail!(
            "field {} on register {} cannot hold value {}",
            field.name,
            register.id,
            value
        );
    }
    let clear_mask = raw_mask << field.lsb;
    let set_mask = (value & raw_mask) << field.lsb;
    let address = register.absolute_address;

    match register.width_bits {
        8 => {
            let clear_mask = u8::try_from(clear_mask).map_err(|_| {
                anyhow!(
                    "field {} bit range {}..={} exceeds register {} width {}",
                    field.name,
                    field.lsb,
                    field.msb,
                    register.id,
                    register.width_bits
                )
            })?;
            let set_mask = u8::try_from(set_mask).map_err(|_| {
                anyhow!(
                    "field {} bit range {}..={} exceeds register {} width {}",
                    field.name,
                    field.lsb,
                    field.msb,
                    register.id,
                    register.width_bits
                )
            })?;
            Ok(format!(
                "{indent}modify_u8(0x{address:X}u64, 0x{clear_mask:02X}u8, 0x{set_mask:02X}u8)?;\n"
            ))
        }
        16 => {
            let clear_mask = u16::try_from(clear_mask).map_err(|_| {
                anyhow!(
                    "field {} bit range {}..={} exceeds register {} width {}",
                    field.name,
                    field.lsb,
                    field.msb,
                    register.id,
                    register.width_bits
                )
            })?;
            let set_mask = u16::try_from(set_mask).map_err(|_| {
                anyhow!(
                    "field {} bit range {}..={} exceeds register {} width {}",
                    field.name,
                    field.lsb,
                    field.msb,
                    register.id,
                    register.width_bits
                )
            })?;
            Ok(format!(
                "{indent}modify_u16(0x{address:X}u64, 0x{clear_mask:04X}u16, 0x{set_mask:04X}u16)?;\n"
            ))
        }
        32 => {
            let clear_mask = u32::try_from(clear_mask).map_err(|_| {
                anyhow!(
                    "field {} bit range {}..={} exceeds register {} width {}",
                    field.name,
                    field.lsb,
                    field.msb,
                    register.id,
                    register.width_bits
                )
            })?;
            let set_mask = u32::try_from(set_mask).map_err(|_| {
                anyhow!(
                    "field {} bit range {}..={} exceeds register {} width {}",
                    field.name,
                    field.lsb,
                    field.msb,
                    register.id,
                    register.width_bits
                )
            })?;
            Ok(format!(
                "{indent}modify_u32(0x{address:X}u64, 0x{clear_mask:08X}u32, 0x{set_mask:08X}u32)?;\n"
            ))
        }
        _ => unreachable!("unsupported widths are rejected before mask computation"),
    }
}

fn field_value_mask(field: &ResolvedField) -> Result<u64> {
    let field_width = field
        .msb
        .checked_sub(field.lsb)
        .and_then(|delta| delta.checked_add(1))
        .ok_or_else(|| anyhow!("field {} has an invalid bit range", field.name))?;
    if field_width > 64 {
        bail!(
            "field {} width {} exceeds the 64-bit mask limit for first-cut lowering",
            field.name,
            field_width
        );
    }
    Ok(if field_width == 64 {
        u64::MAX
    } else {
        (1u64 << field_width) - 1
    })
}

fn shifted_field_mask(field: &ResolvedField, register: &ResolvedRegister) -> Result<u32> {
    if field.msb >= register.width_bits || field.lsb >= register.width_bits {
        bail!(
            "field {} bit range {}..={} exceeds register {} width {}",
            field.name,
            field.lsb,
            field.msb,
            register.id,
            register.width_bits
        );
    }
    let mask = field_value_mask(field)? << field.lsb;
    u32::try_from(mask).map_err(|_| {
        anyhow!(
            "field {} on register {} requires a mask wider than the 32-bit limit for first-cut lowering",
            field.name,
            register.id,
        )
    })
}

fn field_bit_mask(field: &ResolvedField, register: &ResolvedRegister) -> Result<u32> {
    let raw_mask = field_value_mask(field)?;
    if raw_mask.count_ones() != 1 {
        bail!(
            "field {} on register {} is not a single-bit flag",
            field.name,
            register.id
        );
    }
    shifted_field_mask(field, register)
}

fn field_bit_mask_or_range_mask(field: &ResolvedField, register: &ResolvedRegister) -> Result<u32> {
    shifted_field_mask(field, register)
}

fn extract_alternate_function_index(description: &str) -> Option<u64> {
    let af_offset = description.find("AF")?;
    let digits = description[af_offset + 2..]
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

struct EmbassyRegisterScopeInputs<'a> {
    clock_bindings: &'a HashMap<&'a str, McuClockBinding>,
    reset_bindings: &'a HashMap<&'a str, McuResetBinding>,
    interrupt_routes: &'a HashMap<&'a str, McuInterruptRoute>,
    dma_routes: &'a HashMap<&'a str, McuDmaRoute>,
    pin_routes: &'a HashMap<&'a str, McuPinRoute>,
    operations: &'a HashMap<&'a str, SemanticOperation>,
    state_machines: &'a HashMap<&'a str, SemanticStateMachine>,
    register_owners: &'a HashMap<String, Vec<String>>,
}

fn collect_embassy_register_scope(
    drivers: &[EmbassyDriverInstance],
    scope: &EmbassyRegisterScopeInputs<'_>,
) -> Result<HashSet<String>> {
    let mut required = HashSet::new();
    for driver in drivers {
        collect_control_register_peripherals(
            &mut required,
            scope.register_owners,
            driver
                .clock_binding_refs
                .iter()
                .filter_map(|item| scope.clock_bindings.get(item.as_str()))
                .flat_map(|binding| binding.control_refs.iter()),
        );
        collect_control_register_peripherals(
            &mut required,
            scope.register_owners,
            driver
                .reset_binding_refs
                .iter()
                .filter_map(|item| scope.reset_bindings.get(item.as_str()))
                .flat_map(|binding| binding.control_refs.iter()),
        );
        collect_control_register_peripherals(
            &mut required,
            scope.register_owners,
            driver
                .interrupt_route_refs
                .iter()
                .filter_map(|item| scope.interrupt_routes.get(item.as_str()))
                .flat_map(|route| route.control_refs.iter()),
        );
        collect_control_register_peripherals(
            &mut required,
            scope.register_owners,
            driver
                .dma_route_refs
                .iter()
                .filter_map(|item| scope.dma_routes.get(item.as_str()))
                .flat_map(|route| route.control_refs.iter()),
        );
        collect_control_register_peripherals(
            &mut required,
            scope.register_owners,
            driver
                .pin_roles
                .iter()
                .flat_map(|pin_role| pin_role.route_refs.iter())
                .filter_map(|item| scope.pin_routes.get(item.as_str()))
                .flat_map(|route| route.control_refs.iter()),
        );
        for operation_ref in &driver.init_operation_refs {
            let operation = scope
                .operations
                .get(operation_ref.as_str())
                .ok_or_else(|| {
                    anyhow!(
                        "operation reference {} on driver {} could not be resolved",
                        operation_ref,
                        driver.id
                    )
                })?;
            collect_operation_target_peripherals(&mut required, scope.register_owners, operation);
        }
        for state_machine_ref in &driver.state_machine_refs {
            let state_machine = scope
                .state_machines
                .get(state_machine_ref.as_str())
                .ok_or_else(|| {
                    anyhow!(
                        "state machine reference {} on driver {} could not be resolved",
                        state_machine_ref,
                        driver.id
                    )
                })?;
            collect_state_machine_target_peripherals(
                &mut required,
                scope.register_owners,
                state_machine,
            );
        }
        if let Some(bindings) = &driver.time_driver_bindings {
            for binding_ref in [
                &bindings.counter_ref,
                &bindings.alarm_ref,
                &bindings.interrupt_enable_ref,
                &bindings.interrupt_pending_ref,
            ] {
                collect_semantic_target_peripheral(
                    &mut required,
                    scope.register_owners,
                    Some(binding_ref.as_str()),
                );
            }
            for operation_ref in bindings
                .alarm_apply_operation_refs
                .iter()
                .chain(std::iter::once(&bindings.interrupt_clear_operation_ref))
            {
                let operation = scope
                    .operations
                    .get(operation_ref.as_str())
                    .ok_or_else(|| {
                        anyhow!(
                            "operation reference {} on driver {} could not be resolved",
                            operation_ref,
                            driver.id
                        )
                    })?;
                collect_operation_target_peripherals(
                    &mut required,
                    scope.register_owners,
                    operation,
                );
            }
        }
    }
    Ok(required)
}

fn collect_control_register_peripherals<'a, I>(
    required: &mut HashSet<String>,
    register_owners: &HashMap<String, Vec<String>>,
    refs: I,
) where
    I: IntoIterator<Item = &'a String>,
{
    for register_ref in refs {
        if let Some(owners) = register_owners.get(register_ref) {
            required.extend(owners.iter().cloned());
        }
    }
}

fn collect_register_owner_map(peripherals: &[Peripheral]) -> Result<HashMap<String, Vec<String>>> {
    let mut owners = HashMap::new();
    for peripheral in peripherals {
        collect_register_member_owners(&mut owners, peripheral.id.as_str(), &peripheral.registers)?;
    }
    Ok(owners)
}

fn collect_register_member_owners(
    owners: &mut HashMap<String, Vec<String>>,
    peripheral_ref: &str,
    members: &[RegisterBlockMember],
) -> Result<()> {
    for member in members {
        match member {
            RegisterBlockMember::Register(register) => {
                if owners
                    .insert(register.id.clone(), vec![peripheral_ref.to_string()])
                    .is_some()
                {
                    bail!("duplicate register {} is not supported", register.id);
                }
                for field in &register.fields {
                    owners
                        .entry(field.id.clone())
                        .or_default()
                        .push(peripheral_ref.to_string());
                }
            }
            RegisterBlockMember::Cluster(cluster) => {
                collect_register_member_owners(owners, peripheral_ref, &cluster.members)?;
            }
        }
    }
    Ok(())
}

fn collect_operation_target_peripherals(
    required: &mut HashSet<String>,
    register_owners: &HashMap<String, Vec<String>>,
    operation: &SemanticOperation,
) {
    for predicate in operation
        .preconditions
        .iter()
        .chain(operation.postconditions.iter())
    {
        collect_semantic_target_peripheral(
            required,
            register_owners,
            predicate.target_ref.as_deref(),
        );
    }
    for step in &operation.steps {
        collect_semantic_target_peripheral(required, register_owners, step.target_ref.as_deref());
    }
}

fn collect_embassy_field_scope(
    drivers: &[EmbassyDriverInstance],
    operations: &HashMap<&str, SemanticOperation>,
    state_machines: &HashMap<&str, SemanticStateMachine>,
) -> Result<HashSet<String>> {
    let mut required = HashSet::new();
    for driver in drivers {
        for operation_ref in &driver.init_operation_refs {
            let operation = operations.get(operation_ref.as_str()).ok_or_else(|| {
                anyhow!(
                    "operation reference {} on driver {} could not be resolved",
                    operation_ref,
                    driver.id
                )
            })?;
            collect_operation_field_refs(&mut required, operation);
        }
        for state_machine_ref in &driver.state_machine_refs {
            let state_machine =
                state_machines
                    .get(state_machine_ref.as_str())
                    .ok_or_else(|| {
                        anyhow!(
                            "state machine reference {} on driver {} could not be resolved",
                            state_machine_ref,
                            driver.id
                        )
                    })?;
            collect_state_machine_field_refs(&mut required, state_machine);
        }
        if let Some(bindings) = &driver.time_driver_bindings {
            for binding_ref in [
                &bindings.counter_ref,
                &bindings.alarm_ref,
                &bindings.interrupt_enable_ref,
                &bindings.interrupt_pending_ref,
            ] {
                collect_semantic_field_ref(&mut required, Some(binding_ref.as_str()));
            }
            for operation_ref in bindings
                .alarm_apply_operation_refs
                .iter()
                .chain(std::iter::once(&bindings.interrupt_clear_operation_ref))
            {
                let operation = operations.get(operation_ref.as_str()).ok_or_else(|| {
                    anyhow!(
                        "operation reference {} on driver {} could not be resolved",
                        operation_ref,
                        driver.id
                    )
                })?;
                collect_operation_field_refs(&mut required, operation);
            }
        }
    }
    Ok(required)
}

fn collect_state_machine_target_peripherals(
    required: &mut HashSet<String>,
    register_owners: &HashMap<String, Vec<String>>,
    state_machine: &SemanticStateMachine,
) {
    for state in &state_machine.states {
        for predicate in &state.invariants {
            collect_semantic_target_peripheral(
                required,
                register_owners,
                predicate.target_ref.as_deref(),
            );
        }
    }
    for transition in &state_machine.transitions {
        for predicate in &transition.conditions {
            collect_semantic_target_peripheral(
                required,
                register_owners,
                predicate.target_ref.as_deref(),
            );
        }
        for effect in &transition.effects {
            collect_semantic_target_peripheral(
                required,
                register_owners,
                effect.target_ref.as_deref(),
            );
        }
    }
}

fn collect_operation_field_refs(required: &mut HashSet<String>, operation: &SemanticOperation) {
    for predicate in operation
        .preconditions
        .iter()
        .chain(operation.postconditions.iter())
    {
        collect_semantic_field_ref(required, predicate.target_ref.as_deref());
    }
    for step in &operation.steps {
        collect_semantic_field_ref(required, step.target_ref.as_deref());
    }
}

fn collect_state_machine_field_refs(
    required: &mut HashSet<String>,
    state_machine: &SemanticStateMachine,
) {
    for state in &state_machine.states {
        for predicate in &state.invariants {
            collect_semantic_field_ref(required, predicate.target_ref.as_deref());
        }
    }
    for transition in &state_machine.transitions {
        for predicate in &transition.conditions {
            collect_semantic_field_ref(required, predicate.target_ref.as_deref());
        }
        for effect in &transition.effects {
            collect_semantic_field_ref(required, effect.target_ref.as_deref());
        }
    }
}

fn collect_semantic_target_peripheral(
    required: &mut HashSet<String>,
    register_owners: &HashMap<String, Vec<String>>,
    target_ref: Option<&str>,
) {
    let Some(target_ref) = target_ref else {
        return;
    };
    if !target_ref.starts_with("reg.") && !target_ref.starts_with("field.") {
        return;
    }
    if let Some(owners) = register_owners.get(target_ref) {
        required.extend(owners.iter().cloned());
    }
}

fn collect_semantic_field_ref(required: &mut HashSet<String>, target_ref: Option<&str>) {
    let Some(target_ref) = target_ref else {
        return;
    };
    if target_ref.starts_with("field.") {
        required.insert(target_ref.to_string());
    }
}

fn collect_register_map(
    peripherals: &[Peripheral],
    allowed_peripheral_refs: &HashSet<String>,
) -> Result<HashMap<String, ResolvedRegister>> {
    let mut registers = HashMap::new();
    for peripheral in peripherals {
        if !allowed_peripheral_refs.contains(peripheral.id.as_str()) {
            continue;
        }
        if peripheral.registers.is_empty() {
            continue;
        }
        let base_address = peripheral.base_address.ok_or_else(|| {
            anyhow!(
                "peripheral {} is missing baseAddress required for code generation",
                peripheral.id
            )
        })?;
        collect_register_members(
            &mut registers,
            peripheral.id.as_str(),
            base_address,
            0,
            &peripheral.registers,
        )?;
    }
    Ok(registers)
}

fn collect_field_map(
    registers: &HashMap<String, ResolvedRegister>,
    required_field_refs: &HashSet<String>,
) -> Result<HashMap<String, Vec<ResolvedFieldTarget>>> {
    let mut fields = HashMap::new();
    for register in registers.values() {
        for field in &register.fields {
            if !required_field_refs.contains(&field.id) {
                continue;
            }
            let resolved = ResolvedFieldTarget {
                peripheral_ref: register.peripheral_ref.clone(),
                register_id: register.id.clone(),
                field: field.clone(),
            };
            fields
                .entry(field.id.clone())
                .or_insert_with(Vec::new)
                .push(resolved);
        }
    }
    Ok(fields)
}

fn resolve_field_target<'a, F>(
    fields: &'a HashMap<String, Vec<ResolvedFieldTarget>>,
    field_ref: &str,
    expected_peripheral_ref: Option<&str>,
    missing_error: F,
) -> Result<&'a ResolvedFieldTarget>
where
    F: FnOnce() -> anyhow::Error,
{
    let candidates = fields.get(field_ref).ok_or_else(missing_error)?;
    if let Some(expected_peripheral_ref) = expected_peripheral_ref {
        let mut matches = candidates
            .iter()
            .filter(|candidate| candidate.peripheral_ref == expected_peripheral_ref);
        let first = matches.next().ok_or_else(|| {
            anyhow!(
                "field {} does not resolve within peripheral {}",
                field_ref,
                expected_peripheral_ref
            )
        })?;
        if matches.next().is_some() {
            bail!(
                "field {} resolves multiple times within peripheral {}",
                field_ref,
                expected_peripheral_ref
            );
        }
        return Ok(first);
    }
    if candidates.len() != 1 {
        bail!(
            "field {} is ambiguous across multiple peripherals and must be resolved in a target-scoped context",
            field_ref
        );
    }
    Ok(&candidates[0])
}

fn try_resolve_field_target<'a>(
    fields: &'a HashMap<String, Vec<ResolvedFieldTarget>>,
    field_ref: &str,
    expected_peripheral_ref: Option<&str>,
) -> Result<Option<&'a ResolvedFieldTarget>> {
    match fields.get(field_ref) {
        Some(_) => resolve_field_target(fields, field_ref, expected_peripheral_ref, || {
            anyhow!("field {} is unavailable", field_ref)
        })
        .map(Some),
        None => Ok(None),
    }
}

fn collect_register_members(
    registers: &mut HashMap<String, ResolvedRegister>,
    peripheral_ref: &str,
    base_address: u64,
    parent_offset: u64,
    members: &[RegisterBlockMember],
) -> Result<()> {
    for member in members {
        match member {
            RegisterBlockMember::Register(register) => {
                if register.array.is_some() {
                    bail!(
                        "register {} uses arrayed instances, which are not yet supported for Embassy lowering",
                        register.id
                    );
                }
                let absolute_address = checked_offset_add(
                    checked_offset_add(base_address, parent_offset, "parent register offset")?,
                    register.offset_bytes,
                    &format!("register {} offset", register.id),
                )?;
                let resolved = ResolvedRegister {
                    id: register.id.clone(),
                    name: register.name.clone(),
                    peripheral_ref: peripheral_ref.to_string(),
                    absolute_address,
                    width_bits: register.width_bits,
                    fields: register
                        .fields
                        .iter()
                        .map(|field| ResolvedField {
                            id: field.id.clone(),
                            name: field.name.clone(),
                            description: field.description.clone(),
                            lsb: field.bit_range.lsb,
                            msb: field.bit_range.msb,
                        })
                        .collect(),
                };
                if registers.insert(register.id.clone(), resolved).is_some() {
                    bail!("duplicate register {} is not supported", register.id);
                }
            }
            RegisterBlockMember::Cluster(cluster) => {
                if cluster.array.is_some() {
                    bail!(
                        "cluster {} uses arrayed instances, which are not yet supported for Embassy lowering",
                        cluster.name
                    );
                }
                collect_register_members(
                    registers,
                    peripheral_ref,
                    base_address,
                    checked_offset_add(
                        parent_offset,
                        cluster.offset_bytes,
                        &format!("cluster {} offset", cluster.name),
                    )?,
                    &cluster.members,
                )?;
            }
        }
    }
    Ok(())
}

fn checked_offset_add(left: u64, right: u64, context: &str) -> Result<u64> {
    left.checked_add(right)
        .ok_or_else(|| anyhow!("{context} overflows u64 address space"))
}

fn operation_method_slug(operation: &SemanticOperation) -> String {
    let candidate = operation
        .id
        .rsplit('.')
        .next()
        .unwrap_or(operation.name.as_str());
    to_rust_method_name(candidate)
}

fn state_machine_method_slug(state_machine: &SemanticStateMachine) -> String {
    let candidate = state_machine
        .id
        .rsplit('.')
        .next()
        .unwrap_or(state_machine.name.as_str());
    to_rust_method_name(candidate)
}

fn last_ref_segment(reference: &str) -> &str {
    reference.rsplit('.').next().unwrap_or(reference)
}

fn normalize_search_text(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_uppercase())
        .collect()
}

fn subject_aliases(value: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    let normalized = normalize_search_text(value);
    if !normalized.is_empty() {
        aliases.push(normalized.clone());
    }

    let upper = value.to_ascii_uppercase();
    if let Some(port) = upper.strip_prefix("GPIO")
        && !port.is_empty()
    {
        aliases.push(format!("IOP{port}"));
        aliases.push(format!("PORT{port}"));
        aliases.push(format!("IOPORT{port}"));
    }
    if let Some(index) = upper.strip_prefix("PWM")
        && !index.is_empty()
        && index.chars().all(|ch| ch.is_ascii_digit())
    {
        aliases.push(format!("TIM{index}"));
    }
    if let Some(index) = upper.strip_prefix("UART")
        && !index.is_empty()
    {
        aliases.push(format!("USART{index}"));
    }
    if let Some(index) = upper.strip_prefix("USART")
        && !index.is_empty()
    {
        aliases.push(format!("UART{index}"));
    }

    aliases.sort();
    aliases.dedup();
    aliases
}

struct ParsedFieldWrite {
    field_name: String,
    value: u64,
}

fn parse_field_write_expression(
    expression: Option<&SemanticExpression>,
    _value: Option<&Value>,
    operation_id: &str,
    step_index: u32,
) -> Result<ParsedFieldWrite> {
    if let Some(expression) = expression {
        let language = expression.language.as_deref().ok_or_else(|| {
            anyhow!(
                "operation {} step {} is missing expression language required for lowering",
                operation_id,
                step_index
            )
        })?;
        if language != "plain" {
            bail!(
                "operation {} step {} uses unsupported expression language {} for first-cut lowering",
                operation_id,
                step_index,
                language
            );
        }
        return parse_field_write_text(&expression.text).with_context(|| {
            format!("parsing expression on operation {operation_id} step {step_index}")
        });
    }
    bail!(
        "operation {} step {} requires explicit expression text for field-level lowering",
        operation_id,
        step_index
    )
}

fn parse_field_write_text(text: &str) -> Result<ParsedFieldWrite> {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("Set ") {
        if let Some((field_name, value)) = parse_explicit_assignment(rest)? {
            if value != 1 {
                bail!("plain-text write expression {trimmed} contradicts Set semantics");
            }
            return Ok(ParsedFieldWrite { field_name, value });
        }
        return Ok(ParsedFieldWrite {
            field_name: extract_field_name(rest),
            value: 1,
        });
    }
    if let Some(rest) = trimmed.strip_prefix("Clear ") {
        if let Some((field_name, value)) = parse_explicit_assignment(rest)? {
            if value != 0 {
                bail!("plain-text write expression {trimmed} contradicts Clear semantics");
            }
            return Ok(ParsedFieldWrite { field_name, value });
        }
        return Ok(ParsedFieldWrite {
            field_name: extract_field_name(rest),
            value: 0,
        });
    }
    if let Some(rest) = trimmed.strip_prefix("Write ") {
        let mut parts = rest.split('=');
        let left = parts.next().unwrap_or_default().trim();
        let right = parts
            .next()
            .ok_or_else(|| anyhow!("unsupported write expression {trimmed}"))?
            .trim();
        return Ok(ParsedFieldWrite {
            field_name: extract_field_name(left),
            value: parse_integer_literal(right)?,
        });
    }
    bail!("unsupported plain-text write expression {trimmed}")
}

fn parse_explicit_assignment(text: &str) -> Result<Option<(String, u64)>> {
    let Some((left, right)) = text.split_once('=') else {
        return Ok(None);
    };
    Ok(Some((
        extract_field_name(left),
        parse_integer_literal(right.trim())?,
    )))
}

fn transition_effect_value(
    effect: &SemanticSideEffect,
    state_machine: &SemanticStateMachine,
    transition: &SemanticTransition,
    driver_id: &str,
) -> Result<u64> {
    match effect.kind.as_str() {
        "self-set" | "starts-hardware" => Ok(1),
        "self-clear" | "stops-hardware" => Ok(0),
        other => bail!(
            "state machine {} transition {} -> {} on driver {} uses unsupported effect kind {} for first-cut lowering",
            state_machine.id,
            transition.from,
            transition.to,
            driver_id,
            other
        ),
    }
}

fn extract_field_name(text: &str) -> String {
    text.trim()
        .trim_end_matches("= 1")
        .trim_end_matches("= 0")
        .trim()
        .rsplit('.')
        .next()
        .unwrap_or(text.trim())
        .trim()
        .to_string()
}

fn parse_integer_literal(text: &str) -> Result<u64> {
    let trimmed = text.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return u64::from_str_radix(hex, 16)
            .with_context(|| format!("parsing hexadecimal integer literal {trimmed}"));
    }
    trimmed
        .parse::<u64>()
        .with_context(|| format!("parsing integer literal {trimmed}"))
}

fn render_clock_binding_slice(items: &[McuClockBinding]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::ClockBinding {{ id: {}, name: {}, consumer_ref: {}, clock_ref: {}, controller_ref: {}, binding_kind: {}, control_refs: &{}, enable_operation_refs: &{}, disable_operation_refs: &{} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.consumer_ref),
            render_rust_string(&item.clock_ref),
            render_optional_rust_string(item.controller_ref.as_deref()),
            render_rust_string(&item.binding_kind),
            render_string_slice(&item.control_refs),
            render_string_slice(&item.enable_operation_refs),
            render_string_slice(&item.disable_operation_refs)
        )
    }))
}

fn render_reset_binding_slice(items: &[McuResetBinding]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::ResetBinding {{ id: {}, name: {}, target_ref: {}, controller_ref: {}, reset_domain_ref: {}, binding_kind: {}, control_refs: &{}, assert_operation_refs: &{}, release_operation_refs: &{} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.target_ref),
            render_optional_rust_string(item.controller_ref.as_deref()),
            render_optional_rust_string(item.reset_domain_ref.as_deref()),
            render_rust_string(&item.binding_kind),
            render_string_slice(&item.control_refs),
            render_string_slice(&item.assert_operation_refs),
            render_string_slice(&item.release_operation_refs)
        )
    }))
}

fn render_interrupt_source_slice(items: &[McuInterruptSource]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::InterruptSource {{ id: {}, name: {}, source_ref: {}, producer_ref: {}, kind: {}, flag_refs: &{}, clear_operation_refs: &{} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.source_ref),
            render_optional_rust_string(item.producer_ref.as_deref()),
            render_rust_string(&item.kind),
            render_string_slice(&item.flag_refs),
            render_string_slice(&item.clear_operation_refs)
        )
    }))
}

fn render_interrupt_route_slice(items: &[McuInterruptRoute]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::InterruptRoute {{ id: {}, name: {}, source_ref: {}, interrupt_ref: {}, controller_ref: {}, cpu_target_ref: {}, line_index: {}, route_type: {}, control_refs: &{}, acknowledge_operation_refs: &{}, shared_group: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.source_ref),
            render_rust_string(&item.interrupt_ref),
            render_rust_string(&item.controller_ref),
            render_optional_rust_string(item.cpu_target_ref.as_deref()),
            render_optional_u32(item.line_index),
            render_rust_string(&item.route_type),
            render_string_slice(&item.control_refs),
            render_string_slice(&item.acknowledge_operation_refs),
            render_optional_rust_string(item.shared_group.as_deref())
        )
    }))
}

fn render_dma_channel_slice(items: &[McuDmaChannel]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::DmaChannel {{ id: {}, name: {}, controller_ref: {}, target_ref: {}, channel_index: {}, capabilities: &{}, priority_levels: &{} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.controller_ref),
            render_optional_rust_string(item.target_ref.as_deref()),
            item.channel_index,
            render_string_slice(&item.capabilities),
            render_string_slice(&item.priority_levels)
        )
    }))
}

fn render_dma_route_slice(items: &[McuDmaRoute]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::DmaRoute {{ id: {}, name: {}, peripheral_ref: {}, signal: {}, channel_ref: {}, direction: {}, control_refs: &{}, shared_channel_group_ref: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.peripheral_ref),
            render_optional_rust_string(item.signal.as_deref()),
            render_rust_string(&item.channel_ref),
            render_rust_string(&item.direction),
            render_string_slice(&item.control_refs),
            render_optional_rust_string(item.shared_channel_group_ref.as_deref())
        )
    }))
}

fn render_pin_route_slice(items: &[McuPinRoute]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::PinRoute {{ id: {}, name: {}, pin_ref: {}, peripheral_ref: {}, signal: {}, route_type: {}, control_refs: &{}, electrical_constraint_refs: &{}, conflict_refs: &{}, default_after_reset: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.pin_ref),
            render_rust_string(&item.peripheral_ref),
            render_rust_string(&item.signal),
            render_rust_string(&item.route_type),
            render_string_slice(&item.control_refs),
            render_string_slice(&item.electrical_constraint_refs),
            render_string_slice(&item.conflict_refs),
            render_optional_bool(item.default_after_reset)
        )
    }))
}

fn render_semantic_operation_slice(items: &[SemanticOperation]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::SemanticOperation {{ id: {}, name: {}, description: {}, kind: {}, target_refs: &{}, steps: &{}, preconditions: &{}, postconditions: &{} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_optional_rust_string(item.description.as_deref()),
            render_optional_rust_string(item.kind.as_deref()),
            render_string_slice(&item.target_refs),
            render_semantic_operation_step_slice(&item.steps),
            render_semantic_predicate_slice(&item.preconditions),
            render_semantic_predicate_slice(&item.postconditions)
        )
    }))
}

fn render_semantic_operation_step_slice(items: &[SemanticOperationStep]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::SemanticOperationStep {{ index: {}, action: {}, target_ref: {}, expression: {}, value: {}, description: {} }}",
            item.index,
            render_rust_string(&item.action),
            render_optional_rust_string(item.target_ref.as_deref()),
            render_optional_semantic_expression(item.expression.as_ref()),
            render_optional_value_literal(item.value.as_ref()),
            render_optional_rust_string(item.description.as_deref())
        )
    }))
}

fn render_semantic_state_machine_slice(items: &[SemanticStateMachine]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::SemanticStateMachine {{ id: {}, name: {}, description: {}, target_refs: &{}, initial_state: {}, states: &{}, transitions: &{} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_optional_rust_string(item.description.as_deref()),
            render_string_slice(&item.target_refs),
            render_optional_rust_string(item.initial_state.as_deref()),
            render_semantic_state_slice(&item.states),
            render_semantic_transition_slice(&item.transitions)
        )
    }))
}

fn render_semantic_state_slice(items: &[SemanticState]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::SemanticState {{ name: {}, description: {}, invariants: &{} }}",
            render_rust_string(&item.name),
            render_optional_rust_string(item.description.as_deref()),
            render_semantic_predicate_slice(&item.invariants)
        )
    }))
}

fn render_semantic_transition_slice(items: &[SemanticTransition]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::SemanticTransition {{ from: {}, to: {}, trigger: {}, conditions: &{}, effects: &{} }}",
            render_rust_string(&item.from),
            render_rust_string(&item.to),
            render_optional_rust_string(item.trigger.as_deref()),
            render_semantic_predicate_slice(&item.conditions),
            render_semantic_side_effect_slice(&item.effects)
        )
    }))
}

fn render_semantic_side_effect_slice(items: &[SemanticSideEffect]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::SemanticSideEffect {{ kind: {}, target_ref: {}, description: {} }}",
            render_rust_string(&item.kind),
            render_optional_rust_string(item.target_ref.as_deref()),
            render_optional_rust_string(item.description.as_deref())
        )
    }))
}

fn render_semantic_predicate_slice(items: &[SemanticPredicate]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::Predicate {{ kind: {}, target_ref: {}, expression: {}, expected_value: {}, description: {} }}",
            render_rust_string(&item.kind),
            render_optional_rust_string(item.target_ref.as_deref()),
            render_optional_semantic_expression(item.expression.as_ref()),
            render_optional_value_literal(item.expected_value.as_ref()),
            render_optional_rust_string(item.description.as_deref())
        )
    }))
}

fn render_pin_role_slice(const_prefix: &str, items: &[ResolvedEmbassyPinRole]) -> String {
    render_struct_slice(items.iter().enumerate().map(|(index, item)| {
        format!(
            "metadata::PinRole {{ role: {}, signal: {}, routes: {const_prefix}_PIN_ROLE_{index}_ROUTES, requirement: metadata::ResourceRequirement::{} }}",
            render_rust_string(&item.role),
            render_rust_string(&item.signal),
            render_resource_requirement(&item.requirement)
        )
    }))
}

fn render_provenance_source_slice(items: &[HairProvenanceSource]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "ProvenanceSource {{ id: {}, name: {}, kind: {}, path: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_optional_rust_string(item.kind.as_deref()),
            render_optional_rust_string(item.path.as_deref())
        )
    }))
}

fn render_provenance_evidence_slice(items: &[HairProvenanceEvidence]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "ProvenanceEvidence {{ id: {}, name: {}, source_ref: {}, normalized_claim: {}, extraction_method: {}, confidence: {}, locator: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.source_ref),
            render_optional_rust_string(item.normalized_claim.as_deref()),
            render_optional_rust_string(item.extraction_method.as_deref()),
            render_optional_f64(item.confidence),
            render_optional_rust_string(render_locator_summary(item.locator.as_ref()).as_deref())
        )
    }))
}

fn render_resource_requirement(requirement: &ResourceRequirement) -> &'static str {
    match requirement {
        ResourceRequirement::Required => "Required",
        ResourceRequirement::Optional => "Optional",
        ResourceRequirement::MutuallyExclusive => "MutuallyExclusive",
    }
}

fn render_named_entity_slice(items: &[String]) -> String {
    format!(
        "[{}]",
        items
            .iter()
            .map(|item| render_rust_string(item))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn render_struct_slice<I>(items: I) -> String
where
    I: IntoIterator<Item = String>,
{
    format!("[{}]", items.into_iter().collect::<Vec<_>>().join(", "))
}

fn render_string_slice(items: &[String]) -> String {
    format!(
        "[{}]",
        items
            .iter()
            .map(|item| render_rust_string(item))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn render_optional_rust_string(value: Option<&str>) -> String {
    match value {
        Some(text) => format!("Some({})", render_rust_string(text)),
        None => "None".to_string(),
    }
}

fn render_optional_u32(value: Option<u32>) -> String {
    match value {
        Some(number) => format!("Some({number})"),
        None => "None".to_string(),
    }
}

fn render_optional_bool(value: Option<bool>) -> String {
    match value {
        Some(flag) => format!("Some({flag})"),
        None => "None".to_string(),
    }
}

fn render_optional_semantic_expression(value: Option<&SemanticExpression>) -> String {
    match value {
        Some(expression) => format!(
            "Some(metadata::SemanticExpression {{ language: {}, text: {} }})",
            render_optional_rust_string(expression.language.as_deref()),
            render_rust_string(&expression.text)
        ),
        None => "None".to_string(),
    }
}

fn render_optional_value_literal(value: Option<&Value>) -> String {
    match value {
        Some(Value::Number(number)) if number.is_i64() => {
            format!(
                "Some(metadata::ValueLiteral::Integer({}))",
                number
                    .as_i64()
                    .expect("serde_json::Number reported is_i64 but did not yield an i64")
            )
        }
        Some(Value::Number(number)) if number.is_u64() => format!(
            "Some(metadata::ValueLiteral::Unsigned({}u64))",
            number
                .as_u64()
                .expect("serde_json::Number reported is_u64 but did not yield a u64")
        ),
        Some(Value::Number(number)) => format!(
            "Some(metadata::ValueLiteral::Number({}))",
            render_f64_literal(
                number
                    .as_f64()
                    .expect("serde_json::Number should yield f64 in non-integer rendering path"),
            )
        ),
        Some(Value::String(text)) => format!(
            "Some(metadata::ValueLiteral::String({}))",
            render_rust_string(text)
        ),
        Some(Value::Bool(flag)) => format!("Some(metadata::ValueLiteral::Bool({flag}))"),
        Some(other) => format!(
            "Some(metadata::ValueLiteral::String({}))",
            render_rust_string(&other.to_string())
        ),
        None => "None".to_string(),
    }
}

fn render_f64_literal(value: f64) -> String {
    if value.is_nan() {
        return "f64::NAN".to_string();
    }
    if value == f64::INFINITY {
        return "f64::INFINITY".to_string();
    }
    if value == f64::NEG_INFINITY {
        return "f64::NEG_INFINITY".to_string();
    }
    format!("{value:?}f64")
}

fn render_optional_f64(value: Option<f64>) -> String {
    match value {
        Some(number) => format!("Some({})", render_f64_literal(number)),
        None => "None".to_string(),
    }
}

fn render_locator_summary(locator: Option<&HairProvenanceLocator>) -> Option<String> {
    let locator = locator?;
    let mut parts = Vec::new();
    if let Some(fragment) = &locator.fragment {
        parts.push(format!("fragment={fragment}"));
    }
    if let Some(section) = &locator.section {
        parts.push(format!("section={section}"));
    }
    if let Some(line_start) = locator.line_start {
        match locator.line_end {
            Some(line_end) if line_end != line_start => {
                parts.push(format!("lines={line_start}-{line_end}"));
            }
            _ => parts.push(format!("line={line_start}")),
        }
    }
    if let Some(page_start) = locator.page_start {
        match locator.page_end {
            Some(page_end) if page_end != page_start => {
                parts.push(format!("pages={page_start}-{page_end}"));
            }
            _ => parts.push(format!("page={page_start}")),
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(", "))
    }
}

fn render_rust_string(value: &str) -> String {
    format!("{value:?}")
}

fn render_comment_text(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch == '\r' || ch == '\n' || ch.is_control() {
                ' '
            } else {
                ch
            }
        })
        .collect()
}

fn to_rust_const_name(text: &str) -> String {
    let mut output = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_uppercase());
        } else {
            output.push('_');
        }
    }
    if output.is_empty() {
        "GENERATED".to_string()
    } else if output
        .chars()
        .next()
        .map(|ch| ch.is_ascii_alphabetic() || ch == '_')
        != Some(true)
    {
        format!("GENERATED_{output}")
    } else {
        output
    }
}

fn generate_svd(document: &HairDocument) -> Result<String> {
    let architecture = resolve_architecture(document)?;

    let address_unit_bits = architecture
        .and_then(|architecture| architecture.address_unit_bits)
        .unwrap_or(8);
    let width_bits = architecture
        .and_then(|architecture| architecture.data_bus_width_bits)
        .unwrap_or(32);

    let interrupts = document
        .structure
        .device
        .interrupts
        .iter()
        .map(|interrupt| (interrupt.id.as_str(), interrupt))
        .collect::<HashMap<_, _>>();
    let peripherals = document
        .structure
        .device
        .peripherals
        .iter()
        .map(|peripheral| (peripheral.id.as_str(), peripheral))
        .collect::<HashMap<_, _>>();
    let peripheral_interrupts =
        assign_svd_interrupts(&document.structure.device, &peripherals, &interrupts)?;

    let mut xml = XmlWriter::new(XmlOptions {
        indent: xmlwriter::Indent::None,
        ..XmlOptions::default()
    });
    xml.write_declaration();
    xml.start_element("device");
    xml.write_attribute("schemaVersion", "1.3");
    write_text_element(&mut xml, "name", &document.structure.device.name);
    write_text_element(&mut xml, "version", &document.metadata.version);
    write_text_element(
        &mut xml,
        "description",
        document
            .metadata
            .description
            .as_deref()
            .unwrap_or(&document.metadata.title),
    );
    write_text_element(&mut xml, "vendor", &document.structure.device.vendor);
    write_text_element(&mut xml, "addressUnitBits", &address_unit_bits.to_string());
    write_text_element(&mut xml, "width", &width_bits.to_string());

    let cpu = &document.structure.device.cpu;
    xml.start_element("cpu");
    write_text_element(&mut xml, "name", &cpu.name);
    write_text_element(&mut xml, "revision", &cpu.revision);
    write_text_element(&mut xml, "endian", &svd_endian(&cpu.endianness)?);
    write_text_element(
        &mut xml,
        "mpuPresent",
        &cpu.feature_flags.mpu_present.to_string(),
    );
    write_text_element(
        &mut xml,
        "fpuPresent",
        &cpu.feature_flags.fpu_present.to_string(),
    );
    if let Some(fpu_double_precision) = cpu.feature_flags.fpu_double_precision {
        write_text_element(&mut xml, "fpuDP", &fpu_double_precision.to_string());
    }

    fn resolve_architecture(document: &HairDocument) -> Result<Option<&ArchitectureProfile>> {
        if let Some(reference) = document.structure.device.architecture_ref.as_deref() {
            return document
                .structure
                .architectures
                .iter()
                .find(|candidate| candidate.id == reference)
                .map(Some)
                .ok_or_else(|| anyhow!("device architectureRef {reference} did not match any structure.architectures entry"));
        }

        Ok(match document.structure.architectures.as_slice() {
            [architecture] => Some(architecture),
            _ => None,
        })
    }
    if let Some(dsp_present) = cpu.feature_flags.dsp_present {
        write_text_element(&mut xml, "dspPresent", &dsp_present.to_string());
    }
    if let Some(icache_present) = cpu.feature_flags.icache_present {
        write_text_element(&mut xml, "icachePresent", &icache_present.to_string());
    }
    if let Some(dcache_present) = cpu.feature_flags.dcache_present {
        write_text_element(&mut xml, "dcachePresent", &dcache_present.to_string());
    }
    if let Some(itcm_present) = cpu.feature_flags.itcm_present {
        write_text_element(&mut xml, "itcmPresent", &itcm_present.to_string());
    }
    if let Some(dtcm_present) = cpu.feature_flags.dtcm_present {
        write_text_element(&mut xml, "dtcmPresent", &dtcm_present.to_string());
    }
    if let Some(vtor_present) = cpu.feature_flags.vtor_present {
        write_text_element(&mut xml, "vtorPresent", &vtor_present.to_string());
    }
    write_text_element(
        &mut xml,
        "nvicPrioBits",
        &cpu.interrupt_priority_bits.to_string(),
    );
    write_text_element(
        &mut xml,
        "vendorSystickConfig",
        &cpu.feature_flags.vendor_system_timer_config.to_string(),
    );
    xml.end_element();

    xml.start_element("peripherals");
    for peripheral in &document.structure.device.peripherals {
        write_peripheral(
            &mut xml,
            peripheral,
            &peripherals,
            peripheral_interrupts
                .get(peripheral.id.as_str())
                .map(Vec::as_slice)
                .unwrap_or(&[]),
        )?;
    }
    xml.end_element();
    xml.end_element();

    Ok(xml.end_document())
}

fn write_peripheral(
    xml: &mut XmlWriter,
    peripheral: &Peripheral,
    peripherals: &HashMap<&str, &Peripheral>,
    interrupts: &[&Interrupt],
) -> Result<()> {
    xml.start_element("peripheral");
    if let Some(derived_from) = &peripheral.derived_from_ref {
        let derived_peripheral =
            peripherals
                .get(derived_from.as_str())
                .copied()
                .ok_or_else(|| {
                    anyhow!(
                        "peripheral {} references unknown derivedFromRef {}",
                        peripheral.id,
                        derived_from
                    )
                })?;
        xml.write_attribute("derivedFrom", &derived_peripheral.name);
    }
    write_text_element(xml, "name", &peripheral.name);
    if let Some(description) = &peripheral.description {
        write_text_element(xml, "description", description);
    }
    let base_address = peripheral
        .base_address
        .ok_or_else(|| anyhow!("peripheral {} is missing baseAddress", peripheral.id))?;
    write_text_element(xml, "baseAddress", &format_hex(base_address));
    write_text_element(xml, "groupName", &peripheral.peripheral_type);

    for address_block in &peripheral.address_blocks {
        xml.start_element("addressBlock");
        if let Some(name) = &address_block.name {
            write_text_element(xml, "name", name);
        }
        write_text_element(xml, "offset", &format_hex(address_block.offset_bytes));
        write_text_element(xml, "size", &format_hex(address_block.size_bytes));
        write_text_element(xml, "usage", &address_block.usage);
        xml.end_element();
    }

    for interrupt in interrupts {
        xml.start_element("interrupt");
        write_text_element(xml, "name", &interrupt.name);
        if let Some(description) = &interrupt.description {
            write_text_element(xml, "description", description);
        }
        write_text_element(xml, "value", &interrupt.number.to_string());
        xml.end_element();
    }

    if !peripheral.registers.is_empty() {
        let register_names = build_register_name_index(&peripheral.registers)?;
        xml.start_element("registers");
        for member in &peripheral.registers {
            write_register_member(xml, member, &register_names)?;
        }
        xml.end_element();
    }

    xml.end_element();
    Ok(())
}

fn is_svd_system_exception(interrupt: &Interrupt) -> bool {
    interrupt.number < 0
}

fn assign_svd_interrupts<'a>(
    device: &'a Device,
    peripherals: &HashMap<&'a str, &'a Peripheral>,
    interrupts: &HashMap<&'a str, &'a Interrupt>,
) -> Result<HashMap<&'a str, Vec<&'a Interrupt>>> {
    let mut assigned = HashMap::<&str, Vec<&Interrupt>>::new();
    let mut emitted_interrupt_ids = HashSet::<&str>::new();

    for peripheral in &device.peripherals {
        for interrupt_ref in &peripheral.interrupt_refs {
            let interrupt = interrupts
                .get(interrupt_ref.as_str())
                .copied()
                .ok_or_else(|| {
                    anyhow!(
                        "peripheral {} references unknown interrupt {}",
                        peripheral.id,
                        interrupt_ref
                    )
                })?;
            if is_svd_system_exception(interrupt) {
                continue;
            }
            assigned
                .entry(peripheral.id.as_str())
                .or_default()
                .push(interrupt);
            emitted_interrupt_ids.insert(interrupt.id.as_str());
        }
    }

    for interrupt in &device.interrupts {
        if is_svd_system_exception(interrupt) {
            continue;
        }
        if emitted_interrupt_ids.contains(interrupt.id.as_str()) {
            continue;
        }

        let matches = device
            .peripherals
            .iter()
            .filter(|peripheral| peripheral.name == interrupt.name)
            .collect::<Vec<_>>();

        match matches.as_slice() {
            [peripheral] => {
                let peripheral_id = peripheral.id.as_str();
                let peripheral = peripherals.get(peripheral_id).copied().ok_or_else(|| {
                    anyhow!(
                        "internal error: peripheral {} was not indexed",
                        peripheral_id
                    )
                })?;
                assigned
                    .entry(peripheral.id.as_str())
                    .or_default()
                    .push(interrupt);
                emitted_interrupt_ids.insert(interrupt.id.as_str());
            }
            [] => bail!(
                "device interrupt {} ({} at vector {}) is not linked by interruptRefs and could not be attributed to any peripheral for SVD generation",
                interrupt.id,
                interrupt.name,
                interrupt.number
            ),
            _ => bail!(
                "device interrupt {} ({} at vector {}) is not linked by interruptRefs and matches multiple peripherals, so SVD generation cannot attribute it safely",
                interrupt.id,
                interrupt.name,
                interrupt.number
            ),
        }
    }

    Ok(assigned)
}

fn build_register_name_index(
    members: &[RegisterBlockMember],
) -> Result<HashMap<String, RegisterBinding>> {
    let mut names = HashMap::new();
    for member in members {
        if let RegisterBlockMember::Register(register) = member {
            let svd_name = svd_item_name(&register.name, register.array.as_ref())?;
            let binding = RegisterBinding {
                svd_name: svd_name.clone(),
                offset_bytes: register.offset_bytes,
                width_bits: register.width_bits,
                access: register.access.clone(),
                reset_value: register.reset_value.clone(),
                reset_mask: register.reset_mask.clone(),
                array: register.array.clone(),
            };
            if let Some(previous_binding) = names.insert(register.id.clone(), binding) {
                bail!(
                    "duplicate register id {} for SVD generation (names: {} and {})",
                    register.id,
                    previous_binding.svd_name,
                    svd_name
                );
            }
        }
    }
    Ok(names)
}

fn write_register_member(
    xml: &mut XmlWriter,
    member: &RegisterBlockMember,
    register_names: &HashMap<String, RegisterBinding>,
) -> Result<()> {
    match member {
        RegisterBlockMember::Register(register) => write_register(xml, register, register_names),
        RegisterBlockMember::Cluster(cluster) => write_cluster(xml, cluster),
    }
}

fn write_register(
    xml: &mut XmlWriter,
    register: &Register,
    register_names: &HashMap<String, RegisterBinding>,
) -> Result<()> {
    validate_register_field_layout(register)?;
    xml.start_element("register");
    write_array_shape(xml, &register.name, register.array.as_ref())?;
    if let Some(display_name) = &register.display_name {
        write_text_element(xml, "displayName", display_name);
    }
    if let Some(description) = &register.description {
        write_text_element(xml, "description", description);
    }
    if let Some(alternate_of_ref) = &register.alternate_of_ref {
        if alternate_of_ref == &register.id {
            bail!(
                "register {} references itself via alternateOfRef",
                register.id
            );
        }
        let alternate_binding = register_names.get(alternate_of_ref).ok_or_else(|| {
            anyhow!(
                "register {} references unknown alternateOfRef {}",
                register.id,
                alternate_of_ref
            )
        })?;
        validate_alternate_register_contract(register, alternate_of_ref, alternate_binding)?;
        write_text_element(xml, "alternateRegister", &alternate_binding.svd_name);
    }
    write_text_element(xml, "addressOffset", &format_hex(register.offset_bytes));
    write_text_element(xml, "size", &register.width_bits.to_string());
    if let Some(access) = &register.access {
        write_text_element(xml, "access", &svd_access(access)?);
    }
    if let Some(reset_value) = &register.reset_value {
        write_text_element(
            xml,
            "resetValue",
            &format_hex(json_u64(reset_value, "register resetValue")?),
        );
    }
    if let Some(reset_mask) = &register.reset_mask {
        write_text_element(
            xml,
            "resetMask",
            &format_hex(json_u64(reset_mask, "register resetMask")?),
        );
    }

    if !register.fields.is_empty() {
        xml.start_element("fields");
        for field in &register.fields {
            write_field(xml, field)?;
        }
        xml.end_element();
    }
    xml.end_element();
    Ok(())
}

fn validate_register_field_layout(register: &Register) -> Result<()> {
    for field in &register.fields {
        if field.bit_range.msb < field.bit_range.lsb {
            bail!("field {} has msb < lsb", field.id);
        }
        if field.bit_range.msb >= register.width_bits {
            bail!(
                "field {} exceeds register {} widthBits {}",
                field.id,
                register.id,
                register.width_bits
            );
        }
    }

    for (index, left) in register.fields.iter().enumerate() {
        for right in register.fields.iter().skip(index + 1) {
            if left.bit_range.lsb == right.bit_range.lsb
                && left.bit_range.msb == right.bit_range.msb
                && !is_alias_like_duplicate_field(left, right)
            {
                bail!(
                    "register {} has duplicate bit ownership in fields {} and {}",
                    register.id,
                    left.id,
                    right.id
                );
            }
        }
    }

    Ok(())
}

fn is_alias_like_duplicate_field(left: &Field, right: &Field) -> bool {
    let left_name = left.name.as_str();
    let right_name = right.name.as_str();

    // The reference HAIR still carries legacy alias/encoding pseudo-fields.
    // Keep rejecting USB/WWDG-style duplicate ownership while tolerating those patterns.
    has_alias_prefix(left_name, right_name) || has_alias_prefix(right_name, left_name)
}

fn has_alias_prefix(name: &str, base: &str) -> bool {
    name.strip_prefix(base).is_some_and(|suffix| {
        !suffix.is_empty()
            && (suffix.starts_with('_') || suffix.chars().all(|ch| ch.is_ascii_digit()))
    })
}

fn write_cluster(xml: &mut XmlWriter, cluster: &RegisterCluster) -> Result<()> {
    let register_names = build_register_name_index(&cluster.members)?;
    xml.start_element("cluster");
    write_array_shape(xml, &cluster.name, cluster.array.as_ref())?;
    if let Some(description) = &cluster.description {
        write_text_element(xml, "description", description);
    }
    write_text_element(xml, "addressOffset", &format_hex(cluster.offset_bytes));
    for member in &cluster.members {
        write_register_member(xml, member, &register_names)?;
    }
    xml.end_element();
    Ok(())
}

fn validate_alternate_register_contract(
    register: &Register,
    alternate_of_ref: &str,
    alternate_binding: &RegisterBinding,
) -> Result<()> {
    if register.offset_bytes != alternate_binding.offset_bytes {
        bail!(
            "register {} references alternateOfRef {} with mismatched offsetBytes",
            register.id,
            alternate_of_ref
        );
    }
    if register.width_bits != alternate_binding.width_bits {
        bail!(
            "register {} references alternateOfRef {} with mismatched widthBits",
            register.id,
            alternate_of_ref
        );
    }
    if register.access != alternate_binding.access {
        bail!(
            "register {} references alternateOfRef {} with mismatched access",
            register.id,
            alternate_of_ref
        );
    }
    if register.reset_value != alternate_binding.reset_value {
        bail!(
            "register {} references alternateOfRef {} with mismatched resetValue",
            register.id,
            alternate_of_ref
        );
    }
    if register.reset_mask != alternate_binding.reset_mask {
        bail!(
            "register {} references alternateOfRef {} with mismatched resetMask",
            register.id,
            alternate_of_ref
        );
    }
    if register.array != alternate_binding.array {
        bail!(
            "register {} references alternateOfRef {} with mismatched array shape",
            register.id,
            alternate_of_ref
        );
    }
    Ok(())
}

fn write_field(xml: &mut XmlWriter, field: &Field) -> Result<()> {
    xml.start_element("field");
    write_array_shape(xml, &field.name, field.array.as_ref())?;
    if let Some(description) = &field.description {
        write_text_element(xml, "description", description);
    }
    write_text_element(xml, "bitOffset", &field.bit_range.lsb.to_string());
    write_text_element(
        xml,
        "bitWidth",
        &(field.bit_range.msb - field.bit_range.lsb + 1).to_string(),
    );
    if let Some(access) = &field.access {
        write_text_element(xml, "access", &svd_access(access)?);
    }

    if !field.enumerated_sets.is_empty() {
        let enumerated_set = choose_enumerated_set(&field.enumerated_sets)?;
        xml.start_element("enumeratedValues");
        if let Some(usage) = &enumerated_set.usage {
            write_text_element(xml, "usage", usage);
        }
        for value in &enumerated_set.values {
            xml.start_element("enumeratedValue");
            write_text_element(xml, "name", &value.name);
            if let Some(description) = &value.description {
                write_text_element(xml, "description", description);
            }
            if let Some(raw_value) = &value.value {
                write_text_element(
                    xml,
                    "value",
                    &json_u64(raw_value, "enumerated value")?.to_string(),
                );
            }
            if value.is_default.unwrap_or(false) {
                write_text_element(xml, "isDefault", "true");
            }
            xml.end_element();
        }
        xml.end_element();
    }

    xml.end_element();
    Ok(())
}

fn choose_enumerated_set(sets: &[EnumeratedSet]) -> Result<&EnumeratedSet> {
    if let Some(set) = sets
        .iter()
        .find(|set| set.usage.as_deref() == Some("read-write"))
    {
        return Ok(set);
    }
    sets.first()
        .ok_or_else(|| anyhow!("expected at least one enumerated set"))
}

fn write_array_shape(xml: &mut XmlWriter, name: &str, array: Option<&ArrayShape>) -> Result<()> {
    write_text_element(xml, "name", &svd_item_name(name, array)?);
    match array {
        None => {}
        Some(shape) => {
            if shape.axes.len() != 1 {
                bail!("multi-axis arrays are not supported in first-cut SVD generation");
            }
            let axis = &shape.axes[0];
            let stride_bytes = axis
                .stride_bytes
                .ok_or_else(|| anyhow!("arrayed item {} is missing strideBytes", name))?;
            write_text_element(xml, "dim", &axis.count.to_string());
            write_text_element(xml, "dimIncrement", &format_hex(stride_bytes));
            if !axis.labels.is_empty() {
                write_text_element(xml, "dimIndex", &axis.labels.join(","));
            }
        }
    }
    Ok(())
}

fn svd_item_name(name: &str, array: Option<&ArrayShape>) -> Result<String> {
    match array {
        None => Ok(name.to_string()),
        Some(shape) => {
            if shape.axes.len() != 1 {
                bail!("multi-axis arrays are not supported in first-cut SVD generation");
            }
            Ok(format!("{name}%s"))
        }
    }
}

fn write_text_element(xml: &mut XmlWriter, name: &str, value: &str) {
    xml.start_element(name);
    let escaped = escape_xml_text(value);
    xml.write_text(escaped.as_ref());
    xml.end_element();
}

fn escape_xml_text(value: &str) -> Cow<'_, str> {
    if !value.bytes().any(|byte| matches!(byte, b'&' | b'<' | b'>')) {
        return Cow::Borrowed(value);
    }

    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(ch),
        }
    }
    Cow::Owned(escaped)
}

fn json_u64(value: &Value, context: &str) -> Result<u64> {
    match value {
        Value::Number(number) => number
            .as_u64()
            .ok_or_else(|| anyhow!("{context} must be a non-negative integer")),
        Value::String(text) => parse_u64_literal(text, context),
        _ => bail!("{context} must be a non-negative integer"),
    }
}

fn parse_u64_literal(text: &str, context: &str) -> Result<u64> {
    let trimmed = text.trim();
    let normalized = trimmed.replace('_', "");
    let (radix, digits) = if let Some(hex) = normalized
        .strip_prefix("0x")
        .or_else(|| normalized.strip_prefix("0X"))
    {
        (16, hex)
    } else if let Some(binary) = normalized
        .strip_prefix("0b")
        .or_else(|| normalized.strip_prefix("0B"))
    {
        (2, binary)
    } else if let Some(octal) = normalized
        .strip_prefix("0o")
        .or_else(|| normalized.strip_prefix("0O"))
    {
        (8, octal)
    } else {
        (10, normalized.as_str())
    };

    if digits.is_empty() {
        bail!("{context} must be a non-negative integer")
    }

    u64::from_str_radix(digits, radix)
        .map_err(|_| anyhow!("{context} must be a non-negative integer"))
}

fn svd_access(access: &str) -> Result<String> {
    let mapped = match access {
        "read-only" => "read-only",
        "write-only" => "write-only",
        "read-write" => "read-write",
        "write-once" => "writeOnce",
        "read-write-once" => "read-writeOnce",
        unsupported => {
            bail!("access mode {unsupported} is not supported by first-cut SVD generation")
        }
    };
    Ok(mapped.to_string())
}

fn format_hex(value: u64) -> String {
    format!("0x{value:X}")
}

fn svd_endian(endianness: &str) -> Result<String> {
    let mapped = match endianness {
        "little" => "little",
        "big" => "big",
        "selectable" => "selectable",
        unsupported => bail!("endianness {unsupported} is not supported by SVD generation"),
    };
    Ok(mapped.to_string())
}

#[derive(Debug, Clone)]
struct HostSerialRegisterModel {
    driver_id: String,
    sr_address: u64,
    dr_address: u64,
    has_tx: bool,
    has_rx: bool,
    txe_mask: u32,
    tc_mask: u32,
    rxne_mask: u32,
}

#[derive(Debug, Clone)]
struct HostIndexedGpioModel {
    bsrr_address: u64,
    odr_address: u64,
}

#[derive(Debug, Clone)]
struct HostEspGpioModel {
    out_address: u64,
    out_w1ts_address: u64,
    out_w1tc_address: u64,
    enable_address: u64,
    enable_w1ts_address: u64,
    enable_w1tc_address: u64,
}

#[derive(Debug, Clone)]
struct HostUsbStreamModel {
    driver_id: String,
    ep1_address: u64,
    ep1_conf_address: u64,
    has_tx: bool,
    has_rx: bool,
    rdwr_byte_mask: u32,
    rdwr_byte_lsb: u32,
    wr_done_mask: u32,
    serial_in_data_free_mask: u32,
    serial_out_data_avail_mask: u32,
}

fn host_package_name(base: &str) -> String {
    format!("{base}-host")
}

fn host_crate_name(base: &str) -> String {
    format!("{base}_host")
}

fn collect_host_serial_register_models(
    model: &EmbassyGenerationModel,
) -> Result<Vec<HostSerialRegisterModel>> {
    let mut serial_models = Vec::new();
    for driver in &model.drivers {
        if !matches!(driver.driver_kind.as_str(), "uart" | "usart") {
            continue;
        }
        let target_ref = driver.target.target_ref.as_str();
        let has_tx = driver.pin_roles.iter().any(|role| role.signal == "TX");
        let has_rx = driver.pin_roles.iter().any(|role| role.signal == "RX");
        if !has_tx && !has_rx {
            continue;
        }
        let sr = resolve_target_register_by_name(model, target_ref, "SR")?;
        let dr = resolve_target_register_by_name(model, target_ref, "DR")?;
        let txe_mask = if has_tx {
            let txe = resolve_register_field(&sr, "TXE")?;
            field_bit_mask(&txe, &sr)?
        } else {
            0
        };
        let tc_mask = if has_tx {
            let tc = resolve_register_field(&sr, "TC")?;
            field_bit_mask(&tc, &sr)?
        } else {
            0
        };
        let rxne_mask = if has_rx {
            let rxne = resolve_register_field(&sr, "RXNE")?;
            field_bit_mask(&rxne, &sr)?
        } else {
            0
        };
        serial_models.push(HostSerialRegisterModel {
            driver_id: driver.id.clone(),
            sr_address: sr.absolute_address,
            dr_address: dr.absolute_address,
            has_tx,
            has_rx,
            txe_mask,
            tc_mask,
            rxne_mask,
        });
    }
    Ok(serial_models)
}

fn collect_host_indexed_gpio_models(
    model: &EmbassyGenerationModel,
) -> Result<Vec<HostIndexedGpioModel>> {
    let mut gpio_models = Vec::new();
    for driver in &model.drivers {
        if driver.driver_kind != "gpio-port" {
            continue;
        }
        if let ResolvedGpioPortLowering::IndexedFields { odr, bsrr, .. } =
            resolve_gpio_port_lowering(model, driver)?
        {
            gpio_models.push(HostIndexedGpioModel {
                bsrr_address: bsrr.absolute_address,
                odr_address: odr.absolute_address,
            });
        }
    }
    Ok(gpio_models)
}

fn collect_host_usb_stream_models(
    model: &EmbassyGenerationModel,
) -> Result<Vec<HostUsbStreamModel>> {
    let mut usb_models = Vec::new();
    for driver in &model.drivers {
        if driver.driver_kind != "usb-device" {
            continue;
        }
        let target_ref = driver.target.target_ref.as_str();
        let Some(ep1) = try_resolve_target_register_by_name(model, target_ref, "EP1")? else {
            continue;
        };
        let Some(ep1_conf) = try_resolve_target_register_by_name(model, target_ref, "EP1_CONF")?
        else {
            continue;
        };
        let Some(rdwr_byte) = try_resolve_register_field(&ep1, "RDWR_BYTE") else {
            continue;
        };
        let Some(wr_done) = try_resolve_register_field(&ep1_conf, "WR_DONE") else {
            continue;
        };
        let Some(serial_in_data_free) =
            try_resolve_register_field(&ep1_conf, "SERIAL_IN_EP_DATA_FREE")
        else {
            continue;
        };
        let Some(serial_out_data_avail) =
            try_resolve_register_field(&ep1_conf, "SERIAL_OUT_EP_DATA_AVAIL")
        else {
            continue;
        };
        usb_models.push(HostUsbStreamModel {
            driver_id: driver.id.clone(),
            ep1_address: ep1.absolute_address,
            ep1_conf_address: ep1_conf.absolute_address,
            has_tx: true,
            has_rx: true,
            rdwr_byte_mask: field_bit_mask_or_range_mask(&rdwr_byte, &ep1)?,
            rdwr_byte_lsb: rdwr_byte.lsb,
            wr_done_mask: field_bit_mask(&wr_done, &ep1_conf)?,
            serial_in_data_free_mask: field_bit_mask(&serial_in_data_free, &ep1_conf)?,
            serial_out_data_avail_mask: field_bit_mask(&serial_out_data_avail, &ep1_conf)?,
        });
    }
    Ok(usb_models)
}

fn collect_host_esp_gpio_models(model: &EmbassyGenerationModel) -> Result<Vec<HostEspGpioModel>> {
    let mut gpio_models = Vec::new();
    for driver in &model.drivers {
        if driver.driver_kind != "gpio-port" {
            continue;
        }
        if let ResolvedGpioPortLowering::EspRegisters {
            out,
            out_w1ts,
            out_w1tc,
            enable,
            enable_w1ts,
            enable_w1tc,
            ..
        } = resolve_gpio_port_lowering(model, driver)?
        {
            gpio_models.push(HostEspGpioModel {
                out_address: out.absolute_address,
                out_w1ts_address: out_w1ts.absolute_address,
                out_w1tc_address: out_w1tc.absolute_address,
                enable_address: enable.absolute_address,
                enable_w1ts_address: enable_w1ts.absolute_address,
                enable_w1tc_address: enable_w1tc.absolute_address,
            });
        }
    }
    Ok(gpio_models)
}

fn render_embassy_host_cargo_toml(model: &EmbassyGenerationModel) -> String {
    let mut out = format!(
        "[package]\nname = {}\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\nname = {}\npath = \"src/lib.rs\"\n",
        render_rust_string(&host_package_name(&model.crate_info.package_name)),
        render_rust_string(&host_crate_name(&model.crate_info.crate_name))
    );
    if model
        .drivers
        .iter()
        .any(|driver| has_time_driver_tag(&driver.capability_tags))
    {
        out.push_str("\n[dependencies]\n");
        out.push_str(&format!(
            "{}\n",
            render_embassy_time_driver_dependency(&model.drivers)
        ));
        out.push_str("embassy-time-queue-utils = { version = \"0.3.2\", features = [\"generic-queue-8\"] }\n");
    }
    out
}

fn render_embassy_host_lib_rs(model: &EmbassyGenerationModel) -> String {
    let mut out = String::from("pub mod metadata;\n");
    for module_name in model.module_names() {
        if module_name == "metadata" {
            continue;
        }
        out.push_str(&format!("pub mod {module_name};\n"));
    }
    out.push_str("pub mod host;\n");
    out
}

fn render_embassy_host_metadata_rs(model: &EmbassyGenerationModel) -> Result<String> {
    let mut out = render_embassy_metadata_rs(model);
    let serial_models = collect_host_serial_register_models(model)?;
    let usb_models = collect_host_usb_stream_models(model)?;
    let gpio_models = collect_host_indexed_gpio_models(model)?;
    let esp_gpio_models = collect_host_esp_gpio_models(model)?;
    out.push_str(
        "\nuse std::cell::RefCell;\nuse std::collections::{BTreeMap, BTreeSet, VecDeque};\nuse std::sync::{Arc, Mutex};\n\npub type SharedEmulatorState = Arc<GeneratedHostState>;\n\n#[derive(Debug, Default)]\nstruct SerialQueues {\n    tx: VecDeque<u8>,\n    rx: VecDeque<u8>,\n}\n\n#[derive(Debug, Default)]\npub struct GeneratedHostState {\n    registers: Mutex<BTreeMap<u64, u32>>,\n    serial: Mutex<BTreeMap<String, SerialQueues>>,\n    usb_stream: Mutex<BTreeMap<String, SerialQueues>>,\n    pending_irqs: Mutex<BTreeSet<i32>>,\n    dma_completions: Mutex<BTreeMap<String, usize>>,\n}\n\n#[derive(Debug, Clone, Copy)]\nstruct SerialRegisterModel {\n    driver_id: &'static str,\n    sr_address: u64,\n    dr_address: u64,\n    has_tx: bool,\n    has_rx: bool,\n    txe_mask: u32,\n    tc_mask: u32,\n    rxne_mask: u32,\n}\n\n#[derive(Debug, Clone, Copy)]\nstruct UsbStreamModel {\n    driver_id: &'static str,\n    ep1_address: u64,\n    ep1_conf_address: u64,\n    has_tx: bool,\n    has_rx: bool,\n    rdwr_byte_mask: u32,\n    rdwr_byte_lsb: u32,\n    wr_done_mask: u32,\n    serial_in_data_free_mask: u32,\n    serial_out_data_avail_mask: u32,\n}\n\n#[derive(Debug, Clone, Copy)]\nstruct IndexedGpioModel {\n    bsrr_address: u64,\n    odr_address: u64,\n}\n\n#[derive(Debug, Clone, Copy)]\nstruct EspGpioModel {\n    out_address: u64,\n    out_w1ts_address: u64,\n    out_w1tc_address: u64,\n    enable_address: u64,\n    enable_w1ts_address: u64,\n    enable_w1tc_address: u64,\n}\n\nconst SERIAL_REGISTER_MODELS: &[SerialRegisterModel] = &",
    );
    out.push_str(&render_host_serial_register_model_slice(&serial_models));
    out.push_str(";\n");
    out.push_str("const USB_STREAM_MODELS: &[UsbStreamModel] = &");
    out.push_str(&render_host_usb_stream_model_slice(&usb_models));
    out.push_str(";\n");
    out.push_str("const INDEXED_GPIO_MODELS: &[IndexedGpioModel] = &");
    out.push_str(&render_host_indexed_gpio_model_slice(&gpio_models));
    out.push_str(";\n");
    out.push_str("const ESP_GPIO_MODELS: &[EspGpioModel] = &");
    out.push_str(&render_host_esp_gpio_model_slice(&esp_gpio_models));
    out.push_str(";\n\n");
    out.push_str(
        "thread_local! {\n    static CURRENT_EMULATOR: RefCell<Option<SharedEmulatorState>> = const { RefCell::new(None) };\n}\n\npub struct CurrentEmulatorGuard {\n    previous: Option<SharedEmulatorState>,\n}\n\nimpl Drop for CurrentEmulatorGuard {\n    fn drop(&mut self) {\n        CURRENT_EMULATOR.with(|slot| {\n            *slot.borrow_mut() = self.previous.take();\n        });\n    }\n}\n\nfn lock_guard<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {\n    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())\n}\n\npub fn new_emulator_state() -> SharedEmulatorState {\n    Arc::new(GeneratedHostState::default())\n}\n\npub fn install_current_emulator(state: SharedEmulatorState) {\n    CURRENT_EMULATOR.with(|slot| {\n        *slot.borrow_mut() = Some(state);\n    });\n}\n\npub fn push_current_emulator(state: SharedEmulatorState) -> CurrentEmulatorGuard {\n    let previous = CURRENT_EMULATOR.with(|slot| slot.replace(Some(state)));\n    CurrentEmulatorGuard { previous }\n}\n\npub fn clear_current_emulator() {\n    CURRENT_EMULATOR.with(|slot| {\n        slot.borrow_mut().take();\n    });\n}\n\npub fn clear_current_emulator_if_active(state: &SharedEmulatorState) {\n    CURRENT_EMULATOR.with(|slot| {\n        let mut slot = slot.borrow_mut();\n        if slot\n            .as_ref()\n            .is_some_and(|current| Arc::ptr_eq(current, state))\n        {\n            slot.take();\n        }\n    });\n}\n\npub fn initialize_emulator_state(state: &SharedEmulatorState) {\n    let mut registers = lock_guard(&state.registers);\n    for model in SERIAL_REGISTER_MODELS {\n        registers.insert(model.sr_address, model.txe_mask | model.tc_mask);\n        registers.insert(model.dr_address, 0);\n    }\n    for model in USB_STREAM_MODELS {\n        registers.insert(model.ep1_address, 0);\n        registers.insert(model.ep1_conf_address, model.serial_in_data_free_mask);\n    }\n}\n\nfn current_emulator() -> Result<SharedEmulatorState, Error> {\n    CURRENT_EMULATOR.with(|slot| {\n        slot.borrow()\n            .clone()\n            .ok_or(Error::Unsupported(\"host emulator is not active on this thread\"))\n    })\n}\n\nfn serial_model_for_driver(driver_id: &str) -> Option<&'static SerialRegisterModel> {\n    SERIAL_REGISTER_MODELS.iter().find(|model| model.driver_id == driver_id)\n}\n\nfn serial_model_for_dr_address(address: u64) -> Option<&'static SerialRegisterModel> {\n    SERIAL_REGISTER_MODELS.iter().find(|model| model.dr_address == address)\n}\n\nfn usb_stream_model_for_driver(driver_id: &str) -> Option<&'static UsbStreamModel> {\n    USB_STREAM_MODELS.iter().find(|model| model.driver_id == driver_id)\n}\n\nfn usb_stream_model_for_ep1_address(address: u64) -> Option<&'static UsbStreamModel> {\n    USB_STREAM_MODELS.iter().find(|model| model.ep1_address == address)\n}\n\nfn usb_stream_model_for_ep1_conf_address(address: u64) -> Option<&'static UsbStreamModel> {\n    USB_STREAM_MODELS.iter().find(|model| model.ep1_conf_address == address)\n}\n\nfn indexed_gpio_model_for_bsrr(address: u64) -> Option<&'static IndexedGpioModel> {\n    INDEXED_GPIO_MODELS.iter().find(|model| model.bsrr_address == address)\n}\n\nfn esp_gpio_model_for_w1_register(address: u64) -> Option<&'static EspGpioModel> {\n    ESP_GPIO_MODELS.iter().find(|model| {\n        model.out_w1ts_address == address\n            || model.out_w1tc_address == address\n            || model.enable_w1ts_address == address\n            || model.enable_w1tc_address == address\n    })\n}\n\npub fn read_u32_for(state: &SharedEmulatorState, address: u64) -> Result<u32, Error> {\n    if let Some(model) = serial_model_for_dr_address(address) {\n        let mut serial = lock_guard(&state.serial);\n        let queues = serial.entry(model.driver_id.to_string()).or_default();\n        if let Some(byte) = queues.rx.pop_front() {\n            let mut registers = lock_guard(&state.registers);\n            let entry = registers.entry(model.sr_address).or_insert(model.txe_mask | model.tc_mask);\n            if queues.rx.is_empty() {\n                *entry &= !model.rxne_mask;\n            } else {\n                *entry |= model.rxne_mask;\n            }\n            registers.insert(model.dr_address, u32::from(byte));\n            return Ok(u32::from(byte));\n        }\n    }\n    if let Some(model) = usb_stream_model_for_ep1_address(address) {\n        let mut usb_stream = lock_guard(&state.usb_stream);\n        let queues = usb_stream.entry(model.driver_id.to_string()).or_default();\n        if let Some(byte) = queues.rx.pop_front() {\n            let mut registers = lock_guard(&state.registers);\n            let entry = registers.entry(model.ep1_conf_address).or_insert(model.serial_in_data_free_mask);\n            if queues.rx.is_empty() {\n                *entry &= !model.serial_out_data_avail_mask;\n            } else {\n                *entry |= model.serial_out_data_avail_mask;\n            }\n            registers.insert(model.ep1_address, (u32::from(byte) << model.rdwr_byte_lsb) & model.rdwr_byte_mask);\n            return Ok((u32::from(byte) << model.rdwr_byte_lsb) & model.rdwr_byte_mask);\n        }\n    }\n    Ok(*lock_guard(&state.registers).get(&address).unwrap_or(&0))\n}\n\npub fn write_u32_for(state: &SharedEmulatorState, address: u64, value: u32) -> Result<(), Error> {\n    if let Some(model) = serial_model_for_dr_address(address) {\n        let mut serial = lock_guard(&state.serial);\n        let queues = serial.entry(model.driver_id.to_string()).or_default();\n        queues.tx.push_back((value & 0xFF) as u8);\n        let mut registers = lock_guard(&state.registers);\n        registers.insert(model.dr_address, value & 0xFF);\n        let entry = registers.entry(model.sr_address).or_insert(0);\n        *entry |= model.txe_mask | model.tc_mask;\n        return Ok(());\n    }\n    if let Some(model) = usb_stream_model_for_ep1_address(address) {\n        let shifted = (value & model.rdwr_byte_mask) >> model.rdwr_byte_lsb;\n        let mut usb_stream = lock_guard(&state.usb_stream);\n        let queues = usb_stream.entry(model.driver_id.to_string()).or_default();\n        queues.tx.push_back((shifted & 0xFF) as u8);\n        let mut registers = lock_guard(&state.registers);\n        registers.insert(model.ep1_address, value & model.rdwr_byte_mask);\n        return Ok(());\n    }\n    if let Some(model) = usb_stream_model_for_ep1_conf_address(address) {\n        let mut registers = lock_guard(&state.registers);\n        let mut next = value;\n        if value & model.wr_done_mask != 0 {\n            next &= !model.serial_in_data_free_mask;\n        }\n        registers.insert(address, next);\n        return Ok(());\n    }\n    if let Some(model) = indexed_gpio_model_for_bsrr(address) {\n        let mut registers = lock_guard(&state.registers);\n        let odr = registers.entry(model.odr_address).or_insert(0);\n        let set_bits = value & 0xFFFF;\n        let reset_bits = (value >> 16) & 0xFFFF;\n        *odr |= set_bits;\n        *odr &= !reset_bits;\n        registers.insert(address, value);\n        return Ok(());\n    }\n    if let Some(model) = esp_gpio_model_for_w1_register(address) {\n        let mut registers = lock_guard(&state.registers);\n        let (target_address, updated_value) = if address == model.out_w1ts_address {\n            let out = registers.entry(model.out_address).or_insert(0);\n            *out |= value;\n            (model.out_address, *out)\n        } else if address == model.out_w1tc_address {\n            let out = registers.entry(model.out_address).or_insert(0);\n            *out &= !value;\n            (model.out_address, *out)\n        } else if address == model.enable_w1ts_address {\n            let enable = registers.entry(model.enable_address).or_insert(0);\n            *enable |= value;\n            (model.enable_address, *enable)\n        } else {\n            let enable = registers.entry(model.enable_address).or_insert(0);\n            *enable &= !value;\n            (model.enable_address, *enable)\n        };\n        registers.insert(address, value);\n        registers.insert(target_address, updated_value);\n        return Ok(());\n    }\n    lock_guard(&state.registers).insert(address, value);\n    Ok(())\n}\n\npub fn modify_u8_for(state: &SharedEmulatorState, address: u64, clear_mask: u8, set_mask: u8) -> Result<(), Error> {\n    let current = read_u32_for(state, address)? as u8;\n    write_u32_for(state, address, u32::from((current & !clear_mask) | set_mask))\n}\n\npub fn modify_u16_for(state: &SharedEmulatorState, address: u64, clear_mask: u16, set_mask: u16) -> Result<(), Error> {\n    let current = read_u32_for(state, address)? as u16;\n    write_u32_for(state, address, u32::from((current & !clear_mask) | set_mask))\n}\n\npub fn modify_u32_for(state: &SharedEmulatorState, address: u64, clear_mask: u32, set_mask: u32) -> Result<(), Error> {\n    let current = read_u32_for(state, address)?;\n    write_u32_for(state, address, (current & !clear_mask) | set_mask)\n}\n\npub fn read_u32(address: u64) -> Result<u32, Error> {\n    let state = current_emulator()?;\n    read_u32_for(&state, address)\n}\n\npub fn write_u32(address: u64, value: u32) -> Result<(), Error> {\n    let state = current_emulator()?;\n    write_u32_for(&state, address, value)\n}\n\npub fn modify_u8(address: u64, clear_mask: u8, set_mask: u8) -> Result<(), Error> {\n    let state = current_emulator()?;\n    modify_u8_for(&state, address, clear_mask, set_mask)\n}\n\npub fn modify_u16(address: u64, clear_mask: u16, set_mask: u16) -> Result<(), Error> {\n    let state = current_emulator()?;\n    modify_u16_for(&state, address, clear_mask, set_mask)\n}\n\npub fn modify_u32(address: u64, clear_mask: u32, set_mask: u32) -> Result<(), Error> {\n    let state = current_emulator()?;\n    modify_u32_for(&state, address, clear_mask, set_mask)\n}\n\npub fn push_serial_rx_for(state: &SharedEmulatorState, driver_id: &str, bytes: &[u8]) -> Result<(), Error> {\n    let model = serial_model_for_driver(driver_id)\n        .ok_or(Error::InvalidReference(\"unknown serial driver\"))?;\n    if !model.has_rx {\n        return Err(Error::InvalidReference(\"serial driver has no RX capability\"));\n    }\n    let mut serial = lock_guard(&state.serial);\n    let queues = serial.entry(driver_id.to_string()).or_default();\n    for &byte in bytes {\n        queues.rx.push_back(byte);\n    }\n    drop(serial);\n    let mut registers = lock_guard(&state.registers);\n    let entry = registers.entry(model.sr_address).or_insert(model.txe_mask | model.tc_mask);\n    if !bytes.is_empty() {\n        *entry |= model.rxne_mask;\n    }\n    Ok(())\n}\n\npub fn take_serial_tx_for(state: &SharedEmulatorState, driver_id: &str) -> Vec<u8> {\n    let mut serial = lock_guard(&state.serial);\n    let queues = serial.entry(driver_id.to_string()).or_default();\n    queues.tx.drain(..).collect()\n}\n\npub fn push_usb_rx_for(state: &SharedEmulatorState, driver_id: &str, bytes: &[u8]) -> Result<(), Error> {\n    let model = usb_stream_model_for_driver(driver_id)\n        .ok_or(Error::InvalidReference(\"unknown usb stream driver\"))?;\n    if !model.has_rx {\n        return Err(Error::InvalidReference(\"usb stream driver has no RX capability\"));\n    }\n    let mut usb_stream = lock_guard(&state.usb_stream);\n    let queues = usb_stream.entry(driver_id.to_string()).or_default();\n    for &byte in bytes {\n        queues.rx.push_back(byte);\n    }\n    drop(usb_stream);\n    let mut registers = lock_guard(&state.registers);\n    let entry = registers.entry(model.ep1_conf_address).or_insert(model.serial_in_data_free_mask);\n    if !bytes.is_empty() {\n        *entry |= model.serial_out_data_avail_mask;\n    }\n    Ok(())\n}\n\npub fn take_usb_tx_for(state: &SharedEmulatorState, driver_id: &str) -> Vec<u8> {\n    let Some(model) = usb_stream_model_for_driver(driver_id) else {\n        return Vec::new();\n    };\n    let committed = {\n        let registers = lock_guard(&state.registers);\n        let entry = registers.get(&model.ep1_conf_address).copied().unwrap_or(model.serial_in_data_free_mask);\n        entry & model.wr_done_mask != 0\n    };\n    if !committed {\n        return Vec::new();\n    }\n    let mut usb_stream = lock_guard(&state.usb_stream);\n    let queues = usb_stream.entry(driver_id.to_string()).or_default();\n    let drained = queues.tx.drain(..).collect();\n    drop(usb_stream);\n    let mut registers = lock_guard(&state.registers);\n    let entry = registers.entry(model.ep1_conf_address).or_insert(model.serial_in_data_free_mask);\n    *entry |= model.serial_in_data_free_mask;\n    *entry &= !model.wr_done_mask;\n    drained\n}\n\npub fn mark_dma_route_complete_for(state: &SharedEmulatorState, route_id: &str) {\n    let mut dma = lock_guard(&state.dma_completions);\n    *dma.entry(route_id.to_string()).or_default() += 1;\n}\n\npub fn dma_route_completion_count_for(state: &SharedEmulatorState, route_id: &str) -> usize {\n    *lock_guard(&state.dma_completions).get(route_id).unwrap_or(&0)\n}\n\npub fn set_irq_pending_for(state: &SharedEmulatorState, irq_number: i32, pending: bool) {\n    let mut irqs = lock_guard(&state.pending_irqs);\n    if pending {\n        irqs.insert(irq_number);\n    } else {\n        irqs.remove(&irq_number);\n    }\n}\n\npub fn is_irq_pending_for(state: &SharedEmulatorState, irq_number: i32) -> bool {\n    lock_guard(&state.pending_irqs).contains(&irq_number)\n}\n",
    );
    Ok(out)
}

fn render_host_serial_register_model_slice(items: &[HostSerialRegisterModel]) -> String {
    let rendered = items
        .iter()
        .map(|item| {
            format!(
                "SerialRegisterModel {{ driver_id: {}, sr_address: 0x{:X}u64, dr_address: 0x{:X}u64, has_tx: {}, has_rx: {}, txe_mask: 0x{:08X}u32, tc_mask: 0x{:08X}u32, rxne_mask: 0x{:08X}u32 }}",
                render_rust_string(&item.driver_id),
                item.sr_address,
                item.dr_address,
                item.has_tx,
                item.has_rx,
                item.txe_mask,
                item.tc_mask,
                item.rxne_mask
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(", "))
}

fn render_host_usb_stream_model_slice(items: &[HostUsbStreamModel]) -> String {
    let rendered = items
        .iter()
        .map(|item| {
            format!(
                "UsbStreamModel {{ driver_id: {}, ep1_address: 0x{:X}u64, ep1_conf_address: 0x{:X}u64, has_tx: {}, has_rx: {}, rdwr_byte_mask: 0x{:08X}u32, rdwr_byte_lsb: {}u32, wr_done_mask: 0x{:08X}u32, serial_in_data_free_mask: 0x{:08X}u32, serial_out_data_avail_mask: 0x{:08X}u32 }}",
                render_rust_string(&item.driver_id),
                item.ep1_address,
                item.ep1_conf_address,
                item.has_tx,
                item.has_rx,
                item.rdwr_byte_mask,
                item.rdwr_byte_lsb,
                item.wr_done_mask,
                item.serial_in_data_free_mask,
                item.serial_out_data_avail_mask,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(", "))
}

fn render_host_indexed_gpio_model_slice(items: &[HostIndexedGpioModel]) -> String {
    let rendered = items
        .iter()
        .map(|item| {
            format!(
                "IndexedGpioModel {{ bsrr_address: 0x{:X}u64, odr_address: 0x{:X}u64 }}",
                item.bsrr_address, item.odr_address
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(", "))
}

fn render_host_esp_gpio_model_slice(items: &[HostEspGpioModel]) -> String {
    let rendered = items
        .iter()
        .map(|item| {
            format!(
                "EspGpioModel {{ out_address: 0x{:X}u64, out_w1ts_address: 0x{:X}u64, out_w1tc_address: 0x{:X}u64, enable_address: 0x{:X}u64, enable_w1ts_address: 0x{:X}u64, enable_w1tc_address: 0x{:X}u64 }}",
                item.out_address,
                item.out_w1ts_address,
                item.out_w1tc_address,
                item.enable_address,
                item.enable_w1ts_address,
                item.enable_w1tc_address
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(", "))
}

fn render_embassy_host_root_rs(model: &EmbassyGenerationModel) -> String {
    let mut out = String::from(
        "use crate::metadata;\n\n#[derive(Debug, Clone)]\npub struct HostEmulator {\n    state: metadata::SharedEmulatorState,\n}\n\nimpl Default for HostEmulator {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\nimpl Drop for HostEmulator {\n    fn drop(&mut self) {\n        metadata::clear_current_emulator_if_active(&self.state);\n    }\n}\n\nimpl HostEmulator {\n    pub fn new() -> Self {\n        let state = metadata::new_emulator_state();\n        metadata::initialize_emulator_state(&state);\n        Self { state }\n    }\n\n    pub fn activate(&self) {\n        metadata::install_current_emulator(self.state.clone());\n    }\n\n    pub fn activate_scoped(&self) -> metadata::CurrentEmulatorGuard {\n        metadata::push_current_emulator(self.state.clone())\n    }\n\n    pub fn clear_active(&self) {\n        metadata::clear_current_emulator_if_active(&self.state);\n    }\n\n    pub fn state(&self) -> metadata::SharedEmulatorState {\n        self.state.clone()\n    }\n",
    );
    for driver in &model.drivers {
        let accessor = to_rust_method_name(&driver.name);
        let emulator_accessor = format!("{accessor}_emulator");
        out.push_str(&format!(
            "\n    pub fn {accessor}(&self) -> Result<crate::{module}::{type_name}, metadata::Error> {{\n        self.activate();\n        crate::{module}::{type_name}::new(crate::{module}::{const_prefix}_RESOURCES)\n    }}\n\n    pub fn {emulator_accessor}(&self) -> crate::{module}::{type_name}Emulator {{\n        self.activate();\n        crate::{module}::{type_name}Emulator::new(self.state.clone())\n    }}\n",
            module = driver.module_name,
            type_name = driver.type_name,
            const_prefix = to_rust_const_name(&driver.id),
        ));
    }
    out.push_str("}\n");
    out
}

fn render_embassy_host_driver_module(
    model: &EmbassyGenerationModel,
    module_name: &str,
    drivers: &[&ResolvedDriverInstance],
) -> Result<String> {
    let mut out = format!(
        "//! Generated host-emulated Embassy-style {module_name} module for {}.\n\nuse crate::metadata;\n\n{}\n",
        render_comment_text(&model.device_name),
        render_host_mmio_helpers()
    );
    if drivers
        .iter()
        .any(|driver| driver.driver_kind == "gpio-port")
    {
        out.push_str(render_gpio_module_prelude());
    }
    if module_name == "interrupt" {
        out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Irq {\n");
        for interrupt in &model.interrupts {
            out.push_str(&format!(
                "    {} = {},\n",
                to_rust_type_name(&interrupt.name),
                interrupt.number
            ));
        }
        out.push_str("}\n\n");
    }
    out.push_str(&render_module_provenance_const(module_name));
    for driver in drivers {
        out.push_str(&render_embassy_host_driver_instance(model, driver)?);
    }
    Ok(out)
}

fn render_host_mmio_helpers() -> &'static str {
    "#[allow(dead_code)]\nfn modify_u8(address: u64, clear_mask: u8, set_mask: u8) -> Result<(), metadata::Error> {\n    metadata::modify_u8(address, clear_mask, set_mask)\n}\n\n#[allow(dead_code)]\nfn modify_u16(address: u64, clear_mask: u16, set_mask: u16) -> Result<(), metadata::Error> {\n    metadata::modify_u16(address, clear_mask, set_mask)\n}\n\n#[allow(dead_code)]\nfn modify_u32(address: u64, clear_mask: u32, set_mask: u32) -> Result<(), metadata::Error> {\n    metadata::modify_u32(address, clear_mask, set_mask)\n}\n\n#[allow(dead_code)]\nfn read_u32(address: u64) -> Result<u32, metadata::Error> {\n    metadata::read_u32(address)\n}\n\n#[allow(dead_code)]\nfn write_u32(address: u64, value: u32) -> Result<(), metadata::Error> {\n    metadata::write_u32(address, value)\n}\n"
}

fn render_embassy_host_driver_instance(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let const_prefix = to_rust_const_name(&driver.id);
    let support_items = render_embassy_host_driver_support_items(model, driver)?;
    let emulator_items = render_embassy_host_driver_emulator(model, driver)?;
    let mut out = String::new();
    out.push_str(&format!(
        "// Driver instance: {} ({}) from canonical block {} -> {}\n",
        render_comment_text(&driver.name),
        render_comment_text(&driver.driver_kind),
        render_comment_text(&driver.target.id),
        render_comment_text(&driver.target.block_class)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_CLOCK_BINDINGS: &[metadata::ClockBinding] = &{};\n",
        render_clock_binding_slice(&driver.clock_bindings)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_RESET_BINDINGS: &[metadata::ResetBinding] = &{};\n",
        render_reset_binding_slice(&driver.reset_bindings)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_INTERRUPT_SOURCES: &[metadata::InterruptSource] = &{};\n",
        render_interrupt_source_slice(&driver.interrupt_sources)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &{};\n",
        render_interrupt_route_slice(&driver.interrupt_routes)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_DMA_CHANNELS: &[metadata::DmaChannel] = &{};\n",
        render_dma_channel_slice(&driver.dma_channels)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_DMA_ROUTES: &[metadata::DmaRoute] = &{};\n",
        render_dma_route_slice(&driver.dma_routes)
    ));
    for (index, pin_role) in driver.pin_roles.iter().enumerate() {
        out.push_str(&format!(
            "pub const {const_prefix}_PIN_ROLE_{index}_ROUTES: &[metadata::PinRoute] = &{};\n",
            render_pin_route_slice(&pin_role.routes)
        ));
    }
    out.push_str(&format!(
        "pub const {const_prefix}_PIN_ROLES: &[metadata::PinRole] = &{};\n",
        render_pin_role_slice(const_prefix.as_str(), &driver.pin_roles)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_INIT_OPERATIONS: &[metadata::SemanticOperation] = &{};\n",
        render_semantic_operation_slice(&driver.init_operations)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_STATE_MACHINES: &[metadata::SemanticStateMachine] = &{};\n",
        render_semantic_state_machine_slice(&driver.state_machines)
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_CAPABILITY_TAGS: &[&str] = &{};\n\n",
        render_named_entity_slice(&driver.capability_tags)
    ));
    out.push_str(&format!(
        "#[derive(Debug, Clone, Copy)]\npub struct {type_name}Resources {{\n    pub clocks: &'static [metadata::ClockBinding],\n    pub resets: &'static [metadata::ResetBinding],\n    pub interrupt_sources: &'static [metadata::InterruptSource],\n    pub interrupts: &'static [metadata::InterruptRoute],\n    pub dma_channels: &'static [metadata::DmaChannel],\n    pub dma: &'static [metadata::DmaRoute],\n    pub pins: &'static [metadata::PinRole],\n    pub init_operations: &'static [metadata::SemanticOperation],\n    pub state_machines: &'static [metadata::SemanticStateMachine],\n    pub lowering_pattern: Option<&'static str>,\n    pub time_driver_source: Option<&'static str>,\n    pub capability_tags: &'static [&'static str],\n}}\n\npub const {const_prefix}_RESOURCES: {type_name}Resources = {type_name}Resources {{\n    clocks: {const_prefix}_CLOCK_BINDINGS,\n    resets: {const_prefix}_RESET_BINDINGS,\n    interrupt_sources: {const_prefix}_INTERRUPT_SOURCES,\n    interrupts: {const_prefix}_INTERRUPT_ROUTES,\n    dma_channels: {const_prefix}_DMA_CHANNELS,\n    dma: {const_prefix}_DMA_ROUTES,\n    pins: {const_prefix}_PIN_ROLES,\n    init_operations: {const_prefix}_INIT_OPERATIONS,\n    state_machines: {const_prefix}_STATE_MACHINES,\n    lowering_pattern: {lowering_pattern},\n    time_driver_source: {time_driver_source},\n    capability_tags: {const_prefix}_CAPABILITY_TAGS,\n}};\n\n#[derive(Debug, Clone, Copy)]\npub struct {type_name} {{\n    resources: {type_name}Resources,\n}}\n\nimpl {type_name} {{\n    pub fn new(resources: {type_name}Resources) -> Result<Self, metadata::Error> {{\n        Ok(Self {{ resources }})\n    }}\n\n    pub fn resources(&self) -> {type_name}Resources {{\n        self.resources\n    }}\n{methods}\n}}\n\n{support_items}\n{emulator_items}",
        type_name = driver.type_name,
        const_prefix = const_prefix,
        lowering_pattern = render_optional_rust_string(driver.lowering_pattern.as_deref()),
        time_driver_source = render_optional_rust_string(driver.time_driver_source.as_deref()),
        methods = render_driver_methods(model, driver)?,
        support_items = support_items,
        emulator_items = emulator_items
    ));
    Ok(out)
}

fn render_embassy_host_driver_support_items(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let mut out = String::new();
    if driver_uses_pfic_runtime_support(driver) {
        out.push_str(render_pfic_interrupt_support_items());
    }
    if driver.driver_kind == "gpio-port" {
        out.push_str(&render_gpio_support_items(model, driver)?);
    }
    if has_time_driver_tag(&driver.capability_tags) {
        out.push_str(&render_host_time_driver_support_items(driver)?);
    }
    Ok(out)
}

fn render_host_time_driver_support_items(driver: &ResolvedDriverInstance) -> Result<String> {
    let driver_prefix = to_rust_const_name(&driver.id).to_lowercase();
    let init_fn = format!("initialize_{driver_prefix}_time_driver");
    let advance_fn = format!("advance_{driver_prefix}_time_driver");
    match time_driver_source(driver) {
        Some(EMBASSY_TIME_DRIVER_SOURCE_SYSTICK) => Ok(format!(
            "\nuse std::sync::Mutex;\nuse embassy_time_driver::Driver as EmbassyTimeDriver;\nuse embassy_time_queue_utils::Queue as EmbassyTimeQueue;\n\nconst SYST_CSR_ADDRESS: u64 = 0xE000_E010;\nconst SYST_RVR_ADDRESS: u64 = 0xE000_E014;\nconst SYST_CVR_ADDRESS: u64 = 0xE000_E018;\nconst SYST_CSR_ENABLE: u32 = 1 << 0;\nconst SYST_CSR_TICKINT: u32 = 1 << 1;\nconst SYST_CSR_CLKSOURCE: u32 = 1 << 2;\nconst SYST_RELOAD_VALUE: u32 = 15;\n\nstruct GeneratedSystickTimeDriverState {{\n    initialized: bool,\n    ticks: u64,\n    queue: EmbassyTimeQueue,\n}}\n\nstruct GeneratedSystickTimeDriver {{\n    state: Mutex<GeneratedSystickTimeDriverState>,\n}}\n\nimpl GeneratedSystickTimeDriver {{\n    const fn new() -> Self {{\n        Self {{\n            state: Mutex::new(GeneratedSystickTimeDriverState {{\n                initialized: false,\n                ticks: 0,\n                queue: EmbassyTimeQueue::new(),\n            }}),\n        }}\n    }}\n\n    fn init(&self) -> Result<(), metadata::Error> {{\n        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());\n        if state.initialized {{\n            return Ok(());\n        }}\n        state.ticks = 0;\n        metadata::write_u32(SYST_CSR_ADDRESS, 0)?;\n        metadata::write_u32(SYST_RVR_ADDRESS, SYST_RELOAD_VALUE)?;\n        metadata::write_u32(SYST_CVR_ADDRESS, 0)?;\n        metadata::write_u32(\n            SYST_CSR_ADDRESS,\n            SYST_CSR_ENABLE | SYST_CSR_TICKINT | SYST_CSR_CLKSOURCE,\n        )?;\n        state.initialized = true;\n        Ok(())\n    }}\n\n    fn advance(&self, ticks: u64) {{\n        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());\n        if !state.initialized {{\n            return;\n        }}\n        state.ticks = state.ticks.wrapping_add(ticks);\n        let now = state.ticks;\n        let _ = state.queue.next_expiration(now);\n    }}\n}}\n\nimpl EmbassyTimeDriver for GeneratedSystickTimeDriver {{\n    fn now(&self) -> u64 {{\n        self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).ticks\n    }}\n\n    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {{\n        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());\n        let now = state.ticks;\n        let _ = state.queue.schedule_wake(at, waker);\n        let _ = state.queue.next_expiration(now);\n    }}\n}}\n\nembassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedSystickTimeDriver = GeneratedSystickTimeDriver::new());\n\nfn {init_fn}() -> Result<(), metadata::Error> {{\n    GENERATED_TIME_DRIVER.init()\n}}\n\nfn {advance_fn}(ticks: u64) {{\n    GENERATED_TIME_DRIVER.advance(ticks)\n}}\n"
        )),
        Some(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER) => Ok(format!(
            "\nuse std::sync::Mutex;\nuse embassy_time_driver::Driver as EmbassyTimeDriver;\nuse embassy_time_queue_utils::Queue as EmbassyTimeQueue;\n\nstruct GeneratedHardwareTimerTimeDriverState {{\n    initialized: bool,\n    ticks: u64,\n    queue: EmbassyTimeQueue,\n}}\n\nstruct GeneratedHardwareTimerTimeDriver {{\n    state: Mutex<GeneratedHardwareTimerTimeDriverState>,\n}}\n\nimpl GeneratedHardwareTimerTimeDriver {{\n    const fn new() -> Self {{\n        Self {{\n            state: Mutex::new(GeneratedHardwareTimerTimeDriverState {{\n                initialized: false,\n                ticks: 0,\n                queue: EmbassyTimeQueue::new(),\n            }}),\n        }}\n    }}\n\n    fn init(&self) -> Result<(), metadata::Error> {{\n        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());\n        if state.initialized {{\n            return Ok(());\n        }}\n        state.ticks = 0;\n        state.initialized = true;\n        Ok(())\n    }}\n\n    fn advance(&self, ticks: u64) {{\n        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());\n        if !state.initialized {{\n            return;\n        }}\n        state.ticks = state.ticks.wrapping_add(ticks);\n        let now = state.ticks;\n        let _ = state.queue.next_expiration(now);\n    }}\n\n    fn on_interrupt(&self) {{\n        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());\n        let now = state.ticks;\n        let _ = state.queue.next_expiration(now);\n    }}\n\n    fn delay_ticks(&self, ticks: u64) -> Result<(), metadata::Error> {{\n        self.advance(ticks);\n        Ok(())\n    }}\n}}\n\nimpl EmbassyTimeDriver for GeneratedHardwareTimerTimeDriver {{\n    fn now(&self) -> u64 {{\n        self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).ticks\n    }}\n\n    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {{\n        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());\n        let now = state.ticks;\n        let _ = state.queue.schedule_wake(at, waker);\n        let _ = state.queue.next_expiration(now);\n    }}\n}}\n\nembassy_time_driver::time_driver_impl!(static GENERATED_TIME_DRIVER: GeneratedHardwareTimerTimeDriver = GeneratedHardwareTimerTimeDriver::new());\n\nfn {init_fn}() -> Result<(), metadata::Error> {{\n    GENERATED_TIME_DRIVER.init()\n}}\n\nfn {advance_fn}(ticks: u64) {{\n    GENERATED_TIME_DRIVER.advance(ticks)\n}}\n\nfn generated_{driver_prefix}_time_driver_now() -> u64 {{\n    GENERATED_TIME_DRIVER.now()\n}}\n\nfn generated_{driver_prefix}_time_driver_delay_ticks(ticks: u64) -> Result<(), metadata::Error> {{\n    GENERATED_TIME_DRIVER.delay_ticks(ticks)\n}}\n\npub fn generated_{driver_prefix}_time_driver_interrupt() {{\n    GENERATED_TIME_DRIVER.on_interrupt();\n}}\n"
        )),
        Some(other) => bail!(
            "driver {} uses unsupported timeDriverSource {} for generated host time-driver support",
            driver.id,
            other
        ),
        None => bail!(
            "driver {} claims capability tag {} but omits required timeDriverSource",
            driver.id,
            EMBASSY_TIME_DRIVER_TAG
        ),
    }
}

fn render_embassy_host_driver_emulator(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let mut out = format!(
        "#[derive(Debug, Clone)]\npub struct {type_name}Emulator {{\n    state: metadata::SharedEmulatorState,\n}}\n\nimpl {type_name}Emulator {{\n    pub(crate) fn new(state: metadata::SharedEmulatorState) -> Self {{\n        Self {{ state }}\n    }}\n\n    pub fn read_register(&self, address: u64) -> Result<u32, metadata::Error> {{\n        metadata::read_u32_for(&self.state, address)\n    }}\n\n    pub fn write_register(&self, address: u64, value: u32) -> Result<(), metadata::Error> {{\n        metadata::write_u32_for(&self.state, address, value)\n    }}\n\n    pub fn modify_register(&self, address: u64, clear_mask: u32, set_mask: u32) -> Result<(), metadata::Error> {{\n        metadata::modify_u32_for(&self.state, address, clear_mask, set_mask)\n    }}\n",
        type_name = driver.type_name
    );
    if driver.driver_kind == "gpio-port" {
        out.push_str(&render_host_gpio_emulator_methods(model, driver)?);
    }
    if matches!(driver.driver_kind.as_str(), "uart" | "usart") {
        let has_tx = driver.pin_roles.iter().any(|role| role.signal == "TX");
        let has_rx = driver.pin_roles.iter().any(|role| role.signal == "RX");
        if has_rx {
            out.push_str(&format!(
                "    pub fn push_received_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {{\n        metadata::push_serial_rx_for(&self.state, {}, bytes)\n    }}\n\n",
                render_rust_string(&driver.id),
            ));
        }
        if has_tx {
            out.push_str(&format!(
                "    pub fn take_transmitted_bytes(&self) -> Vec<u8> {{\n        metadata::take_serial_tx_for(&self.state, {})\n    }}\n\n",
                render_rust_string(&driver.id),
            ));
        }
    }
    if driver.driver_kind == "usb-device" {
        out.push_str(&format!(
            "    pub fn push_received_bytes(&self, bytes: &[u8]) -> Result<(), metadata::Error> {{\n        metadata::push_usb_rx_for(&self.state, {}, bytes)\n    }}\n\n",
            render_rust_string(&driver.id),
        ));
        out.push_str(&format!(
            "    pub fn take_transmitted_bytes(&self) -> Vec<u8> {{\n        metadata::take_usb_tx_for(&self.state, {})\n    }}\n\n",
            render_rust_string(&driver.id),
        ));
    }
    if driver.driver_kind == "interrupt" {
        out.push_str(
            "    pub fn trigger(&self, irq: crate::interrupt::Irq) {\n        metadata::set_irq_pending_for(&self.state, irq as i32, true)\n    }\n\n    pub fn clear(&self, irq: crate::interrupt::Irq) {\n        metadata::set_irq_pending_for(&self.state, irq as i32, false)\n    }\n\n    pub fn is_pending(&self, irq: crate::interrupt::Irq) -> bool {\n        metadata::is_irq_pending_for(&self.state, irq as i32)\n    }\n\n",
        );
    }
    if driver.driver_kind == "dma" {
        out.push_str(
            "    pub fn mark_route_complete(&self, route_id: &str) {\n        metadata::mark_dma_route_complete_for(&self.state, route_id)\n    }\n\n    pub fn completion_count(&self, route_id: &str) -> usize {\n        metadata::dma_route_completion_count_for(&self.state, route_id)\n    }\n\n",
        );
    }
    if has_time_driver_tag(&driver.capability_tags) {
        out.push_str(&format!(
            "    pub fn advance_ticks(&self, ticks: u64) {{\n        advance_{}_time_driver(ticks)\n    }}\n\n",
            to_rust_const_name(&driver.id).to_lowercase()
        ));
    }
    out.push_str("}\n");
    Ok(out)
}

fn render_host_gpio_emulator_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    let lowering = resolve_gpio_port_lowering(model, driver)?;
    let mut out = String::new();
    match lowering {
        ResolvedGpioPortLowering::IndexedFields { idr, odr, pins, .. } => {
            out.push_str(
                "    pub fn set_input_level(&self, pin_name: &str, high: bool) -> Result<(), metadata::Error> {\n        match pin_name {\n",
            );
            for pin in &pins {
                out.push_str(&format!(
                    "            {} => metadata::modify_u32_for(&self.state, 0x{:X}u64, 0x{:08X}u32, if high {{ 0x{:08X}u32 }} else {{ 0x00000000u32 }})?,\n",
                    render_rust_string(&pin.pin_name),
                    idr.absolute_address,
                    pin.idr_mask,
                    pin.idr_mask,
                ));
            }
            out.push_str(
                "            _ => return Err(metadata::Error::InvalidReference(\"unknown GPIO pin\")),\n        }\n        Ok(())\n    }\n\n    pub fn output_level(&self, pin_name: &str) -> Result<bool, metadata::Error> {\n        match pin_name {\n",
            );
            for pin in &pins {
                out.push_str(&format!(
                    "            {} => Ok((metadata::read_u32_for(&self.state, 0x{:X}u64)? & 0x{:08X}u32) != 0),\n",
                    render_rust_string(&pin.pin_name),
                    odr.absolute_address,
                    pin.odr_mask,
                ));
            }
            out.push_str(
                "            _ => Err(metadata::Error::InvalidReference(\"unknown GPIO pin\")),\n        }\n    }\n\n",
            );
        }
        ResolvedGpioPortLowering::WchCfgRegisters { idr, odr, pins } => {
            out.push_str(
                "    pub fn set_input_level(&self, pin_name: &str, high: bool) -> Result<(), metadata::Error> {\n        match pin_name {\n",
            );
            for pin in &pins {
                out.push_str(&format!(
                    "            {} => metadata::modify_u32_for(&self.state, 0x{:X}u64, 0x{:08X}u32, if high {{ 0x{:08X}u32 }} else {{ 0x00000000u32 }})?,\n",
                    render_rust_string(&pin.pin_name),
                    idr.absolute_address,
                    pin.idr_mask,
                    pin.idr_mask,
                ));
            }
            out.push_str(
                "            _ => return Err(metadata::Error::InvalidReference(\"unknown GPIO pin\")),\n        }\n        Ok(())\n    }\n\n    pub fn output_level(&self, pin_name: &str) -> Result<bool, metadata::Error> {\n        match pin_name {\n",
            );
            for pin in &pins {
                out.push_str(&format!(
                    "            {} => Ok((metadata::read_u32_for(&self.state, 0x{:X}u64)? & 0x{:08X}u32) != 0),\n",
                    render_rust_string(&pin.pin_name),
                    odr.absolute_address,
                    pin.odr_mask,
                ));
            }
            out.push_str(
                "            _ => Err(metadata::Error::InvalidReference(\"unknown GPIO pin\")),\n        }\n    }\n\n",
            );
        }
        ResolvedGpioPortLowering::BitmaskRegisters { pins, .. } => {
            out.push_str(
                "    pub fn set_input_level(&self, pin_name: &str, high: bool) -> Result<(), metadata::Error> {\n        match pin_name {\n",
            );
            for pin in &pins {
                out.push_str(&format!(
                    "            {} => metadata::write_u32_for(&self.state, 0x{:X}u64, if high {{ 0x{:08X}u32 }} else {{ 0x00000000u32 }})?,\n",
                    render_rust_string(&pin.pin_name),
                    pin.data_alias_address,
                    pin.bit_mask,
                ));
            }
            out.push_str(
                "            _ => return Err(metadata::Error::InvalidReference(\"unknown GPIO pin\")),\n        }\n        Ok(())\n    }\n\n    pub fn output_level(&self, pin_name: &str) -> Result<bool, metadata::Error> {\n        match pin_name {\n",
            );
            for pin in &pins {
                out.push_str(&format!(
                    "            {} => Ok(metadata::read_u32_for(&self.state, 0x{:X}u64)? != 0),\n",
                    render_rust_string(&pin.pin_name),
                    pin.data_alias_address,
                ));
            }
            out.push_str(
                "            _ => Err(metadata::Error::InvalidReference(\"unknown GPIO pin\")),\n        }\n    }\n\n",
            );
        }
        ResolvedGpioPortLowering::EspRegisters {
            input,
            out: out_register,
            pins,
            ..
        } => {
            out.push_str(
                "    pub fn set_input_level(&self, pin_name: &str, high: bool) -> Result<(), metadata::Error> {\n        match pin_name {\n",
            );
            for pin in &pins {
                out.push_str(&format!(
                    "            {} => metadata::modify_u32_for(&self.state, 0x{:X}u64, 0x{:08X}u32, if high {{ 0x{:08X}u32 }} else {{ 0x00000000u32 }})?,\n",
                    render_rust_string(&pin.pin_name),
                    input.absolute_address,
                    pin.bit_mask,
                    pin.bit_mask,
                ));
            }
            out.push_str(
                "            _ => return Err(metadata::Error::InvalidReference(\"unknown GPIO pin\")),\n        }\n        Ok(())\n    }\n\n    pub fn output_level(&self, pin_name: &str) -> Result<bool, metadata::Error> {\n        match pin_name {\n",
            );
            for pin in &pins {
                out.push_str(&format!(
                    "            {} => Ok((metadata::read_u32_for(&self.state, 0x{:X}u64)? & 0x{:08X}u32) != 0),\n",
                    render_rust_string(&pin.pin_name),
                    out_register.absolute_address,
                    pin.bit_mask,
                ));
            }
            out.push_str(
                "            _ => Err(metadata::Error::InvalidReference(\"unknown GPIO pin\")),\n        }\n    }\n\n",
            );
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, tempdir};

    #[test]
    fn validates_the_reference_hair_document() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let validator = build_validator(&repo_root).expect("validator");
        let document = load_json_file(
            &repo_root
                .join("evidence")
                .join("wch")
                .join("ch32v203c8t6")
                .join("hair.json"),
        )
        .expect("document");
        let result = validator.validate(&document);
        assert!(result.is_ok(), "reference HAIR should validate");
    }

    #[test]
    fn generate_svd_emits_usbfs_field_metadata_for_reference_hair() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let document = load_validated_hair_document(
            &repo_root
                .join("evidence")
                .join("wch")
                .join("ch32v203c8t6")
                .join("hair.json"),
            &repo_root,
        )
        .expect("reference document should load");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains("USB device physical port control register."));
        assert!(svd.contains("<name>DMA_EN</name>"));
        assert!(svd.contains("<name>MASK_T_RES</name>"));
        assert!(svd.contains("<name>OTG_EN</name>"));
        assert!(svd.contains("USB host endpoint PID."));
    }

    #[test]
    fn diff_reports_structural_changes() {
        let left = serde_json::json!({"metadata": {"version": "0.1.0"}, "items": [1, 2]});
        let right =
            serde_json::json!({"metadata": {"version": "0.2.0"}, "items": [1, 3], "new": true});
        let differences = diff_values(&left, &right, "");
        let summaries = differences
            .iter()
            .map(Difference::summary)
            .collect::<Vec<_>>();
        assert!(
            summaries
                .iter()
                .any(|line| line.contains("/metadata/version"))
        );
        assert!(summaries.iter().any(|line| line.contains("/items/1")));
        assert!(summaries.iter().any(|line| line.contains("/new")));
    }

    #[test]
    fn generate_svd_emits_expected_core_elements() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "architectures": [
                    {
                        "id": "arch.test",
                        "addressUnitBits": 8,
                        "dataBusWidthBits": 32
                    }
                ],
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "architectureRef": "arch.test",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "interrupts": [
                        {
                            "id": "int.timer",
                            "name": "TIMER",
                            "number": 5
                        }
                    ],
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "interruptRefs": ["int.timer"],
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl",
                                    "name": "CTRL",
                                    "offsetBytes": 0,
                                    "widthBits": 32,
                                    "access": "read-write",
                                    "fields": [
                                        {
                                            "id": "field.enable",
                                            "name": "ENABLE",
                                            "bitRange": {
                                                "lsb": 0,
                                                "msb": 0
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains("<device"));
        assert!(svd.contains("<name>TEST123</name>"));
        assert!(svd.contains("<cpu><name>QingKe V4B</name><revision>V4B</revision><endian>little</endian><mpuPresent>false</mpuPresent><fpuPresent>false</fpuPresent><nvicPrioBits>4</nvicPrioBits><vendorSystickConfig>true</vendorSystickConfig></cpu>"));
        assert!(svd.contains("<baseAddress>0x40000000</baseAddress>"));
        assert!(svd.contains("<name>CTRL</name>"));
        assert!(svd.contains("<name>ENABLE</name>"));
    }

    #[test]
    fn generate_svd_resolves_peripheral_derived_from_ref_to_name() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "architectures": [
                    {
                        "id": "arch.test",
                        "addressUnitBits": 8,
                        "dataBusWidthBits": 32
                    }
                ],
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "architectureRef": "arch.test",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.base",
                            "name": "BASE",
                            "type": "Base",
                            "baseAddress": 1073741824,
                            "addressBlocks": [
                                {
                                    "offsetBytes": 0,
                                    "sizeBytes": 4,
                                    "usage": "registers"
                                }
                            ]
                        },
                        {
                            "id": "periph.derived",
                            "name": "DERIVED",
                            "type": "Base",
                            "derivedFromRef": "periph.base",
                            "baseAddress": 1073745920,
                            "addressBlocks": [
                                {
                                    "offsetBytes": 0,
                                    "sizeBytes": 4,
                                    "usage": "registers"
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains(r#"<peripheral derivedFrom="BASE"><name>DERIVED</name>"#));
        assert!(!svd.contains(r#"derivedFrom="periph.base""#));
    }

    #[test]
    fn generate_svd_emits_alternate_register_views() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl.output",
                                    "name": "CTRL_Output",
                                    "displayName": "CTRL",
                                    "offsetBytes": 0,
                                    "widthBits": 32,
                                    "access": "read-write",
                                    "fields": [
                                        {
                                            "id": "field.output",
                                            "name": "MODE",
                                            "bitRange": {
                                                "lsb": 0,
                                                "msb": 1
                                            }
                                        }
                                    ]
                                },
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl.input",
                                    "name": "CTRL_Input",
                                    "displayName": "CTRL",
                                    "alternateOfRef": "reg.ctrl.output",
                                    "offsetBytes": 0,
                                    "widthBits": 32,
                                    "access": "read-write",
                                    "fields": [
                                        {
                                            "id": "field.input",
                                            "name": "FILTER",
                                            "bitRange": {
                                                "lsb": 0,
                                                "msb": 3
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains("<name>CTRL_Output</name><displayName>CTRL</displayName>"));
        assert!(svd.contains("<name>CTRL_Input</name><displayName>CTRL</displayName>"));
        assert!(svd.contains("<alternateRegister>CTRL_Output</alternateRegister>"));
    }

    #[test]
    fn generate_svd_uses_array_name_for_alternate_register_views() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl.output",
                                    "name": "CTRL_Output",
                                    "displayName": "CTRL",
                                    "offsetBytes": 0,
                                    "widthBits": 32,
                                    "array": {
                                        "axes": [
                                            {
                                                "name": "channel",
                                                "count": 2,
                                                "strideBytes": 4
                                            }
                                        ]
                                    }
                                },
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl.input",
                                    "name": "CTRL_Input",
                                    "displayName": "CTRL",
                                    "alternateOfRef": "reg.ctrl.output",
                                    "offsetBytes": 0,
                                    "widthBits": 32,
                                    "array": {
                                        "axes": [
                                            {
                                                "name": "channel",
                                                "count": 2,
                                                "strideBytes": 4
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains("<name>CTRL_Output%s</name>"));
        assert!(svd.contains("<alternateRegister>CTRL_Output%s</alternateRegister>"));
    }

    #[test]
    fn generate_svd_rejects_unknown_alternate_register_reference() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl.input",
                                    "name": "CTRL_Input",
                                    "alternateOfRef": "reg.ctrl.output",
                                    "offsetBytes": 0,
                                    "widthBits": 32
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let error = generate_svd(&document).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("references unknown alternateOfRef")
        );
    }

    #[test]
    fn generate_svd_rejects_self_referential_alternate_register() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl",
                                    "name": "CTRL",
                                    "alternateOfRef": "reg.ctrl",
                                    "offsetBytes": 0,
                                    "widthBits": 32
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let error = generate_svd(&document).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("register reg.ctrl references itself via alternateOfRef")
        );
    }

    #[test]
    fn generate_svd_rejects_cross_scope_alternate_register_reference() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl.output",
                                    "name": "CTRL_Output",
                                    "offsetBytes": 0,
                                    "widthBits": 32
                                },
                                {
                                    "kind": "cluster",
                                    "id": "cluster.inner",
                                    "name": "INNER",
                                    "offsetBytes": 16,
                                    "members": [
                                        {
                                            "kind": "register",
                                            "id": "reg.inner.input",
                                            "name": "INNER_Input",
                                            "alternateOfRef": "reg.ctrl.output",
                                            "offsetBytes": 0,
                                            "widthBits": 32
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let error = generate_svd(&document).expect_err("generation should fail");
        assert!(error.to_string().contains(
            "register reg.inner.input references unknown alternateOfRef reg.ctrl.output"
        ));
    }

    #[test]
    fn generate_svd_rejects_incompatible_alternate_register_contract() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl.output",
                                    "name": "CTRL_Output",
                                    "offsetBytes": 0,
                                    "widthBits": 32,
                                    "access": "read-write",
                                    "resetValue": 0
                                },
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl.input",
                                    "name": "CTRL_Input",
                                    "alternateOfRef": "reg.ctrl.output",
                                    "offsetBytes": 0,
                                    "widthBits": 16,
                                    "access": "read-write",
                                    "resetValue": 0
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let error = generate_svd(&document).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("references alternateOfRef reg.ctrl.output with mismatched widthBits")
        );
    }

    #[test]
    fn generate_svd_rejects_duplicate_bit_ownership_in_one_register_view() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl",
                                    "name": "CTRL",
                                    "offsetBytes": 0,
                                    "widthBits": 8,
                                    "fields": [
                                        {
                                            "id": "field.ctrl.enable",
                                            "name": "ENABLE",
                                            "bitRange": {
                                                "lsb": 1,
                                                "msb": 1
                                            }
                                        },
                                        {
                                            "id": "field.ctrl.detect",
                                            "name": "DETECT",
                                            "bitRange": {
                                                "lsb": 1,
                                                "msb": 1
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let error = generate_svd(&document).expect_err("generation should fail");
        assert!(error.to_string().contains(
            "register reg.ctrl has duplicate bit ownership in fields field.ctrl.enable and field.ctrl.detect"
        ));
    }

    #[test]
    fn generate_svd_allows_aggregate_and_alias_style_field_overlap_patterns() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl",
                                    "name": "CTRL",
                                    "offsetBytes": 0,
                                    "widthBits": 8,
                                    "fields": [
                                        {
                                            "id": "field.ctrl.mode",
                                            "name": "MODE",
                                            "bitRange": {
                                                "lsb": 0,
                                                "msb": 1
                                            }
                                        },
                                        {
                                            "id": "field.ctrl.mode_1",
                                            "name": "MODE_1",
                                            "bitRange": {
                                                "lsb": 0,
                                                "msb": 1
                                            }
                                        },
                                        {
                                            "id": "field.ctrl.mode0",
                                            "name": "MODE0",
                                            "bitRange": {
                                                "lsb": 0,
                                                "msb": 0
                                            }
                                        },
                                        {
                                            "id": "field.ctrl.mode1",
                                            "name": "MODE1",
                                            "bitRange": {
                                                "lsb": 1,
                                                "msb": 1
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("generation should succeed");
        assert!(svd.contains("<name>MODE</name>"));
        assert!(svd.contains("<name>MODE_1</name>"));
        assert!(svd.contains("<name>MODE0</name>"));
        assert!(svd.contains("<name>MODE1</name>"));
    }

    #[test]
    fn generate_svd_rejects_unrelated_numbered_duplicate_fields() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": [
                        {
                            "id": "periph.timer",
                            "name": "TIMER",
                            "type": "TIM",
                            "baseAddress": 1073741824u64,
                            "registers": [
                                {
                                    "kind": "register",
                                    "id": "reg.ctrl",
                                    "name": "CTRL",
                                    "offsetBytes": 0,
                                    "widthBits": 8,
                                    "fields": [
                                        {
                                            "id": "field.ctrl.flag1",
                                            "name": "FLAG1",
                                            "bitRange": {
                                                "lsb": 0,
                                                "msb": 0
                                            }
                                        },
                                        {
                                            "id": "field.ctrl.flag8",
                                            "name": "FLAG8",
                                            "bitRange": {
                                                "lsb": 0,
                                                "msb": 0
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let error = generate_svd(&document).expect_err("generation should fail");
        assert!(error.to_string().contains(
            "register reg.ctrl has duplicate bit ownership in fields field.ctrl.flag1 and field.ctrl.flag8"
        ));
    }

    #[test]
    fn generate_svd_infers_same_name_interrupt_linkage() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "architectures": [
                    {
                        "id": "arch.test",
                        "addressUnitBits": 8,
                        "dataBusWidthBits": 32
                    }
                ],
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "architectureRef": "arch.test",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "interrupts": [
                        {
                            "id": "int.wwdg",
                            "name": "WWDG",
                            "number": 16
                        }
                    ],
                    "peripherals": [
                        {
                            "id": "periph.wwdg",
                            "name": "WWDG",
                            "type": "WWDG",
                            "baseAddress": 1073753088u64,
                            "registers": []
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains("<interrupt><name>WWDG</name><value>16</value></interrupt>"));
    }

    #[test]
    fn generate_svd_rejects_unattributed_interrupts() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "architectures": [
                    {
                        "id": "arch.test",
                        "addressUnitBits": 8,
                        "dataBusWidthBits": 32
                    }
                ],
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "architectureRef": "arch.test",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "interrupts": [
                        {
                            "id": "int.timer",
                            "name": "TIMER",
                            "number": 5
                        }
                    ],
                    "peripherals": [
                        {
                            "id": "periph.uart",
                            "name": "UART0",
                            "type": "UART",
                            "baseAddress": 1073741824u64,
                            "registers": []
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let error = generate_svd(&document).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("is not linked by interruptRefs and could not be attributed")
        );
    }

    #[test]
    fn generate_svd_ignores_unlinked_systick_exception() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "architectures": [
                    {
                        "id": "arch.test",
                        "addressUnitBits": 8,
                        "dataBusWidthBits": 32
                    }
                ],
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "architectureRef": "arch.test",
                    "cpu": {
                        "name": "Cortex-M4",
                        "revision": "r0p1",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": true,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": false
                        }
                    },
                    "interrupts": [
                        {
                            "id": "irq.systick",
                            "name": "SysTick",
                            "number": -1
                        },
                        {
                            "id": "int.wwdg",
                            "name": "WWDG",
                            "number": 16
                        }
                    ],
                    "peripherals": [
                        {
                            "id": "periph.wwdg",
                            "name": "WWDG",
                            "type": "WWDG",
                            "baseAddress": 1073753088u64,
                            "registers": []
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains("<interrupt><name>WWDG</name><value>16</value></interrupt>"));
        assert!(!svd.contains("<interrupt><name>SysTick</name><value>-1</value></interrupt>"));
    }

    #[test]
    fn generate_svd_ignores_linked_systick_exception() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "architectures": [
                    {
                        "id": "arch.test",
                        "addressUnitBits": 8,
                        "dataBusWidthBits": 32
                    }
                ],
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "architectureRef": "arch.test",
                    "cpu": {
                        "name": "Cortex-M4",
                        "revision": "r0p1",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": true,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": false
                        }
                    },
                    "interrupts": [
                        {
                            "id": "irq.systick",
                            "name": "SysTick",
                            "number": -1
                        },
                        {
                            "id": "int.wwdg",
                            "name": "WWDG",
                            "number": 16
                        }
                    ],
                    "peripherals": [
                        {
                            "id": "periph.core",
                            "name": "SysTick",
                            "type": "Core",
                            "baseAddress": 3758153744u64,
                            "interruptRefs": ["irq.systick"],
                            "registers": []
                        },
                        {
                            "id": "periph.wwdg",
                            "name": "WWDG",
                            "type": "WWDG",
                            "baseAddress": 1073753088u64,
                            "interruptRefs": ["int.wwdg"],
                            "registers": []
                        }
                    ]
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains("<interrupt><name>WWDG</name><value>16</value></interrupt>"));
        assert!(!svd.contains("<interrupt><name>SysTick</name><value>-1</value></interrupt>"));
    }

    #[test]
    fn loads_json_from_git_selector_syntax_errors() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let error = load_diff_operand(&repo_root, "git:HEAD").expect_err("selector should fail");
        assert!(error.to_string().contains("git selector"));
    }

    #[test]
    fn loads_relative_diff_operand_from_repo_root() {
        let temp = tempdir().expect("temp dir");
        let repo_root = temp.path().join("repo");
        fs::create_dir_all(repo_root.join("docs")).expect("docs dir");
        fs::write(
            repo_root.join("docs").join("left.json"),
            r#"{"metadata":{"version":"0.1.0"}}"#,
        )
        .expect("json file");

        let value = load_diff_operand(&repo_root, "docs/left.json").expect("load diff operand");
        assert_eq!(value["metadata"]["version"], "0.1.0");
    }

    #[test]
    fn write_outcome_preserves_exact_output_without_extra_newline() {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let outcome = CommandOutcome {
            exit_code: 1,
            stdout: "line one\nline two".to_string(),
            stderr: "warn".to_string(),
        };

        let exit_code = write_outcome(&mut stdout, &mut stderr, &outcome).expect("write output");

        assert_eq!(stdout, b"line one\nline two");
        assert_eq!(stderr, b"warn");
        assert_eq!(exit_code, process::ExitCode::from(1));
    }

    #[test]
    fn normalize_exit_code_clamps_out_of_range_values() {
        assert_eq!(normalize_exit_code(0), process::ExitCode::from(0));
        assert_eq!(normalize_exit_code(255), process::ExitCode::from(255));
        assert_eq!(normalize_exit_code(256), process::ExitCode::from(255));
        assert_eq!(normalize_exit_code(-1), process::ExitCode::from(1));
    }

    #[test]
    fn finds_schema_root_without_git_metadata() {
        let temp = tempdir().expect("temp dir");
        let nested = temp.path().join("snapshot").join("subdir");
        fs::create_dir_all(nested.join("schema")).expect("schema dir");
        fs::write(nested.join("schema").join("hair.json"), "{}").expect("schema file");

        let found = find_schema_root(&nested).expect("schema root");
        assert_eq!(found, nested);
    }

    #[test]
    fn caches_documents_by_fragment_stripped_uri() {
        let temp = tempdir().expect("temp dir");
        let schema_root = temp.path().join("schema");
        fs::create_dir_all(&schema_root).expect("schema dir");
        fs::write(
            schema_root.join("common.json"),
            r#"{"$id":"https://hair.dev/schema/common.json","type":"object"}"#,
        )
        .expect("schema file");

        let bundle = HairSchemaBundle::new(schema_root);
        bundle
            .load_document("https://hair.dev/schema/common.json#/$defs/one")
            .expect("first load");
        bundle
            .load_document("https://hair.dev/schema/common.json#/$defs/two")
            .expect("second load");

        let cache = bundle
            .cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        assert_eq!(cache.len(), 1);
        assert!(cache.contains_key("https://hair.dev/schema/common.json"));
    }

    #[test]
    fn generate_svd_uses_single_architecture_when_reference_is_omitted() {
        let document: HairDocument = serde_json::from_value(serde_json::json!({
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.device",
                "title": "Test Device",
                "version": "0.1.0"
            },
            "structure": {
                "architectures": [
                    {
                        "id": "arch.test",
                        "addressUnitBits": 16,
                        "dataBusWidthBits": 64
                    }
                ],
                "device": {
                    "name": "TEST123",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "peripherals": []
                }
            }
        }))
        .expect("fixture should parse");

        let svd = generate_svd(&document).expect("svd generation");
        assert!(svd.contains("<addressUnitBits>16</addressUnitBits>"));
        assert!(svd.contains("<width>64</width>"));
    }

    #[test]
    fn load_validated_hair_document_rejects_schema_invalid_subset() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut file = NamedTempFile::new().expect("temp file");
        write!(
            file,
            r#"{{
                "metadata": {{
                    "title": "Test Device",
                    "version": "0.1.0"
                }},
                "structure": {{
                    "device": {{
                        "name": "TEST123",
                        "vendor": "TestVendor",
                        "cpu": {{
                            "name": "QingKe V4B",
                            "revision": "V4B",
                            "endianness": "little",
                            "interruptPriorityBits": 4,
                            "featureFlags": {{
                                "mpuPresent": false,
                                "fpuPresent": false,
                                "vendorSystemTimerConfig": true
                            }}
                        }},
                        "peripherals": []
                    }}
                }}
            }}"#
        )
        .expect("write temp file");

        let parsed: HairDocument =
            serde_json::from_value(load_json_file(file.path()).expect("json instance"))
                .expect("subset still deserializes");
        assert_eq!(parsed.metadata.title, "Test Device");

        let error =
            load_validated_hair_document(file.path(), &repo_root).expect_err("schema should fail");
        assert!(
            error
                .to_string()
                .contains("does not conform to the HAIR schema")
        );
    }

    #[test]
    fn json_u64_accepts_string_literals() {
        assert_eq!(
            json_u64(&Value::String("0x10".to_string()), "test").unwrap(),
            16
        );
        assert_eq!(
            json_u64(&Value::String("42".to_string()), "test").unwrap(),
            42
        );
        assert_eq!(
            json_u64(&Value::String("0b1010".to_string()), "test").unwrap(),
            10
        );
    }

    #[test]
    fn validate_reports_invalid_json() {
        let mut file = NamedTempFile::new().expect("temp file");
        writeln!(file, "{{").expect("write temp file");
        let error = load_json_file(file.path()).expect_err("invalid json should fail");
        assert!(error.to_string().contains("parsing"));
    }

    #[test]
    fn generate_svd_accepts_canonical_term_normalization_metadata() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut document = load_json_file(
            &repo_root
                .join("evidence")
                .join("wch")
                .join("ch32v203c8t6")
                .join("hair.json"),
        )
        .expect("document");

        let normalization = document
            .as_object_mut()
            .expect("reference document should be an object")
            .entry("normalization".to_string())
            .or_insert_with(|| serde_json::json!({}));
        let normalization = normalization
            .as_object_mut()
            .expect("normalization should be an object");
        let extend_unique = |items: &mut Vec<Value>, additions: [Value; 3]| {
            for addition in additions {
                let addition_id = addition
                    .get("id")
                    .and_then(Value::as_str)
                    .expect("normalization entry should have an id");
                if items
                    .iter()
                    .all(|existing| existing.get("id").and_then(Value::as_str) != Some(addition_id))
                {
                    items.push(addition);
                }
            }
        };
        extend_unique(
            normalization
                .entry("canonicalTerms".to_string())
                .or_insert_with(|| serde_json::json!([]))
                .as_array_mut()
                .expect("canonicalTerms should be an array"),
            [
                serde_json::json!({
                    "id": "term.peripheral.uart",
                    "name": "UART",
                    "scope": "peripheral"
                }),
                serde_json::json!({
                    "id": "term.register.baud-rate",
                    "name": "Baud-rate register",
                    "scope": "register"
                }),
                serde_json::json!({
                    "id": "term.field.enable",
                    "name": "Enable",
                    "scope": "field"
                }),
            ],
        );
        extend_unique(
            normalization
                .entry("mappings".to_string())
                .or_insert_with(|| serde_json::json!([]))
                .as_array_mut()
                .expect("mappings should be an array"),
            [
                serde_json::json!({
                    "id": "norm.uart4",
                    "name": "UART4 normalization",
                    "targetRef": "periph.uart4",
                    "canonicalTermRefs": ["term.peripheral.uart"]
                }),
                serde_json::json!({
                    "id": "norm.uart4-brr",
                    "name": "UART4 BRR normalization",
                    "targetRef": "reg.uart4.brr",
                    "canonicalTermRefs": ["term.register.baud-rate"]
                }),
                serde_json::json!({
                    "id": "norm.uart4-ue",
                    "name": "UART4 UE normalization",
                    "targetRef": "field.usart_ctlr1.ue",
                    "canonicalTermRefs": ["term.field.enable"],
                    "vendorNames": ["UE"]
                }),
            ],
        );

        let mut file = NamedTempFile::new().expect("temp file");
        serde_json::to_writer(&mut file, &document).expect("write temp file");

        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("document with canonical terminology should validate");
        let svd = generate_svd(&validated)
            .expect("normalization metadata should not break svd generation");
        assert!(svd.contains("<device"));
    }

    #[test]
    fn generate_embassy_emits_compilable_crate_for_supported_subset() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let document = load_validated_hair_document(fixture.path(), &repo_root)
            .expect("embassy fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&document, output_dir.path()).expect("embassy generation");

        assert!(output_dir.path().join("Cargo.toml").exists());
        assert!(output_dir.path().join("src").join("lib.rs").exists());
        assert!(output_dir.path().join("src").join("uart.rs").exists());
        assert!(output_dir.path().join("src").join("interrupt.rs").exists());
        let metadata_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("metadata.rs"))
                .expect("metadata.rs");
        let rcc_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("rcc.rs")).expect("rcc.rs");
        let timer_rs = std::fs::read_to_string(output_dir.path().join("src").join("timer.rs"))
            .expect("timer.rs");
        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        assert!(metadata_rs.contains("GENERATED_PROVENANCE_SOURCE_IDS"));
        assert!(metadata_rs.contains("GENERATED_PROVENANCE_EVIDENCE_IDS"));
        assert!(metadata_rs.contains("document_version"));
        assert!(metadata_rs.contains("pub struct SemanticOperation"));
        assert!(metadata_rs.contains("pub struct SemanticStateMachine"));
        assert!(rcc_rs.contains("binding_kind: \"gated\""));
        assert!(rcc_rs.contains("controller_ref: Some(\"block.rcc\")"));
        assert!(uart_rs.contains("reset_domain_ref: Some(\"rst.apb2\")"));
        assert!(rcc_rs.contains("control_refs: &[\"reg.rcc.apb2pcenr\"]"));
        assert!(uart_rs.contains("default_after_reset: Some(true)"));
        assert!(rcc_rs.contains("steps: &[metadata::SemanticOperationStep"));
        assert!(timer_rs.contains("transitions: &[metadata::SemanticTransition"));
        assert!(uart_rs.contains("MODULE_PROVENANCE"));

        let cargo_output = Command::new("cargo")
            .arg("check")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo check");
        assert!(
            cargo_output.status.success(),
            "generated crate should compile:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_host_emits_compilable_crate_with_emulator_handles() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let document = load_validated_hair_document(fixture.path(), &repo_root)
            .expect("embassy fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_host_crate(&document, output_dir.path()).expect("embassy-host generation");

        assert!(output_dir.path().join("Cargo.toml").exists());
        assert!(output_dir.path().join("src").join("host.rs").exists());
        let host_rs = std::fs::read_to_string(output_dir.path().join("src").join("host.rs"))
            .expect("host.rs");
        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        let gpio_rs = std::fs::read_to_string(output_dir.path().join("src").join("gpio.rs"))
            .expect("gpio.rs");
        let spi_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("spi.rs")).expect("spi.rs");
        let i2c_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("i2c.rs")).expect("i2c.rs");
        let metadata_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("metadata.rs"))
                .expect("metadata.rs");
        assert!(host_rs.contains("pub struct HostEmulator"));
        assert!(uart_rs.contains("pub struct Usart1Emulator"));
        assert!(gpio_rs.contains("pub struct GpioAEmulator"));
        assert!(!spi_rs.contains("push_received_bytes"));
        assert!(!spi_rs.contains("take_transmitted_bytes"));
        assert!(!i2c_rs.contains("push_received_bytes"));
        assert!(!i2c_rs.contains("take_transmitted_bytes"));
        assert!(metadata_rs.contains("thread_local!"));
        assert!(metadata_rs.contains("pub struct CurrentEmulatorGuard"));
        assert!(metadata_rs.contains("pub fn clear_current_emulator()"));
        assert!(metadata_rs.contains("host emulator is not active on this thread"));
        assert!(metadata_rs.contains("unknown serial driver"));
        assert!(metadata_rs.contains("serial driver has no RX capability"));
        assert!(host_rs.contains("pub fn activate_scoped(&self)"));
        assert!(host_rs.contains("pub fn clear_active(&self)"));
        assert!(host_rs.contains("impl Drop for HostEmulator"));

        let tests_dir = output_dir.path().join("tests");
        fs::create_dir_all(&tests_dir).expect("create tests dir");
        fs::write(
            tests_dir.join("host_smoke.rs"),
            r#"use fixture_embassy_hal_host::host::HostEmulator;
use fixture_embassy_hal_host::gpio::Level;
use fixture_embassy_hal_host::interrupt::Irq;

#[test]
fn host_emulator_wires_hal_and_companion_state() {
    assert!(fixture_embassy_hal_host::metadata::read_u32(0).is_err());

    {
        let host = HostEmulator::new();
        assert!(fixture_embassy_hal_host::metadata::push_serial_rx_for(
            &host.state(),
            "serial.unknown",
            b"X",
        )
        .is_err());

        let gpio = host.gpio_a().expect("gpio");
        let gpio_emulator = host.gpio_a_emulator();
        let pin = gpio.pa0();
        gpio_emulator.set_input_level("PA0", true).expect("set input high");
        assert!(pin.is_high().expect("pin high"));
        let output = pin.into_output(Level::Low).expect("output");
        output.set_high().expect("set high");
        assert!(gpio_emulator.output_level("PA0").expect("read output"));

        let usart = host.usart1().expect("usart");
        let usart_emulator = host.usart1_emulator();
        usart_emulator.push_received_bytes(b"A").expect("inject rx");
        assert_eq!(usart.read_byte().expect("read byte"), b'A');
        usart.write_byte(b'B').expect("write byte");
        assert_eq!(usart_emulator.take_transmitted_bytes(), b"B");

        let dma = host.dma1_emulator();
        dma.mark_route_complete("dmaroute.usart1_tx");
        assert_eq!(dma.completion_count("dmaroute.usart1_tx"), 1);

        let interrupt = host.pfic_emulator();
        interrupt.trigger(Irq::USART1);
        assert!(interrupt.is_pending(Irq::USART1));
        interrupt.clear(Irq::USART1);
        assert!(!interrupt.is_pending(Irq::USART1));

        let host2 = HostEmulator::new();
        let gpio2 = host2.gpio_a_emulator();
        gpio_emulator.write_register(0x1234, 1).expect("write state 1");
        gpio2.write_register(0x1234, 2).expect("write state 2");

        fixture_embassy_hal_host::metadata::clear_current_emulator();
        let _guard1 = host.activate_scoped();
        assert_eq!(fixture_embassy_hal_host::metadata::read_u32(0x1234).expect("read host1"), 1);
        {
            let _guard2 = host2.activate_scoped();
            assert_eq!(fixture_embassy_hal_host::metadata::read_u32(0x1234).expect("read host2"), 2);
        }
        assert_eq!(fixture_embassy_hal_host::metadata::read_u32(0x1234).expect("restore host1"), 1);
        host.clear_active();
        assert!(fixture_embassy_hal_host::metadata::read_u32(0x1234).is_err());
    }

    assert!(fixture_embassy_hal_host::metadata::read_u32(0).is_err());
}
"#,
        )
        .expect("write smoke test");

        let cargo_output = Command::new("cargo")
            .arg("test")
            .arg("--quiet")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo test");
        assert!(
            cargo_output.status.success(),
            "generated host crate should compile and pass smoke tests:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_host_tracks_esp_gpio_output_state() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let document = load_validated_hair_document(
            &repo_root
                .join("evidence")
                .join("espressif")
                .join("esp32-c3fn4")
                .join("hair.json"),
            &repo_root,
        )
        .expect("esp32-c3 hair should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_host_crate(&document, output_dir.path()).expect("embassy-host generation");

        let metadata_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("metadata.rs"))
                .expect("metadata.rs");
        assert!(metadata_rs.contains("ESP_GPIO_MODELS"));
        assert!(metadata_rs.contains("esp_gpio_model_for_w1_register"));

        let tests_dir = output_dir.path().join("tests");
        fs::create_dir_all(&tests_dir).expect("create tests dir");
        fs::write(
            tests_dir.join("esp_gpio_host.rs"),
            r#"use esp32c3fn4_generated_host::gpio::Level;
use esp32c3fn4_generated_host::host::HostEmulator;

#[test]
fn host_emulator_tracks_esp_gpio_output_level() {
    let host = HostEmulator::new();
    let gpio = host.gpio_port().expect("gpio");
    let gpio_emulator = host.gpio_port_emulator();
    let pin = gpio.gpio8().into_output(Level::Low).expect("output");

    assert!(!gpio_emulator.output_level("GPIO8").expect("initial output"));

    pin.set_high().expect("set high");
    assert!(gpio_emulator.output_level("GPIO8").expect("high output"));

    pin.set_low().expect("set low");
    assert!(!gpio_emulator.output_level("GPIO8").expect("low output"));
}
"#,
        )
        .expect("write esp gpio host test");

        let cargo_output = Command::new("cargo")
            .arg("test")
            .arg("--quiet")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo test");
        assert!(
            cargo_output.status.success(),
            "generated host crate should compile and pass ESP GPIO smoke test:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_emits_esp_usb_module() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let document = load_validated_hair_document(
            &repo_root
                .join("evidence")
                .join("espressif")
                .join("esp32-c3fn4")
                .join("hair.json"),
            &repo_root,
        )
        .expect("esp32-c3 hair should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&document, output_dir.path()).expect("embassy generation");

        let lib_rs = std::fs::read_to_string(output_dir.path().join("src").join("lib.rs"))
            .expect("generated lib.rs");
        let usb_rs = std::fs::read_to_string(output_dir.path().join("src").join("usb.rs"))
            .expect("generated usb.rs");
        assert!(lib_rs.contains("pub mod usb;"));
        assert!(usb_rs.contains("pub fn apply_preserve_serial_jtag_link(&self)"));
        assert!(usb_rs.contains("lowering_pattern: Some(\"serial-jtag-preserve-link\")"));
        assert!(!usb_rs.contains("pub fn enable_usb_pad(&self)"));
        assert!(!usb_rs.contains("pub fn assert_reset(&self)"));
        assert!(usb_rs.contains("pub fn write_serial_packet(&self, bytes: &[u8])"));
        assert!(usb_rs.contains("pub fn is_usb_bus_reset_pending(&self)"));
    }

    #[test]
    fn generate_embassy_usb_byte_helpers_shift_nonzero_rdwr_fields() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut document = load_json_file(
            &repo_root
                .join("evidence")
                .join("espressif")
                .join("esp32-c3fn4")
                .join("hair.json"),
        )
        .expect("esp32-c3 hair json");

        let usb = fixture_peripheral_mut(&mut document, "per.usb_device");
        let ep1 = usb
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("usb registers")
            .iter_mut()
            .find(|register| register["name"] == "EP1")
            .and_then(Value::as_object_mut)
            .expect("EP1 register");
        let rdwr_byte = ep1
            .get_mut("fields")
            .and_then(Value::as_array_mut)
            .expect("EP1 fields")
            .iter_mut()
            .find(|field| field["name"] == "RDWR_BYTE")
            .and_then(Value::as_object_mut)
            .expect("RDWR_BYTE field");
        let bit_range = rdwr_byte
            .get_mut("bitRange")
            .and_then(Value::as_object_mut)
            .expect("RDWR_BYTE bitRange");
        bit_range.insert("lsb".to_string(), serde_json::json!(8));
        bit_range.insert("msb".to_string(), serde_json::json!(15));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mutated esp32-c3 hair should validate structurally");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let usb_rs = std::fs::read_to_string(output_dir.path().join("src").join("usb.rs"))
            .expect("generated usb.rs");
        assert!(usb_rs.contains("write_u32(0x60043000u64, u32::from(byte) << 8)?;"));
        assert!(usb_rs.contains("Ok(((read_u32(0x60043000u64)? & 0x0000FF00u32) >> 8) as u8)"));
    }

    #[test]
    fn generate_embassy_rejects_usb_preserve_link_pattern_without_required_operation() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut document = load_json_file(
            &repo_root
                .join("evidence")
                .join("espressif")
                .join("esp32-c3fn4")
                .join("hair.json"),
        )
        .expect("esp32-c3 hair json");

        let profiles = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object");
        let drivers = profiles
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let usb_driver = drivers
            .iter_mut()
            .find(|driver| driver["id"] == "drv.usb_device")
            .and_then(Value::as_object_mut)
            .expect("usb driver");
        usb_driver.insert(
            "initOperationRefs".to_string(),
            serde_json::json!(["op.usb_device.complete_serial_in_packet"]),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mutated esp32-c3 hair should still validate structurally");
        let output_dir = tempdir().expect("tempdir");
        let error = generate_embassy_crate(&validated, output_dir.path())
            .expect_err("generation should fail without preserve-link operation");
        assert!(error.to_string().contains(USB_PRESERVE_LINK_OPERATION_ID));
    }

    #[test]
    fn generate_embassy_host_supports_esp_usb_serial_jtag_emulation() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let document = load_validated_hair_document(
            &repo_root
                .join("evidence")
                .join("espressif")
                .join("esp32-c3fn4")
                .join("hair.json"),
            &repo_root,
        )
        .expect("esp32-c3 hair should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_host_crate(&document, output_dir.path()).expect("embassy-host generation");

        let usb_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("usb.rs")).expect("usb.rs");
        assert!(usb_rs.contains("pub struct UsbSerialJtagEmulator"));
        assert!(usb_rs.contains("pub fn push_received_bytes(&self, bytes: &[u8])"));
        assert!(usb_rs.contains("pub fn take_transmitted_bytes(&self) -> Vec<u8>"));

        let tests_dir = output_dir.path().join("tests");
        fs::create_dir_all(&tests_dir).expect("create tests dir");
        fs::write(
            tests_dir.join("esp_usb_host.rs"),
            r#"use esp32c3fn4_generated_host::host::HostEmulator;

#[test]
fn host_emulator_tracks_esp_usb_serial_jtag_streams() {
    let host = HostEmulator::new();
    let usb = host.usb_serial_jtag().expect("usb");
    let usb_emulator = host.usb_serial_jtag_emulator();

    assert!(usb.serial_in_ready().expect("serial in ready"));
    usb_emulator.push_received_bytes(b"A").expect("inject rx");
    assert!(usb.serial_out_data_available().expect("serial out available"));
    assert_eq!(usb.read_serial_byte().expect("read byte"), b'A');

    usb.write_serial_bytes(b"BC").expect("write bytes");
    assert_eq!(usb_emulator.take_transmitted_bytes(), b"");
    assert!(usb.serial_in_ready().expect("serial in still ready"));
    usb.complete_write_packet().expect("commit packet");
    assert!(!usb.serial_in_ready().expect("serial in blocked"));
    assert_eq!(usb_emulator.take_transmitted_bytes(), b"BC");
    assert!(usb.serial_in_ready().expect("serial in ready after drain"));
}
"#,
        )
        .expect("write esp usb host test");

        let cargo_output = Command::new("cargo")
            .arg("test")
            .arg("--quiet")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo test");
        assert!(
            cargo_output.status.success(),
            "generated host crate should compile and pass ESP USB host smoke test:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_host_gates_serial_emulator_methods_by_pin_role() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let uart = drivers[2].as_object_mut().expect("uart driver");
        let pin_roles = uart
            .get_mut("pinRoles")
            .and_then(Value::as_array_mut)
            .expect("pinRoles");
        pin_roles.retain(|role| {
            role.as_object()
                .and_then(|role| role.get("signal"))
                .and_then(Value::as_str)
                != Some("RX")
        });

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("tx-only uart fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_host_crate(&validated, output_dir.path())
            .expect("embassy-host generation");

        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        assert!(!uart_rs.contains("push_received_bytes"));
        assert!(uart_rs.contains("take_transmitted_bytes"));
    }

    #[test]
    fn generate_embassy_host_rejects_reserved_host_module_name() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[0]
            .as_object_mut()
            .expect("rcc driver")
            .insert("modulePath".to_string(), Value::String("host".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("reserved host module fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error = generate_embassy_host_crate(&validated, temp.path())
            .expect_err("host generation should fail");
        assert!(
            error
                .to_string()
                .contains("reserved generated host module name host")
        );
    }

    #[test]
    fn generate_embassy_host_rejects_colliding_accessor_names() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[0]
            .as_object_mut()
            .expect("rcc driver")
            .insert("name".to_string(), Value::String("FooBar".to_string()));
        drivers[1]
            .as_object_mut()
            .expect("gpio driver")
            .insert("name".to_string(), Value::String("FooBAR".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("colliding accessor fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error = generate_embassy_host_crate(&validated, temp.path())
            .expect_err("host generation should fail");
        assert!(error.to_string().contains("HostEmulator method foo_bar"));
    }

    #[test]
    fn generate_embassy_host_rejects_keyword_accessor_names() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[0]
            .as_object_mut()
            .expect("rcc driver")
            .insert("name".to_string(), Value::String("Type".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("keyword accessor fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error = generate_embassy_host_crate(&validated, temp.path())
            .expect_err("host generation should fail");
        assert!(
            error
                .to_string()
                .contains("reserved Rust keyword type as HostEmulator method")
        );
    }

    #[test]
    fn generate_embassy_host_supports_interrupt_scoped_time_driver() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let interrupts = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("interrupts")
            .and_then(Value::as_array_mut)
            .expect("interrupts");
        interrupts.push(serde_json::json!({
            "id": "irq.systick",
            "name": "SysTick",
            "number": -1,
            "controllerRef": "ic.nvic"
        }));
        let mcu = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object");
        let interrupt_topology = mcu
            .get_mut("interruptTopology")
            .and_then(Value::as_object_mut)
            .expect("interrupt topology");
        interrupt_topology
            .get_mut("sources")
            .and_then(Value::as_array_mut)
            .expect("interrupt sources")
            .push(serde_json::json!({
                "id": "isrc.systick",
                "name": "SysTick source",
                "sourceRef": "block.cpu0",
                "producerRef": "block.cpu0",
                "kind": "timer"
            }));
        interrupt_topology
            .get_mut("routes")
            .and_then(Value::as_array_mut)
            .expect("interrupt routes")
            .push(serde_json::json!({
                "id": "iroute.systick",
                "name": "SysTick route",
                "sourceRef": "isrc.systick",
                "interruptRef": "irq.systick",
                "controllerRef": "block.pfic",
                "cpuTargetRef": "block.cpu0",
                "routeType": "hardwired"
            }));
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let nvic = drivers
            .iter()
            .find(|driver| {
                driver
                    .as_object()
                    .and_then(|driver| driver.get("driverKind"))
                    .and_then(Value::as_str)
                    == Some("interrupt")
            })
            .expect("interrupt driver")
            .clone();
        let mut time_driver = nvic.as_object().expect("nvic object").clone();
        time_driver.insert("id".to_string(), Value::String("drv.time".to_string()));
        time_driver.insert("name".to_string(), Value::String("Time".to_string()));
        time_driver.insert("modulePath".to_string(), Value::String("time".to_string()));
        time_driver.insert(
            "interruptRouteRefs".to_string(),
            serde_json::json!(["iroute.systick"]),
        );
        time_driver.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );
        time_driver.insert(
            "timeDriverSource".to_string(),
            Value::String(EMBASSY_TIME_DRIVER_SOURCE_SYSTICK.to_string()),
        );
        drivers.push(Value::Object(time_driver));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("interrupt time-driver fixture should validate");
        let output_dir = tempdir().expect("tempdir");
        generate_embassy_host_crate(&validated, output_dir.path())
            .expect("embassy-host generation");

        let cargo_toml = std::fs::read_to_string(output_dir.path().join("Cargo.toml"))
            .expect("generated Cargo.toml");
        let lib_rs = std::fs::read_to_string(output_dir.path().join("src").join("lib.rs"))
            .expect("generated lib.rs");
        let host_rs = std::fs::read_to_string(output_dir.path().join("src").join("host.rs"))
            .expect("generated host.rs");
        let time_rs = std::fs::read_to_string(output_dir.path().join("src").join("time.rs"))
            .expect("generated time module");
        assert!(cargo_toml.contains("embassy-time-driver"));
        assert!(cargo_toml.contains("embassy-time-queue-utils"));
        assert!(lib_rs.contains("pub mod time;"));
        assert!(host_rs.contains("pub fn time(&self)"));
        assert!(host_rs.contains("pub fn time_emulator(&self)"));
        assert!(time_rs.contains("init_time_driver"));
        assert!(time_rs.contains("advance_drv_time_time_driver"));
        assert!(time_rs.contains("state.ticks = state.ticks.wrapping_add(ticks);"));
        assert!(!time_rs.contains("for _ in 0..ticks"));
        assert!(time_rs.contains("time_driver_impl!"));

        let cargo_output = Command::new("cargo")
            .arg("check")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo check");
        assert!(
            cargo_output.status.success(),
            "generated time-driver host crate should compile:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_host_supports_hardware_timer_scoped_time_driver() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let tim1 = peripherals
            .iter_mut()
            .find(|peripheral| {
                peripheral
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("periph.tim1")
            })
            .and_then(Value::as_object_mut)
            .expect("tim1 peripheral");
        let registers = tim1
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim1 registers");
        registers.extend([
            serde_json::json!({ "id": "reg.tim1.conf", "name": "CONF", "kind": "register", "offsetBytes": 4, "widthBits": 32, "fields": [
                { "id": "field.tim1.conf.clk_en", "name": "CLK_EN", "bitRange": { "lsb": 0, "msb": 0 } },
                { "id": "field.tim1.conf.target0_work_en", "name": "TARGET0_WORK_EN", "bitRange": { "lsb": 1, "msb": 1 } },
                { "id": "field.tim1.conf.timer_unit0_work_en", "name": "TIMER_UNIT0_WORK_EN", "bitRange": { "lsb": 2, "msb": 2 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_op", "name": "UNIT0_OP", "kind": "register", "offsetBytes": 8, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_op.timer_unit0_value_valid", "name": "TIMER_UNIT0_VALUE_VALID", "bitRange": { "lsb": 0, "msb": 0 } },
                { "id": "field.tim1.unit0_op.timer_unit0_update", "name": "TIMER_UNIT0_UPDATE", "bitRange": { "lsb": 1, "msb": 1 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_value_hi", "name": "UNIT0_VALUE_HI", "kind": "register", "offsetBytes": 12, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_value_hi.timer_unit0_value_hi", "name": "TIMER_UNIT0_VALUE_HI", "bitRange": { "lsb": 0, "msb": 19 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_value_lo", "name": "UNIT0_VALUE_LO", "kind": "register", "offsetBytes": 16, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_value_lo.timer_unit0_value_lo", "name": "TIMER_UNIT0_VALUE_LO", "bitRange": { "lsb": 0, "msb": 31 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_hi", "name": "TARGET0_HI", "kind": "register", "offsetBytes": 20, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_hi.timer_target0_hi", "name": "TIMER_TARGET0_HI", "bitRange": { "lsb": 0, "msb": 19 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_lo", "name": "TARGET0_LO", "kind": "register", "offsetBytes": 24, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_lo.timer_target0_lo", "name": "TIMER_TARGET0_LO", "bitRange": { "lsb": 0, "msb": 31 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_conf", "name": "TARGET0_CONF", "kind": "register", "offsetBytes": 28, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_conf.target0_period", "name": "TARGET0_PERIOD", "bitRange": { "lsb": 0, "msb": 7 } },
                { "id": "field.tim1.target0_conf.target0_period_mode", "name": "TARGET0_PERIOD_MODE", "bitRange": { "lsb": 8, "msb": 8 } },
                { "id": "field.tim1.target0_conf.target0_timer_unit_sel", "name": "TARGET0_TIMER_UNIT_SEL", "bitRange": { "lsb": 9, "msb": 9 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.comp0_load", "name": "COMP0_LOAD", "kind": "register", "offsetBytes": 32, "widthBits": 32, "fields": [
                { "id": "field.tim1.comp0_load.timer_comp0_load", "name": "TIMER_COMP0_LOAD", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.int_ena", "name": "INT_ENA", "kind": "register", "offsetBytes": 36, "widthBits": 32, "fields": [
                { "id": "field.tim1.int_ena.target0_int_ena", "name": "TARGET0_INT_ENA", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.int_clr", "name": "INT_CLR", "kind": "register", "offsetBytes": 40, "widthBits": 32, "fields": [
                { "id": "field.tim1.int_clr.target0_int_clr", "name": "TARGET0_INT_CLR", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
        ]);

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        operations.push(serde_json::json!({
            "id": "op.tim1.clear_target0",
            "name": "Clear TIM1 target0 interrupt",
            "kind": "mode-transition",
            "targetRefs": ["periph.tim1"],
            "steps": [{
                "index": 0,
                "action": "write",
                "targetRef": "reg.tim1.int_clr",
                "expression": { "language": "plain", "text": "Set TARGET0_INT_CLR = 1" }
            }]
        }));

        let interrupt_sources = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("interruptTopology")
            .and_then(Value::as_object_mut)
            .expect("interrupt topology")
            .get_mut("sources")
            .and_then(Value::as_array_mut)
            .expect("interrupt sources");
        interrupt_sources
            .iter_mut()
            .find(|source| {
                source
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("isrc.tim1")
            })
            .and_then(Value::as_object_mut)
            .expect("tim1 interrupt source")
            .insert(
                "clearOperationRefs".to_string(),
                serde_json::json!(["op.tim1.clear_target0"]),
            );

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let tim1 = drivers[5].as_object_mut().expect("timer driver");
        tim1.insert("modulePath".to_string(), Value::String("time".to_string()));
        tim1.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );
        tim1.insert(
            "timeDriverSource".to_string(),
            Value::String(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER.to_string()),
        );
        tim1.insert(
            "timeDriverTickHz".to_string(),
            serde_json::json!(16_000_000u64),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("hardware timer time-driver fixture should validate");
        let output_dir = tempdir().expect("tempdir");
        generate_embassy_host_crate(&validated, output_dir.path())
            .expect("embassy-host generation");

        let time_rs = std::fs::read_to_string(output_dir.path().join("src").join("time.rs"))
            .expect("generated time module");
        let cargo_toml = std::fs::read_to_string(output_dir.path().join("Cargo.toml"))
            .expect("generated Cargo.toml");
        assert!(time_rs.contains("generated_drv_tim1_time_driver_interrupt"));
        assert!(time_rs.contains("generated_drv_tim1_time_driver_delay_ticks"));
        assert!(time_rs.contains("GeneratedHardwareTimerTimeDriver"));
        assert!(cargo_toml.contains("tick-hz-16_000_000"));

        let cargo_output = Command::new("cargo")
            .arg("check")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo check");
        assert!(
            cargo_output.status.success(),
            "generated hardware timer host crate should compile:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn embassy_generation_model_keeps_interrupt_module_when_interrupt_driver_moves() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let document = load_validated_hair_document(fixture.path(), &repo_root)
            .expect("embassy fixture should validate");
        let mut model = EmbassyGenerationModel::from_document(&document).expect("generation model");
        model.interrupts.clear();
        let interrupt_driver = model
            .drivers
            .iter_mut()
            .find(|driver| driver.driver_kind == "interrupt")
            .expect("interrupt driver");
        interrupt_driver.module_name = "irqctl".to_string();

        let module_names = model.module_names();
        let driver_groups = model.driver_groups();

        assert!(module_names.iter().any(|name| name == "interrupt"));
        assert!(driver_groups.iter().any(|(name, _)| name == "interrupt"));
        assert!(driver_groups.iter().any(|(name, _)| name == "irqctl"));
    }

    #[test]
    fn generate_embassy_preserves_numeric_value_literals_without_losing_u64_precision() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let tim_operation = operations[2].as_object_mut().expect("tim operation");
        let tim_steps = tim_operation
            .get_mut("steps")
            .and_then(Value::as_array_mut)
            .expect("tim steps");
        tim_steps[0]
            .as_object_mut()
            .expect("tim step")
            .insert("value".to_string(), serde_json::json!(1.5));

        let state_machines = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("stateMachines")
            .and_then(Value::as_array_mut)
            .expect("state machines");
        let tim_state_machine = state_machines[0]
            .as_object_mut()
            .expect("tim state machine");
        let states = tim_state_machine
            .get_mut("states")
            .and_then(Value::as_array_mut)
            .expect("states");
        let enabled_state = states[1].as_object_mut().expect("enabled state");
        let invariants = enabled_state
            .get_mut("invariants")
            .and_then(Value::as_array_mut)
            .expect("invariants");
        invariants[0].as_object_mut().expect("invariant").insert(
            "expectedValue".to_string(),
            serde_json::json!(18446744073709551615u64),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("numeric literal fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let timer_rs = std::fs::read_to_string(output_dir.path().join("src").join("timer.rs"))
            .expect("timer.rs");
        assert!(timer_rs.contains("Some(metadata::ValueLiteral::Number(1.5f64))"));
        assert!(
            timer_rs.contains("Some(metadata::ValueLiteral::Unsigned(18446744073709551615u64))")
        );

        let cargo_output = Command::new("cargo")
            .arg("check")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo check");
        assert!(
            cargo_output.status.success(),
            "generated crate should compile:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_derives_methods_from_lowering_inputs_instead_of_stubs() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let document = load_validated_hair_document(fixture.path(), &repo_root)
            .expect("embassy fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&document, output_dir.path()).expect("embassy generation");

        let rcc_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("rcc.rs")).expect("rcc.rs");
        let gpio_rs = std::fs::read_to_string(output_dir.path().join("src").join("gpio.rs"))
            .expect("gpio.rs");
        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        let timer_rs = std::fs::read_to_string(output_dir.path().join("src").join("timer.rs"))
            .expect("timer.rs");
        let pwm_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("pwm.rs")).expect("pwm.rs");
        let adc_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("adc.rs")).expect("adc.rs");
        let dma_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("dma.rs")).expect("dma.rs");
        let spi_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("spi.rs")).expect("spi.rs");

        assert!(rcc_rs.contains("pub fn enable_usart1_clock"));
        assert!(rcc_rs.contains("pub fn apply_init"));
        assert!(gpio_rs.contains("pub fn enable_clock"));
        assert!(gpio_rs.contains("pub fn assert_reset"));
        assert!(gpio_rs.contains("pub enum Pull"));
        assert!(gpio_rs.contains("pub enum Level"));
        assert!(gpio_rs.contains("pub struct GpioAFlex"));
        assert!(gpio_rs.contains("pub struct GpioAInput"));
        assert!(gpio_rs.contains("pub struct GpioAOutput"));
        assert!(gpio_rs.contains("#[derive(Debug, Clone)]\npub struct GpioAFlex"));
        assert!(gpio_rs.contains("#[derive(Debug, Clone)]\npub struct GpioAInput"));
        assert!(gpio_rs.contains("#[derive(Debug, Clone)]\npub struct GpioAOutput"));
        assert!(!gpio_rs.contains("#[derive(Debug, Clone, Copy)]\npub struct GpioAFlex"));
        assert!(!gpio_rs.contains("#[derive(Debug, Clone, Copy)]\npub struct GpioAInput"));
        assert!(!gpio_rs.contains("#[derive(Debug, Clone, Copy)]\npub struct GpioAOutput"));
        assert!(gpio_rs.contains("pub fn pa0(&self) -> GpioAFlex"));
        assert!(gpio_rs.contains("pub fn into_input(self, pull: Pull)"));
        assert!(gpio_rs.contains("pub fn into_output(self, initial_level: Level)"));
        assert!(gpio_rs.contains("pub fn set_pull(&self, pull: Pull)"));
        assert!(gpio_rs.contains("pub fn is_high(&self) -> Result<bool, metadata::Error>"));
        assert!(
            gpio_rs.contains("pub fn get_output_level(&self) -> Result<Level, metadata::Error>")
        );
        assert!(gpio_rs.contains("write_u32(self.bsrr_addr, self.bsrr_set_mask)?;"));
        assert!(uart_rs.contains("pub fn enable_clock"));
        assert!(uart_rs.contains("pub fn release_reset"));
        assert!(uart_rs.contains("pub fn configure_tx_pa9_route"));
        assert!(uart_rs.contains("pub fn configure_rx_pa10_route"));
        assert!(uart_rs.contains("pub fn configure_8n1"));
        assert!(uart_rs.contains("0x00000400u32"));
        assert!(uart_rs.contains("0x00000200u32"));
        assert!(uart_rs.contains("pub fn set_baud_divider"));
        assert!(uart_rs.contains(") | ((u32::from(fraction) & 0xFu32) << 0))?;"));
        assert!(uart_rs.contains("pub fn write_byte"));
        assert!(uart_rs.contains("pub fn write_bytes"));
        assert!(uart_rs.contains("pub fn read_byte"));
        assert!(uart_rs.contains("pub fn flush"));
        assert!(uart_rs.contains("pub fn enable_txe_interrupt"));
        assert!(uart_rs.contains("pub fn enable_rxne_interrupt"));
        assert!(uart_rs.contains("pub fn enable_tx_dma"));
        assert!(uart_rs.contains("pub fn enable_rx_dma"));
        assert!(!uart_rs.contains("pub async fn read"));
        assert!(!uart_rs.contains("pub async fn write"));
        assert!(spi_rs.contains("pub fn enable_tx_dma"));
        assert!(spi_rs.contains("pub fn enable_rx_dma"));
        assert!(spi_rs.contains("pub fn enable_dma_segment_done_interrupt"));
        assert!(timer_rs.contains("pub fn apply_enable"));
        assert!(timer_rs.contains("pub fn transition_disabled_to_enabled"));
        assert!(timer_rs.contains("modify_u16("));
        assert!(pwm_rs.contains("pub fn transition_enabled_to_disabled"));
        assert!(adc_rs.contains("pub fn apply_calibrate"));
        assert!(adc_rs.contains("pub fn enable_dma"));
        assert!(adc_rs.contains("pub fn reset_dma_fsm"));
        assert!(adc_rs.contains("pub fn enable_done_interrupt"));
        assert!(adc_rs.contains("modify_u32("));
        assert!(dma_rs.contains("pub fn enable_clock"));
    }

    #[test]
    fn generate_embassy_injects_gpio_prelude_for_non_gpio_module_path() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let driver_instances = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let gpio_driver = driver_instances
            .iter_mut()
            .find(|driver| driver["id"] == "drv.gpioa")
            .and_then(Value::as_object_mut)
            .expect("gpio driver");
        gpio_driver.insert("modulePath".to_string(), Value::String("porta".to_string()));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture with custom gpio module path should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let porta_rs = std::fs::read_to_string(output_dir.path().join("src").join("porta.rs"))
            .expect("porta.rs");
        assert!(porta_rs.contains("pub enum Pull"));
        assert!(porta_rs.contains("pub enum Level"));
        assert!(porta_rs.contains("pub struct GpioAFlex"));

        let cargo_output = Command::new("cargo")
            .arg("check")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo check");
        assert!(
            cargo_output.status.success(),
            "generated crate should compile:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_rejects_gpio_driver_without_required_pull_register() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let gpioa_registers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals")
            .iter_mut()
            .find_map(|peripheral| {
                let peripheral = peripheral.as_object_mut()?;
                (peripheral.get("id").and_then(Value::as_str) == Some("periph.gpioa"))
                    .then_some(peripheral)
            })
            .expect("gpioa peripheral")
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("gpioa registers");
        gpioa_registers.retain(|register| {
            register
                .as_object()
                .and_then(|register| register.get("name"))
                .and_then(Value::as_str)
                != Some("PUPDR")
        });

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture without pupdr should still validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("does not expose a supported GPIO register layout")
        );
    }

    #[test]
    fn generate_embassy_supports_bitmask_gpio_ports() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let gpioa_registers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals")
            .iter_mut()
            .find_map(|peripheral| {
                let peripheral = peripheral.as_object_mut()?;
                (peripheral.get("id").and_then(Value::as_str) == Some("periph.gpioa"))
                    .then_some(peripheral)
            })
            .expect("gpioa peripheral")
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("gpioa registers");
        *gpioa_registers = serde_json::from_value(serde_json::json!([
            { "id": "reg.gpioa.data", "name": "DATA", "kind": "register", "offsetBytes": 1020, "widthBits": 32, "fields": [
                { "id": "field.gpioa.data.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.dir", "name": "DIR", "kind": "register", "offsetBytes": 1024, "widthBits": 32, "fields": [
                { "id": "field.gpioa.dir.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.afsel", "name": "AFSEL", "kind": "register", "offsetBytes": 1056, "widthBits": 32, "fields": [
                { "id": "field.gpioa.afsel.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.pur", "name": "PUR", "kind": "register", "offsetBytes": 1296, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pur.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.pdr", "name": "PDR", "kind": "register", "offsetBytes": 1300, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pdr.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.den", "name": "DEN", "kind": "register", "offsetBytes": 1308, "widthBits": 32, "fields": [
                { "id": "field.gpioa.den.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] }
        ]))
        .expect("bitmask gpio registers");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture with bitmask gpio should validate");
        let output_dir = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let gpioa_rs = fs::read_to_string(output_dir.path().join("src").join("gpio.rs"))
            .expect("generated gpio module");
        assert!(gpioa_rs.contains("dir_addr: u64"));
        assert!(gpioa_rs.contains("afsel_addr: u64"));
        assert!(gpioa_rs.contains("data_alias_addr: u64"));
        assert!(gpioa_rs.contains("modify_u32(self.dir_addr, self.bit_mask, self.bit_mask)?;"));
        assert!(gpioa_rs.contains("write_u32(self.data_alias_addr, self.bit_mask)?;"));

        let cargo_output = Command::new("cargo")
            .arg("check")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo check");
        assert!(
            cargo_output.status.success(),
            "generated crate should compile:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_rejects_bitmask_gpio_data_without_canonical_alias_layout() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let gpioa_registers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals")
            .iter_mut()
            .find_map(|peripheral| {
                let peripheral = peripheral.as_object_mut()?;
                (peripheral.get("id").and_then(Value::as_str) == Some("periph.gpioa"))
                    .then_some(peripheral)
            })
            .expect("gpioa peripheral")
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("gpioa registers");
        *gpioa_registers = serde_json::from_value(serde_json::json!([
            { "id": "reg.gpioa.data", "name": "DATA", "kind": "register", "offsetBytes": 0, "widthBits": 32, "fields": [
                { "id": "field.gpioa.data.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.dir", "name": "DIR", "kind": "register", "offsetBytes": 1024, "widthBits": 32, "fields": [
                { "id": "field.gpioa.dir.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.afsel", "name": "AFSEL", "kind": "register", "offsetBytes": 1056, "widthBits": 32, "fields": [
                { "id": "field.gpioa.afsel.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.pur", "name": "PUR", "kind": "register", "offsetBytes": 1296, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pur.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.pdr", "name": "PDR", "kind": "register", "offsetBytes": 1300, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pdr.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.den", "name": "DEN", "kind": "register", "offsetBytes": 1308, "widthBits": 32, "fields": [
                { "id": "field.gpioa.den.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] }
        ]))
        .expect("bitmask gpio registers");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture with noncanonical data offset should still validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("unaligned GPIO base address"));
    }

    #[test]
    fn generate_embassy_rejects_bitmask_gpio_with_nonzero_gpio_field_lsb() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let gpioa_registers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals")
            .iter_mut()
            .find_map(|peripheral| {
                let peripheral = peripheral.as_object_mut()?;
                (peripheral.get("id").and_then(Value::as_str) == Some("periph.gpioa"))
                    .then_some(peripheral)
            })
            .expect("gpioa peripheral")
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("gpioa registers");
        *gpioa_registers = serde_json::from_value(serde_json::json!([
            { "id": "reg.gpioa.data", "name": "DATA", "kind": "register", "offsetBytes": 1020, "widthBits": 32, "fields": [
                { "id": "field.gpioa.data.gpio", "name": "GPIO", "bitRange": { "lsb": 8, "msb": 15 } }
            ] },
            { "id": "reg.gpioa.dir", "name": "DIR", "kind": "register", "offsetBytes": 1024, "widthBits": 32, "fields": [
                { "id": "field.gpioa.dir.gpio", "name": "GPIO", "bitRange": { "lsb": 8, "msb": 15 } }
            ] },
            { "id": "reg.gpioa.afsel", "name": "AFSEL", "kind": "register", "offsetBytes": 1056, "widthBits": 32, "fields": [
                { "id": "field.gpioa.afsel.gpio", "name": "GPIO", "bitRange": { "lsb": 8, "msb": 15 } }
            ] },
            { "id": "reg.gpioa.pur", "name": "PUR", "kind": "register", "offsetBytes": 1296, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pur.gpio", "name": "GPIO", "bitRange": { "lsb": 8, "msb": 15 } }
            ] },
            { "id": "reg.gpioa.pdr", "name": "PDR", "kind": "register", "offsetBytes": 1300, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pdr.gpio", "name": "GPIO", "bitRange": { "lsb": 8, "msb": 15 } }
            ] },
            { "id": "reg.gpioa.den", "name": "DEN", "kind": "register", "offsetBytes": 1308, "widthBits": 32, "fields": [
                { "id": "field.gpioa.den.gpio", "name": "GPIO", "bitRange": { "lsb": 8, "msb": 15 } }
            ] }
        ]))
        .expect("bitmask gpio registers");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture with out-of-window bitmask should still validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("to start at bit 0"));
    }

    #[test]
    fn generate_embassy_rejects_bitmask_gpio_data_mask_outside_alias_window() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let pins = document
            .as_object_mut()
            .expect("document object")
            .get_mut("physical")
            .and_then(Value::as_object_mut)
            .expect("physical object")
            .get_mut("pins")
            .and_then(Value::as_array_mut)
            .expect("pins");
        let pa0 = pins
            .iter_mut()
            .find(|pin| pin["id"] == "pin.pa0")
            .and_then(Value::as_object_mut)
            .expect("pa0 pin");
        pa0.insert("index".to_string(), serde_json::json!(8));

        let gpioa_registers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals")
            .iter_mut()
            .find_map(|peripheral| {
                let peripheral = peripheral.as_object_mut()?;
                (peripheral.get("id").and_then(Value::as_str) == Some("periph.gpioa"))
                    .then_some(peripheral)
            })
            .expect("gpioa peripheral")
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("gpioa registers");
        *gpioa_registers = serde_json::from_value(serde_json::json!([
            { "id": "reg.gpioa.data", "name": "DATA", "kind": "register", "offsetBytes": 1020, "widthBits": 32, "fields": [
                { "id": "field.gpioa.data.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 8 } }
            ] },
            { "id": "reg.gpioa.dir", "name": "DIR", "kind": "register", "offsetBytes": 1024, "widthBits": 32, "fields": [
                { "id": "field.gpioa.dir.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 8 } }
            ] },
            { "id": "reg.gpioa.afsel", "name": "AFSEL", "kind": "register", "offsetBytes": 1056, "widthBits": 32, "fields": [
                { "id": "field.gpioa.afsel.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 8 } }
            ] },
            { "id": "reg.gpioa.pur", "name": "PUR", "kind": "register", "offsetBytes": 1296, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pur.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 8 } }
            ] },
            { "id": "reg.gpioa.pdr", "name": "PDR", "kind": "register", "offsetBytes": 1300, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pdr.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 8 } }
            ] },
            { "id": "reg.gpioa.den", "name": "DEN", "kind": "register", "offsetBytes": 1308, "widthBits": 32, "fields": [
                { "id": "field.gpioa.den.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 8 } }
            ] }
        ]))
        .expect("bitmask gpio registers");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture with out-of-window bitmask should still validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("masked DATA alias window"));
    }

    #[test]
    fn generate_embassy_rejects_bitmask_gpio_registers_across_blocks() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let gpioa_registers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals")
            .iter_mut()
            .find_map(|peripheral| {
                let peripheral = peripheral.as_object_mut()?;
                (peripheral.get("id").and_then(Value::as_str) == Some("periph.gpioa"))
                    .then_some(peripheral)
            })
            .expect("gpioa peripheral")
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("gpioa registers");
        *gpioa_registers = serde_json::from_value(serde_json::json!([
            { "id": "reg.gpioa.data", "name": "DATA", "kind": "register", "offsetBytes": 1020, "widthBits": 32, "fields": [
                { "id": "field.gpioa.data.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.dir", "name": "DIR", "kind": "register", "offsetBytes": 1024, "widthBits": 32, "fields": [
                { "id": "field.gpioa.dir.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.afsel", "name": "AFSEL", "kind": "register", "offsetBytes": 5152, "widthBits": 32, "fields": [
                { "id": "field.gpioa.afsel.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.pur", "name": "PUR", "kind": "register", "offsetBytes": 1296, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pur.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.pdr", "name": "PDR", "kind": "register", "offsetBytes": 1300, "widthBits": 32, "fields": [
                { "id": "field.gpioa.pdr.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] },
            { "id": "reg.gpioa.den", "name": "DEN", "kind": "register", "offsetBytes": 1308, "widthBits": 32, "fields": [
                { "id": "field.gpioa.den.gpio", "name": "GPIO", "bitRange": { "lsb": 0, "msb": 7 } }
            ] }
        ]))
        .expect("bitmask gpio registers");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture with cross-block afsel should still validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("is in GPIO block"));
    }

    #[test]
    fn generate_embassy_rejects_gpio_driver_with_route_for_other_port() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let routes = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("pinTopology")
            .and_then(Value::as_object_mut)
            .expect("pinTopology object")
            .get_mut("routes")
            .and_then(Value::as_array_mut)
            .expect("routes");
        let gpio_route = routes
            .iter_mut()
            .find(|route| route["id"] == "pinroute.gpioa.pa0")
            .and_then(Value::as_object_mut)
            .expect("gpio route");
        gpio_route.insert(
            "peripheralRef".to_string(),
            serde_json::json!("periph.gpiob"),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture with mismatched gpio route should still validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains(
            "references pin route pinroute.gpioa.pa0 for periph.gpiob instead of periph.gpioa"
        ));
    }

    #[test]
    fn generate_embassy_rejects_operation_steps_without_expression_text() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let adc_operation = operations[1].as_object_mut().expect("adc operation");
        let steps = adc_operation
            .get_mut("steps")
            .and_then(Value::as_array_mut)
            .expect("steps");
        let first_step = steps[0].as_object_mut().expect("first step");
        first_step.remove("expression");
        first_step.insert("value".to_string(), serde_json::json!(1));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("value-only operation fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("requires explicit expression text for field-level lowering")
        );
    }

    #[test]
    fn generate_embassy_rejects_non_plain_expression_languages() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let adc_operation = operations[1].as_object_mut().expect("adc operation");
        let steps = adc_operation
            .get_mut("steps")
            .and_then(Value::as_array_mut)
            .expect("steps");
        steps[0]
            .as_object_mut()
            .expect("first step")
            .get_mut("expression")
            .and_then(Value::as_object_mut)
            .expect("expression")
            .insert("language".to_string(), serde_json::json!("cel"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("non-plain expression fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("uses unsupported expression language cel")
        );
    }

    #[test]
    fn generate_embassy_rejects_contradictory_set_clear_expressions() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let adc_operation = operations[1].as_object_mut().expect("adc operation");
        let steps = adc_operation
            .get_mut("steps")
            .and_then(Value::as_array_mut)
            .expect("steps");
        steps[0]
            .as_object_mut()
            .expect("first step")
            .get_mut("expression")
            .and_then(Value::as_object_mut)
            .expect("expression")
            .insert("text".to_string(), serde_json::json!("Set ADON = 0"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("contradictory expression fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .chain()
                .any(|cause| cause.to_string().contains("contradicts Set semantics"))
        );
    }

    #[test]
    fn generate_embassy_rejects_fields_that_exceed_register_width() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let tim1 = fixture_peripheral_mut(&mut document, "periph.tim1");
        let registers = tim1
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim1 registers");
        let ctlr1 = registers[0].as_object_mut().expect("tim1 ctlr1");
        let fields = ctlr1
            .get_mut("fields")
            .and_then(Value::as_array_mut)
            .expect("tim1 fields");
        let cen = fields[0].as_object_mut().expect("tim1 cen field");
        cen.get_mut("bitRange")
            .and_then(Value::as_object_mut)
            .expect("bit range")
            .insert("msb".to_string(), serde_json::json!(16));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("out-of-range field fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("exceeds register"));
    }

    #[test]
    fn generate_embassy_rejects_state_machine_transitions_with_multiple_effect_targets() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let state_machines = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("stateMachines")
            .and_then(Value::as_array_mut)
            .expect("state machines");
        let tim1 = state_machines[0]
            .as_object_mut()
            .expect("tim1 state machine");
        let transitions = tim1
            .get_mut("transitions")
            .and_then(Value::as_array_mut)
            .expect("tim1 transitions");
        let enable_transition = transitions[0].as_object_mut().expect("enable transition");
        let effects = enable_transition
            .get_mut("effects")
            .and_then(Value::as_array_mut)
            .expect("effects");
        effects.push(serde_json::json!({
            "kind": "updates-status",
            "targetRef": "field.tim1.ctlr1.cen"
        }));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("multi-effect state machine fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("requires exactly one effect for first-cut lowering")
        );
    }

    #[test]
    fn generate_embassy_rejects_duplicate_operation_step_indices() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let adc_operation = operations[1].as_object_mut().expect("adc operation");
        let steps = adc_operation
            .get_mut("steps")
            .and_then(Value::as_array_mut)
            .expect("steps");
        steps[1]
            .as_object_mut()
            .expect("second step")
            .insert("index".to_string(), serde_json::json!(0));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("duplicate-step fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("reuses step index 0"));
    }

    #[test]
    fn generate_embassy_rejects_operation_steps_outside_driver_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let tim1_operation = operations[2].as_object_mut().expect("tim1 operation");
        let steps = tim1_operation
            .get_mut("steps")
            .and_then(Value::as_array_mut)
            .expect("steps");
        steps[0]
            .as_object_mut()
            .expect("tim1 step")
            .insert("targetRef".to_string(), serde_json::json!("reg.pwm1.ctlr1"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mismatched operation scope fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error.to_string().contains(
                "references register reg.pwm1.ctlr1 on periph.pwm1 instead of periph.tim1"
            )
        );
    }

    #[test]
    fn generate_embassy_rejects_predicate_targets_outside_driver_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let adc_operation = operations[1].as_object_mut().expect("adc operation");
        let preconditions = adc_operation
            .get_mut("preconditions")
            .and_then(Value::as_array_mut)
            .expect("preconditions");
        preconditions[0]
            .as_object_mut()
            .expect("first precondition")
            .insert(
                "targetRef".to_string(),
                Value::String("reg.tim1.ctlr1".to_string()),
            );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("predicate scope fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("predicate"));
        assert!(error.to_string().contains("instead of periph.adc1"));
    }

    #[test]
    fn generate_embassy_rejects_state_machine_effects_that_do_not_target_fields() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let state_machines = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("stateMachines")
            .and_then(Value::as_array_mut)
            .expect("state machines");
        let tim1 = state_machines[0]
            .as_object_mut()
            .expect("tim1 state machine");
        let transitions = tim1
            .get_mut("transitions")
            .and_then(Value::as_array_mut)
            .expect("tim1 transitions");
        let enable_transition = transitions[0].as_object_mut().expect("enable transition");
        let effects = enable_transition
            .get_mut("effects")
            .and_then(Value::as_array_mut)
            .expect("effects");
        effects[0]
            .as_object_mut()
            .expect("enable effect")
            .insert("targetRef".to_string(), serde_json::json!("reg.tim1.ctlr1"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("register-target state machine fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains(
            "requires effect targetRef reg.tim1.ctlr1 to resolve to a field for first-cut lowering"
        ));
    }

    #[test]
    fn generate_embassy_rejects_arrayed_registers() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let tim1 = fixture_peripheral_mut(&mut document, "periph.tim1");
        let registers = tim1
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim1 registers");
        registers[0].as_object_mut().expect("tim1 ctlr1").insert(
            "array".to_string(),
            serde_json::json!({ "axes": [{ "name": "channel", "count": 2, "strideBytes": 4 }] }),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("arrayed register fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("register reg.tim1.ctlr1 uses arrayed instances")
        );
    }

    #[test]
    fn generate_embassy_rejects_overflowing_register_addresses() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let tim1 = fixture_peripheral_mut(&mut document, "periph.tim1");
        tim1.insert("baseAddress".to_string(), serde_json::json!(u64::MAX));
        let registers = tim1
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim1 registers");
        registers[0]
            .as_object_mut()
            .expect("tim1 ctlr1")
            .insert("offsetBytes".to_string(), serde_json::json!(1));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("overflowing register-address fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("register reg.tim1.ctlr1 offset overflows u64 address space")
        );
    }

    #[test]
    fn generate_embassy_rejects_arrayed_clusters() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let tim1 = fixture_peripheral_mut(&mut document, "periph.tim1");
        tim1.insert(
            "registers".to_string(),
            serde_json::json!([
                {
                    "id": "cluster.tim1.ch",
                    "kind": "cluster",
                    "name": "CH",
                    "offsetBytes": 0,
                    "array": { "axes": [{ "name": "channel", "count": 2, "strideBytes": 8 }] },
                    "members": [
                        {
                            "kind": "register",
                            "id": "reg.tim1.ccr",
                            "name": "CCR",
                            "offsetBytes": 0,
                            "widthBits": 16,
                            "fields": [
                                { "id": "field.tim1.ccr.value", "name": "VALUE", "bitRange": { "lsb": 0, "msb": 15 } }
                            ]
                        }
                    ]
                }
            ]),
        );
        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let tim1_enable = operations
            .iter_mut()
            .find(|item| item.get("id").and_then(Value::as_str) == Some("op.tim1.enable"))
            .and_then(Value::as_object_mut)
            .expect("tim1 enable operation");
        tim1_enable
            .get_mut("steps")
            .and_then(Value::as_array_mut)
            .expect("tim1 enable steps")[0]
            .as_object_mut()
            .expect("tim1 enable step")
            .insert("targetRef".to_string(), serde_json::json!("reg.tim1.ccr"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("arrayed cluster fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("cluster CH uses arrayed instances")
        );
    }

    #[test]
    fn generate_embassy_ignores_unclaimed_arrayed_clusters() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        peripherals.push(serde_json::json!({
            "id": "periph.can1",
            "name": "CAN1",
            "kind": "peripheral",
            "type": "CAN",
            "baseAddress": 1073893376u64,
            "registers": [
                {
                    "id": "cluster.can1.txmailbox",
                    "kind": "cluster",
                    "name": "TxMailBox",
                    "offsetBytes": 384,
                    "array": { "axes": [{ "name": "mailbox", "count": 3, "strideBytes": 16 }] },
                    "members": [
                        {
                            "kind": "register",
                            "id": "reg.can1.tir",
                            "name": "TIR",
                            "offsetBytes": 0,
                            "widthBits": 32,
                            "fields": [
                                { "id": "field.can1.tir.txrq", "name": "TXRQ", "bitRange": { "lsb": 0, "msb": 0 } }
                            ]
                        }
                    ]
                }
            ]
        }));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("fixture with unrelated CAN cluster should validate");
        let temp = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, temp.path())
            .expect("generation should ignore unrelated arrayed clusters");
    }

    #[test]
    fn generate_embassy_rejects_overflowing_cluster_offsets() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let tim1 = fixture_peripheral_mut(&mut document, "periph.tim1");
        tim1.insert(
            "registers".to_string(),
            serde_json::json!([
                {
                    "id": "cluster.tim1.ch",
                    "kind": "cluster",
                    "name": "CH",
                    "offsetBytes": u64::MAX,
                    "members": [
                        {
                            "id": "cluster.tim1.sub",
                            "kind": "cluster",
                            "name": "SUB",
                            "offsetBytes": 1,
                            "members": [
                                {
                                    "kind": "register",
                                    "id": "reg.tim1.ccr",
                                    "name": "CCR",
                                    "offsetBytes": 0,
                                    "widthBits": 16,
                                    "fields": [
                                        { "id": "field.tim1.ccr.value", "name": "VALUE", "bitRange": { "lsb": 0, "msb": 15 } }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]),
        );
        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        let tim1_enable = operations
            .iter_mut()
            .find(|item| item.get("id").and_then(Value::as_str) == Some("op.tim1.enable"))
            .and_then(Value::as_object_mut)
            .expect("tim1 enable operation");
        tim1_enable
            .get_mut("steps")
            .and_then(Value::as_array_mut)
            .expect("tim1 enable steps")[0]
            .as_object_mut()
            .expect("tim1 enable step")
            .insert("targetRef".to_string(), serde_json::json!("reg.tim1.ccr"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("overflowing cluster-offset fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.chain().any(|cause| {
            cause
                .to_string()
                .contains("offset overflows u64 address space")
        }));
    }

    #[test]
    fn generate_embassy_disambiguates_duplicate_clock_binding_names() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let clock_bindings = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("clockResetTopology")
            .and_then(Value::as_object_mut)
            .expect("clockResetTopology object")
            .get_mut("clockBindings")
            .and_then(Value::as_array_mut)
            .expect("clock bindings");
        clock_bindings.push(serde_json::json!({
            "id": "clk.usart1.alt",
            "name": "USART1 alternate clock",
            "consumerRef": "periph.usart1",
            "clockRef": "clk.apb2",
            "bindingKind": "gated",
            "controlRefs": ["reg.rcc.apb2pcenr"]
        }));

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let usart1 = drivers[2].as_object_mut().expect("usart1 driver");
        usart1
            .get_mut("clockBindingRefs")
            .and_then(Value::as_array_mut)
            .expect("clockBindingRefs")
            .push(serde_json::json!("clk.usart1.alt"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("duplicate clock-binding fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        assert!(uart_rs.contains("pub fn enable_usart1_clk_usart1_clock"));
        assert!(uart_rs.contains("pub fn enable_usart1_clk_usart1_alt_clock"));
    }

    #[test]
    fn generate_embassy_disambiguates_duplicate_reset_binding_names() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let reset_bindings = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("clockResetTopology")
            .and_then(Value::as_object_mut)
            .expect("clockResetTopology object")
            .get_mut("resetBindings")
            .and_then(Value::as_array_mut)
            .expect("reset bindings");
        reset_bindings.push(serde_json::json!({
            "id": "rst.usart1.alt",
            "name": "USART1 alternate reset",
            "targetRef": "periph.usart1",
            "bindingKind": "software",
            "controlRefs": ["reg.rcc.apb2prstr"]
        }));

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let usart1 = drivers[2].as_object_mut().expect("usart1 driver");
        usart1
            .get_mut("resetBindingRefs")
            .and_then(Value::as_array_mut)
            .expect("resetBindingRefs")
            .push(serde_json::json!("rst.usart1.alt"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("duplicate reset-binding fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        assert!(uart_rs.contains("pub fn assert_usart1_rst_usart1_reset"));
        assert!(uart_rs.contains("pub fn assert_usart1_rst_usart1_alt_reset"));
    }

    #[test]
    fn generate_embassy_disambiguates_duplicate_transition_pairs() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let state_machines = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("stateMachines")
            .and_then(Value::as_array_mut)
            .expect("state machines");
        state_machines.push(serde_json::json!({
            "id": "sm.tim1.alt",
            "name": "TIM1 alternate modes",
            "targetRefs": ["periph.tim1"],
            "initialState": "disabled",
            "states": [{ "name": "disabled" }, { "name": "enabled" }],
            "transitions": [
                {
                    "from": "disabled",
                    "to": "enabled",
                    "trigger": "alternate enable",
                    "effects": [{ "kind": "starts-hardware", "targetRef": "field.tim1.ctlr1.cen" }]
                }
            ]
        }));

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let tim1 = drivers[5].as_object_mut().expect("tim1 driver");
        tim1.get_mut("stateMachineRefs")
            .and_then(Value::as_array_mut)
            .expect("stateMachineRefs")
            .push(serde_json::json!("sm.tim1.alt"));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("duplicate transition-pair fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let timer_rs = std::fs::read_to_string(output_dir.path().join("src").join("timer.rs"))
            .expect("timer.rs");
        assert!(timer_rs.contains("pub fn transition_tim1_disabled_to_enabled"));
        assert!(timer_rs.contains("pub fn transition_alt_disabled_to_enabled"));
    }

    #[test]
    fn generate_embassy_rejects_unsupported_register_widths_without_panicking() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        let tim1 = fixture_peripheral_mut(&mut document, "periph.tim1");
        let registers = tim1
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim1 registers");
        let ctlr1 = registers[0].as_object_mut().expect("tim1 ctlr1");
        ctlr1.insert("widthBits".to_string(), serde_json::json!(128));
        let fields = ctlr1
            .get_mut("fields")
            .and_then(Value::as_array_mut)
            .expect("tim1 fields");
        fields[0]
            .as_object_mut()
            .expect("tim1 cen field")
            .get_mut("bitRange")
            .and_then(Value::as_object_mut)
            .expect("bit range")
            .insert("msb".to_string(), serde_json::json!(80));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("unsupported-width fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("register reg.tim1.ctlr1 uses unsupported width 128")
        );
    }

    #[test]
    fn render_rust_string_uses_rust_escapes_for_control_chars() {
        let rendered = render_rust_string("line\u{1}break");
        assert!(rendered.starts_with('"'));
        assert!(rendered.ends_with('"'));
        assert!(!rendered.contains('\u{1}'));
    }

    #[test]
    fn write_text_element_escapes_ampersands_once() {
        let mut xml = XmlWriter::new(XmlOptions {
            indent: xmlwriter::Indent::None,
            ..XmlOptions::default()
        });
        xml.start_element("root");
        write_text_element(&mut xml, "description", "clock control & status <ok>");
        xml.end_element();
        let rendered = xml.end_document();

        assert!(rendered.contains("clock control &amp; status &lt;ok&gt;"));
        assert!(!rendered.contains("&amp;amp;"));
    }

    #[test]
    fn generate_embassy_allows_control_chars_in_strings_without_breaking_codegen() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        document
            .as_object_mut()
            .expect("document object")
            .get_mut("provenance")
            .and_then(Value::as_object_mut)
            .expect("provenance object")
            .get_mut("evidence")
            .and_then(Value::as_array_mut)
            .expect("evidence")[0]
            .as_object_mut()
            .expect("evidence item")
            .insert(
                "normalizedClaim".to_string(),
                Value::String("line1\nline2\u{1}".to_string()),
            );

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[2].as_object_mut().expect("uart driver").insert(
            "name".to_string(),
            Value::String("Usart1\ncontrol\u{7}comment".to_string()),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("control-char fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let cargo_output = Command::new("cargo")
            .arg("check")
            .current_dir(output_dir.path())
            .output()
            .expect("cargo check");
        assert!(
            cargo_output.status.success(),
            "generated crate should compile:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&cargo_output.stdout),
            String::from_utf8_lossy(&cargo_output.stderr)
        );
    }

    #[test]
    fn generate_embassy_sanitizes_doc_comment_text() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");

        document
            .as_object_mut()
            .expect("document object")
            .get_mut("metadata")
            .and_then(Value::as_object_mut)
            .expect("metadata object")
            .insert(
                "title".to_string(),
                Value::String("Fixture\nTitle\u{0}".to_string()),
            );

        document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .insert(
                "name".to_string(),
                Value::String("FIXTURE\nDEVICE\u{7}".to_string()),
            );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("doc comment control-char fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let metadata_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("metadata.rs"))
                .expect("metadata.rs");
        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        let interrupt_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("interrupt.rs"))
                .expect("interrupt.rs");
        assert!(
            metadata_rs.contains("//! Generated Embassy-style HAL metadata for Fixture Title ")
        );
        assert!(uart_rs.contains("//! Generated Embassy-style uart module for FIXTURE DEVICE "));
        assert!(
            interrupt_rs
                .contains("//! Generated Embassy-style interrupt module for FIXTURE DEVICE ")
        );
        assert!(interrupt_rs.contains("pub fn enable_irq(&self, irq: Irq)"));
        assert!(interrupt_rs.contains("pub fn is_irq_active(&self, irq: Irq)"));
    }

    #[test]
    fn generate_embassy_rejects_custom_driver_kind() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(true);
        let document = load_validated_hair_document(fixture.path(), &repo_root)
            .expect("custom fixture should still validate");
        let output_dir = tempdir().expect("tempdir");

        let error = generate_embassy_crate(&document, output_dir.path())
            .expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("outside the supported first-cut Embassy subset")
        );
    }

    #[test]
    fn generate_embassy_uses_raw_wch_pfic_irq_indices() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let document = load_validated_hair_document(fixture.path(), &repo_root)
            .expect("fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&document, output_dir.path()).expect("embassy generation");

        let interrupt_rs =
            std::fs::read_to_string(output_dir.path().join("src").join("interrupt.rs"))
                .expect("interrupt.rs");
        assert!(interrupt_rs.contains("let irq_index = irq as u32;"));
        assert!(interrupt_rs.contains("if irq_index < PFIC_EXTERNAL_IRQ_OFFSET"));
        assert!(interrupt_rs.contains("base + u64::from((irq_index / 32) * 4)"));
        assert!(interrupt_rs.contains("1u32 << (irq_index % 32)"));
        assert!(!interrupt_rs.contains(".checked_sub(PFIC_EXTERNAL_IRQ_OFFSET)"));
    }

    #[test]
    fn generate_embassy_requires_embassy_profile() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .remove("embassyHal");
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("document without embassyHal still validates");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("missing profiles.embassyHal"));
    }

    #[test]
    fn generate_embassy_requires_mcu_profile() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .remove("mcuSoc");
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("document without mcuSoc still validates");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("missing profiles.mcuSoc"));
    }

    #[test]
    fn generate_embassy_rejects_unresolved_target_ref() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[0].as_object_mut().expect("driver instance").insert(
            "targetRef".to_string(),
            Value::String("block.missing".to_string()),
        );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("document with missing targetRef still validates");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("unknown canonical block"));
    }

    #[test]
    fn generate_embassy_rejects_unknown_interrupt_route_target() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("interruptTopology")
            .and_then(Value::as_object_mut)
            .expect("interruptTopology object")
            .get_mut("routes")
            .and_then(Value::as_array_mut)
            .expect("routes")[0]
            .as_object_mut()
            .expect("interrupt route")
            .insert(
                "interruptRef".to_string(),
                Value::String("irq.missing".to_string()),
            );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("missing interrupt target fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("unknown device interrupt"));
    }

    #[test]
    fn embassy_pin_role_requirement_deserializes_as_enum() {
        let role: EmbassyPinRole = serde_json::from_value(serde_json::json!({
            "role": "tx",
            "signal": "TX",
            "routeRefs": ["pinroute.usart1.tx"],
            "requirement": "required"
        }))
        .expect("valid requirement should deserialize");
        assert!(matches!(role.requirement, ResourceRequirement::Required));
    }

    #[test]
    fn generate_embassy_rejects_unknown_dma_channel_for_non_dma_driver() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("dmaTopology")
            .and_then(Value::as_object_mut)
            .expect("dmaTopology object")
            .get_mut("routes")
            .and_then(Value::as_array_mut)
            .expect("routes")[0]
            .as_object_mut()
            .expect("dma route")
            .insert(
                "channelRef".to_string(),
                Value::String("dma.missing".to_string()),
            );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("missing dma channel fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error.to_string().contains("DMA channel reference")
                || error.to_string().contains("unknown channel")
        );
    }

    #[test]
    fn generate_embassy_rejects_duplicate_canonical_block_ids() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("canonicalBlocks")
            .and_then(Value::as_array_mut)
            .expect("canonicalBlocks")
            .push(serde_json::json!({
                "id": "block.usart1",
                "name": "Duplicate USART1 block",
                "targetRef": "periph.spi1",
                "blockClass": "spi"
            }));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("duplicate canonical block fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("duplicate canonical block id block.usart1")
        );
    }

    #[test]
    fn generate_embassy_rejects_keyword_module_names() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[0]
            .as_object_mut()
            .expect("rcc driver")
            .insert("modulePath".to_string(), Value::String("type".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("keyword module fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("reserved Rust keyword"));
    }

    #[test]
    fn generate_embassy_rejects_extended_reserved_module_keywords() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[0]
            .as_object_mut()
            .expect("rcc driver")
            .insert("modulePath".to_string(), Value::String("union".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("extended keyword module fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("reserved Rust keyword union"));
    }

    #[test]
    fn generate_embassy_rejects_reserved_interrupt_module_for_non_interrupt_driver() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[2].as_object_mut().expect("uart driver").insert(
            "modulePath".to_string(),
            Value::String("interrupt".to_string()),
        );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("reserved interrupt module fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("reserved interrupt modulePath"));
    }

    #[test]
    fn generate_embassy_rejects_reserved_time_module_without_capability_tag() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[5]
            .as_object_mut()
            .expect("timer driver")
            .insert("modulePath".to_string(), Value::String("time".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("reserved time module fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("reserved time modulePath"));
    }

    #[test]
    fn generate_embassy_supports_interrupt_scoped_time_driver() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let interrupts = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("interrupts")
            .and_then(Value::as_array_mut)
            .expect("interrupts");
        interrupts.push(serde_json::json!({
            "id": "irq.systick",
            "name": "SysTick",
            "number": -1,
            "controllerRef": "ic.nvic"
        }));
        let mcu = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object");
        let interrupt_topology = mcu
            .get_mut("interruptTopology")
            .and_then(Value::as_object_mut)
            .expect("interrupt topology");
        interrupt_topology
            .get_mut("sources")
            .and_then(Value::as_array_mut)
            .expect("interrupt sources")
            .push(serde_json::json!({
                "id": "isrc.systick",
                "name": "SysTick source",
                "sourceRef": "block.cpu0",
                "producerRef": "block.cpu0",
                "kind": "timer"
            }));
        interrupt_topology
            .get_mut("routes")
            .and_then(Value::as_array_mut)
            .expect("interrupt routes")
            .push(serde_json::json!({
                "id": "iroute.systick",
                "name": "SysTick route",
                "sourceRef": "isrc.systick",
                "interruptRef": "irq.systick",
                "controllerRef": "block.pfic",
                "cpuTargetRef": "block.cpu0",
                "routeType": "hardwired"
            }));
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let nvic = drivers
            .iter()
            .find(|driver| {
                driver
                    .as_object()
                    .and_then(|driver| driver.get("driverKind"))
                    .and_then(Value::as_str)
                    == Some("interrupt")
            })
            .expect("interrupt driver")
            .clone();
        let mut time_driver = nvic.as_object().expect("nvic object").clone();
        time_driver.insert("id".to_string(), Value::String("drv.time".to_string()));
        time_driver.insert("name".to_string(), Value::String("Time".to_string()));
        time_driver.insert("modulePath".to_string(), Value::String("time".to_string()));
        time_driver.insert(
            "interruptRouteRefs".to_string(),
            serde_json::json!(["iroute.systick"]),
        );
        time_driver.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );
        time_driver.insert(
            "timeDriverSource".to_string(),
            Value::String(EMBASSY_TIME_DRIVER_SOURCE_SYSTICK.to_string()),
        );
        drivers.push(Value::Object(time_driver));

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("interrupt time-driver fixture should validate");
        let output_dir = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let cargo_toml = std::fs::read_to_string(output_dir.path().join("Cargo.toml"))
            .expect("generated Cargo.toml");
        let lib_rs = std::fs::read_to_string(output_dir.path().join("src").join("lib.rs"))
            .expect("generated lib.rs");
        let time_rs = std::fs::read_to_string(output_dir.path().join("src").join("time.rs"))
            .expect("generated time module");
        assert!(cargo_toml.contains("embassy-time-driver"));
        assert!(cargo_toml.contains("embassy-time-queue-utils"));
        assert!(lib_rs.contains("pub mod time;"));
        assert!(time_rs.contains("init_time_driver"));
        assert!(time_rs.contains("extern \"C\" fn SysTick()"));
        assert!(time_rs.contains("#[allow(dead_code)]"));
        assert!(time_rs.contains("time_driver_impl!"));
    }

    #[test]
    fn generate_embassy_rejects_time_driver_without_source_selector() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let interrupt = drivers[9].as_object_mut().expect("interrupt driver");
        interrupt.insert("modulePath".to_string(), Value::String("time".to_string()));
        interrupt.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("time-driver without source fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("omits required timeDriverSource")
        );
    }

    #[test]
    fn generate_embassy_supports_hardware_timer_scoped_time_driver() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let tim1 = peripherals
            .iter_mut()
            .find(|peripheral| {
                peripheral
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("periph.tim1")
            })
            .and_then(Value::as_object_mut)
            .expect("tim1 peripheral");
        let registers = tim1
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim1 registers");
        registers.extend([
            serde_json::json!({ "id": "reg.tim1.conf", "name": "CONF", "kind": "register", "offsetBytes": 4, "widthBits": 32, "fields": [
                { "id": "field.tim1.conf.clk_en", "name": "CLK_EN", "bitRange": { "lsb": 0, "msb": 0 } },
                { "id": "field.tim1.conf.target0_work_en", "name": "TARGET0_WORK_EN", "bitRange": { "lsb": 1, "msb": 1 } },
                { "id": "field.tim1.conf.timer_unit0_work_en", "name": "TIMER_UNIT0_WORK_EN", "bitRange": { "lsb": 2, "msb": 2 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_op", "name": "UNIT0_OP", "kind": "register", "offsetBytes": 8, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_op.timer_unit0_value_valid", "name": "TIMER_UNIT0_VALUE_VALID", "bitRange": { "lsb": 0, "msb": 0 } },
                { "id": "field.tim1.unit0_op.timer_unit0_update", "name": "TIMER_UNIT0_UPDATE", "bitRange": { "lsb": 1, "msb": 1 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_value_hi", "name": "UNIT0_VALUE_HI", "kind": "register", "offsetBytes": 12, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_value_hi.timer_unit0_value_hi", "name": "TIMER_UNIT0_VALUE_HI", "bitRange": { "lsb": 0, "msb": 19 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_value_lo", "name": "UNIT0_VALUE_LO", "kind": "register", "offsetBytes": 16, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_value_lo.timer_unit0_value_lo", "name": "TIMER_UNIT0_VALUE_LO", "bitRange": { "lsb": 0, "msb": 31 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_hi", "name": "TARGET0_HI", "kind": "register", "offsetBytes": 20, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_hi.timer_target0_hi", "name": "TIMER_TARGET0_HI", "bitRange": { "lsb": 0, "msb": 19 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_lo", "name": "TARGET0_LO", "kind": "register", "offsetBytes": 24, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_lo.timer_target0_lo", "name": "TIMER_TARGET0_LO", "bitRange": { "lsb": 0, "msb": 31 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_conf", "name": "TARGET0_CONF", "kind": "register", "offsetBytes": 28, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_conf.target0_period", "name": "TARGET0_PERIOD", "bitRange": { "lsb": 0, "msb": 7 } },
                { "id": "field.tim1.target0_conf.target0_period_mode", "name": "TARGET0_PERIOD_MODE", "bitRange": { "lsb": 8, "msb": 8 } },
                { "id": "field.tim1.target0_conf.target0_timer_unit_sel", "name": "TARGET0_TIMER_UNIT_SEL", "bitRange": { "lsb": 9, "msb": 9 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.comp0_load", "name": "COMP0_LOAD", "kind": "register", "offsetBytes": 32, "widthBits": 32, "fields": [
                { "id": "field.tim1.comp0_load.timer_comp0_load", "name": "TIMER_COMP0_LOAD", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.int_ena", "name": "INT_ENA", "kind": "register", "offsetBytes": 36, "widthBits": 32, "fields": [
                { "id": "field.tim1.int_ena.target0_int_ena", "name": "TARGET0_INT_ENA", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.int_clr", "name": "INT_CLR", "kind": "register", "offsetBytes": 40, "widthBits": 32, "fields": [
                { "id": "field.tim1.int_clr.target0_int_clr", "name": "TARGET0_INT_CLR", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
        ]);

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        operations.push(serde_json::json!({
            "id": "op.tim1.clear_target0",
            "name": "Clear TIM1 target0 interrupt",
            "kind": "mode-transition",
            "targetRefs": ["periph.tim1"],
            "steps": [{
                "index": 0,
                "action": "write",
                "targetRef": "reg.tim1.int_clr",
                "expression": { "language": "plain", "text": "Set TARGET0_INT_CLR = 1" }
            }]
        }));

        let interrupt_sources = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("interruptTopology")
            .and_then(Value::as_object_mut)
            .expect("interrupt topology")
            .get_mut("sources")
            .and_then(Value::as_array_mut)
            .expect("interrupt sources");
        interrupt_sources
            .iter_mut()
            .find(|source| {
                source
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("isrc.tim1")
            })
            .and_then(Value::as_object_mut)
            .expect("tim1 interrupt source")
            .insert(
                "clearOperationRefs".to_string(),
                serde_json::json!(["op.tim1.clear_target0"]),
            );

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let tim1 = drivers[5].as_object_mut().expect("timer driver");
        tim1.insert("modulePath".to_string(), Value::String("time".to_string()));
        tim1.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );
        tim1.insert(
            "timeDriverSource".to_string(),
            Value::String(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER.to_string()),
        );
        tim1.insert(
            "timeDriverTickHz".to_string(),
            serde_json::json!(16_000_000u64),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("time-driver timer fixture should validate");
        let output_dir = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");
        let cargo_toml = std::fs::read_to_string(output_dir.path().join("Cargo.toml"))
            .expect("generated Cargo.toml");
        let time_rs = std::fs::read_to_string(output_dir.path().join("src").join("time.rs"))
            .expect("generated time module");
        assert!(time_rs.contains("delay_ticks"));
        assert!(time_rs.contains("generated_drv_tim1_time_driver_interrupt"));
        assert!(time_rs.contains("TARGET0_HI"));
        assert!(time_rs.contains("fn disarm_alarm(&self)"));
        assert!(time_rs.contains("let should_rearm = critical_section::with(|cs| {"));
        assert!(!time_rs.contains("                    fn "));
        assert!(cargo_toml.contains("tick-hz-16_000_000"));
    }

    #[test]
    fn generate_embassy_supports_counter_compare_time_driver_when_bindings_are_explicit() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let input = repo_root.join(r"evidence\wch\ch32v203g6u6\hair.json");
        let mut document = load_json_file(&input).expect("ch32v203g6u6 hair json");
        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let tim4 = peripherals
            .iter_mut()
            .find(|peripheral| {
                peripheral
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("periph.tim4")
            })
            .and_then(Value::as_object_mut)
            .expect("tim4 peripheral");
        let registers = tim4
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim4 registers");
        for (register_id, field_name, new_field_id) in [
            ("reg.tim4.dmaintenr", "CC1IE", "field.tim4_dmaintenr.cc1ie"),
            ("reg.tim4.intfr", "CC1IF", "field.tim4_intfr.cc1if"),
        ] {
            let register = registers
                .iter_mut()
                .find(|register| {
                    register
                        .as_object()
                        .and_then(|item| item.get("id"))
                        .and_then(Value::as_str)
                        == Some(register_id)
                })
                .and_then(Value::as_object_mut)
                .expect("tim4 register");
            let fields = register
                .get_mut("fields")
                .and_then(Value::as_array_mut)
                .expect("register fields");
            let field = fields
                .iter_mut()
                .find(|field| {
                    field
                        .as_object()
                        .and_then(|item| item.get("name"))
                        .and_then(Value::as_str)
                        == Some(field_name)
                })
                .and_then(Value::as_object_mut)
                .expect("target field");
            field.insert("id".to_string(), Value::String(new_field_id.to_string()));
        }
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let tim4_index = drivers
            .iter()
            .position(|driver| {
                driver
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("drv.time-tim4")
            })
            .or_else(|| {
                drivers.iter().position(|driver| {
                    driver
                        .as_object()
                        .and_then(|item| item.get("id"))
                        .and_then(Value::as_str)
                        == Some("drv.tim4")
                })
            })
            .expect("tim4 time driver index");
        let tim4 = drivers
            .get_mut(tim4_index)
            .and_then(Value::as_object_mut)
            .expect("tim4 time driver");
        tim4.insert("modulePath".to_string(), Value::String("time".to_string()));
        tim4.insert(
            "loweringPattern".to_string(),
            Value::String(TIMER_LOWERING_COUNTER_COMPARE.to_string()),
        );
        tim4.insert(
            "timeDriverSource".to_string(),
            Value::String(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER.to_string()),
        );
        tim4.insert("timeDriverTickHz".to_string(), serde_json::json!(1_000u64));
        tim4.insert(
            "interruptRouteRefs".to_string(),
            serde_json::json!(["iroute.tim4.cc"]),
        );
        tim4.insert(
            "initOperationRefs".to_string(),
            serde_json::json!([
                "op.tim4.configure_counter_compare_timebase",
                "op.tim4.enable"
            ]),
        );
        tim4.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );
        tim4.insert(
            "timeDriverBindings".to_string(),
            serde_json::json!({
                "counterRef": "reg.tim4.cnt",
                "alarmRef": "reg.tim4.ch1cvr",
                "alarmApplyOperationRefs": [],
                "interruptEnableRef": "field.tim4_dmaintenr.cc1ie",
                "interruptPendingRef": "field.tim4_intfr.cc1if",
                "interruptClearOperationRef": "op.tim4.clear_cc1"
            }),
        );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("counter-compare time-driver fixture should validate");
        let output_dir = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, output_dir.path()).expect("embassy generation");

        let cargo_toml = std::fs::read_to_string(output_dir.path().join("Cargo.toml"))
            .expect("generated Cargo.toml");
        let lib_rs = std::fs::read_to_string(output_dir.path().join("src").join("lib.rs"))
            .expect("generated lib.rs");
        let time_rs = std::fs::read_to_string(output_dir.path().join("src").join("time.rs"))
            .expect("generated time module");

        assert!(cargo_toml.contains("embassy-time-driver"));
        assert!(cargo_toml.contains("tick-hz-1_000"));
        assert!(lib_rs.contains("pub mod time;"));
        assert!(time_rs.contains("generated_drv_time_tim4_time_driver_interrupt"));
        assert!(time_rs.contains("GENERATED_TIME_COUNTER_ADDRESS"));
        assert!(time_rs.contains("GENERATED_TIME_INTERRUPT_PENDING_MASK"));
        assert!(time_rs.contains("delay_ticks"));
    }

    #[test]
    fn generate_embassy_rejects_hardware_timer_time_driver_without_tick_rate() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let tim1 = peripherals
            .iter_mut()
            .find(|peripheral| {
                peripheral
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("periph.tim1")
            })
            .and_then(Value::as_object_mut)
            .expect("tim1 peripheral");
        let registers = tim1
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim1 registers");
        registers.extend([
            serde_json::json!({ "id": "reg.tim1.conf", "name": "CONF", "kind": "register", "offsetBytes": 4, "widthBits": 32, "fields": [
                { "id": "field.tim1.conf.clk_en", "name": "CLK_EN", "bitRange": { "lsb": 0, "msb": 0 } },
                { "id": "field.tim1.conf.target0_work_en", "name": "TARGET0_WORK_EN", "bitRange": { "lsb": 1, "msb": 1 } },
                { "id": "field.tim1.conf.timer_unit0_work_en", "name": "TIMER_UNIT0_WORK_EN", "bitRange": { "lsb": 2, "msb": 2 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_op", "name": "UNIT0_OP", "kind": "register", "offsetBytes": 8, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_op.timer_unit0_value_valid", "name": "TIMER_UNIT0_VALUE_VALID", "bitRange": { "lsb": 0, "msb": 0 } },
                { "id": "field.tim1.unit0_op.timer_unit0_update", "name": "TIMER_UNIT0_UPDATE", "bitRange": { "lsb": 1, "msb": 1 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_value_hi", "name": "UNIT0_VALUE_HI", "kind": "register", "offsetBytes": 12, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_value_hi.timer_unit0_value_hi", "name": "TIMER_UNIT0_VALUE_HI", "bitRange": { "lsb": 0, "msb": 19 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_value_lo", "name": "UNIT0_VALUE_LO", "kind": "register", "offsetBytes": 16, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_value_lo.timer_unit0_value_lo", "name": "TIMER_UNIT0_VALUE_LO", "bitRange": { "lsb": 0, "msb": 31 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_hi", "name": "TARGET0_HI", "kind": "register", "offsetBytes": 20, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_hi.timer_target0_hi", "name": "TIMER_TARGET0_HI", "bitRange": { "lsb": 0, "msb": 19 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_lo", "name": "TARGET0_LO", "kind": "register", "offsetBytes": 24, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_lo.timer_target0_lo", "name": "TIMER_TARGET0_LO", "bitRange": { "lsb": 0, "msb": 31 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_conf", "name": "TARGET0_CONF", "kind": "register", "offsetBytes": 28, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_conf.target0_period", "name": "TARGET0_PERIOD", "bitRange": { "lsb": 0, "msb": 7 } },
                { "id": "field.tim1.target0_conf.target0_period_mode", "name": "TARGET0_PERIOD_MODE", "bitRange": { "lsb": 8, "msb": 8 } },
                { "id": "field.tim1.target0_conf.target0_timer_unit_sel", "name": "TARGET0_TIMER_UNIT_SEL", "bitRange": { "lsb": 9, "msb": 9 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.comp0_load", "name": "COMP0_LOAD", "kind": "register", "offsetBytes": 32, "widthBits": 32, "fields": [
                { "id": "field.tim1.comp0_load.timer_comp0_load", "name": "TIMER_COMP0_LOAD", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.int_ena", "name": "INT_ENA", "kind": "register", "offsetBytes": 36, "widthBits": 32, "fields": [
                { "id": "field.tim1.int_ena.target0_int_ena", "name": "TARGET0_INT_ENA", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.int_clr", "name": "INT_CLR", "kind": "register", "offsetBytes": 40, "widthBits": 32, "fields": [
                { "id": "field.tim1.int_clr.target0_int_clr", "name": "TARGET0_INT_CLR", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
        ]);

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        operations.push(serde_json::json!({
            "id": "op.tim1.clear_target0",
            "name": "Clear TIM1 target0 interrupt",
            "kind": "mode-transition",
            "targetRefs": ["periph.tim1"],
            "steps": [{
                "index": 0,
                "action": "write",
                "targetRef": "reg.tim1.int_clr",
                "expression": { "language": "plain", "text": "Set TARGET0_INT_CLR = 1" }
            }]
        }));

        let interrupt_sources = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("interruptTopology")
            .and_then(Value::as_object_mut)
            .expect("interrupt topology")
            .get_mut("sources")
            .and_then(Value::as_array_mut)
            .expect("interrupt sources");
        interrupt_sources
            .iter_mut()
            .find(|source| {
                source
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("isrc.tim1")
            })
            .and_then(Value::as_object_mut)
            .expect("tim1 interrupt source")
            .insert(
                "clearOperationRefs".to_string(),
                serde_json::json!(["op.tim1.clear_target0"]),
            );

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let tim1 = drivers[5].as_object_mut().expect("timer driver");
        tim1.insert("modulePath".to_string(), Value::String("time".to_string()));
        tim1.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );
        tim1.insert(
            "timeDriverSource".to_string(),
            Value::String(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER.to_string()),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("hardware timer time-driver fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("omits required timeDriverTickHz")
        );
    }

    #[test]
    fn generate_embassy_rejects_hardware_timer_clear_operation_outside_driver_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let tim1 = peripherals
            .iter_mut()
            .find(|peripheral| {
                peripheral
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("periph.tim1")
            })
            .and_then(Value::as_object_mut)
            .expect("tim1 peripheral");
        let registers = tim1
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("tim1 registers");
        registers.extend([
            serde_json::json!({ "id": "reg.tim1.conf", "name": "CONF", "kind": "register", "offsetBytes": 4, "widthBits": 32, "fields": [
                { "id": "field.tim1.conf.clk_en", "name": "CLK_EN", "bitRange": { "lsb": 0, "msb": 0 } },
                { "id": "field.tim1.conf.target0_work_en", "name": "TARGET0_WORK_EN", "bitRange": { "lsb": 1, "msb": 1 } },
                { "id": "field.tim1.conf.timer_unit0_work_en", "name": "TIMER_UNIT0_WORK_EN", "bitRange": { "lsb": 2, "msb": 2 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_op", "name": "UNIT0_OP", "kind": "register", "offsetBytes": 8, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_op.timer_unit0_value_valid", "name": "TIMER_UNIT0_VALUE_VALID", "bitRange": { "lsb": 0, "msb": 0 } },
                { "id": "field.tim1.unit0_op.timer_unit0_update", "name": "TIMER_UNIT0_UPDATE", "bitRange": { "lsb": 1, "msb": 1 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_value_hi", "name": "UNIT0_VALUE_HI", "kind": "register", "offsetBytes": 12, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_value_hi.timer_unit0_value_hi", "name": "TIMER_UNIT0_VALUE_HI", "bitRange": { "lsb": 0, "msb": 19 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.unit0_value_lo", "name": "UNIT0_VALUE_LO", "kind": "register", "offsetBytes": 16, "widthBits": 32, "fields": [
                { "id": "field.tim1.unit0_value_lo.timer_unit0_value_lo", "name": "TIMER_UNIT0_VALUE_LO", "bitRange": { "lsb": 0, "msb": 31 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_hi", "name": "TARGET0_HI", "kind": "register", "offsetBytes": 20, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_hi.timer_target0_hi", "name": "TIMER_TARGET0_HI", "bitRange": { "lsb": 0, "msb": 19 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_lo", "name": "TARGET0_LO", "kind": "register", "offsetBytes": 24, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_lo.timer_target0_lo", "name": "TIMER_TARGET0_LO", "bitRange": { "lsb": 0, "msb": 31 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.target0_conf", "name": "TARGET0_CONF", "kind": "register", "offsetBytes": 28, "widthBits": 32, "fields": [
                { "id": "field.tim1.target0_conf.target0_period", "name": "TARGET0_PERIOD", "bitRange": { "lsb": 0, "msb": 7 } },
                { "id": "field.tim1.target0_conf.target0_period_mode", "name": "TARGET0_PERIOD_MODE", "bitRange": { "lsb": 8, "msb": 8 } },
                { "id": "field.tim1.target0_conf.target0_timer_unit_sel", "name": "TARGET0_TIMER_UNIT_SEL", "bitRange": { "lsb": 9, "msb": 9 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.comp0_load", "name": "COMP0_LOAD", "kind": "register", "offsetBytes": 32, "widthBits": 32, "fields": [
                { "id": "field.tim1.comp0_load.timer_comp0_load", "name": "TIMER_COMP0_LOAD", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.int_ena", "name": "INT_ENA", "kind": "register", "offsetBytes": 36, "widthBits": 32, "fields": [
                { "id": "field.tim1.int_ena.target0_int_ena", "name": "TARGET0_INT_ENA", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
            serde_json::json!({ "id": "reg.tim1.int_clr", "name": "INT_CLR", "kind": "register", "offsetBytes": 40, "widthBits": 32, "fields": [
                { "id": "field.tim1.int_clr.target0_int_clr", "name": "TARGET0_INT_CLR", "bitRange": { "lsb": 0, "msb": 0 } }
            ]}),
        ]);

        let operations = document
            .as_object_mut()
            .expect("document object")
            .get_mut("semantics")
            .and_then(Value::as_object_mut)
            .expect("semantics object")
            .get_mut("operations")
            .and_then(Value::as_array_mut)
            .expect("operations");
        operations.push(serde_json::json!({
            "id": "op.tim1.clear_target0",
            "name": "Clear TIM1 target0 interrupt",
            "kind": "mode-transition",
            "targetRefs": ["periph.usart1"],
            "steps": [{
                "index": 0,
                "action": "write",
                "targetRef": "reg.tim1.int_clr",
                "expression": { "language": "plain", "text": "Set TARGET0_INT_CLR = 1" }
            }]
        }));

        let interrupt_sources = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("interruptTopology")
            .and_then(Value::as_object_mut)
            .expect("interrupt topology")
            .get_mut("sources")
            .and_then(Value::as_array_mut)
            .expect("interrupt sources");
        interrupt_sources
            .iter_mut()
            .find(|source| {
                source
                    .as_object()
                    .and_then(|item| item.get("id"))
                    .and_then(Value::as_str)
                    == Some("isrc.tim1")
            })
            .and_then(Value::as_object_mut)
            .expect("tim1 interrupt source")
            .insert(
                "clearOperationRefs".to_string(),
                serde_json::json!(["op.tim1.clear_target0"]),
            );

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let tim1 = drivers[5].as_object_mut().expect("timer driver");
        tim1.insert("modulePath".to_string(), Value::String("time".to_string()));
        tim1.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );
        tim1.insert(
            "timeDriverSource".to_string(),
            Value::String(EMBASSY_TIME_DRIVER_SOURCE_HARDWARE_TIMER.to_string()),
        );
        tim1.insert(
            "timeDriverTickHz".to_string(),
            serde_json::json!(16_000_000u64),
        );

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("hardware timer time-driver fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains(
            "clear operation op.tim1.clear_target0 targets periph.usart1 instead of periph.tim1"
        ));
    }

    #[test]
    fn generate_embassy_rejects_duplicate_time_driver_capability_tags() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        for index in [5usize, 6usize] {
            let driver = drivers[index].as_object_mut().expect("time-capable driver");
            driver.insert(
                "capabilityTags".to_string(),
                serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
            );
        }
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("duplicate time-driver fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("exactly one generated async timing provider")
        );
    }

    #[test]
    fn generate_embassy_rejects_time_driver_without_interrupt_route() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let interrupt = drivers[9].as_object_mut().expect("interrupt driver");
        interrupt.insert("modulePath".to_string(), Value::String("time".to_string()));
        interrupt.insert(
            "capabilityTags".to_string(),
            serde_json::json!([EMBASSY_TIME_DRIVER_TAG]),
        );
        interrupt.insert(
            "timeDriverSource".to_string(),
            Value::String(EMBASSY_TIME_DRIVER_SOURCE_SYSTICK.to_string()),
        );
        interrupt.insert("interruptRouteRefs".to_string(), serde_json::json!([]));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("time-driver without interrupt fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(
            error
                .to_string()
                .contains("interrupt driver drv.pfic requires interrupt routes")
        );
    }

    #[test]
    fn generate_embassy_rejects_keyword_driver_type_name() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[2]
            .as_object_mut()
            .expect("uart driver")
            .insert("name".to_string(), Value::String("self".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("keyword driver name fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("reserved Rust keyword Self"));
        assert!(error.to_string().contains("driver type"));
    }

    #[test]
    fn generate_embassy_rejects_invalid_package_name() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("crate")
            .and_then(Value::as_object_mut)
            .expect("crate object")
            .insert(
                "packageName".to_string(),
                Value::String("fixture embassy hal".to_string()),
            );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("invalid package fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("packageName"));
    }

    #[test]
    fn generate_embassy_rejects_invalid_crate_name() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("crate")
            .and_then(Value::as_object_mut)
            .expect("crate object")
            .insert(
                "crateName".to_string(),
                Value::String("fixture-embassy-hal".to_string()),
            );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("invalid crate fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("crateName"));
    }

    #[test]
    fn generate_embassy_rejects_colliding_driver_type_names() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let spi = drivers[3].as_object_mut().expect("spi driver");
        spi.insert("name".to_string(), Value::String("Usart1".to_string()));
        spi.insert("modulePath".to_string(), Value::String("uart".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("colliding driver fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("same Rust type"));
    }

    #[test]
    fn generate_embassy_rejects_colliding_interrupt_variants() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let interrupts = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("interrupts")
            .and_then(Value::as_array_mut)
            .expect("interrupts");
        interrupts[1]
            .as_object_mut()
            .expect("spi interrupt")
            .insert("name".to_string(), Value::String("USART1".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("colliding interrupt fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("same Rust enum variant"));
    }

    #[test]
    fn generate_embassy_rejects_keyword_interrupt_variant_name() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let interrupts = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("interrupts")
            .and_then(Value::as_array_mut)
            .expect("interrupts");
        interrupts[0]
            .as_object_mut()
            .expect("usart interrupt")
            .insert("name".to_string(), Value::String("self".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("keyword interrupt fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("reserved Rust keyword Self"));
        assert!(error.to_string().contains("interrupt variant"));
    }

    #[test]
    fn generate_embassy_rejects_extended_reserved_crate_name() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("crate")
            .and_then(Value::as_object_mut)
            .expect("crate object")
            .insert("crateName".to_string(), Value::String("union".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("extended keyword crate fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("reserved Rust keyword"));
        assert!(error.to_string().contains("crateName union"));
    }

    #[test]
    fn generate_embassy_rejects_mismatched_clock_binding_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[2].as_object_mut().expect("uart driver").insert(
            "clockBindingRefs".to_string(),
            serde_json::json!(["clk.spi1"]),
        );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mismatched clock binding fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("clock binding"));
        assert!(error.to_string().contains("instead of periph.usart1"));
    }

    #[test]
    fn generate_embassy_rejects_mismatched_interrupt_route_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[2].as_object_mut().expect("uart driver").insert(
            "interruptRouteRefs".to_string(),
            serde_json::json!(["iroute.spi1"]),
        );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mismatched interrupt route fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("interrupt route"));
        assert!(error.to_string().contains("instead of periph.usart1"));
    }

    #[test]
    fn generate_embassy_rejects_mismatched_dma_route_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        drivers[2].as_object_mut().expect("uart driver").insert(
            "dmaRouteRefs".to_string(),
            serde_json::json!(["dmaroute.adc1"]),
        );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mismatched dma route fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("DMA route"));
        assert!(error.to_string().contains("instead of periph.usart1"));
    }

    #[test]
    fn generate_embassy_rejects_spi_without_base_address_for_executable_lowering() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        peripherals
            .iter_mut()
            .find(|peripheral| peripheral.get("id").and_then(Value::as_str) == Some("periph.spi1"))
            .expect("spi1 peripheral")
            .as_object_mut()
            .expect("spi1 peripheral object")
            .remove("baseAddress");
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("spi fixture without baseAddress should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("spi driver drv.spi1"));
        assert!(error.to_string().contains("missing baseAddress"));
    }

    #[test]
    fn generate_embassy_reports_unknown_control_target_kind() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let bindings = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("clockResetTopology")
            .and_then(Value::as_object_mut)
            .expect("clockResetTopology object")
            .get_mut("clockBindings")
            .and_then(Value::as_array_mut)
            .expect("clock bindings");
        bindings
            .iter_mut()
            .find(|binding| binding.get("id").and_then(Value::as_str) == Some("clk.spi1"))
            .expect("spi1 clock binding")
            .as_object_mut()
            .expect("spi1 clock binding object")
            .insert(
                "controlRefs".to_string(),
                serde_json::json!(["field.does.not.exist"]),
            );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("invalid control target fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("clock binding clk.spi1"));
        assert!(error.to_string().contains("unknown control target"));
        assert!(
            error
                .to_string()
                .contains("expected either a register id or a field id")
        );
    }

    #[test]
    fn generate_embassy_rejects_mismatched_pin_route_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let uart = drivers[2].as_object_mut().expect("uart driver");
        let pin_roles = uart
            .get_mut("pinRoles")
            .and_then(Value::as_array_mut)
            .expect("pinRoles");
        pin_roles[0].as_object_mut().expect("tx role").insert(
            "routeRefs".to_string(),
            serde_json::json!(["pinroute.spi1.mosi"]),
        );
        pin_roles[0]
            .as_object_mut()
            .expect("tx role")
            .insert("signal".to_string(), Value::String("MOSI".to_string()));
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mismatched pin route fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("pin route"));
        assert!(error.to_string().contains("instead of periph.usart1"));
    }

    #[test]
    fn generate_embassy_rejects_mismatched_interrupt_controller_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("interruptTopology")
            .and_then(Value::as_object_mut)
            .expect("interruptTopology object")
            .get_mut("routes")
            .and_then(Value::as_array_mut)
            .expect("routes")[0]
            .as_object_mut()
            .expect("interrupt route")
            .insert(
                "controllerRef".to_string(),
                Value::String("block.usart1".to_string()),
            );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mismatched interrupt controller fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("controller"));
        assert!(error.to_string().contains("instead of block.pfic"));
    }

    #[test]
    fn generate_embassy_rejects_mismatched_dma_controller_scope() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let dma_topology = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("mcuSoc")
            .and_then(Value::as_object_mut)
            .expect("mcuSoc object")
            .get_mut("dmaTopology")
            .and_then(Value::as_object_mut)
            .expect("dmaTopology object");
        dma_topology
            .get_mut("channels")
            .and_then(Value::as_array_mut)
            .expect("channels")
            .push(serde_json::json!({
                "id": "dma.chan99",
                "name": "DMAX Channel 99",
                "controllerRef": "block.usart1",
                "channelIndex": 99
            }));
        dma_topology
            .get_mut("routes")
            .and_then(Value::as_array_mut)
            .expect("routes")[0]
            .as_object_mut()
            .expect("dma route")
            .insert(
                "channelRef".to_string(),
                Value::String("dma.chan99".to_string()),
            );
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("mismatched dma controller fixture should validate");
        let temp = tempdir().expect("tempdir");
        let error =
            generate_embassy_crate(&validated, temp.path()).expect_err("generation should fail");
        assert!(error.to_string().contains("through controller"));
        assert!(error.to_string().contains("instead of block.dma1"));
    }

    #[test]
    fn generate_embassy_allows_polling_uart_without_interrupt_or_dma_routes() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let uart = drivers[2].as_object_mut().expect("uart driver");
        uart.remove("interruptRouteRefs");
        uart.remove("dmaRouteRefs");
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("polling uart fixture should validate");
        let temp = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, temp.path())
            .expect("polling uart generation should succeed");
    }

    #[test]
    fn generate_embassy_allows_polling_uart_without_irq_or_dma_registers() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let device = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object");
        let peripherals = device
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let usart_template = peripherals
            .iter_mut()
            .find(|peripheral| peripheral["id"] == "periph.usart6")
            .and_then(Value::as_object_mut)
            .expect("usart6 peripheral");
        let registers = usart_template
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("usart6 registers");
        registers.retain(|register| register["name"] != "CR3");
        let cr1_fields = registers
            .iter_mut()
            .find(|register| register["name"] == "CR1")
            .and_then(Value::as_object_mut)
            .expect("cr1 register")
            .get_mut("fields")
            .and_then(Value::as_array_mut)
            .expect("cr1 fields");
        cr1_fields.retain(|field| field["name"] != "TXEIE" && field["name"] != "RXNEIE");

        let drivers = document
            .as_object_mut()
            .expect("document object")
            .get_mut("profiles")
            .and_then(Value::as_object_mut)
            .expect("profiles object")
            .get_mut("embassyHal")
            .and_then(Value::as_object_mut)
            .expect("embassyHal object")
            .get_mut("driverInstances")
            .and_then(Value::as_array_mut)
            .expect("driverInstances");
        let uart = drivers[2].as_object_mut().expect("uart driver");
        uart.remove("interruptRouteRefs");
        uart.remove("dmaRouteRefs");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("polling uart fixture should validate");
        let temp = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, temp.path())
            .expect("polling uart generation should succeed without irq/dma registers");
    }

    #[test]
    fn generate_embassy_keeps_clock_reset_helpers_when_uart_template_is_missing() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let device = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object");
        let peripherals = device
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let usart = peripherals
            .iter_mut()
            .find(|peripheral| peripheral["id"] == "periph.usart1")
            .and_then(Value::as_object_mut)
            .expect("usart1 peripheral");
        usart.remove("derivedFromRef");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("uart-without-template fixture should validate");
        let temp = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, temp.path())
            .expect("uart generation should keep clock/reset helpers without template");
        let uart_rs =
            std::fs::read_to_string(temp.path().join("src").join("uart.rs")).expect("uart.rs");
        assert!(uart_rs.contains("pub fn enable_clock"));
        assert!(uart_rs.contains("pub fn release_reset"));
        assert!(!uart_rs.contains("pub fn configure_8n1"));
    }

    #[test]
    fn generate_embassy_keeps_clock_reset_helpers_when_uart_cr1_is_missing() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let device = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object");
        let peripherals = device
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let usart_template = peripherals
            .iter_mut()
            .find(|peripheral| peripheral["id"] == "periph.usart6")
            .and_then(Value::as_object_mut)
            .expect("usart6 peripheral");
        let registers = usart_template
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("usart6 registers");
        registers.retain(|register| register["name"] != "CR1");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("uart-without-cr1 fixture should validate");
        let temp = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, temp.path())
            .expect("uart generation should keep clock/reset helpers without cr1");
        let uart_rs =
            std::fs::read_to_string(temp.path().join("src").join("uart.rs")).expect("uart.rs");
        assert!(uart_rs.contains("pub fn enable_clock"));
        assert!(uart_rs.contains("pub fn release_reset"));
        assert!(!uart_rs.contains("pub fn configure_8n1"));
    }

    #[test]
    fn generate_embassy_keeps_clock_reset_helpers_when_uart_brr_is_missing() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let usart_template = fixture_peripheral_mut(&mut document, "periph.usart6");
        let registers = usart_template
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("usart6 registers");
        registers.retain(|register| register["name"] != "BRR");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("uart-without-brr fixture should validate");
        let temp = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, temp.path())
            .expect("uart generation should keep clock/reset helpers without brr");
        let uart_rs =
            std::fs::read_to_string(temp.path().join("src").join("uart.rs")).expect("uart.rs");
        assert!(uart_rs.contains("pub fn enable_clock"));
        assert!(uart_rs.contains("pub fn release_reset"));
        assert!(!uart_rs.contains("pub fn configure_8n1"));
    }

    #[test]
    fn generate_embassy_keeps_clock_reset_helpers_when_uart_cr2_is_missing() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        let usart_template = fixture_peripheral_mut(&mut document, "periph.usart6");
        let registers = usart_template
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("usart6 registers");
        registers.retain(|register| register["name"] != "CR2");

        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("uart-without-cr2 fixture should validate");
        let temp = tempdir().expect("tempdir");
        generate_embassy_crate(&validated, temp.path())
            .expect("uart generation should keep clock/reset helpers without cr2");
        let uart_rs =
            std::fs::read_to_string(temp.path().join("src").join("uart.rs")).expect("uart.rs");
        assert!(uart_rs.contains("pub fn enable_clock"));
        assert!(uart_rs.contains("pub fn release_reset"));
        assert!(!uart_rs.contains("pub fn configure_8n1"));
    }

    #[test]
    fn field_value_mask_rejects_fields_wider_than_u64() {
        let error = field_value_mask(&ResolvedField {
            id: "field.test.wide".to_string(),
            name: "WIDE".to_string(),
            description: None,
            lsb: 0,
            msb: 64,
        })
        .expect_err("wider-than-u64 field mask should be rejected");
        assert!(error.to_string().contains("exceeds the 64-bit mask limit"));
    }

    #[test]
    fn shifted_field_mask_reports_32_bit_lowering_limit() {
        let register = ResolvedRegister {
            id: "reg.test.wide".to_string(),
            name: "WIDE".to_string(),
            peripheral_ref: "periph.test".to_string(),
            absolute_address: 0,
            width_bits: 64,
            fields: Vec::new(),
        };
        let field = ResolvedField {
            id: "field.test.high".to_string(),
            name: "HIGH".to_string(),
            description: None,
            lsb: 32,
            msb: 33,
        };

        let error = shifted_field_mask(&field, &register)
            .expect_err("mask above bit 31 should be rejected for first-cut lowering");
        assert!(
            error
                .to_string()
                .contains("requires a mask wider than the 32-bit limit for first-cut lowering")
        );
    }

    fn fixture_peripheral_mut<'a>(
        document: &'a mut Value,
        peripheral_id: &str,
    ) -> &'a mut Map<String, Value> {
        document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals")
            .iter_mut()
            .find(|peripheral| peripheral["id"] == peripheral_id)
            .and_then(Value::as_object_mut)
            .unwrap_or_else(|| panic!("missing fixture peripheral {peripheral_id}"))
    }

    #[test]
    fn generate_embassy_uses_canonical_mappings_for_renamed_uart_entities() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        rename_fixture_usart_for_canonical_resolution(&mut document);
        add_fixture_usart_canonical_mappings(&mut document, false);
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("canonicalized uart fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path())
            .expect("embassy generation should use canonical uart mappings");

        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        assert!(uart_rs.contains("pub fn enable"));
        assert!(uart_rs.contains("pub fn disable"));
        assert!(uart_rs.contains("pub fn set_baud_divider"));
        assert!(uart_rs.contains("pub fn write_byte"));
    }

    #[test]
    fn generate_embassy_supports_ch32_usart_alias_register_names() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        rename_fixture_usart_for_ch32_aliases(&mut document);
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("CH32-aliased uart fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        generate_embassy_crate(&validated, output_dir.path())
            .expect("embassy generation should support CH32 USART aliases");

        let uart_rs = std::fs::read_to_string(output_dir.path().join("src").join("uart.rs"))
            .expect("uart.rs");
        assert!(uart_rs.contains("pub fn configure_8n1"));
        assert!(uart_rs.contains("pub fn set_baud_divider"));
        assert!(uart_rs.contains("pub fn write_byte"));
        assert!(uart_rs.contains("pub fn enable_tx_dma"));
        assert!(uart_rs.contains("pub fn enable_rx_dma"));
    }

    #[test]
    fn generate_embassy_rejects_ambiguous_canonical_uart_register_mappings() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = write_embassy_fixture(false);
        let mut document = load_json_file(fixture.path()).expect("fixture json");
        rename_fixture_usart_for_canonical_resolution(&mut document);
        add_fixture_usart_canonical_mappings(&mut document, true);
        let file = write_temp_json(&document);
        let validated = load_validated_hair_document(file.path(), &repo_root)
            .expect("ambiguous canonical uart fixture should validate");
        let output_dir = tempdir().expect("tempdir");

        let error = generate_embassy_crate(&validated, output_dir.path())
            .expect_err("ambiguous canonical register mapping should fail");
        assert!(
            error
                .to_string()
                .contains("multiple registers mapped to canonical term term.register.baud-rate")
        );
    }

    fn write_embassy_fixture(use_custom_driver: bool) -> NamedTempFile {
        let file = NamedTempFile::new().expect("temp file");
        let uart_driver_kind = if use_custom_driver { "custom" } else { "usart" };
        let document = serde_json::json!({
            "documentKind": "hair",
            "schemaVersion": "0.1.0",
            "metadata": {
                "id": "test.embassy-device",
                "role": "device-variant",
                "title": "Embassy Fixture Device",
                "version": "0.1.0"
            },
            "provenance": {
                "sources": [
                    {
                        "id": "src.fixture",
                        "name": "Fixture Source",
                        "kind": "generated",
                        "path": "fixtures/embassy.json"
                    }
                ],
                "evidence": [
                    {
                        "id": "e.fixture",
                        "name": "Fixture evidence",
                        "sourceRef": "src.fixture",
                        "normalizedClaim": "Synthetic fixture for Embassy HAL generation tests."
                    }
                ]
            },
            "structure": {
                "device": {
                    "id": "device.fixture",
                    "name": "FIXTURE1",
                    "vendor": "TestVendor",
                    "cpu": {
                        "name": "QingKe V4B",
                        "revision": "V4B",
                        "endianness": "little",
                        "interruptPriorityBits": 4,
                        "featureFlags": {
                            "mpuPresent": false,
                            "fpuPresent": false,
                            "vendorSystemTimerConfig": true
                        }
                    },
                    "interrupts": [
                        { "id": "irq.usart1", "name": "USART1", "number": 1 },
                        { "id": "irq.spi1", "name": "SPI1", "number": 2 },
                        { "id": "irq.i2c1", "name": "I2C1", "number": 3 },
                        { "id": "irq.tim1", "name": "TIM1", "number": 4 },
                        { "id": "irq.adc1", "name": "ADC1", "number": 5 }
                    ],
                    "peripherals": [
                        { "id": "periph.rcc", "name": "RCC", "kind": "peripheral", "type": "RCC", "baseAddress": 1073872896u64, "registers": [
                            { "id": "reg.rcc.apb2pcenr", "name": "APB2PCENR", "kind": "register", "offsetBytes": 0, "widthBits": 32, "fields": [
                                { "id": "field.rcc.apb2pcenr.iopaen", "name": "IOPAEN", "description": "I/O port A clock enable", "bitRange": { "lsb": 2, "msb": 2 } },
                                { "id": "field.rcc.apb2pcenr.usart1en", "name": "USART1EN", "description": "USART1 clock enable", "bitRange": { "lsb": 14, "msb": 14 } },
                                { "id": "field.rcc.apb2pcenr.spi1en", "name": "SPI1EN", "description": "SPI1 clock enable", "bitRange": { "lsb": 12, "msb": 12 } },
                                { "id": "field.rcc.apb2pcenr.tim1en", "name": "TIM1EN", "description": "TIM1 clock enable", "bitRange": { "lsb": 11, "msb": 11 } },
                                { "id": "field.rcc.apb2pcenr.adc1en", "name": "ADC1EN", "description": "ADC1 clock enable", "bitRange": { "lsb": 9, "msb": 9 } }
                            ] },
                            { "id": "reg.rcc.apb1pcenr", "name": "APB1PCENR", "kind": "register", "offsetBytes": 4, "widthBits": 32, "fields": [
                                { "id": "field.rcc.apb1pcenr.i2c1en", "name": "I2C1EN", "description": "I2C1 clock enable", "bitRange": { "lsb": 21, "msb": 21 } }
                            ] },
                            { "id": "reg.rcc.ahbpcenr", "name": "AHBPCENR", "kind": "register", "offsetBytes": 8, "widthBits": 32, "fields": [
                                { "id": "field.rcc.ahbpcenr.dma1en", "name": "DMA1EN", "description": "DMA1 clock enable", "bitRange": { "lsb": 0, "msb": 0 } }
                            ] },
                            { "id": "reg.rcc.apb2prstr", "name": "APB2PRSTR", "kind": "register", "offsetBytes": 12, "widthBits": 32, "fields": [
                                { "id": "field.rcc.apb2prstr.ioparst", "name": "IOPARST", "description": "I/O port A reset", "bitRange": { "lsb": 2, "msb": 2 } },
                                { "id": "field.rcc.apb2prstr.usart1rst", "name": "USART1RST", "description": "USART1 reset", "bitRange": { "lsb": 14, "msb": 14 } },
                                { "id": "field.rcc.apb2prstr.spi1rst", "name": "SPI1RST", "description": "SPI1 reset", "bitRange": { "lsb": 12, "msb": 12 } },
                                { "id": "field.rcc.apb2prstr.tim1rst", "name": "TIM1RST", "description": "TIM1 reset", "bitRange": { "lsb": 11, "msb": 11 } },
                                { "id": "field.rcc.apb2prstr.adc1rst", "name": "ADC1RST", "description": "ADC1 reset", "bitRange": { "lsb": 9, "msb": 9 } }
                            ] },
                            { "id": "reg.rcc.apb1prstr", "name": "APB1PRSTR", "kind": "register", "offsetBytes": 16, "widthBits": 32, "fields": [
                                { "id": "field.rcc.apb1prstr.i2c1rst", "name": "I2C1RST", "description": "I2C1 reset", "bitRange": { "lsb": 21, "msb": 21 } }
                            ] }
                        ] },
                        { "id": "periph.gpioa", "name": "GPIOA", "kind": "peripheral", "type": "GPIO", "baseAddress": 1073876992u64, "registers": [
                            { "id": "reg.gpioa.moder", "name": "MODER", "kind": "register", "offsetBytes": 0, "widthBits": 32, "fields": [
                                { "id": "field.gpioa.moder.moder12", "name": "MODER12", "bitRange": { "lsb": 24, "msb": 25 } },
                                { "id": "field.gpioa.moder.moder11", "name": "MODER11", "bitRange": { "lsb": 22, "msb": 23 } },
                                { "id": "field.gpioa.moder.moder10", "name": "MODER10", "bitRange": { "lsb": 20, "msb": 21 } },
                                { "id": "field.gpioa.moder.moder9", "name": "MODER9", "bitRange": { "lsb": 18, "msb": 19 } },
                                { "id": "field.gpioa.moder.moder8", "name": "MODER8", "bitRange": { "lsb": 16, "msb": 17 } },
                                { "id": "field.gpioa.moder.moder0", "name": "MODER0", "bitRange": { "lsb": 0, "msb": 1 } }
                            ] },
                            { "id": "reg.gpioa.pupdr", "name": "PUPDR", "kind": "register", "offsetBytes": 12, "widthBits": 32, "fields": [
                                { "id": "field.gpioa.pupdr.pupdr0", "name": "PUPDR0", "bitRange": { "lsb": 0, "msb": 1 } }
                            ] },
                            { "id": "reg.gpioa.idr", "name": "IDR", "kind": "register", "offsetBytes": 16, "widthBits": 32, "fields": [
                                { "id": "field.gpioa.idr.idr0", "name": "IDR0", "bitRange": { "lsb": 0, "msb": 0 } }
                            ] },
                            { "id": "reg.gpioa.odr", "name": "ODR", "kind": "register", "offsetBytes": 20, "widthBits": 32, "fields": [
                                { "id": "field.gpioa.odr.odr0", "name": "ODR0", "bitRange": { "lsb": 0, "msb": 0 } }
                            ] },
                            { "id": "reg.gpioa.bsrr", "name": "BSRR", "kind": "register", "offsetBytes": 24, "widthBits": 32, "fields": [
                                { "id": "field.gpioa.bsrr.bs0", "name": "BS0", "bitRange": { "lsb": 0, "msb": 0 } },
                                { "id": "field.gpioa.bsrr.br0", "name": "BR0", "bitRange": { "lsb": 16, "msb": 16 } }
                            ] },
                            { "id": "reg.gpioa.afrh", "name": "AFRH", "kind": "register", "offsetBytes": 36, "widthBits": 32, "fields": [
                                { "id": "field.gpioa.afrh.afrh12", "name": "AFRH12", "bitRange": { "lsb": 16, "msb": 19 } },
                                { "id": "field.gpioa.afrh.afrh11", "name": "AFRH11", "bitRange": { "lsb": 12, "msb": 15 } },
                                { "id": "field.gpioa.afrh.afrh10", "name": "AFRH10", "bitRange": { "lsb": 8, "msb": 11 } },
                                { "id": "field.gpioa.afrh.afrh9", "name": "AFRH9", "bitRange": { "lsb": 4, "msb": 7 } },
                                { "id": "field.gpioa.afrh.afrh8", "name": "AFRH8", "bitRange": { "lsb": 0, "msb": 3 } }
                            ] }
                        ] },
                        { "id": "periph.gpiob", "name": "GPIOB", "kind": "peripheral", "type": "GPIO", "baseAddress": 1073878016u64, "registers": [
                            { "id": "reg.gpiob.moder", "name": "MODER", "kind": "register", "offsetBytes": 0, "widthBits": 32, "fields": [
                                { "id": "field.gpiob.moder.moder7", "name": "MODER7", "bitRange": { "lsb": 14, "msb": 15 } },
                                { "id": "field.gpiob.moder.moder6", "name": "MODER6", "bitRange": { "lsb": 12, "msb": 13 } }
                            ] },
                            { "id": "reg.gpiob.afrl", "name": "AFRL", "kind": "register", "offsetBytes": 32, "widthBits": 32, "fields": [
                                { "id": "field.gpiob.afrl.afrl7", "name": "AFRL7", "bitRange": { "lsb": 28, "msb": 31 } },
                                { "id": "field.gpiob.afrl.afrl6", "name": "AFRL6", "bitRange": { "lsb": 24, "msb": 27 } }
                            ] }
                        ] },
                        { "id": "periph.usart6", "name": "USART6", "kind": "peripheral", "type": "USART", "baseAddress": 1073815552u64, "registers": [
                            { "id": "reg.usart6.sr", "name": "SR", "kind": "register", "offsetBytes": 0, "widthBits": 32, "fields": [
                                { "id": "field.usart6.sr.txe", "name": "TXE", "description": "Transmit data register empty", "bitRange": { "lsb": 7, "msb": 7 } },
                                { "id": "field.usart6.sr.tc", "name": "TC", "description": "Transmission complete", "bitRange": { "lsb": 6, "msb": 6 } },
                                { "id": "field.usart6.sr.rxne", "name": "RXNE", "description": "Read data register not empty", "bitRange": { "lsb": 5, "msb": 5 } }
                            ] },
                            { "id": "reg.usart6.dr", "name": "DR", "kind": "register", "offsetBytes": 4, "widthBits": 32, "fields": [
                                { "id": "field.usart6.dr.dr", "name": "DR", "description": "Data value", "bitRange": { "lsb": 0, "msb": 8 } }
                            ] },
                            { "id": "reg.usart6.brr", "name": "BRR", "kind": "register", "offsetBytes": 8, "widthBits": 32, "fields": [
                                { "id": "field.usart6.brr.div_mantissa", "name": "DIV_Mantissa", "description": "Mantissa of USARTDIV", "bitRange": { "lsb": 4, "msb": 15 } },
                                { "id": "field.usart6.brr.div_fraction", "name": "DIV_Fraction", "description": "Fraction of USARTDIV", "bitRange": { "lsb": 0, "msb": 3 } }
                            ] },
                            { "id": "reg.usart6.cr1", "name": "CR1", "kind": "register", "offsetBytes": 12, "widthBits": 32, "fields": [
                                { "id": "field.usart6.cr1.over8", "name": "OVER8", "description": "Oversampling mode", "bitRange": { "lsb": 15, "msb": 15 } },
                                { "id": "field.usart6.cr1.ue", "name": "UE", "description": "USART enable", "bitRange": { "lsb": 13, "msb": 13 } },
                                { "id": "field.usart6.cr1.m", "name": "M", "description": "Word length", "bitRange": { "lsb": 12, "msb": 12 } },
                                { "id": "field.usart6.cr1.pce", "name": "PCE", "description": "Parity control enable", "bitRange": { "lsb": 10, "msb": 10 } },
                                { "id": "field.usart6.cr1.ps", "name": "PS", "description": "Parity selection", "bitRange": { "lsb": 9, "msb": 9 } },
                                { "id": "field.usart6.cr1.txeie", "name": "TXEIE", "description": "TXE interrupt enable", "bitRange": { "lsb": 7, "msb": 7 } },
                                { "id": "field.usart6.cr1.rxneie", "name": "RXNEIE", "description": "RXNE interrupt enable", "bitRange": { "lsb": 5, "msb": 5 } },
                                { "id": "field.usart6.cr1.te", "name": "TE", "description": "Transmitter enable", "bitRange": { "lsb": 3, "msb": 3 } },
                                { "id": "field.usart6.cr1.re", "name": "RE", "description": "Receiver enable", "bitRange": { "lsb": 2, "msb": 2 } }
                            ] },
                            { "id": "reg.usart6.cr2", "name": "CR2", "kind": "register", "offsetBytes": 16, "widthBits": 32, "fields": [
                                { "id": "field.usart6.cr2.stop", "name": "STOP", "description": "STOP bits", "bitRange": { "lsb": 12, "msb": 13 } }
                            ] },
                            { "id": "reg.usart6.cr3", "name": "CR3", "kind": "register", "offsetBytes": 20, "widthBits": 32, "fields": [
                                { "id": "field.usart6.cr3.dmat", "name": "DMAT", "description": "DMA enable transmitter", "bitRange": { "lsb": 7, "msb": 7 } },
                                { "id": "field.usart6.cr3.dmar", "name": "DMAR", "description": "DMA enable receiver", "bitRange": { "lsb": 6, "msb": 6 } }
                            ] }
                        ] },
                        { "id": "periph.usart1", "name": "USART1", "kind": "peripheral", "type": "USART", "baseAddress": 1073811456u64, "derivedFromRef": "periph.usart6", "interruptRefs": ["irq.usart1"] },
                        { "id": "periph.spi1", "name": "SPI1", "kind": "peripheral", "type": "SPI", "baseAddress": 1073811648u64, "interruptRefs": ["irq.spi1"], "registers": [
                            { "id": "reg.spi1.dma_conf", "name": "DMA_CONF", "kind": "register", "offsetBytes": 0, "widthBits": 32, "fields": [
                                { "id": "field.spi1.dma_conf.dma_rx_ena", "name": "DMA_RX_ENA", "description": "DMA RX enable", "bitRange": { "lsb": 0, "msb": 0 } },
                                { "id": "field.spi1.dma_conf.dma_tx_ena", "name": "DMA_TX_ENA", "description": "DMA TX enable", "bitRange": { "lsb": 1, "msb": 1 } }
                            ] },
                            { "id": "reg.spi1.dma_int_ena", "name": "DMA_INT_ENA", "kind": "register", "offsetBytes": 4, "widthBits": 32, "fields": [
                                { "id": "field.spi1.dma_int_ena.dma_seg_trans_done_int_ena", "name": "DMA_SEG_TRANS_DONE_INT_ENA", "description": "DMA segment transfer done interrupt enable", "bitRange": { "lsb": 0, "msb": 0 } }
                            ] }
                        ] },
                        { "id": "periph.i2c1", "name": "I2C1", "kind": "peripheral", "type": "I2C", "interruptRefs": ["irq.i2c1"] },
                        { "id": "periph.tim1", "name": "TIM1", "kind": "peripheral", "type": "TIM", "interruptRefs": ["irq.tim1"], "baseAddress": 1073812480u64, "registers": [
                            { "id": "reg.tim1.ctlr1", "name": "CTLR1", "kind": "register", "offsetBytes": 0, "widthBits": 16, "fields": [
                                { "id": "field.tim1.ctlr1.cen", "name": "CEN", "description": "Counter enable", "bitRange": { "lsb": 0, "msb": 0 } }
                            ] }
                        ] },
                        { "id": "periph.pwm1", "name": "PWM1", "kind": "peripheral", "type": "PWM", "baseAddress": 1073812544u64, "registers": [
                            { "id": "reg.pwm1.ctlr1", "name": "CTLR1", "kind": "register", "offsetBytes": 0, "widthBits": 16, "fields": [
                                { "id": "field.pwm1.ctlr1.cen", "name": "CEN", "description": "Counter enable", "bitRange": { "lsb": 0, "msb": 0 } }
                            ] }
                        ] },
                        { "id": "periph.adc1", "name": "ADC1", "kind": "peripheral", "type": "ADC", "interruptRefs": ["irq.adc1"], "baseAddress": 1073811456u64, "registers": [
                            { "id": "reg.adc1.ctlr2", "name": "CTLR2", "kind": "register", "offsetBytes": 8, "widthBits": 32, "fields": [
                                { "id": "field.adc1.ctlr2.adon", "name": "ADON", "description": "ADC enable", "bitRange": { "lsb": 0, "msb": 0 } },
                                { "id": "field.adc1.ctlr2.cal", "name": "CAL", "description": "Calibration start", "bitRange": { "lsb": 2, "msb": 2 } },
                                { "id": "field.adc1.ctlr2.rstcal", "name": "RSTCAL", "description": "Reset calibration", "bitRange": { "lsb": 3, "msb": 3 } }
                            ] },
                            { "id": "reg.adc1.dma_conf", "name": "DMA_CONF", "kind": "register", "offsetBytes": 12, "widthBits": 32, "fields": [
                                { "id": "field.adc1.dma_conf.apb_adc_trans", "name": "APB_ADC_TRANS", "description": "ADC DMA transfer enable", "bitRange": { "lsb": 0, "msb": 0 } },
                                { "id": "field.adc1.dma_conf.apb_adc_reset_fsm", "name": "APB_ADC_RESET_FSM", "description": "ADC DMA state machine reset", "bitRange": { "lsb": 1, "msb": 1 } }
                            ] },
                            { "id": "reg.adc1.int_ena", "name": "INT_ENA", "kind": "register", "offsetBytes": 16, "widthBits": 32, "fields": [
                                { "id": "field.adc1.int_ena.apb_saradc1_done_int_ena", "name": "APB_SARADC1_DONE_INT_ENA", "description": "ADC1 done interrupt enable", "bitRange": { "lsb": 0, "msb": 0 } }
                            ] }
                        ] },
                        { "id": "periph.dma1", "name": "DMA1", "kind": "peripheral", "type": "DMA" },
                        { "id": "periph.pfic", "name": "PFIC", "kind": "peripheral", "type": "INTC" }
                    ]
                }
            },
            "semantics": {
                "operations": [
                    { "id": "op.rcc.init", "name": "Initialize RCC", "description": "Enable the APB2 USART1 gate.", "kind": "initialization", "targetRefs": ["periph.rcc"], "steps": [{ "index": 0, "action": "write", "targetRef": "reg.rcc.apb2pcenr", "expression": { "language": "plain", "text": "Set USART1EN = 1" }, "description": "Turn on USART1 clocking." }], "postconditions": [{ "kind": "clock-enabled", "targetRef": "periph.usart1", "description": "USART1 clock is running." }] },
                    { "id": "op.adc.calibrate", "name": "Calibrate ADC", "description": "Bring ADC1 online and start calibration.", "kind": "initialization", "targetRefs": ["periph.adc1"], "steps": [{ "index": 0, "action": "write", "targetRef": "reg.adc1.ctlr2", "expression": { "language": "plain", "text": "Set ADON = 1" } }, { "index": 1, "action": "write", "targetRef": "reg.adc1.ctlr2", "expression": { "language": "plain", "text": "Set RSTCAL = 1" }, "description": "Reset calibration state." }, { "index": 2, "action": "write", "targetRef": "reg.adc1.ctlr2", "expression": { "language": "plain", "text": "Set CAL = 1" } }], "preconditions": [{ "kind": "clock-enabled", "targetRef": "periph.adc1", "description": "ADC1 clock must already be running." }] },
                    { "id": "op.tim1.enable", "name": "Enable TIM1", "kind": "mode-transition", "targetRefs": ["periph.tim1"], "steps": [{ "index": 0, "action": "write", "targetRef": "reg.tim1.ctlr1", "expression": { "language": "plain", "text": "Set CEN = 1" }, "value": 1 }] },
                    { "id": "op.pwm1.enable", "name": "Enable PWM1", "kind": "mode-transition", "targetRefs": ["periph.pwm1"], "steps": [{ "index": 0, "action": "write", "targetRef": "reg.pwm1.ctlr1", "expression": { "language": "plain", "text": "Set CEN = 1" } }] }
                ],
                "stateMachines": [
                    { "id": "sm.tim1", "name": "TIM1 modes", "description": "Minimal counter enable state machine.", "targetRefs": ["periph.tim1"], "initialState": "disabled", "states": [{ "name": "disabled", "description": "Counter stopped." }, { "name": "enabled", "description": "Counter running.", "invariants": [{ "kind": "field-value", "targetRef": "field.tim1.ctlr1.cen", "expectedValue": 1 }] }], "transitions": [{ "from": "disabled", "to": "enabled", "trigger": "enable requested", "conditions": [{ "kind": "clock-enabled", "targetRef": "periph.tim1" }], "effects": [{ "kind": "starts-hardware", "targetRef": "field.tim1.ctlr1.cen" }] }, { "from": "enabled", "to": "disabled", "trigger": "disable requested", "effects": [{ "kind": "stops-hardware", "targetRef": "field.tim1.ctlr1.cen" }] }] },
                    { "id": "sm.pwm1", "name": "PWM1 modes", "targetRefs": ["periph.pwm1"], "initialState": "disabled", "states": [{ "name": "disabled" }, { "name": "enabled" }], "transitions": [{ "from": "disabled", "to": "enabled", "trigger": "enable requested", "effects": [{ "kind": "starts-hardware", "targetRef": "field.pwm1.ctlr1.cen" }] }, { "from": "enabled", "to": "disabled", "trigger": "disable requested", "effects": [{ "kind": "stops-hardware", "targetRef": "field.pwm1.ctlr1.cen" }] }] }
                ]
            },
            "physical": {
                "pins": [
                    { "id": "pin.pa0", "name": "PA0", "modes": ["alternate-function"], "port": "A", "index": 0 },
                    { "id": "pin.pa1", "name": "PA1", "modes": ["alternate-function"], "port": "A", "index": 1 },
                    { "id": "pin.pa2", "name": "PA2", "modes": ["alternate-function"], "port": "A", "index": 2 },
                    { "id": "pin.pa3", "name": "PA3", "modes": ["alternate-function"], "port": "A", "index": 3 },
                    { "id": "pin.pa5", "name": "PA5", "modes": ["alternate-function"], "port": "A", "index": 5 },
                    { "id": "pin.pa6", "name": "PA6", "modes": ["alternate-function"], "port": "A", "index": 6 },
                    { "id": "pin.pa7", "name": "PA7", "modes": ["alternate-function"], "port": "A", "index": 7 },
                    { "id": "pin.pb6", "name": "PB6", "modes": ["alternate-function"], "port": "B", "index": 6 },
                    { "id": "pin.pb7", "name": "PB7", "modes": ["alternate-function"], "port": "B", "index": 7 },
                    { "id": "pin.pa8", "name": "PA8", "modes": ["alternate-function"], "port": "A", "index": 8 },
                    { "id": "pin.pa9", "name": "PA9", "modes": ["alternate-function"], "port": "A", "index": 9 },
                    { "id": "pin.pa10", "name": "PA10", "modes": ["alternate-function"], "port": "A", "index": 10 }
                ]
            },
            "profiles": {
                "mcuSoc": {
                    "profileKind": "mcu-soc",
                    "deviceClass": "mcu",
                    "canonicalBlocks": [
                        { "id": "block.rcc", "name": "RCC block", "targetRef": "periph.rcc", "blockClass": "clock-controller" },
                        { "id": "block.gpioa", "name": "GPIOA block", "targetRef": "periph.gpioa", "blockClass": "gpio-port" },
                        { "id": "block.usart1", "name": "USART1 block", "targetRef": "periph.usart1", "blockClass": "usart" },
                        { "id": "block.spi1", "name": "SPI1 block", "targetRef": "periph.spi1", "blockClass": "spi" },
                        { "id": "block.i2c1", "name": "I2C1 block", "targetRef": "periph.i2c1", "blockClass": "i2c" },
                        { "id": "block.tim1", "name": "TIM1 block", "targetRef": "periph.tim1", "blockClass": "timer-general" },
                        { "id": "block.pwm1", "name": "PWM1 block", "targetRef": "periph.pwm1", "blockClass": "pwm" },
                        { "id": "block.adc1", "name": "ADC1 block", "targetRef": "periph.adc1", "blockClass": "adc" },
                        { "id": "block.dma1", "name": "DMA1 block", "targetRef": "periph.dma1", "blockClass": "dma-controller" },
                        { "id": "block.pfic", "name": "PFIC block", "targetRef": "periph.pfic", "blockClass": "interrupt-controller" }
                    ],
                    "clockResetTopology": {
                        "clockBindings": [
                            { "id": "clk.gpioa", "name": "GPIOA clock", "consumerRef": "periph.gpioa", "clockRef": "clk.apb2", "controllerRef": "block.rcc", "bindingKind": "gated", "controlRefs": ["reg.rcc.apb2pcenr"] },
                            { "id": "clk.usart1", "name": "USART1 clock", "consumerRef": "periph.usart1", "clockRef": "clk.apb2", "controllerRef": "block.rcc", "bindingKind": "gated", "controlRefs": ["reg.rcc.apb2pcenr"], "enableOperationRefs": ["op.rcc.init"] },
                            { "id": "clk.spi1", "name": "SPI1 clock", "consumerRef": "periph.spi1", "clockRef": "clk.apb2", "controllerRef": "block.rcc", "bindingKind": "gated", "controlRefs": ["reg.rcc.apb2pcenr"] },
                            { "id": "clk.i2c1", "name": "I2C1 clock", "consumerRef": "periph.i2c1", "clockRef": "clk.apb1", "controllerRef": "block.rcc", "bindingKind": "gated", "controlRefs": ["reg.rcc.apb1pcenr"] },
                            { "id": "clk.tim1", "name": "TIM1 clock", "consumerRef": "periph.tim1", "clockRef": "clk.apb2", "controllerRef": "block.rcc", "bindingKind": "gated", "controlRefs": ["reg.rcc.apb2pcenr"], "enableOperationRefs": ["op.tim1.enable"] },
                            { "id": "clk.pwm1", "name": "PWM1 clock", "consumerRef": "periph.pwm1", "clockRef": "clk.apb2", "controllerRef": "block.rcc", "bindingKind": "gated", "controlRefs": ["reg.rcc.apb2pcenr"], "enableOperationRefs": ["op.pwm1.enable"] },
                            { "id": "clk.adc1", "name": "ADC1 clock", "consumerRef": "periph.adc1", "clockRef": "clk.apb2", "controllerRef": "block.rcc", "bindingKind": "gated", "controlRefs": ["reg.rcc.apb2pcenr"], "enableOperationRefs": ["op.adc.calibrate"] },
                            { "id": "clk.dma1", "name": "DMA1 clock", "consumerRef": "periph.dma1", "clockRef": "clk.ahb", "controllerRef": "block.rcc", "bindingKind": "gated", "controlRefs": ["reg.rcc.ahbpcenr"] }
                        ],
                        "resetBindings": [
                            { "id": "rst.gpioa", "name": "GPIOA reset", "targetRef": "periph.gpioa", "controllerRef": "block.rcc", "resetDomainRef": "rst.apb2", "bindingKind": "software", "controlRefs": ["reg.rcc.apb2prstr"] },
                            { "id": "rst.usart1", "name": "USART1 reset", "targetRef": "periph.usart1", "controllerRef": "block.rcc", "resetDomainRef": "rst.apb2", "bindingKind": "software", "controlRefs": ["reg.rcc.apb2prstr"] },
                            { "id": "rst.spi1", "name": "SPI1 reset", "targetRef": "periph.spi1", "controllerRef": "block.rcc", "resetDomainRef": "rst.apb2", "bindingKind": "software", "controlRefs": ["reg.rcc.apb2prstr"] },
                            { "id": "rst.i2c1", "name": "I2C1 reset", "targetRef": "periph.i2c1", "controllerRef": "block.rcc", "resetDomainRef": "rst.apb1", "bindingKind": "software", "controlRefs": ["reg.rcc.apb1prstr"] },
                            { "id": "rst.tim1", "name": "TIM1 reset", "targetRef": "periph.tim1", "controllerRef": "block.rcc", "resetDomainRef": "rst.apb2", "bindingKind": "software", "controlRefs": ["reg.rcc.apb2prstr"] },
                            { "id": "rst.pwm1", "name": "PWM1 reset", "targetRef": "periph.pwm1", "controllerRef": "block.rcc", "resetDomainRef": "rst.apb2", "bindingKind": "software", "controlRefs": ["reg.rcc.apb2prstr"] },
                            { "id": "rst.adc1", "name": "ADC1 reset", "targetRef": "periph.adc1", "controllerRef": "block.rcc", "resetDomainRef": "rst.apb2", "bindingKind": "software", "controlRefs": ["reg.rcc.apb2prstr"] }
                        ]
                    },
                    "interruptTopology": {
                        "sources": [
                            { "id": "isrc.usart1", "name": "USART1 source", "sourceRef": "periph.usart1", "producerRef": "block.usart1", "kind": "peripheral" },
                            { "id": "isrc.spi1", "name": "SPI1 source", "sourceRef": "periph.spi1", "producerRef": "block.spi1", "kind": "peripheral" },
                            { "id": "isrc.i2c1", "name": "I2C1 source", "sourceRef": "periph.i2c1", "producerRef": "block.i2c1", "kind": "peripheral" },
                            { "id": "isrc.tim1", "name": "TIM1 source", "sourceRef": "periph.tim1", "producerRef": "block.tim1", "kind": "timer" },
                            { "id": "isrc.adc1", "name": "ADC1 source", "sourceRef": "periph.adc1", "producerRef": "block.adc1", "kind": "peripheral" }
                        ],
                        "routes": [
                            { "id": "iroute.usart1", "name": "USART1 route", "sourceRef": "isrc.usart1", "interruptRef": "irq.usart1", "controllerRef": "block.pfic", "routeType": "hardwired" },
                            { "id": "iroute.spi1", "name": "SPI1 route", "sourceRef": "isrc.spi1", "interruptRef": "irq.spi1", "controllerRef": "block.pfic", "routeType": "hardwired" },
                            { "id": "iroute.i2c1", "name": "I2C1 route", "sourceRef": "isrc.i2c1", "interruptRef": "irq.i2c1", "controllerRef": "block.pfic", "routeType": "hardwired" },
                            { "id": "iroute.tim1", "name": "TIM1 route", "sourceRef": "isrc.tim1", "interruptRef": "irq.tim1", "controllerRef": "block.pfic", "cpuTargetRef": "block.pfic", "lineIndex": 25, "routeType": "hardwired", "sharedGroup": "tim-group" },
                            { "id": "iroute.adc1", "name": "ADC1 route", "sourceRef": "isrc.adc1", "interruptRef": "irq.adc1", "controllerRef": "block.pfic", "routeType": "hardwired" }
                        ]
                    },
                    "dmaTopology": {
                        "channels": [
                            { "id": "dma.chan1", "name": "DMA1 Channel 1", "controllerRef": "block.dma1", "targetRef": "periph.usart1", "channelIndex": 1, "capabilities": ["tx", "peripheral"], "priorityLevels": ["low", "high"] },
                            { "id": "dma.chan2", "name": "DMA1 Channel 2", "controllerRef": "block.dma1", "targetRef": "periph.usart1", "channelIndex": 2, "capabilities": ["rx", "peripheral"], "priorityLevels": ["low", "high"] }
                        ],
                        "routes": [
                            { "id": "dmaroute.usart1_tx", "name": "USART1 TX DMA", "peripheralRef": "periph.usart1", "signal": "TX", "channelRef": "dma.chan1", "direction": "memory-to-peripheral", "controlRefs": ["reg.rcc.ahbpcenr"] },
                            { "id": "dmaroute.usart1_rx", "name": "USART1 RX DMA", "peripheralRef": "periph.usart1", "signal": "RX", "channelRef": "dma.chan2", "direction": "peripheral-to-memory", "controlRefs": ["reg.rcc.ahbpcenr"] },
                            { "id": "dmaroute.spi1_tx", "name": "SPI1 TX DMA", "peripheralRef": "periph.spi1", "signal": "MOSI", "channelRef": "dma.chan1", "direction": "memory-to-peripheral", "controlRefs": ["reg.rcc.ahbpcenr"] },
                            { "id": "dmaroute.spi1_rx", "name": "SPI1 RX DMA", "peripheralRef": "periph.spi1", "signal": "MISO", "channelRef": "dma.chan2", "direction": "peripheral-to-memory", "controlRefs": ["reg.rcc.ahbpcenr"] },
                            { "id": "dmaroute.adc1", "name": "ADC1 DMA", "peripheralRef": "periph.adc1", "signal": "IN0", "channelRef": "dma.chan1", "direction": "peripheral-to-memory", "controlRefs": ["reg.rcc.ahbpcenr"] }
                        ]
                    },
                    "pinTopology": {
                        "routes": [
                            { "id": "pinroute.gpioa.pa0", "name": "GPIOA PA0", "pinRef": "pin.pa0", "peripheralRef": "periph.gpioa", "signal": "GPIO0", "routeType": "hardwired", "defaultAfterReset": true },
                            { "id": "pinroute.usart1.tx", "name": "USART1 TX", "description": "Alternate-function route AF7.", "pinRef": "pin.pa9", "peripheralRef": "periph.usart1", "signal": "TX", "routeType": "muxed", "controlRefs": ["reg.gpioa.moder", "reg.gpioa.afrh"] },
                            { "id": "pinroute.usart1.rx", "name": "USART1 RX", "description": "Alternate-function route AF7.", "pinRef": "pin.pa10", "peripheralRef": "periph.usart1", "signal": "RX", "routeType": "muxed", "controlRefs": ["reg.gpioa.moder", "reg.gpioa.afrh"], "defaultAfterReset": true },
                            { "id": "pinroute.spi1.sck", "name": "SPI1 SCK", "pinRef": "pin.pa5", "peripheralRef": "periph.spi1", "signal": "SCK", "routeType": "hardwired" },
                            { "id": "pinroute.spi1.mosi", "name": "SPI1 MOSI", "pinRef": "pin.pa7", "peripheralRef": "periph.spi1", "signal": "MOSI", "routeType": "hardwired" },
                            { "id": "pinroute.spi1.miso", "name": "SPI1 MISO", "pinRef": "pin.pa6", "peripheralRef": "periph.spi1", "signal": "MISO", "routeType": "hardwired" },
                            { "id": "pinroute.i2c1.scl", "name": "I2C1 SCL", "pinRef": "pin.pa1", "peripheralRef": "periph.i2c1", "signal": "SCL", "routeType": "hardwired", "electricalConstraintRefs": ["elec.i2c1.scl"] },
                            { "id": "pinroute.i2c1.sda", "name": "I2C1 SDA", "pinRef": "pin.pa2", "peripheralRef": "periph.i2c1", "signal": "SDA", "routeType": "hardwired", "electricalConstraintRefs": ["elec.i2c1.sda"], "conflictRefs": ["pinroute.adc1.in0"] },
                            { "id": "pinroute.tim1.ch1", "name": "TIM1 CH1", "pinRef": "pin.pa8", "peripheralRef": "periph.tim1", "signal": "CH1", "routeType": "hardwired", "controlRefs": ["reg.tim1.ctlr1"] },
                            { "id": "pinroute.pwm1.out", "name": "PWM1 OUT", "pinRef": "pin.pa3", "peripheralRef": "periph.pwm1", "signal": "PWM_OUT", "routeType": "hardwired", "controlRefs": ["reg.pwm1.ctlr1"] },
                            { "id": "pinroute.adc1.in0", "name": "ADC1 IN0", "pinRef": "pin.pa0", "peripheralRef": "periph.adc1", "signal": "IN0", "routeType": "hardwired", "conflictRefs": ["pinroute.i2c1.sda"] }
                        ]
                    }
                },
                "embassyHal": {
                    "profileKind": "embassy-hal",
                    "crate": {
                        "packageName": "fixture-embassy-hal",
                        "crateName": "fixture_embassy_hal",
                        "targetArchitecture": "riscv32imac",
                        "featureFlags": ["dma", "async"]
                    },
                    "driverInstances": [
                        { "id": "drv.rcc", "name": "Rcc", "targetRef": "block.rcc", "driverKind": "rcc", "modulePath": "rcc", "clockBindingRefs": ["clk.usart1"], "initOperationRefs": ["op.rcc.init"] },
                        { "id": "drv.gpioa", "name": "GpioA", "targetRef": "block.gpioa", "driverKind": "gpio-port", "modulePath": "gpio", "clockBindingRefs": ["clk.gpioa"], "resetBindingRefs": ["rst.gpioa"], "pinRoles": [{ "role": "gpio0", "signal": "GPIO0", "routeRefs": ["pinroute.gpioa.pa0"], "requirement": "required" }] },
                        { "id": "drv.usart1", "name": "Usart1", "targetRef": "block.usart1", "driverKind": uart_driver_kind, "modulePath": "uart", "clockBindingRefs": ["clk.usart1"], "resetBindingRefs": ["rst.usart1"], "interruptRouteRefs": ["iroute.usart1"], "dmaRouteRefs": ["dmaroute.usart1_tx", "dmaroute.usart1_rx"], "pinRoles": [{ "role": "tx", "signal": "TX", "routeRefs": ["pinroute.usart1.tx"], "requirement": "required" }, { "role": "rx", "signal": "RX", "routeRefs": ["pinroute.usart1.rx"], "requirement": "required" }] },
                        { "id": "drv.spi1", "name": "Spi1", "targetRef": "block.spi1", "driverKind": "spi", "modulePath": "spi", "clockBindingRefs": ["clk.spi1"], "resetBindingRefs": ["rst.spi1"], "interruptRouteRefs": ["iroute.spi1"], "dmaRouteRefs": ["dmaroute.spi1_tx", "dmaroute.spi1_rx"], "pinRoles": [{ "role": "sck", "signal": "SCK", "routeRefs": ["pinroute.spi1.sck"], "requirement": "required" }, { "role": "mosi", "signal": "MOSI", "routeRefs": ["pinroute.spi1.mosi"], "requirement": "required" }, { "role": "miso", "signal": "MISO", "routeRefs": ["pinroute.spi1.miso"], "requirement": "required" }] },
                        { "id": "drv.i2c1", "name": "I2c1", "targetRef": "block.i2c1", "driverKind": "i2c", "modulePath": "i2c", "clockBindingRefs": ["clk.i2c1"], "resetBindingRefs": ["rst.i2c1"], "interruptRouteRefs": ["iroute.i2c1"], "pinRoles": [{ "role": "scl", "signal": "SCL", "routeRefs": ["pinroute.i2c1.scl"], "requirement": "required" }, { "role": "sda", "signal": "SDA", "routeRefs": ["pinroute.i2c1.sda"], "requirement": "required" }] },
                        { "id": "drv.tim1", "name": "Tim1", "targetRef": "block.tim1", "driverKind": "timer", "modulePath": "timer", "clockBindingRefs": ["clk.tim1"], "resetBindingRefs": ["rst.tim1"], "interruptRouteRefs": ["iroute.tim1"], "pinRoles": [{ "role": "ch1", "signal": "CH1", "routeRefs": ["pinroute.tim1.ch1"], "requirement": "required" }], "initOperationRefs": ["op.tim1.enable"], "stateMachineRefs": ["sm.tim1"] },
                        { "id": "drv.pwm1", "name": "Pwm1", "targetRef": "block.pwm1", "driverKind": "pwm", "modulePath": "pwm", "clockBindingRefs": ["clk.pwm1"], "resetBindingRefs": ["rst.pwm1"], "pinRoles": [{ "role": "out", "signal": "PWM_OUT", "routeRefs": ["pinroute.pwm1.out"], "requirement": "required" }], "initOperationRefs": ["op.pwm1.enable"], "stateMachineRefs": ["sm.pwm1"] },
                        { "id": "drv.adc1", "name": "Adc1", "targetRef": "block.adc1", "driverKind": "adc", "modulePath": "adc", "clockBindingRefs": ["clk.adc1"], "resetBindingRefs": ["rst.adc1"], "interruptRouteRefs": ["iroute.adc1"], "dmaRouteRefs": ["dmaroute.adc1"], "pinRoles": [{ "role": "input0", "signal": "IN0", "routeRefs": ["pinroute.adc1.in0"], "requirement": "required" }], "initOperationRefs": ["op.adc.calibrate"] },
                        { "id": "drv.dma1", "name": "Dma1", "targetRef": "block.dma1", "driverKind": "dma", "modulePath": "dma", "clockBindingRefs": ["clk.dma1"], "dmaRouteRefs": ["dmaroute.usart1_tx", "dmaroute.usart1_rx", "dmaroute.spi1_tx", "dmaroute.spi1_rx", "dmaroute.adc1"] },
                        { "id": "drv.pfic", "name": "Pfic", "targetRef": "block.pfic", "driverKind": "interrupt", "modulePath": "interrupt", "interruptRouteRefs": ["iroute.usart1", "iroute.spi1", "iroute.i2c1", "iroute.tim1", "iroute.adc1"] }
                    ]
                }
            }
        });
        fs::write(
            file.path(),
            serde_json::to_string_pretty(&document).expect("serialize fixture"),
        )
        .expect("write fixture");
        file
    }

    fn rename_fixture_usart_for_canonical_resolution(document: &mut Value) {
        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let usart_template = peripherals
            .iter_mut()
            .find(|peripheral| peripheral["id"] == "periph.usart6")
            .and_then(Value::as_object_mut)
            .expect("usart6 peripheral");
        let registers = usart_template
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("usart6 registers");
        registers
            .iter_mut()
            .find(|register| register["id"] == "reg.usart6.brr")
            .and_then(Value::as_object_mut)
            .expect("brr register")
            .insert("name".to_string(), Value::String("BAUDCFG".to_string()));
        let cr1_fields = registers
            .iter_mut()
            .find(|register| register["id"] == "reg.usart6.cr1")
            .and_then(Value::as_object_mut)
            .expect("cr1 register")
            .get_mut("fields")
            .and_then(Value::as_array_mut)
            .expect("cr1 fields");
        cr1_fields
            .iter_mut()
            .find(|field| field["id"] == "field.usart6.cr1.ue")
            .and_then(Value::as_object_mut)
            .expect("ue field")
            .insert("name".to_string(), Value::String("ENABLE_GATE".to_string()));
    }

    fn rename_fixture_usart_for_ch32_aliases(document: &mut Value) {
        let peripherals = document
            .as_object_mut()
            .expect("document object")
            .get_mut("structure")
            .and_then(Value::as_object_mut)
            .expect("structure object")
            .get_mut("device")
            .and_then(Value::as_object_mut)
            .expect("device object")
            .get_mut("peripherals")
            .and_then(Value::as_array_mut)
            .expect("peripherals");
        let usart_template = peripherals
            .iter_mut()
            .find(|peripheral| peripheral["id"] == "periph.usart6")
            .and_then(Value::as_object_mut)
            .expect("usart6 peripheral");
        let registers = usart_template
            .get_mut("registers")
            .and_then(Value::as_array_mut)
            .expect("usart6 registers");
        for register in registers {
            let Some(register_object) = register.as_object_mut() else {
                continue;
            };
            match register_object
                .get("id")
                .and_then(Value::as_str)
                .expect("register id")
            {
                "reg.usart6.sr" => {
                    register_object.insert("name".to_string(), Value::String("STATR".to_string()));
                }
                "reg.usart6.dr" => {
                    register_object.insert("name".to_string(), Value::String("DATAR".to_string()));
                }
                "reg.usart6.cr1" => {
                    register_object.insert("name".to_string(), Value::String("CTLR1".to_string()));
                }
                "reg.usart6.cr2" => {
                    register_object.insert("name".to_string(), Value::String("CTLR2".to_string()));
                }
                "reg.usart6.cr3" => {
                    register_object.insert("name".to_string(), Value::String("CTLR3".to_string()));
                }
                _ => {}
            }
        }
    }

    fn add_fixture_usart_canonical_mappings(document: &mut Value, ambiguous_baud_register: bool) {
        if ambiguous_baud_register {
            let peripherals = document
                .as_object_mut()
                .expect("document object")
                .get_mut("structure")
                .and_then(Value::as_object_mut)
                .expect("structure object")
                .get_mut("device")
                .and_then(Value::as_object_mut)
                .expect("device object")
                .get_mut("peripherals")
                .and_then(Value::as_array_mut)
                .expect("peripherals");
            let usart_template = peripherals
                .iter_mut()
                .find(|peripheral| peripheral["id"] == "periph.usart6")
                .and_then(Value::as_object_mut)
                .expect("usart6 peripheral");
            usart_template
                .get_mut("registers")
                .and_then(Value::as_array_mut)
                .expect("usart6 registers")
                .push(serde_json::json!({
                    "id": "reg.usart6.alt_baud",
                    "name": "ALTBAUD",
                    "kind": "register",
                    "offsetBytes": 24,
                    "widthBits": 32,
                    "fields": [
                        {
                            "id": "field.usart6.alt_baud.div_mantissa",
                            "name": "DIV_Mantissa",
                            "bitRange": { "lsb": 4, "msb": 15 }
                        },
                        {
                            "id": "field.usart6.alt_baud.div_fraction",
                            "name": "DIV_Fraction",
                            "bitRange": { "lsb": 0, "msb": 3 }
                        }
                    ]
                }));
        }

        document.as_object_mut().expect("document object").insert(
            "normalization".to_string(),
            serde_json::json!({
                "canonicalTerms": [
                    {
                        "id": "term.register.baud-rate",
                        "name": "Baud-rate register",
                        "scope": "register"
                    },
                    {
                        "id": "term.field.enable",
                        "name": "Enable",
                        "scope": "field"
                    }
                ],
                "mappings": [
                    {
                        "id": "norm.reg.usart6.brr",
                        "name": "USART6 baud-rate register",
                        "targetRef": "reg.usart6.brr",
                        "canonicalTermRefs": ["term.register.baud-rate"]
                    },
                    {
                        "id": "norm.field.usart6.cr1.ue",
                        "name": "USART6 enable field",
                        "targetRef": "field.usart6.cr1.ue",
                        "canonicalTermRefs": ["term.field.enable"]
                    }
                ]
            }),
        );
        if ambiguous_baud_register {
            document["normalization"]["mappings"]
                .as_array_mut()
                .expect("normalization mappings")
                .push(serde_json::json!({
                    "id": "norm.reg.usart6.alt-baud",
                    "name": "USART6 alternate baud-rate register",
                    "targetRef": "reg.usart6.alt_baud",
                    "canonicalTermRefs": ["term.register.baud-rate"]
                }));
        }
    }

    fn write_temp_json(document: &Value) -> NamedTempFile {
        let file = NamedTempFile::new().expect("temp file");
        fs::write(
            file.path(),
            serde_json::to_string_pretty(document).expect("serialize temp json"),
        )
        .expect("write temp json");
        file
    }
}
