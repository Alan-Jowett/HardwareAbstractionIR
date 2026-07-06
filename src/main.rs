use std::collections::{BTreeSet, HashMap, HashSet};
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
            render_interrupt_module(&model, &driver_group)
        } else {
            render_driver_module(&model, &module_name, &driver_group)
        };
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
    structure: HairStructure,
    #[serde(default)]
    semantics: HairSemantics,
    #[serde(default)]
    physical: HairPhysical,
    #[serde(default)]
    profiles: HairProfiles,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentMetadata {
    title: String,
    version: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairStructure {
    #[serde(default)]
    architectures: Vec<ArchitectureProfile>,
    device: Device,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArchitectureProfile {
    id: String,
    #[serde(default)]
    address_unit_bits: Option<u32>,
    #[serde(default)]
    data_bus_width_bits: Option<u32>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CpuModel {
    name: String,
    revision: String,
    endianness: String,
    interrupt_priority_bits: u32,
    feature_flags: CpuFeatureFlags,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddressBlock {
    #[serde(default)]
    name: Option<String>,
    offset_bytes: u64,
    size_bytes: u64,
    usage: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum RegisterBlockMember {
    Register(Register),
    Cluster(RegisterCluster),
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BitRange {
    lsb: u32,
    msb: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnumeratedSet {
    #[serde(default)]
    usage: Option<String>,
    values: Vec<EnumeratedValue>,
}

#[derive(Debug, Deserialize)]
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
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticStateMachine {
    id: String,
    name: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HairPhysical {
    #[serde(default)]
    pins: Vec<PhysicalPin>,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuClockBinding {
    id: String,
    name: String,
    consumer_ref: String,
    clock_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuResetBinding {
    id: String,
    name: String,
    target_ref: String,
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
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuInterruptRoute {
    id: String,
    name: String,
    source_ref: String,
    interrupt_ref: String,
    #[allow(dead_code)]
    controller_ref: String,
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
    channel_index: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuDmaRoute {
    id: String,
    name: String,
    peripheral_ref: String,
    channel_ref: String,
    direction: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuPinTopology {
    #[serde(default)]
    routes: Vec<McuPinRoute>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McuPinRoute {
    id: String,
    name: String,
    pin_ref: String,
    peripheral_ref: String,
    signal: String,
    route_type: String,
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
struct EmbassyDriverInstance {
    id: String,
    name: String,
    target_ref: String,
    driver_kind: String,
    module_path: String,
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
    requirement: String,
}

#[derive(Debug, Clone)]
struct ResolvedEmbassyPinRole {
    role: String,
    signal: String,
    requirement: String,
    routes: Vec<McuPinRoute>,
}

#[derive(Debug, Clone)]
struct ResolvedDriverInstance {
    id: String,
    name: String,
    type_name: String,
    driver_kind: String,
    module_name: String,
    target: McuCanonicalBlock,
    clock_bindings: Vec<McuClockBinding>,
    reset_bindings: Vec<McuResetBinding>,
    interrupt_routes: Vec<McuInterruptRoute>,
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

#[allow(dead_code)]
#[derive(Debug)]
struct EmbassyGenerationModel {
    crate_info: EmbassyCrateMetadata,
    document_title: String,
    device_name: String,
    target_architecture: Option<String>,
    feature_flags: Vec<String>,
    interrupts: Vec<Interrupt>,
    pins: Vec<PhysicalPin>,
    drivers: Vec<ResolvedDriverInstance>,
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

        let canonical_blocks = mcu
            .canonical_blocks
            .iter()
            .map(|item| (item.id.as_str(), item.clone()))
            .collect::<HashMap<_, _>>();
        let clock_bindings = mcu
            .clock_reset_topology
            .as_ref()
            .map(|topology| {
                topology
                    .clock_bindings
                    .iter()
                    .map(|item| (item.id.as_str(), item.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let reset_bindings = mcu
            .clock_reset_topology
            .as_ref()
            .map(|topology| {
                topology
                    .reset_bindings
                    .iter()
                    .map(|item| (item.id.as_str(), item.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let interrupt_routes = mcu
            .interrupt_topology
            .as_ref()
            .map(|topology| {
                topology
                    .routes
                    .iter()
                    .map(|item| (item.id.as_str(), item.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let interrupt_sources = mcu
            .interrupt_topology
            .as_ref()
            .map(|topology| {
                topology
                    .sources
                    .iter()
                    .map(|item| (item.id.as_str(), item.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let dma_routes = mcu
            .dma_topology
            .as_ref()
            .map(|topology| {
                topology
                    .routes
                    .iter()
                    .map(|item| (item.id.as_str(), item.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let dma_channels = mcu
            .dma_topology
            .as_ref()
            .map(|topology| {
                topology
                    .channels
                    .iter()
                    .map(|item| (item.id.as_str(), item.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let pin_routes = mcu
            .pin_topology
            .as_ref()
            .map(|topology| {
                topology
                    .routes
                    .iter()
                    .map(|item| (item.id.as_str(), item.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let operations = document
            .semantics
            .operations
            .iter()
            .map(|item| (item.id.as_str(), item.clone()))
            .collect::<HashMap<_, _>>();
        let state_machines = document
            .semantics
            .state_machines
            .iter()
            .map(|item| (item.id.as_str(), item.clone()))
            .collect::<HashMap<_, _>>();

        let mut drivers = Vec::with_capacity(embassy.driver_instances.len());
        for driver in &embassy.driver_instances {
            validate_supported_driver_kind(&driver.driver_kind)?;
            let module_name = validate_module_name(&driver.module_path)?;
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
            let dma_routes =
                resolve_ref_list(&driver.dma_route_refs, &dma_routes, "DMA route", &driver.id)?;
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

            drivers.push(ResolvedDriverInstance {
                id: driver.id.clone(),
                name: driver.name.clone(),
                type_name: to_rust_type_name(&driver.name),
                driver_kind: driver.driver_kind.clone(),
                module_name,
                target,
                clock_bindings,
                reset_bindings,
                interrupt_routes,
                dma_routes,
                pin_roles,
                init_operations,
                state_machines,
                capability_tags: driver.capability_tags.clone(),
            });
        }

        validate_driver_name_collisions(&drivers)?;
        validate_interrupt_name_collisions(&document.structure.device.interrupts)?;

        Ok(Self {
            crate_info: embassy.crate_info.clone(),
            document_title: document.metadata.title.clone(),
            device_name: document.structure.device.name.clone(),
            target_architecture: embassy.crate_info.target_architecture.clone(),
            feature_flags: embassy.crate_info.feature_flags.clone(),
            interrupts: document.structure.device.interrupts.clone(),
            pins: document.physical.pins.clone(),
            drivers,
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
        if !self.interrupts.is_empty() && !names.iter().any(|name| name == "interrupt") {
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
        if !self.interrupts.is_empty() {
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
        | "dma" | "interrupt" => Ok(()),
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
    )
}

fn validate_driver_name_collisions(drivers: &[ResolvedDriverInstance]) -> Result<()> {
    let mut type_names = HashMap::<(&str, &str), &str>::new();
    let mut const_names = HashMap::<(&str, String), &str>::new();
    for driver in drivers {
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

fn validate_interrupt_name_collisions(interrupts: &[Interrupt]) -> Result<()> {
    let mut variants = HashMap::<String, &str>::new();
    for interrupt in interrupts {
        let variant = to_rust_type_name(&interrupt.name);
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

fn validate_driver_resource_scope(
    driver: &EmbassyDriverInstance,
    target: &McuCanonicalBlock,
    resources: &DriverResourceScopeInputs<'_>,
) -> Result<()> {
    match driver.driver_kind.as_str() {
        "uart" | "usart" | "spi" | "i2c" | "timer" | "pwm" | "adc" | "gpio-port" => {
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
            let claims_async_dma =
                !requirements.interrupt_routes.is_empty() || !requirements.dma_routes.is_empty();
            if claims_async_dma && requirements.interrupt_routes.is_empty() {
                bail!(
                    "{} driver {} requires interrupt routes for async DMA-backed operation",
                    driver.driver_kind,
                    driver.id
                );
            }
            if claims_async_dma && requirements.dma_routes.is_empty() {
                bail!(
                    "{} driver {} requires DMA routes for async DMA-backed operation",
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
        }
        "timer" | "pwm" => {
            if requirements.pin_roles.is_empty() {
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
        "adc" => {
            if requirements.init_operations.is_empty() {
                bail!(
                    "adc driver {} requires at least one init operation",
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

fn render_embassy_cargo_toml(model: &EmbassyGenerationModel) -> String {
    format!(
        "[package]\nname = {}\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\nname = {}\npath = \"src/lib.rs\"\n",
        render_rust_string(&model.crate_info.package_name),
        render_rust_string(&model.crate_info.crate_name)
    )
}

fn render_embassy_lib_rs(model: &EmbassyGenerationModel) -> String {
    let mut out = String::from("#![no_std]\n\n");
    for module_name in model.module_names() {
        out.push_str(&format!("pub mod {module_name};\n"));
    }
    out
}

fn render_embassy_metadata_rs(model: &EmbassyGenerationModel) -> String {
    let doc_comment_title = model.document_title.replace(['\r', '\n'], " ");
    format!(
        "//! Generated Embassy-style HAL metadata for {}.\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum ResourceRequirement {{\n    Required,\n    Optional,\n    MutuallyExclusive,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Error {{\n    Unsupported(&'static str),\n    InvalidReference(&'static str),\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct ClockBinding {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub consumer_ref: &'static str,\n    pub clock_ref: &'static str,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct ResetBinding {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub target_ref: &'static str,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct InterruptRoute {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub source_ref: &'static str,\n    pub interrupt_ref: &'static str,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct DmaRoute {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub peripheral_ref: &'static str,\n    pub channel_ref: &'static str,\n    pub direction: &'static str,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct PinRoute {{\n    pub id: &'static str,\n    pub name: &'static str,\n    pub pin_ref: &'static str,\n    pub peripheral_ref: &'static str,\n    pub signal: &'static str,\n    pub route_type: &'static str,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct PinRole {{\n    pub role: &'static str,\n    pub signal: &'static str,\n    pub routes: &'static [PinRoute],\n    pub requirement: ResourceRequirement,\n}}\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct GeneratedMetadata {{\n    pub document_title: &'static str,\n    pub device_name: &'static str,\n    pub target_architecture: Option<&'static str>,\n    pub feature_flags: &'static [&'static str],\n}}\n\npub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata {{\n    document_title: {},\n    device_name: {},\n    target_architecture: {},\n    feature_flags: &{},\n}};\n",
        doc_comment_title,
        render_rust_string(&model.document_title),
        render_rust_string(&model.device_name),
        render_optional_rust_string(model.target_architecture.as_deref()),
        render_string_slice(&model.feature_flags),
    )
}

fn render_interrupt_module(
    model: &EmbassyGenerationModel,
    drivers: &[&ResolvedDriverInstance],
) -> String {
    let mut out = String::from(
        "use crate::metadata;\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Irq {\n",
    );
    for interrupt in &model.interrupts {
        out.push_str(&format!(
            "    {} = {},\n",
            to_rust_type_name(&interrupt.name),
            interrupt.number
        ));
    }
    out.push_str("}\n\n");
    for driver in drivers {
        out.push_str(&render_driver_instance(driver));
    }
    out
}

fn render_driver_module(
    model: &EmbassyGenerationModel,
    module_name: &str,
    drivers: &[&ResolvedDriverInstance],
) -> String {
    let mut out = format!(
        "//! Generated Embassy-style {module_name} module for {}.\n\nuse crate::metadata;\n\n",
        model.device_name
    );
    for driver in drivers {
        out.push_str(&render_driver_instance(driver));
    }
    out
}

fn render_driver_instance(driver: &ResolvedDriverInstance) -> String {
    let const_prefix = to_rust_const_name(&driver.id);
    let mut out = String::new();
    out.push_str(&format!(
        "// Driver instance: {} ({}) from canonical block {} -> {}\n",
        driver.name, driver.driver_kind, driver.target.id, driver.target.block_class
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
        "pub const {const_prefix}_INTERRUPT_ROUTES: &[metadata::InterruptRoute] = &{};\n",
        render_interrupt_route_slice(&driver.interrupt_routes)
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
        "pub const {const_prefix}_INIT_OPERATIONS: &[&str] = &{};\n",
        render_named_entity_slice(
            &driver
                .init_operations
                .iter()
                .map(|item| item.id.clone())
                .collect::<Vec<_>>()
        )
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_STATE_MACHINES: &[&str] = &{};\n",
        render_named_entity_slice(
            &driver
                .state_machines
                .iter()
                .map(|item| item.id.clone())
                .collect::<Vec<_>>()
        )
    ));
    out.push_str(&format!(
        "pub const {const_prefix}_CAPABILITY_TAGS: &[&str] = &{};\n\n",
        render_named_entity_slice(&driver.capability_tags)
    ));
    out.push_str(&format!(
        "#[derive(Debug, Clone, Copy)]\npub struct {type_name}Resources {{\n    pub clocks: &'static [metadata::ClockBinding],\n    pub resets: &'static [metadata::ResetBinding],\n    pub interrupts: &'static [metadata::InterruptRoute],\n    pub dma: &'static [metadata::DmaRoute],\n    pub pins: &'static [metadata::PinRole],\n    pub init_operations: &'static [&'static str],\n    pub state_machines: &'static [&'static str],\n    pub capability_tags: &'static [&'static str],\n}}\n\npub const {const_prefix}_RESOURCES: {type_name}Resources = {type_name}Resources {{\n    clocks: {const_prefix}_CLOCK_BINDINGS,\n    resets: {const_prefix}_RESET_BINDINGS,\n    interrupts: {const_prefix}_INTERRUPT_ROUTES,\n    dma: {const_prefix}_DMA_ROUTES,\n    pins: {const_prefix}_PIN_ROLES,\n    init_operations: {const_prefix}_INIT_OPERATIONS,\n    state_machines: {const_prefix}_STATE_MACHINES,\n    capability_tags: {const_prefix}_CAPABILITY_TAGS,\n}};\n\n#[derive(Debug, Clone, Copy)]\npub struct {type_name} {{\n    resources: {type_name}Resources,\n}}\n\nimpl {type_name} {{\n    pub fn new(resources: {type_name}Resources) -> Result<Self, metadata::Error> {{\n        Ok(Self {{ resources }})\n    }}\n\n    pub fn resources(&self) -> {type_name}Resources {{\n        self.resources\n    }}\n{methods}\n}}\n\n",
        type_name = driver.type_name,
        const_prefix = const_prefix,
        methods = render_driver_methods(driver)
    ));
    out
}

fn render_driver_methods(driver: &ResolvedDriverInstance) -> String {
    match driver.driver_kind.as_str() {
        "rcc" => "    pub fn init(&self) -> Result<(), metadata::Error> {\n        Ok(())\n    }\n".to_string(),
        "gpio-port" => "    pub fn configure(&self, _signal: &str) -> Result<(), metadata::Error> {\n        Ok(())\n    }\n".to_string(),
        "uart" | "usart" => "    pub async fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, metadata::Error> {\n        Ok(0)\n    }\n\n    pub async fn write(&mut self, _buffer: &[u8]) -> Result<(), metadata::Error> {\n        Ok(())\n    }\n".to_string(),
        "spi" => "    pub async fn transfer(&mut self, _read: &mut [u8], _write: &[u8]) -> Result<(), metadata::Error> {\n        Ok(())\n    }\n".to_string(),
        "i2c" => "    pub async fn read(&mut self, _address: u8, _buffer: &mut [u8]) -> Result<(), metadata::Error> {\n        Ok(())\n    }\n\n    pub async fn write(&mut self, _address: u8, _buffer: &[u8]) -> Result<(), metadata::Error> {\n        Ok(())\n    }\n".to_string(),
        "timer" => "    pub fn start(&mut self) -> Result<(), metadata::Error> {\n        Ok(())\n    }\n".to_string(),
        "pwm" => "    pub fn set_duty(&mut self, _duty: u16) -> Result<(), metadata::Error> {\n        Ok(())\n    }\n".to_string(),
        "adc" => "    pub async fn read_sample(&mut self) -> Result<u16, metadata::Error> {\n        Ok(0)\n    }\n".to_string(),
        "dma" => "    pub async fn copy(&mut self, _source: &[u8], _destination: &mut [u8]) -> Result<usize, metadata::Error> {\n        Ok(0)\n    }\n".to_string(),
        "interrupt" => "    pub fn bind(&self) -> &'static [metadata::InterruptRoute] {\n        self.resources.interrupts\n    }\n".to_string(),
        _ => "    pub fn unsupported(&self) -> Result<(), metadata::Error> {\n        Err(metadata::Error::Unsupported(\"unsupported driver kind\"))\n    }\n".to_string(),
    }
}

fn render_clock_binding_slice(items: &[McuClockBinding]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::ClockBinding {{ id: {}, name: {}, consumer_ref: {}, clock_ref: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.consumer_ref),
            render_rust_string(&item.clock_ref)
        )
    }))
}

fn render_reset_binding_slice(items: &[McuResetBinding]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::ResetBinding {{ id: {}, name: {}, target_ref: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.target_ref)
        )
    }))
}

fn render_interrupt_route_slice(items: &[McuInterruptRoute]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::InterruptRoute {{ id: {}, name: {}, source_ref: {}, interrupt_ref: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.source_ref),
            render_rust_string(&item.interrupt_ref)
        )
    }))
}

fn render_dma_route_slice(items: &[McuDmaRoute]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::DmaRoute {{ id: {}, name: {}, peripheral_ref: {}, channel_ref: {}, direction: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.peripheral_ref),
            render_rust_string(&item.channel_ref),
            render_rust_string(&item.direction)
        )
    }))
}

fn render_pin_route_slice(items: &[McuPinRoute]) -> String {
    render_struct_slice(items.iter().map(|item| {
        format!(
            "metadata::PinRoute {{ id: {}, name: {}, pin_ref: {}, peripheral_ref: {}, signal: {}, route_type: {} }}",
            render_rust_string(&item.id),
            render_rust_string(&item.name),
            render_rust_string(&item.pin_ref),
            render_rust_string(&item.peripheral_ref),
            render_rust_string(&item.signal),
            render_rust_string(&item.route_type)
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

fn render_resource_requirement(requirement: &str) -> &'static str {
    match requirement {
        "required" => "Required",
        "optional" => "Optional",
        "mutually-exclusive" => "MutuallyExclusive",
        _ => "Required",
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

fn render_rust_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"<invalid>\"".to_string())
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
            assigned
                .entry(peripheral.id.as_str())
                .or_default()
                .push(interrupt);
            emitted_interrupt_ids.insert(interrupt.id.as_str());
        }
    }

    for interrupt in &device.interrupts {
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
    if field.bit_range.msb < field.bit_range.lsb {
        bail!("field {} has msb < lsb", field.id);
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
    xml.write_text(value);
    xml.end_element();
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
                        { "id": "periph.rcc", "name": "RCC", "kind": "peripheral", "type": "RCC" },
                        { "id": "periph.gpioa", "name": "GPIOA", "kind": "peripheral", "type": "GPIO" },
                        { "id": "periph.usart1", "name": "USART1", "kind": "peripheral", "type": "USART", "interruptRefs": ["irq.usart1"] },
                        { "id": "periph.spi1", "name": "SPI1", "kind": "peripheral", "type": "SPI", "interruptRefs": ["irq.spi1"] },
                        { "id": "periph.i2c1", "name": "I2C1", "kind": "peripheral", "type": "I2C", "interruptRefs": ["irq.i2c1"] },
                        { "id": "periph.tim1", "name": "TIM1", "kind": "peripheral", "type": "TIM", "interruptRefs": ["irq.tim1"] },
                        { "id": "periph.pwm1", "name": "PWM1", "kind": "peripheral", "type": "PWM" },
                        { "id": "periph.adc1", "name": "ADC1", "kind": "peripheral", "type": "ADC", "interruptRefs": ["irq.adc1"] },
                        { "id": "periph.dma1", "name": "DMA1", "kind": "peripheral", "type": "DMA" },
                        { "id": "periph.pfic", "name": "PFIC", "kind": "peripheral", "type": "INTC" }
                    ]
                }
            },
            "semantics": {
                "operations": [
                    { "id": "op.rcc.init", "name": "Initialize RCC", "kind": "initialization", "targetRefs": ["periph.rcc"], "steps": [{ "index": 0, "action": "write", "description": "Enable clocks" }] },
                    { "id": "op.adc.calibrate", "name": "Calibrate ADC", "kind": "initialization", "targetRefs": ["periph.adc1"], "steps": [{ "index": 0, "action": "write", "description": "Start calibration" }] },
                    { "id": "op.tim1.enable", "name": "Enable TIM1", "kind": "mode-transition", "targetRefs": ["periph.tim1"], "steps": [{ "index": 0, "action": "write", "description": "Enable timer" }] },
                    { "id": "op.pwm1.enable", "name": "Enable PWM1", "kind": "mode-transition", "targetRefs": ["periph.pwm1"], "steps": [{ "index": 0, "action": "write", "description": "Enable PWM" }] }
                ],
                "stateMachines": [
                    { "id": "sm.tim1", "name": "TIM1 modes", "targetRefs": ["periph.tim1"], "states": [{ "name": "disabled" }, { "name": "enabled" }], "transitions": [{ "from": "disabled", "to": "enabled" }] },
                    { "id": "sm.pwm1", "name": "PWM1 modes", "targetRefs": ["periph.pwm1"], "states": [{ "name": "disabled" }, { "name": "enabled" }], "transitions": [{ "from": "disabled", "to": "enabled" }] }
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
                            { "id": "clk.usart1", "name": "USART1 clock", "consumerRef": "periph.usart1", "clockRef": "clk.apb2", "bindingKind": "gated" },
                            { "id": "clk.spi1", "name": "SPI1 clock", "consumerRef": "periph.spi1", "clockRef": "clk.apb2", "bindingKind": "gated" },
                            { "id": "clk.i2c1", "name": "I2C1 clock", "consumerRef": "periph.i2c1", "clockRef": "clk.apb1", "bindingKind": "gated" },
                            { "id": "clk.tim1", "name": "TIM1 clock", "consumerRef": "periph.tim1", "clockRef": "clk.apb2", "bindingKind": "gated" },
                            { "id": "clk.pwm1", "name": "PWM1 clock", "consumerRef": "periph.pwm1", "clockRef": "clk.apb2", "bindingKind": "gated" },
                            { "id": "clk.adc1", "name": "ADC1 clock", "consumerRef": "periph.adc1", "clockRef": "clk.apb2", "bindingKind": "gated" }
                        ],
                        "resetBindings": [
                            { "id": "rst.usart1", "name": "USART1 reset", "targetRef": "periph.usart1", "bindingKind": "software" },
                            { "id": "rst.spi1", "name": "SPI1 reset", "targetRef": "periph.spi1", "bindingKind": "software" },
                            { "id": "rst.i2c1", "name": "I2C1 reset", "targetRef": "periph.i2c1", "bindingKind": "software" },
                            { "id": "rst.tim1", "name": "TIM1 reset", "targetRef": "periph.tim1", "bindingKind": "software" },
                            { "id": "rst.pwm1", "name": "PWM1 reset", "targetRef": "periph.pwm1", "bindingKind": "software" },
                            { "id": "rst.adc1", "name": "ADC1 reset", "targetRef": "periph.adc1", "bindingKind": "software" }
                        ]
                    },
                    "interruptTopology": {
                        "sources": [
                            { "id": "isrc.usart1", "name": "USART1 source", "sourceRef": "periph.usart1", "kind": "peripheral" },
                            { "id": "isrc.spi1", "name": "SPI1 source", "sourceRef": "periph.spi1", "kind": "peripheral" },
                            { "id": "isrc.i2c1", "name": "I2C1 source", "sourceRef": "periph.i2c1", "kind": "peripheral" },
                            { "id": "isrc.tim1", "name": "TIM1 source", "sourceRef": "periph.tim1", "kind": "timer" },
                            { "id": "isrc.adc1", "name": "ADC1 source", "sourceRef": "periph.adc1", "kind": "peripheral" }
                        ],
                        "routes": [
                            { "id": "iroute.usart1", "name": "USART1 route", "sourceRef": "isrc.usart1", "interruptRef": "irq.usart1", "controllerRef": "block.pfic", "routeType": "hardwired" },
                            { "id": "iroute.spi1", "name": "SPI1 route", "sourceRef": "isrc.spi1", "interruptRef": "irq.spi1", "controllerRef": "block.pfic", "routeType": "hardwired" },
                            { "id": "iroute.i2c1", "name": "I2C1 route", "sourceRef": "isrc.i2c1", "interruptRef": "irq.i2c1", "controllerRef": "block.pfic", "routeType": "hardwired" },
                            { "id": "iroute.tim1", "name": "TIM1 route", "sourceRef": "isrc.tim1", "interruptRef": "irq.tim1", "controllerRef": "block.pfic", "routeType": "hardwired" },
                            { "id": "iroute.adc1", "name": "ADC1 route", "sourceRef": "isrc.adc1", "interruptRef": "irq.adc1", "controllerRef": "block.pfic", "routeType": "hardwired" }
                        ]
                    },
                    "dmaTopology": {
                        "channels": [
                            { "id": "dma.chan1", "name": "DMA1 Channel 1", "controllerRef": "block.dma1", "channelIndex": 1 },
                            { "id": "dma.chan2", "name": "DMA1 Channel 2", "controllerRef": "block.dma1", "channelIndex": 2 }
                        ],
                        "routes": [
                            { "id": "dmaroute.usart1_tx", "name": "USART1 TX DMA", "peripheralRef": "periph.usart1", "channelRef": "dma.chan1", "direction": "memory-to-peripheral" },
                            { "id": "dmaroute.usart1_rx", "name": "USART1 RX DMA", "peripheralRef": "periph.usart1", "channelRef": "dma.chan2", "direction": "peripheral-to-memory" },
                            { "id": "dmaroute.adc1", "name": "ADC1 DMA", "peripheralRef": "periph.adc1", "channelRef": "dma.chan1", "direction": "peripheral-to-memory" }
                        ]
                    },
                    "pinTopology": {
                        "routes": [
                            { "id": "pinroute.gpioa.pa0", "name": "GPIOA PA0", "pinRef": "pin.pa0", "peripheralRef": "periph.gpioa", "signal": "GPIO0", "routeType": "hardwired" },
                            { "id": "pinroute.usart1.tx", "name": "USART1 TX", "pinRef": "pin.pa9", "peripheralRef": "periph.usart1", "signal": "TX", "routeType": "hardwired" },
                            { "id": "pinroute.usart1.rx", "name": "USART1 RX", "pinRef": "pin.pa10", "peripheralRef": "periph.usart1", "signal": "RX", "routeType": "hardwired" },
                            { "id": "pinroute.spi1.sck", "name": "SPI1 SCK", "pinRef": "pin.pa5", "peripheralRef": "periph.spi1", "signal": "SCK", "routeType": "hardwired" },
                            { "id": "pinroute.spi1.mosi", "name": "SPI1 MOSI", "pinRef": "pin.pa7", "peripheralRef": "periph.spi1", "signal": "MOSI", "routeType": "hardwired" },
                            { "id": "pinroute.spi1.miso", "name": "SPI1 MISO", "pinRef": "pin.pa6", "peripheralRef": "periph.spi1", "signal": "MISO", "routeType": "hardwired" },
                            { "id": "pinroute.i2c1.scl", "name": "I2C1 SCL", "pinRef": "pin.pa1", "peripheralRef": "periph.i2c1", "signal": "SCL", "routeType": "hardwired" },
                            { "id": "pinroute.i2c1.sda", "name": "I2C1 SDA", "pinRef": "pin.pa2", "peripheralRef": "periph.i2c1", "signal": "SDA", "routeType": "hardwired" },
                            { "id": "pinroute.tim1.ch1", "name": "TIM1 CH1", "pinRef": "pin.pa8", "peripheralRef": "periph.tim1", "signal": "CH1", "routeType": "hardwired" },
                            { "id": "pinroute.pwm1.out", "name": "PWM1 OUT", "pinRef": "pin.pa3", "peripheralRef": "periph.pwm1", "signal": "PWM_OUT", "routeType": "hardwired" },
                            { "id": "pinroute.adc1.in0", "name": "ADC1 IN0", "pinRef": "pin.pa0", "peripheralRef": "periph.adc1", "signal": "IN0", "routeType": "hardwired" }
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
                        { "id": "drv.gpioa", "name": "GpioA", "targetRef": "block.gpioa", "driverKind": "gpio-port", "modulePath": "gpio", "pinRoles": [{ "role": "gpio0", "signal": "GPIO0", "routeRefs": ["pinroute.gpioa.pa0"], "requirement": "required" }] },
                        { "id": "drv.usart1", "name": "Usart1", "targetRef": "block.usart1", "driverKind": uart_driver_kind, "modulePath": "uart", "clockBindingRefs": ["clk.usart1"], "resetBindingRefs": ["rst.usart1"], "interruptRouteRefs": ["iroute.usart1"], "dmaRouteRefs": ["dmaroute.usart1_tx", "dmaroute.usart1_rx"], "pinRoles": [{ "role": "tx", "signal": "TX", "routeRefs": ["pinroute.usart1.tx"], "requirement": "required" }, { "role": "rx", "signal": "RX", "routeRefs": ["pinroute.usart1.rx"], "requirement": "required" }] },
                        { "id": "drv.spi1", "name": "Spi1", "targetRef": "block.spi1", "driverKind": "spi", "modulePath": "spi", "clockBindingRefs": ["clk.spi1"], "resetBindingRefs": ["rst.spi1"], "interruptRouteRefs": ["iroute.spi1"], "pinRoles": [{ "role": "sck", "signal": "SCK", "routeRefs": ["pinroute.spi1.sck"], "requirement": "required" }, { "role": "mosi", "signal": "MOSI", "routeRefs": ["pinroute.spi1.mosi"], "requirement": "required" }, { "role": "miso", "signal": "MISO", "routeRefs": ["pinroute.spi1.miso"], "requirement": "required" }] },
                        { "id": "drv.i2c1", "name": "I2c1", "targetRef": "block.i2c1", "driverKind": "i2c", "modulePath": "i2c", "clockBindingRefs": ["clk.i2c1"], "resetBindingRefs": ["rst.i2c1"], "interruptRouteRefs": ["iroute.i2c1"], "pinRoles": [{ "role": "scl", "signal": "SCL", "routeRefs": ["pinroute.i2c1.scl"], "requirement": "required" }, { "role": "sda", "signal": "SDA", "routeRefs": ["pinroute.i2c1.sda"], "requirement": "required" }] },
                        { "id": "drv.tim1", "name": "Tim1", "targetRef": "block.tim1", "driverKind": "timer", "modulePath": "timer", "clockBindingRefs": ["clk.tim1"], "resetBindingRefs": ["rst.tim1"], "interruptRouteRefs": ["iroute.tim1"], "pinRoles": [{ "role": "ch1", "signal": "CH1", "routeRefs": ["pinroute.tim1.ch1"], "requirement": "required" }], "initOperationRefs": ["op.tim1.enable"], "stateMachineRefs": ["sm.tim1"] },
                        { "id": "drv.pwm1", "name": "Pwm1", "targetRef": "block.pwm1", "driverKind": "pwm", "modulePath": "pwm", "clockBindingRefs": ["clk.pwm1"], "resetBindingRefs": ["rst.pwm1"], "pinRoles": [{ "role": "out", "signal": "PWM_OUT", "routeRefs": ["pinroute.pwm1.out"], "requirement": "required" }], "initOperationRefs": ["op.pwm1.enable"], "stateMachineRefs": ["sm.pwm1"] },
                        { "id": "drv.adc1", "name": "Adc1", "targetRef": "block.adc1", "driverKind": "adc", "modulePath": "adc", "clockBindingRefs": ["clk.adc1"], "resetBindingRefs": ["rst.adc1"], "interruptRouteRefs": ["iroute.adc1"], "dmaRouteRefs": ["dmaroute.adc1"], "pinRoles": [{ "role": "input0", "signal": "IN0", "routeRefs": ["pinroute.adc1.in0"], "requirement": "required" }], "initOperationRefs": ["op.adc.calibrate"] },
                        { "id": "drv.dma1", "name": "Dma1", "targetRef": "block.dma1", "driverKind": "dma", "modulePath": "dma", "dmaRouteRefs": ["dmaroute.usart1_tx", "dmaroute.usart1_rx", "dmaroute.adc1"] },
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
