use std::fs;
use std::process::Command;

fn khb() -> Command {
    Command::new(env!("CARGO_BIN_EXE_khb"))
}

#[test]
fn documented_build_and_acknowledge_flow() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("notes.md"), "# Current truth\n").unwrap();
    fs::write(
        temp.path().join("handoff.yaml"),
        r#"project:
  title: Atlas
  summary: Reporting migration
  owner: { name: Priya }
  prepared_at: 2026-08-27
sections:
  - title: Start here
    artifacts:
      - id: notes
        title: Current notes
        kind: file
        path: notes.md
        owner: Priya
        required: true
gaps: []
"#,
    )
    .unwrap();
    let yaml = temp.path().join("handoff.yaml");
    let bundle = temp.path().join("bundle");
    let status = khb()
        .args([
            "build",
            yaml.to_str().unwrap(),
            "--output",
            bundle.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(bundle.join("index.html").is_file());
    assert!(bundle.join("files/notes-notes.md").is_file());
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(bundle.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["summary"]["verified"], 1);
    assert!(manifest["sections"][0]["artifacts"][0]["sha256"].is_string());

    let ack = temp.path().join("ack.json");
    let output = khb()
        .args([
            "--json",
            "acknowledge",
            bundle.join("manifest.json").to_str().unwrap(),
            "--recipient",
            "Sam Rivera",
            "--accept",
            "notes",
            "--output",
            ack.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .starts_with("{\"ok\":true"));
    let receipt: serde_json::Value = serde_json::from_slice(&fs::read(ack).unwrap()).unwrap();
    assert_eq!(receipt["accepted"][0], "notes");
}

#[test]
fn check_json_fails_for_missing_file() {
    let temp = tempfile::tempdir().unwrap();
    fs::write(
        temp.path().join("handoff.yaml"),
        r#"project:
  title: Atlas
  summary: Reporting migration
  owner: { name: Priya }
  prepared_at: 2026-08-27
sections:
  - title: Sources
    artifacts:
      - { id: missing, title: Missing, kind: file, path: missing.md, owner: Priya }
"#,
    )
    .unwrap();
    let output = khb()
        .args([
            "--json",
            "check",
            temp.path().join("handoff.yaml").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["ok"], false);
    assert_eq!(json["result"]["errors"], 1);
}

#[test]
fn init_refuses_to_overwrite() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("handoff.yaml");
    fs::write(&path, "keep me").unwrap();
    let output = khb()
        .args(["init", path.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(fs::read_to_string(path).unwrap(), "keep me");
}
