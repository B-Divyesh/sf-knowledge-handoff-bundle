use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::thread;

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

#[test]
fn build_with_a_broken_checked_link_returns_link_failure() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        for _ in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0; 1024];
            let read = stream.read(&mut request).unwrap();
            let request = String::from_utf8_lossy(&request[..read]);
            let (status, body) = if request.starts_with("GET /robots.txt ") {
                ("200 OK", "User-agent: *\nAllow: /\n")
            } else {
                ("404 Not Found", "not found")
            };
            write!(
                stream,
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }
    });

    let temp = tempfile::tempdir().unwrap();
    fs::write(
        temp.path().join("handoff.yaml"),
        format!(
            r#"project:
  title: Atlas
  summary: Reporting migration
  owner: {{ name: Priya }}
  prepared_at: 2026-08-27
sections:
  - title: Sources
    artifacts:
      - id: status
        title: Current status
        kind: url
        url: http://{address}/missing
        owner: Priya
"#
        ),
    )
    .unwrap();
    let bundle = temp.path().join("bundle");
    let output = khb()
        .args([
            "--json",
            "build",
            temp.path().join("handoff.yaml").to_str().unwrap(),
            "--output",
            bundle.to_str().unwrap(),
            "--check-links",
        ])
        .output()
        .unwrap();
    server.join().unwrap();

    assert_eq!(output.status.code(), Some(3));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["ok"], false);
    assert_eq!(json["result"]["summary"]["errors"], 1);
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(bundle.join("manifest.json")).unwrap()).unwrap();
    assert!(manifest["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|finding| finding["code"] == "link.http"));
}
