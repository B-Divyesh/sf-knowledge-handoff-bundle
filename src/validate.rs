use crate::model::{ArtifactKind, Finding, Handoff, Severity};
use chrono::{NaiveDate, Utc};
use std::collections::HashSet;
use std::path::Path;
use url::Url;

pub fn validate(handoff: &Handoff, input_dir: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut ids = HashSet::new();

    if handoff.project.title.trim().is_empty() {
        error(
            &mut findings,
            "project.title",
            "Project title cannot be empty",
            None,
        );
    }
    if handoff.project.summary.trim().is_empty() {
        error(
            &mut findings,
            "project.summary",
            "Project summary cannot be empty",
            None,
        );
    }
    validate_date(
        &handoff.project.prepared_at,
        "project.prepared_at",
        None,
        &mut findings,
    );
    if let Some(date) = &handoff.project.expires_at {
        validate_expiry(date, "project.expires_at", None, &mut findings);
    }

    if handoff.sections.is_empty() {
        warning(
            &mut findings,
            "sections.empty",
            "No artifact sections are defined; add at least one source of truth",
            None,
        );
    }

    for section in &handoff.sections {
        for artifact in &section.artifacts {
            if !valid_id(&artifact.id) {
                error(
                    &mut findings,
                    "artifact.id",
                    "Artifact ID must use lowercase letters, numbers, '-' or '_'",
                    Some(&artifact.id),
                );
            }
            if !ids.insert(artifact.id.clone()) {
                error(
                    &mut findings,
                    "artifact.duplicate",
                    "Artifact ID is duplicated",
                    Some(&artifact.id),
                );
            }
            if artifact.owner.trim().is_empty() {
                error(
                    &mut findings,
                    "artifact.owner",
                    "Artifact owner cannot be empty",
                    Some(&artifact.id),
                );
            }
            if let Some(date) = &artifact.expires_at {
                validate_expiry(
                    date,
                    "artifact.expires_at",
                    Some(&artifact.id),
                    &mut findings,
                );
            }

            match artifact.kind {
                ArtifactKind::File => {
                    if artifact.url.is_some() {
                        error(
                            &mut findings,
                            "artifact.file_url",
                            "File artifact cannot also define url",
                            Some(&artifact.id),
                        );
                    }
                    match artifact.path.as_deref() {
                        None | Some("") => error(
                            &mut findings,
                            "artifact.path",
                            "File artifact requires path",
                            Some(&artifact.id),
                        ),
                        Some(path) if Path::new(path).is_absolute() => error(
                            &mut findings,
                            "artifact.absolute_path",
                            "File paths must be relative to the YAML file",
                            Some(&artifact.id),
                        ),
                        Some(path) if path.split('/').any(|part| part == "..") => error(
                            &mut findings,
                            "artifact.parent_path",
                            "File path cannot contain '..'",
                            Some(&artifact.id),
                        ),
                        Some(path) if !input_dir.join(path).is_file() => error(
                            &mut findings,
                            "artifact.missing",
                            &format!("Local file was not found: {path}"),
                            Some(&artifact.id),
                        ),
                        _ => {}
                    }
                }
                ArtifactKind::Url => {
                    if artifact.path.is_some() {
                        error(
                            &mut findings,
                            "artifact.url_path",
                            "URL artifact cannot also define path",
                            Some(&artifact.id),
                        );
                    }
                    match artifact.url.as_deref().and_then(|u| Url::parse(u).ok()) {
                        None => error(
                            &mut findings,
                            "artifact.url",
                            "URL artifact requires a valid http or https URL",
                            Some(&artifact.id),
                        ),
                        Some(url) if !matches!(url.scheme(), "http" | "https") => error(
                            &mut findings,
                            "artifact.url_scheme",
                            "Only http and https URLs can be included",
                            Some(&artifact.id),
                        ),
                        Some(url) if !url.username().is_empty() || url.password().is_some() => {
                            error(
                                &mut findings,
                                "artifact.url_credentials",
                                "URL contains credentials; remove them before building",
                                Some(&artifact.id),
                            )
                        }
                        Some(url) if url.query_pairs().any(|(key, _)| looks_secret(&key)) => error(
                            &mut findings,
                            "artifact.url_secret",
                            "URL query contains a credential-like key; use a safe public URL",
                            Some(&artifact.id),
                        ),
                        _ => {}
                    }
                }
            }
        }
    }

    for gap in &handoff.gaps {
        if !ids.insert(gap.id.clone()) {
            error(
                &mut findings,
                "gap.duplicate",
                "Gap ID is duplicated",
                Some(&gap.id),
            );
        }
        if gap.next_step.trim().is_empty() {
            error(
                &mut findings,
                "gap.next_step",
                "Gap requires a next step",
                Some(&gap.id),
            );
        }
    }

    if handoff.gaps.is_empty() {
        findings.push(Finding {
            severity: Severity::Info,
            code: "gaps.none".into(),
            message: "No unresolved gaps were declared".into(),
            artifact_id: None,
        });
    }
    findings
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

fn looks_secret(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    ["token", "key", "secret", "password", "signature", "sig"]
        .iter()
        .any(|word| key.contains(word))
}

fn validate_date(value: &str, code: &str, id: Option<&str>, findings: &mut Vec<Finding>) {
    if NaiveDate::parse_from_str(value, "%Y-%m-%d").is_err() {
        error(findings, code, "Date must use YYYY-MM-DD", id);
    }
}

fn validate_expiry(value: &str, code: &str, id: Option<&str>, findings: &mut Vec<Finding>) {
    match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        Err(_) => error(findings, code, "Date must use YYYY-MM-DD", id),
        Ok(date) if date < Utc::now().date_naive() => {
            warning(findings, "expiry.past", &format!("Expired on {value}"), id)
        }
        Ok(date) if date <= Utc::now().date_naive() + chrono::Duration::days(30) => warning(
            findings,
            "expiry.soon",
            &format!("Expires soon on {value}"),
            id,
        ),
        _ => {}
    }
}

fn error(findings: &mut Vec<Finding>, code: &str, message: &str, id: Option<&str>) {
    findings.push(Finding {
        severity: Severity::Error,
        code: code.into(),
        message: message.into(),
        artifact_id: id.map(str::to_owned),
    });
}

fn warning(findings: &mut Vec<Finding>, code: &str, message: &str, id: Option<&str>) {
    findings.push(Finding {
        severity: Severity::Warning,
        code: code.into(),
        message: message.into(),
        artifact_id: id.map(str::to_owned),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_secret_urls_and_parent_paths() {
        let yaml = r#"
project:
  title: Test
  summary: A summary
  owner: { name: Owner }
  prepared_at: 2026-08-27
sections:
  - title: Sources
    artifacts:
      - { id: source, title: Source, kind: url, url: "https://example.test/x?api_token=nope", owner: Owner }
      - { id: file, title: File, kind: file, path: "../secret", owner: Owner }
"#;
        let handoff: Handoff = serde_yaml::from_str(yaml).unwrap();
        let findings = validate(&handoff, Path::new("."));
        assert!(findings.iter().any(|f| f.code == "artifact.url_secret"));
        assert!(findings.iter().any(|f| f.code == "artifact.parent_path"));
    }
}
