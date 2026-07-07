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
            render_interrupt_module(&model, &driver_group)?
        } else {
            render_driver_module(&model, &module_name, &driver_group)?
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
    #[serde(default)]
    provenance: HairProvenance,
    structure: HairStructure,
    #[serde(default)]
    semantics: HairSemantics,
    #[serde(default)]
    physical: HairPhysical,
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
    peripherals: HashMap<String, Peripheral>,
    peripheral_names: HashMap<String, String>,
    registers: HashMap<String, ResolvedRegister>,
    fields: HashMap<String, ResolvedFieldTarget>,
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

            drivers.push(ResolvedDriverInstance {
                id: driver.id.clone(),
                name: driver.name.clone(),
                type_name: to_rust_type_name(&driver.name),
                driver_kind: driver.driver_kind.clone(),
                module_name,
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
            peripherals,
            peripheral_names,
            registers,
            fields,
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

fn validate_driver_semantic_scope(
    driver: &EmbassyDriverInstance,
    target: &McuCanonicalBlock,
    init_operations: &[SemanticOperation],
    state_machines: &[SemanticStateMachine],
    registers: &HashMap<String, ResolvedRegister>,
    fields: &HashMap<String, ResolvedFieldTarget>,
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
    fields: &HashMap<String, ResolvedFieldTarget>,
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
    fields: &HashMap<String, ResolvedFieldTarget>,
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
            if let Some(field_target) = fields.get(effect_target_ref) {
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
    fields: &HashMap<String, ResolvedFieldTarget>,
) -> Result<()> {
    let Some(predicate_target_ref) = predicate.target_ref.as_deref() else {
        return Ok(());
    };
    if !predicate_target_ref.starts_with("reg.") && !predicate_target_ref.starts_with("field.") {
        return Ok(());
    }
    if let Some(field_target) = fields.get(predicate_target_ref) {
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
            if requirements.clock_bindings.is_empty() && requirements.reset_bindings.is_empty() {
                bail!(
                    "gpio-port driver {} requires clock or reset bindings for first-cut executable lowering",
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
        "#[derive(Debug, Clone, Copy)]\npub struct {type_name}Resources {{\n    pub clocks: &'static [metadata::ClockBinding],\n    pub resets: &'static [metadata::ResetBinding],\n    pub interrupt_sources: &'static [metadata::InterruptSource],\n    pub interrupts: &'static [metadata::InterruptRoute],\n    pub dma_channels: &'static [metadata::DmaChannel],\n    pub dma: &'static [metadata::DmaRoute],\n    pub pins: &'static [metadata::PinRole],\n    pub init_operations: &'static [metadata::SemanticOperation],\n    pub state_machines: &'static [metadata::SemanticStateMachine],\n    pub capability_tags: &'static [&'static str],\n}}\n\npub const {const_prefix}_RESOURCES: {type_name}Resources = {type_name}Resources {{\n    clocks: {const_prefix}_CLOCK_BINDINGS,\n    resets: {const_prefix}_RESET_BINDINGS,\n    interrupt_sources: {const_prefix}_INTERRUPT_SOURCES,\n    interrupts: {const_prefix}_INTERRUPT_ROUTES,\n    dma_channels: {const_prefix}_DMA_CHANNELS,\n    dma: {const_prefix}_DMA_ROUTES,\n    pins: {const_prefix}_PIN_ROLES,\n    init_operations: {const_prefix}_INIT_OPERATIONS,\n    state_machines: {const_prefix}_STATE_MACHINES,\n    capability_tags: {const_prefix}_CAPABILITY_TAGS,\n}};\n\n#[derive(Debug, Clone, Copy)]\npub struct {type_name} {{\n    resources: {type_name}Resources,\n}}\n\nimpl {type_name} {{\n    pub fn new(resources: {type_name}Resources) -> Result<Self, metadata::Error> {{\n        Ok(Self {{ resources }})\n    }}\n\n    pub fn resources(&self) -> {type_name}Resources {{\n        self.resources\n    }}\n{methods}\n}}\n\n",
        type_name = driver.type_name,
        const_prefix = const_prefix,
        methods = render_driver_methods(model, driver)?
    ));
    Ok(out)
}

fn render_driver_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<String> {
    if driver.driver_kind == "interrupt" {
        return Ok(
            "    pub fn bind(&self) -> &'static [metadata::InterruptRoute] {\n        self.resources.interrupts\n    }\n"
                .to_string(),
        );
    }

    let mut methods = Vec::new();
    methods.extend(render_clock_binding_methods(model, driver)?);
    methods.extend(render_reset_binding_methods(model, driver)?);
    methods.extend(render_usart_methods(model, driver)?);
    methods.extend(render_operation_methods(model, driver)?);
    methods.extend(render_state_machine_methods(model, driver)?);

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

fn render_mmio_helpers() -> &'static str {
    "#[allow(dead_code)]\nfn checked_address(address: u64, align: usize) -> Result<usize, metadata::Error> {\n    let address = usize::try_from(address)\n        .map_err(|_| metadata::Error::Unsupported(\"MMIO address does not fit usize on this target\"))?;\n    if address % align != 0 {\n        return Err(metadata::Error::Unsupported(\"MMIO address is not naturally aligned for the target register width\"));\n    }\n    Ok(address)\n}\n\n#[allow(dead_code)]\nfn modify_u8(address: u64, clear_mask: u8, set_mask: u8) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u8>())?;\n    unsafe {\n        let current = read_volatile(address as *const u8);\n        write_volatile(address as *mut u8, (current & !clear_mask) | set_mask);\n    }\n    Ok(())\n}\n\n#[allow(dead_code)]\nfn modify_u16(address: u64, clear_mask: u16, set_mask: u16) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u16>())?;\n    unsafe {\n        let current = read_volatile(address as *const u16);\n        write_volatile(address as *mut u16, (current & !clear_mask) | set_mask);\n    }\n    Ok(())\n}\n\n#[allow(dead_code)]\nfn modify_u32(address: u64, clear_mask: u32, set_mask: u32) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u32>())?;\n    unsafe {\n        let current = read_volatile(address as *const u32);\n        write_volatile(address as *mut u32, (current & !clear_mask) | set_mask);\n    }\n    Ok(())\n}\n\n#[allow(dead_code)]\nfn read_u32(address: u64) -> Result<u32, metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u32>())?;\n    unsafe { Ok(read_volatile(address as *const u32)) }\n}\n\n#[allow(dead_code)]\nfn write_u32(address: u64, value: u32) -> Result<(), metadata::Error> {\n    let address = checked_address(address, core::mem::align_of::<u32>())?;\n    unsafe {\n        write_volatile(address as *mut u32, value);\n    }\n    Ok(())\n}\n"
}

fn render_clock_binding_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
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
            let register = resolve_control_register(
                model,
                &binding.control_refs,
                "clock binding",
                &binding.id,
            )?;
            let field = resolve_control_field(
                register,
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
            let register = resolve_control_register(
                model,
                &binding.control_refs,
                "reset binding",
                &binding.id,
            )?;
            let field = resolve_control_field(
                register,
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

fn render_usart_methods(
    model: &EmbassyGenerationModel,
    driver: &ResolvedDriverInstance,
) -> Result<Vec<GeneratedMethod>> {
    if !matches!(driver.driver_kind.as_str(), "uart" | "usart") {
        return Ok(Vec::new());
    }

    let target_ref = driver.target.target_ref.as_str();
    let has_tx = driver.pin_roles.iter().any(|role| role.signal == "TX");
    let has_rx = driver.pin_roles.iter().any(|role| role.signal == "RX");
    let has_irq = !driver.interrupt_routes.is_empty();
    let has_tx_dma = driver
        .dma_routes
        .iter()
        .any(|route| route.signal.as_deref() == Some("TX"));
    let has_rx_dma = driver
        .dma_routes
        .iter()
        .any(|route| route.signal.as_deref() == Some("RX"));

    let Some(cr1) = try_resolve_target_register_by_name(model, target_ref, "CR1")? else {
        return Ok(Vec::new());
    };
    let brr = resolve_target_register_by_name(model, target_ref, "BRR")?;
    let cr2 = resolve_target_register_by_name(model, target_ref, "CR2")?;

    let div_mantissa = resolve_register_field(&brr, "DIV_Mantissa")?;
    let div_fraction = resolve_register_field(&brr, "DIV_Fraction")?;
    let ue = resolve_register_field(&cr1, "UE")?;
    let te = resolve_register_field(&cr1, "TE")?;
    let re = resolve_register_field(&cr1, "RE")?;
    let over8 = resolve_register_field(&cr1, "OVER8")?;
    let word_length = resolve_register_field(&cr1, "M")?;
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
            "    pub fn set_baud_divider(&self, mantissa: u16, fraction: u8) -> Result<(), metadata::Error> {{\n        if u32::from(mantissa) > 0x{mantissa_value_mask:X}u32 {{\n            return Err(metadata::Error::Unsupported(\"USART baud mantissa exceeds modeled field width\"));\n        }}\n        if u32::from(fraction) > 0x{fraction_value_mask:X}u32 {{\n            return Err(metadata::Error::Unsupported(\"USART baud fraction exceeds modeled field width\"));\n        }}\n        modify_u32(0x{brr_address:X}u64, 0x{mantissa_clear_mask:08X}u32, (u32::from(mantissa) & 0x{mantissa_value_mask:X}u32) << {mantissa_lsb})?;\n        modify_u32(0x{brr_address:X}u64, 0x{fraction_clear_mask:08X}u32, (u32::from(fraction) & 0x{fraction_value_mask:X}u32) << {fraction_lsb})?;\n        Ok(())\n    }}\n",
            brr_address = brr.absolute_address,
            mantissa_lsb = div_mantissa.lsb,
            fraction_lsb = div_fraction.lsb,
        ),
    });

    if has_tx {
        let sr = resolve_target_register_by_name(model, target_ref, "SR")?;
        let dr = resolve_target_register_by_name(model, target_ref, "DR")?;
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
        let sr = resolve_target_register_by_name(model, target_ref, "SR")?;
        let dr = resolve_target_register_by_name(model, target_ref, "DR")?;
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
        let cr3 = resolve_target_register_by_name(model, target_ref, "CR3")?;
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
        let cr3 = resolve_target_register_by_name(model, target_ref, "CR3")?;
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
    resolve_target_register_from_template(target, target_base, template, register_name)
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
    resolve_target_register_from_template(target, target_base, template, register_name).map(Some)
}

fn resolve_target_register_from_template(
    target: &Peripheral,
    target_base: u64,
    template: &Peripheral,
    register_name: &str,
) -> Result<ResolvedRegister> {
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
        bail!(
            "peripheral {} does not expose register {} through its explicit structural template",
            target.id,
            register_name
        );
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
    Ok(resolved)
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
    let field_target = model.fields.get(effect_target_ref).ok_or_else(|| {
        anyhow!(
            "state machine {} transition {} -> {} on driver {} requires effect targetRef {} to resolve to a field for first-cut lowering",
            state_machine.id,
            transition.from,
            transition.to,
            driver.id,
            effect_target_ref
        )
    })?;
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

fn resolve_control_register<'a>(
    model: &'a EmbassyGenerationModel,
    control_refs: &[String],
    kind: &str,
    owner_id: &str,
) -> Result<&'a ResolvedRegister> {
    let control_ref = control_refs.first().ok_or_else(|| {
        anyhow!("{kind} {owner_id} is missing controlRefs required for executable lowering")
    })?;
    if control_refs.len() > 1 {
        bail!("{kind} {owner_id} references multiple controlRefs, which is not yet supported");
    }
    model.registers.get(control_ref).ok_or_else(|| {
        anyhow!("{kind} {owner_id} references unknown control register {control_ref}")
    })
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
            "field {} bit range {}..={} exceeds register {} width {}",
            field.name,
            field.lsb,
            field.msb,
            register.id,
            register.width_bits
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
) -> Result<HashMap<String, ResolvedFieldTarget>> {
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
            if fields.insert(field.id.clone(), resolved).is_some() {
                bail!("duplicate field {} is not supported", field.id);
            }
        }
    }
    Ok(fields)
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

        assert!(rcc_rs.contains("pub fn enable_usart1_clock"));
        assert!(rcc_rs.contains("pub fn apply_init"));
        assert!(gpio_rs.contains("pub fn enable_clock"));
        assert!(gpio_rs.contains("pub fn assert_reset"));
        assert!(uart_rs.contains("pub fn enable_clock"));
        assert!(uart_rs.contains("pub fn release_reset"));
        assert!(uart_rs.contains("pub fn configure_8n1"));
        assert!(uart_rs.contains("pub fn set_baud_divider"));
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
        assert!(timer_rs.contains("pub fn apply_enable"));
        assert!(timer_rs.contains("pub fn transition_disabled_to_enabled"));
        assert!(timer_rs.contains("modify_u16("));
        assert!(pwm_rs.contains("pub fn transition_enabled_to_disabled"));
        assert!(adc_rs.contains("pub fn apply_calibrate"));
        assert!(adc_rs.contains("modify_u32("));
        assert!(dma_rs.contains("pub fn enable_clock"));
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
        let tim1 = peripherals[5].as_object_mut().expect("tim1 peripheral");
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
        let tim1 = peripherals[5].as_object_mut().expect("tim1 peripheral");
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
        let tim1 = peripherals[5].as_object_mut().expect("tim1 peripheral");
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
        let tim1 = peripherals[5].as_object_mut().expect("tim1 peripheral");
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
        let tim1 = peripherals[5].as_object_mut().expect("tim1 peripheral");
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
        let tim1 = peripherals[5].as_object_mut().expect("tim1 peripheral");
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
        assert!(
            metadata_rs.contains("//! Generated Embassy-style HAL metadata for Fixture Title ")
        );
        assert!(uart_rs.contains("//! Generated Embassy-style uart module for FIXTURE DEVICE "));
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
                        { "id": "periph.gpioa", "name": "GPIOA", "kind": "peripheral", "type": "GPIO" },
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
                        { "id": "periph.spi1", "name": "SPI1", "kind": "peripheral", "type": "SPI", "interruptRefs": ["irq.spi1"] },
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
                            { "id": "dmaroute.adc1", "name": "ADC1 DMA", "peripheralRef": "periph.adc1", "signal": "IN0", "channelRef": "dma.chan1", "direction": "peripheral-to-memory", "controlRefs": ["reg.rcc.ahbpcenr"] }
                        ]
                    },
                    "pinTopology": {
                        "routes": [
                            { "id": "pinroute.gpioa.pa0", "name": "GPIOA PA0", "pinRef": "pin.pa0", "peripheralRef": "periph.gpioa", "signal": "GPIO0", "routeType": "hardwired", "defaultAfterReset": true },
                            { "id": "pinroute.usart1.tx", "name": "USART1 TX", "pinRef": "pin.pa9", "peripheralRef": "periph.usart1", "signal": "TX", "routeType": "hardwired", "controlRefs": ["reg.rcc.apb2pcenr"] },
                            { "id": "pinroute.usart1.rx", "name": "USART1 RX", "pinRef": "pin.pa10", "peripheralRef": "periph.usart1", "signal": "RX", "routeType": "hardwired", "defaultAfterReset": true },
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
                        { "id": "drv.spi1", "name": "Spi1", "targetRef": "block.spi1", "driverKind": "spi", "modulePath": "spi", "clockBindingRefs": ["clk.spi1"], "resetBindingRefs": ["rst.spi1"], "interruptRouteRefs": ["iroute.spi1"], "pinRoles": [{ "role": "sck", "signal": "SCK", "routeRefs": ["pinroute.spi1.sck"], "requirement": "required" }, { "role": "mosi", "signal": "MOSI", "routeRefs": ["pinroute.spi1.mosi"], "requirement": "required" }, { "role": "miso", "signal": "MISO", "routeRefs": ["pinroute.spi1.miso"], "requirement": "required" }] },
                        { "id": "drv.i2c1", "name": "I2c1", "targetRef": "block.i2c1", "driverKind": "i2c", "modulePath": "i2c", "clockBindingRefs": ["clk.i2c1"], "resetBindingRefs": ["rst.i2c1"], "interruptRouteRefs": ["iroute.i2c1"], "pinRoles": [{ "role": "scl", "signal": "SCL", "routeRefs": ["pinroute.i2c1.scl"], "requirement": "required" }, { "role": "sda", "signal": "SDA", "routeRefs": ["pinroute.i2c1.sda"], "requirement": "required" }] },
                        { "id": "drv.tim1", "name": "Tim1", "targetRef": "block.tim1", "driverKind": "timer", "modulePath": "timer", "clockBindingRefs": ["clk.tim1"], "resetBindingRefs": ["rst.tim1"], "interruptRouteRefs": ["iroute.tim1"], "pinRoles": [{ "role": "ch1", "signal": "CH1", "routeRefs": ["pinroute.tim1.ch1"], "requirement": "required" }], "initOperationRefs": ["op.tim1.enable"], "stateMachineRefs": ["sm.tim1"] },
                        { "id": "drv.pwm1", "name": "Pwm1", "targetRef": "block.pwm1", "driverKind": "pwm", "modulePath": "pwm", "clockBindingRefs": ["clk.pwm1"], "resetBindingRefs": ["rst.pwm1"], "pinRoles": [{ "role": "out", "signal": "PWM_OUT", "routeRefs": ["pinroute.pwm1.out"], "requirement": "required" }], "initOperationRefs": ["op.pwm1.enable"], "stateMachineRefs": ["sm.pwm1"] },
                        { "id": "drv.adc1", "name": "Adc1", "targetRef": "block.adc1", "driverKind": "adc", "modulePath": "adc", "clockBindingRefs": ["clk.adc1"], "resetBindingRefs": ["rst.adc1"], "interruptRouteRefs": ["iroute.adc1"], "dmaRouteRefs": ["dmaroute.adc1"], "pinRoles": [{ "role": "input0", "signal": "IN0", "routeRefs": ["pinroute.adc1.in0"], "requirement": "required" }], "initOperationRefs": ["op.adc.calibrate"] },
                        { "id": "drv.dma1", "name": "Dma1", "targetRef": "block.dma1", "driverKind": "dma", "modulePath": "dma", "clockBindingRefs": ["clk.dma1"], "dmaRouteRefs": ["dmaroute.usart1_tx", "dmaroute.usart1_rx", "dmaroute.adc1"] },
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
