use crate::model::{
    ArtifactKind, Finding, Handoff, Manifest, ManifestArtifact, ManifestSection, ManifestSummary,
    Severity,
};
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const BUNDLE_CSS: &str = include_str!("../templates/bundle.css");
const BUNDLE_JS: &str = include_str!("../templates/bundle.js");

pub fn build_bundle(
    handoff: &Handoff,
    findings: Vec<Finding>,
    input_dir: &Path,
    output: &Path,
    force: bool,
) -> Result<Manifest, String> {
    prepare_output(output, input_dir, force)?;
    fs::create_dir_all(output.join("assets")).map_err(|e| e.to_string())?;
    fs::create_dir_all(output.join("files")).map_err(|e| e.to_string())?;

    let mut sections = Vec::new();
    for section in &handoff.sections {
        let mut artifacts = Vec::new();
        for artifact in &section.artifacts {
            let artifact_findings: Vec<&Finding> = findings
                .iter()
                .filter(|f| f.artifact_id.as_deref() == Some(&artifact.id))
                .collect();
            let (status, detail) = artifact_status(&artifact.kind, &artifact_findings);
            let mut rendered = ManifestArtifact {
                id: artifact.id.clone(),
                title: artifact.title.clone(),
                kind: artifact.kind.clone(),
                owner: artifact.owner.clone(),
                required: artifact.required,
                note: artifact.note.clone(),
                expires_at: artifact.expires_at.clone(),
                href: artifact.url.clone(),
                sha256: None,
                bytes: None,
                status,
                status_detail: detail,
            };
            if matches!(artifact.kind, ArtifactKind::File) {
                if let Some(relative) = &artifact.path {
                    let source = input_dir.join(relative);
                    if source.is_file() {
                        let bytes = fs::read(&source).map_err(|e| e.to_string())?;
                        let basename = source
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("artifact");
                        let safe_name = sanitize_filename(basename);
                        let bundle_path = format!("files/{}-{safe_name}", artifact.id);
                        fs::write(output.join(&bundle_path), &bytes).map_err(|e| e.to_string())?;
                        rendered.href = Some(bundle_path);
                        rendered.sha256 = Some(hex_digest(&bytes));
                        rendered.bytes = Some(bytes.len() as u64);
                    }
                }
            }
            artifacts.push(rendered);
        }
        sections.push(ManifestSection {
            title: section.title.clone(),
            artifacts,
        });
    }

    let all_artifacts: Vec<&ManifestArtifact> = sections
        .iter()
        .flat_map(|section| section.artifacts.iter())
        .collect();
    let summary = ManifestSummary {
        artifacts: all_artifacts.len(),
        required: all_artifacts.iter().filter(|a| a.required).count(),
        verified: all_artifacts
            .iter()
            .filter(|a| a.status == "verified")
            .count(),
        warnings: findings
            .iter()
            .filter(|f| f.severity == Severity::Warning)
            .count(),
        errors: findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .count(),
        gaps: handoff.gaps.len(),
    };
    let manifest = Manifest {
        format: "knowledge-handoff-bundle/1".into(),
        generated_at: Utc::now().to_rfc3339(),
        project: handoff.project.clone(),
        sections,
        gaps: handoff.gaps.clone(),
        findings,
        summary,
    };
    let manifest_json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    fs::write(output.join("manifest.json"), &manifest_json).map_err(|e| e.to_string())?;
    let manifest_hash = hex_digest(manifest_json.as_bytes());
    let html = render_html(&manifest, &manifest_hash)?;
    fs::write(output.join("index.html"), html).map_err(|e| e.to_string())?;
    fs::write(output.join("assets/bundle.css"), BUNDLE_CSS).map_err(|e| e.to_string())?;
    fs::write(output.join("assets/bundle.js"), BUNDLE_JS).map_err(|e| e.to_string())?;
    Ok(manifest)
}

pub fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn prepare_output(output: &Path, input_dir: &Path, force: bool) -> Result<(), String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let absolute = if output.is_absolute() {
        output.to_path_buf()
    } else {
        cwd.join(output)
    };
    let resolved_output = if output.exists() {
        fs::canonicalize(output).map_err(|e| e.to_string())?
    } else {
        absolute.clone()
    };
    let resolved_input = fs::canonicalize(input_dir).map_err(|e| e.to_string())?;
    let dangerous = resolved_output.parent().is_none()
        || resolved_output == cwd
        || resolved_output == resolved_input;
    if dangerous {
        return Err(
            "Refusing to use a repository, input directory, or filesystem root as output".into(),
        );
    }
    if output.exists() {
        let non_empty = output
            .read_dir()
            .map_err(|e| e.to_string())?
            .next()
            .is_some();
        if non_empty && !force {
            return Err("Output directory is not empty; pass --force to replace it".into());
        }
        if force {
            fs::remove_dir_all(output).map_err(|e| e.to_string())?;
        }
    }
    fs::create_dir_all(output).map_err(|e| e.to_string())
}

