use crate::bundle::build_bundle;
use crate::model::{Finding, Handoff, Manifest, Severity};
use crate::validate;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

const SAMPLE_YAML: &str = include_str!("../examples/atlas/handoff.yaml");
const SAMPLE_DECISIONS: &[u8] = include_bytes!("../examples/atlas/decisions.md");
const SAMPLE_RUNBOOK: &[u8] = include_bytes!("../examples/atlas/runbook.txt");

/// Build the shipped Atlas sample without making a network request. The demo
/// includes recorded good and bad link outcomes so it remains useful offline.
pub fn build_demo(output: Option<&Path>, force: bool) -> Result<(PathBuf, Manifest), String> {
    let root = std::env::temp_dir().join(format!(
        "knowledge-handoff-bundle-demo-{}-{}",
        std::process::id(),
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));
    let input = root.join("sample-input");
    fs::create_dir_all(&input).map_err(|error| error.to_string())?;
    fs::write(input.join("handoff.yaml"), SAMPLE_YAML).map_err(|error| error.to_string())?;
    fs::write(input.join("decisions.md"), SAMPLE_DECISIONS).map_err(|error| error.to_string())?;
    fs::write(input.join("runbook.txt"), SAMPLE_RUNBOOK).map_err(|error| error.to_string())?;

    let handoff: Handoff = serde_yaml::from_str(SAMPLE_YAML).map_err(|error| error.to_string())?;
    let mut findings = validate::validate(&handoff, &input);
    findings.extend(recorded_link_findings());
    let destination = match output {
        Some(path) => path.to_path_buf(),
        None => root.join("bundle"),
    };
    let manifest = build_bundle(&handoff, findings, &input, &destination, force)?;
    Ok((destination, manifest))
}

fn recorded_link_findings() -> Vec<Finding> {
    vec![
        Finding {
            severity: Severity::Info,
            code: "link.demo_ok".into(),
            message: "Recorded sample check: reachable (HTTP 200)".into(),
            artifact_id: Some("release-notes".into()),
        },
        Finding {
            severity: Severity::Error,
            code: "link.demo_http".into(),
            message: "Recorded sample check: HTTP 404; replace this URL".into(),
            artifact_id: Some("legacy-dashboard".into()),
        },
    ]
}
