use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Handoff {
    pub project: Project,
    #[serde(default)]
    pub sections: Vec<Section>,
    #[serde(default)]
    pub gaps: Vec<Gap>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub title: String,
    pub summary: String,
    pub owner: Owner,
    pub prepared_at: String,
    #[serde(default)]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Owner {
    pub name: String,
    #[serde(default)]
    pub contact: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Section {
    pub title: String,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactKind {
    File,
    Url,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Artifact {
    pub id: String,
    pub title: String,
    pub kind: ArtifactKind,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    pub owner: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Gap {
    pub id: String,
    pub title: String,
    pub owner: String,
    pub next_step: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub format: String,
    pub generated_at: String,
    pub project: Project,
    pub sections: Vec<ManifestSection>,
    pub gaps: Vec<Gap>,
    pub findings: Vec<Finding>,
    pub summary: ManifestSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSection {
    pub title: String,
    pub artifacts: Vec<ManifestArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestArtifact {
    pub id: String,
    pub title: String,
    pub kind: ArtifactKind,
    pub owner: String,
    pub required: bool,
    pub note: Option<String>,
    pub expires_at: Option<String>,
    pub href: Option<String>,
    pub sha256: Option<String>,
    pub bytes: Option<u64>,
    pub status: String,
    pub status_detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSummary {
    pub artifacts: usize,
    pub required: usize,
    pub verified: usize,
    pub warnings: usize,
    pub errors: usize,
    pub gaps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acknowledgement {
    pub format: String,
    pub project: String,
    pub recipient: String,
    pub accepted: Vec<String>,
    pub note: Option<String>,
    pub acknowledged_at: String,
    pub manifest_sha256: String,
}
