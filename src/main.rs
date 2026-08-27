mod bundle;
mod linkcheck;
mod model;
mod validate;

use bundle::{build_bundle, hex_digest};
use chrono::Utc;
use clap::{Parser, Subcommand};
use model::{Acknowledgement, Handoff, Manifest, Severity};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const STARTER: &str = r#"project:
  title: Project name
  summary: What the next owner is taking over and why it matters.
  owner:
    name: Departing owner
    contact: owner@example.test
  prepared_at: 2026-08-27
  expires_at: 2026-11-27
sections:
  - title: Start here
    artifacts:
      - id: readme
        title: Project overview
        kind: file
        path: README.md
        owner: Project team
        required: true
        note: Read this first.
      - id: status
        title: Current project status
        kind: url
        url: https://example.com/status
        owner: Departing owner
        expires_at: 2026-09-30
gaps:
  - id: access
    title: Recipient access is not confirmed
    owner: Operations
    next_step: Confirm the recipient can open each required system.
"#;

#[derive(Parser)]
#[command(
    name = "khb",
    version,
    about = "Build a portable, verifiable project handoff",
    long_about = "Turn a YAML checklist and its local files/public URLs into a static handoff site with hashes, owners, expiry warnings, known gaps, and recipient acknowledgement.",
    after_help = "Exit codes: 0 success, 2 invalid input/CI warning, 3 link failure, 4 filesystem/build failure.\nDocs: https://knowledge-handoff-bundle.sociobot.in"
)]
struct Cli {
    /// Print machine-readable JSON only
    #[arg(long, global = true)]
    json: bool,
    /// CI mode: no decoration and warnings cause a failing exit code
    #[arg(long, global = true)]
    ci: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Write an annotated starter YAML file
    Init {
        #[arg(default_value = "handoff.yaml")]
        file: PathBuf,
    },
    /// Validate YAML, local files, dates, and optionally public links
    Check {
        /// YAML handoff checklist to validate
        file: PathBuf,
        /// Check public URLs (rate-limited and robots-aware)
        #[arg(long)]
        check_links: bool,
    },
    /// Build the static handoff directory
    Build {
        /// YAML handoff checklist to build
        file: PathBuf,
        /// Output directory for the portable bundle
        #[arg(short, long, default_value = "handoff-bundle")]
        output: PathBuf,
        /// Check public links during the build
        #[arg(long)]
        check_links: bool,
        /// Replace a non-empty output directory
        #[arg(long)]
        force: bool,
    },
    /// Export a recipient acknowledgement tied to a manifest hash
    Acknowledge {
        /// Generated bundle manifest.json
        manifest: PathBuf,
        /// Recipient's full name
        #[arg(long)]
        recipient: String,
        /// Artifact ID reviewed by the recipient; repeat for multiple IDs
        #[arg(long = "accept")]
        accepted: Vec<String>,
        /// Optional unresolved access or context note
        #[arg(long)]
        note: Option<String>,
        /// JSON acknowledgement destination
        #[arg(short, long, default_value = "acknowledgement.json")]
        output: PathBuf,
    },
}

#[derive(Serialize)]
struct CommandResult<T: Serialize> {
    ok: bool,
    command: &'static str,
    result: T,
}

#[derive(Serialize)]
struct ErrorResult<'a> {
    ok: bool,
    error: &'a str,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(code) => ExitCode::from(code),
        Err((code, message)) => {
            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string(&ErrorResult {
                        ok: false,
                        error: &message
                    })
                    .unwrap()
                );
            } else {
                eprintln!("khb: {message}");
            }
            ExitCode::from(code)
        }
    }
}