fn artifact_status(kind: &ArtifactKind, findings: &[&Finding]) -> (String, String) {
    if let Some(finding) = findings.iter().find(|f| f.severity == Severity::Error) {
        return ("broken".into(), finding.message.clone());
    }
    if let Some(finding) = findings.iter().find(|f| f.severity == Severity::Warning) {
        return ("warning".into(), finding.message.clone());
    }
    match kind {
        ArtifactKind::File => ("verified".into(), "Copied and SHA-256 hashed".into()),
        ArtifactKind::Url if findings.iter().any(|f| f.code == "link.ok") => {
            ("verified".into(), "Public link reached during build".into())
        }
        ArtifactKind::Url => ("unchecked".into(), "Link check was not requested".into()),
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') {
                c
            } else {
                '-'
            }
        })
        .collect()
}

fn render_html(manifest: &Manifest, manifest_hash: &str) -> Result<String, String> {
    let data = serde_json::to_string(manifest)
        .map_err(|e| e.to_string())?
        .replace('<', "\\u003c");
    let title = escape(&manifest.project.title);
    Ok(format!(
        r##"<!doctype html>
<html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="description" content="Portable project handoff for {title}"><title>{title} — Knowledge handoff</title><link rel="stylesheet" href="assets/bundle.css"><script defer src="assets/bundle.js"></script></head>
<body><a class="skip" href="#main">Skip to handoff</a><header class="tape-head"><span>KHB // recipient copy</span><span>Generated {date}</span></header>
<main id="main"><section class="cover"><p class="eyebrow">Project handoff · source-of-truth bundle</p><h1>{title}</h1><p class="summary">{summary}</p><dl class="owner"><div><dt>Prepared by</dt><dd>{owner}</dd></div><div><dt>Prepared</dt><dd>{prepared}</dd></div><div><dt>Bundle health</dt><dd>{health}</dd></div></dl></section>
<section aria-labelledby="contents"><div class="section-title"><p>Side A</p><h2 id="contents">Artifact tracklist</h2></div><div class="filters" role="group" aria-label="Filter artifacts"><button class="filter active" type="button" data-filter="all" aria-pressed="true">All</button><button class="filter" type="button" data-filter="required" aria-pressed="false">Required</button><button class="filter" type="button" data-filter="attention" aria-pressed="false">Needs attention</button></div><div id="artifact-list"></div><p class="empty" id="filter-empty" hidden>No tracks match this filter.</p></section>
<section aria-labelledby="gaps"><div class="section-title"><p>Side B</p><h2 id="gaps">Known gaps</h2></div><div id="gap-list"></div></section>
<section class="ack" aria-labelledby="ack-title"><div class="section-title"><p>Dub copy</p><h2 id="ack-title">Acknowledge receipt</h2></div><p>Mark each artifact you reviewed, then export a receipt. Review state stays in this browser.</p><label for="recipient">Recipient name</label><input id="recipient" autocomplete="name" required><label for="ack-note">Note (optional)</label><textarea id="ack-note" rows="3"></textarea><button class="primary" id="export" type="button">Export acknowledgement</button><p id="ack-status" class="status-line" aria-live="polite"></p></section>
<noscript><p class="noscript">JavaScript is off. The immutable manifest remains available at <a href="manifest.json">manifest.json</a>; use <code>khb acknowledge</code> to export a receipt.</p></noscript></main>
<footer><p>Portable by design · no network, no tracking</p><a href="manifest.json">Open manifest.json</a></footer><script type="application/json" id="manifest">{data}</script><div id="bundle-meta" data-hash="{manifest_hash}"></div></body></html>"##,
        date = escape(&manifest.generated_at[..10]),
        summary = escape(&manifest.project.summary),
        owner = escape(&manifest.project.owner.name),
        prepared = escape(&manifest.project.prepared_at),
        health = if manifest.summary.errors == 0 {
            "Ready to review"
        } else {
            "Action required"
        },
    ))
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filenames_are_safe() {
        assert_eq!(sanitize_filename("run book?.md"), "run-book-.md");
    }

    #[test]
    fn html_escapes_project_copy() {
        assert_eq!(escape("A & <B>"), "A &amp; &lt;B&gt;");
    }

    #[test]
    fn output_cannot_replace_input_directory() {
        let input = tempfile::tempdir().unwrap();
        let result = prepare_output(input.path(), input.path(), true);
        assert!(result.is_err());
        assert!(input.path().exists());
    }
}
