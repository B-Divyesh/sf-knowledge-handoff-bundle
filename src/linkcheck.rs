use crate::model::{ArtifactKind, Finding, Handoff, Severity};
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, RANGE};
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use url::Url;

const USER_AGENT: &str =
    "knowledge-handoff-bundle/0.1 (+https://knowledge-handoff-bundle.sociobot.in)";

pub fn check_links(handoff: &Handoff) -> Result<Vec<Finding>, String> {
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(12))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|e| e.to_string())?;
    let mut findings = Vec::new();
    let mut robots: HashMap<String, RobotsPolicy> = HashMap::new();
    let mut last_request: HashMap<String, Instant> = HashMap::new();

    for artifact in handoff
        .sections
        .iter()
        .flat_map(|section| section.artifacts.iter())
        .filter(|artifact| matches!(artifact.kind, ArtifactKind::Url))
    {
        let Some(raw_url) = &artifact.url else {
            continue;
        };
        let Ok(url) = Url::parse(raw_url) else {
            continue;
        };
        let origin = url.origin().ascii_serialization();
        let policy = if let Some(policy) = robots.get(&origin) {
            policy.clone()
        } else {
            wait_for_origin(&origin, &mut last_request);
            let policy = fetch_robots(&client, &url);
            last_request.insert(origin.clone(), Instant::now());
            robots.insert(origin.clone(), policy.clone());
            policy
        };

        if !policy.allows(url.path()) {
            findings.push(Finding {
                severity: Severity::Warning,
                code: "link.robots_denied".into(),
                message: "Not checked because robots.txt disallows this path".into(),
                artifact_id: Some(artifact.id.clone()),
            });
            continue;
        }

        wait_for_origin(&origin, &mut last_request);
        let result = client
            .get(url.clone())
            .header(RANGE, HeaderValue::from_static("bytes=0-0"))
            .send();
        last_request.insert(origin, Instant::now());

        match result {
            Ok(response)
                if response.status().is_success() || response.status().is_redirection() =>
            {
                findings.push(Finding {
                    severity: Severity::Info,
                    code: "link.ok".into(),
                    message: format!("Reachable (HTTP {})", response.status().as_u16()),
                    artifact_id: Some(artifact.id.clone()),
                });
            }
            Ok(response) => findings.push(Finding {
                severity: Severity::Error,
                code: "link.http".into(),
                message: format!(
                    "Link returned HTTP {}; verify access or replace it",
                    response.status().as_u16()
                ),
                artifact_id: Some(artifact.id.clone()),
            }),
            Err(error) => findings.push(Finding {
                severity: Severity::Error,
                code: "link.network".into(),
                message: format!("Link check failed: {error}"),
                artifact_id: Some(artifact.id.clone()),
            }),
        }
    }
    Ok(findings)
}

#[derive(Clone, Default)]
struct RobotsPolicy {
    disallow: Vec<String>,
    allow: Vec<String>,
}

impl RobotsPolicy {
    fn allows(&self, path: &str) -> bool {
        let longest_allow = self
            .allow
            .iter()
            .filter(|rule| path.starts_with(rule.as_str()))
            .map(String::len)
            .max()
            .unwrap_or(0);
        let longest_deny = self
            .disallow
            .iter()
            .filter(|rule| !rule.is_empty() && path.starts_with(rule.as_str()))
            .map(String::len)
            .max()
            .unwrap_or(0);
        longest_deny == 0 || longest_allow >= longest_deny
    }
}

fn fetch_robots(client: &Client, target: &Url) -> RobotsPolicy {
    let mut robots_url = target.clone();
    robots_url.set_path("/robots.txt");
    robots_url.set_query(None);
    robots_url.set_fragment(None);
    let Ok(response) = client.get(robots_url).send() else {
        return RobotsPolicy::default();
    };
    if !response.status().is_success() {
        return RobotsPolicy::default();
    }
    let Ok(body) = response.text() else {
        return RobotsPolicy::default();
    };
    parse_robots(&body)
}

fn parse_robots(body: &str) -> RobotsPolicy {
    let mut policy = RobotsPolicy::default();
    let mut applies = false;
    for line in body.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        if key == "user-agent" {
            let agent = value.to_ascii_lowercase();
            applies = agent == "*" || agent.contains("knowledge-handoff-bundle");
        } else if applies && key == "disallow" {
            policy.disallow.push(value.to_string());
        } else if applies && key == "allow" {
            policy.allow.push(value.to_string());
        }
    }
    policy
}

fn wait_for_origin(origin: &str, requests: &mut HashMap<String, Instant>) {
    if let Some(last) = requests.get(origin) {
        let elapsed = last.elapsed();
        if elapsed < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) - elapsed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robots_prefers_more_specific_allow() {
        let policy = parse_robots("User-agent: *\nDisallow: /private\nAllow: /private/public\n");
        assert!(!policy.allows("/private/notes"));
        assert!(policy.allows("/private/public/index"));
        assert!(policy.allows("/other"));
    }
}
