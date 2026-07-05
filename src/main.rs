use std::collections::{BTreeSet, HashMap};
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

#[derive(Debug, Deserialize)]
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
    name: String,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArrayShape {
    axes: Vec<ArrayAxis>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArrayAxis {
    count: u32,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    stride_bytes: Option<u64>,
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
        write_peripheral(&mut xml, peripheral, &peripherals, &interrupts)?;
    }
    xml.end_element();
    xml.end_element();

    Ok(xml.end_document())
}

fn write_peripheral(
    xml: &mut XmlWriter,
    peripheral: &Peripheral,
    peripherals: &HashMap<&str, &Peripheral>,
    interrupts: &HashMap<&str, &Interrupt>,
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
        xml.start_element("interrupt");
        write_text_element(xml, "name", &interrupt.name);
        if let Some(description) = &interrupt.description {
            write_text_element(xml, "description", description);
        }
        write_text_element(xml, "value", &interrupt.number.to_string());
        xml.end_element();
    }

    if !peripheral.registers.is_empty() {
        xml.start_element("registers");
        for member in &peripheral.registers {
            write_register_member(xml, member)?;
        }
        xml.end_element();
    }

    xml.end_element();
    Ok(())
}

fn write_register_member(xml: &mut XmlWriter, member: &RegisterBlockMember) -> Result<()> {
    match member {
        RegisterBlockMember::Register(register) => write_register(xml, register),
        RegisterBlockMember::Cluster(cluster) => write_cluster(xml, cluster),
    }
}

fn write_register(xml: &mut XmlWriter, register: &Register) -> Result<()> {
    xml.start_element("register");
    write_array_shape(xml, &register.name, register.array.as_ref())?;
    if let Some(description) = &register.description {
        write_text_element(xml, "description", description);
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
    xml.start_element("cluster");
    write_array_shape(xml, &cluster.name, cluster.array.as_ref())?;
    if let Some(description) = &cluster.description {
        write_text_element(xml, "description", description);
    }
    write_text_element(xml, "addressOffset", &format_hex(cluster.offset_bytes));
    for member in &cluster.members {
        write_register_member(xml, member)?;
    }
    xml.end_element();
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
    match array {
        None => {
            write_text_element(xml, "name", name);
        }
        Some(shape) => {
            if shape.axes.len() != 1 {
                bail!("multi-axis arrays are not supported in first-cut SVD generation");
            }
            let axis = &shape.axes[0];
            let stride_bytes = axis
                .stride_bytes
                .ok_or_else(|| anyhow!("arrayed item {} is missing strideBytes", name))?;
            write_text_element(xml, "name", &format!("{name}%s"));
            write_text_element(xml, "dim", &axis.count.to_string());
            write_text_element(xml, "dimIncrement", &format_hex(stride_bytes));
            if !axis.labels.is_empty() {
                write_text_element(xml, "dimIndex", &axis.labels.join(","));
            }
        }
    }
    Ok(())
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
}