fn run(cli: &Cli) -> Result<u8, (u8, String)> {
    match &cli.command {
        Command::Init { file } => {
            if file.exists() {
                return Err((
                    4,
                    format!("{} already exists; nothing was overwritten", file.display()),
                ));
            }
            fs::write(file, STARTER).map_err(io_error)?;
            emit(cli.json, "init", true, serde_json::json!({ "file": file }))?;
            if !cli.json {
                println!(
                    "Wrote {}. Add files, owners, URLs, and known gaps, then run `khb check`. ",
                    file.display()
                );
            }
            Ok(0)
        }
        Command::Check { file, check_links } => {
            let handoff = read_handoff(file)?;
            let input_dir = parent_dir(file);
            let mut findings = validate::validate(&handoff, &input_dir);
            let schema_errors = findings.iter().any(|f| f.severity == Severity::Error);
            if !schema_errors && *check_links {
                findings.extend(linkcheck::check_links(&handoff).map_err(|e| (3, e))?);
            }
            let errors = findings
                .iter()
                .filter(|f| f.severity == Severity::Error)
                .count();
            let warnings = findings
                .iter()
                .filter(|f| f.severity == Severity::Warning)
                .count();
            if cli.json {
                emit(
                    cli.json,
                    "check",
                    errors == 0 && !(cli.ci && warnings > 0),
                    serde_json::json!({
                        "file": file,
                        "errors": errors,
                        "warnings": warnings,
                        "findings": findings
                    }),
                )?;
            } else {
                print_findings(&findings);
                println!(
                    "Checked {}: {errors} errors, {warnings} warnings.",
                    file.display()
                );
            }
            if errors > 0 {
                let only_links = findings
                    .iter()
                    .filter(|f| f.severity == Severity::Error)
                    .all(|f| f.code.starts_with("link."));
                Ok(if only_links { 3 } else { 2 })
            } else if cli.ci && warnings > 0 {
                Ok(2)
            } else {
                Ok(0)
            }
        }
        Command::Build {
            file,
            output,
            check_links,
            force,
        } => {
            let handoff = read_handoff(file)?;
            let input_dir = parent_dir(file);
            let mut findings = validate::validate(&handoff, &input_dir);
            let validation_errors = findings
                .iter()
                .filter(|f| f.severity == Severity::Error)
                .count();
            if validation_errors > 0 {
                if cli.json {
                    emit(
                        cli.json,
                        "build",
                        false,
                        serde_json::json!({ "built": false, "findings": findings }),
                    )?;
                } else {
                    print_findings(&findings);
                }
                return Ok(2);
            }
            if *check_links {
                findings.extend(linkcheck::check_links(&handoff).map_err(|e| (3, e))?);
            }
            let warning_count = findings
                .iter()
                .filter(|f| f.severity == Severity::Warning)
                .count();
            let manifest =
                build_bundle(&handoff, findings, &input_dir, output, *force).map_err(|e| (4, e))?;
            if cli.json {
                emit(
                    cli.json,
                    "build",
                    !(cli.ci && (warning_count > 0 || manifest.summary.errors > 0)),
                    serde_json::json!({
                        "output": output,
                        "summary": manifest.summary
                    }),
                )?;
            } else {
                println!(
                    "Built {} with {} artifacts and {} known gaps. Open {}/index.html.",
                    output.display(),
                    manifest.summary.artifacts,
                    manifest.summary.gaps,
                    output.display()
                );
            }
            Ok(
                if cli.ci && (warning_count > 0 || manifest.summary.errors > 0) {
                    2
                } else {
                    0
                },
            )
        }
        Command::Acknowledge {
            manifest,
            recipient,
            accepted,
            note,
            output,
        } => {
            if recipient.trim().is_empty() {
                return Err((2, "Recipient name cannot be empty".into()));
            }
            let bytes = fs::read(manifest).map_err(io_error)?;
            let parsed: Manifest = serde_json::from_slice(&bytes)
                .map_err(|e| (2, format!("Could not parse {}: {e}", manifest.display())))?;
            if parsed.format != "knowledge-handoff-bundle/1" {
                return Err((2, "Unsupported manifest format".into()));
            }
            let artifact_ids: HashSet<&str> = parsed
                .sections
                .iter()
                .flat_map(|section| {
                    section
                        .artifacts
                        .iter()
                        .map(|artifact| artifact.id.as_str())
                })
                .collect();
            let mut unique = Vec::new();
            for id in accepted {
                if !artifact_ids.contains(id.as_str()) {
                    return Err((2, format!("Unknown artifact ID: {id}")));
                }
                if !unique.contains(id) {
                    unique.push(id.clone());
                }
            }
            unique.sort();
            let ack = Acknowledgement {
                format: "knowledge-handoff-acknowledgement/1".into(),
                project: parsed.project.title,
                recipient: recipient.trim().into(),
                accepted: unique,
                note: note.clone().filter(|value| !value.trim().is_empty()),
                acknowledged_at: Utc::now().to_rfc3339(),
                manifest_sha256: hex_digest(&bytes),
            };
            let json = serde_json::to_string_pretty(&ack).map_err(|e| (4, e.to_string()))? + "\n";
            fs::write(output, json).map_err(io_error)?;
            if cli.json {
                emit(
                    cli.json,
                    "acknowledge",
                    true,
                    serde_json::json!({ "output": output, "acknowledgement": ack }),
                )?;
            } else {
                println!("Exported {} for {}.", output.display(), ack.recipient);
            }
            Ok(0)
        }
    }
}

fn read_handoff(path: &Path) -> Result<Handoff, (u8, String)> {
    let source = fs::read_to_string(path).map_err(io_error)?;
    serde_yaml::from_str(&source)
        .map_err(|e| (2, format!("Could not parse {}: {e}", path.display())))
}

fn parent_dir(path: &Path) -> PathBuf {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
}

fn io_error(error: std::io::Error) -> (u8, String) {
    (4, error.to_string())
}

fn emit<T: Serialize>(
    json: bool,
    command: &'static str,
    ok: bool,
    value: T,
) -> Result<(), (u8, String)> {
    if json {
        let output = serde_json::to_string(&CommandResult {
            ok,
            command,
            result: value,
        })
        .map_err(|e| (4, e.to_string()))?;
        println!("{output}");
    }
    Ok(())
}

fn print_findings(findings: &[model::Finding]) {
    for finding in findings {
        let marker = match finding.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARN ",
            Severity::Info => "INFO ",
        };
        let id = finding
            .artifact_id
            .as_ref()
            .map(|id| format!(" [{id}]"))
            .unwrap_or_default();
        println!("{marker}{id}: {}", finding.message);
    }
}
