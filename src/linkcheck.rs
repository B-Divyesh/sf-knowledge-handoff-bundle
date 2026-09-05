use crate::model::{ArtifactKind, Finding, Handoff, Severity};
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, RANGE, RETRY_AFTER};
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
    let mut schedules: HashMap<String, OriginSchedule> = HashMap::new();

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
            wait_for_origin(&origin, &mut schedules);
            let (policy, retry_after) = fetch_robots(&client, &url);
            record_request(&origin, retry_after, &mut schedules);
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

        wait_for_origin(&origin, &mut schedules);
        let result = client
            .get(url.clone())
            .header(RANGE, HeaderValue::from_static("bytes=0-0"))
            .send();
        let retry_after = result.as_ref().ok().and_then(retry_after);
        record_request(&origin, retry_after, &mut schedules);

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

fn fetch_robots(client: &Client, target: &Url) -> (RobotsPolicy, Option<Duration>) {
    let mut robots_url = target.clone();
    robots_url.set_path("/robots.txt");
    robots_url.set_query(None);
    robots_url.set_fragment(None);
    let Ok(response) = client.get(robots_url).send() else {
        return (RobotsPolicy::default(), None);
    };
    let retry_after = retry_after(&response);
    if !response.status().is_success() {
        return (RobotsPolicy::default(), retry_after);
    }
    let Ok(body) = response.text() else {
        return (RobotsPolicy::default(), retry_after);
    };
    (parse_robots(&body), retry_after)
}

fn parse_robots(body: &str) -> RobotsPolicy {
    let mut policy = RobotsPolicy::default();
    let mut applies = false;
    let mut group_has_rules = false;
    for line in body.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        if key == "user-agent" {
            if group_has_rules {
                applies = false;
                group_has_rules = false;
            }
            let agent = value.to_ascii_lowercase();
            applies |= agent == "*" || agent.contains("knowledge-handoff-bundle");
        } else if applies && key == "disallow" {
            policy.disallow.push(value.to_string());
            group_has_rules = true;
        } else if applies && key == "allow" {
            policy.allow.push(value.to_string());
            group_has_rules = true;
        } else if matches!(key.as_str(), "allow" | "disallow") {
            group_has_rules = true;
        }
    }
    policy
}

#[derive(Default)]
struct OriginSchedule {
    last_request: Option<Instant>,
    retry_until: Option<Instant>,
}

fn wait_for_origin(origin: &str, schedules: &mut HashMap<String, OriginSchedule>) {
    let schedule = schedules.entry(origin.to_owned()).or_default();
    let one_second_after_last = schedule
        .last_request
        .map(|last| last + Duration::from_secs(1));
    let not_before = match (one_second_after_last, schedule.retry_until) {
        (Some(rate_limit), Some(retry_after)) => Some(rate_limit.max(retry_after)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    };
    if let Some(not_before) = not_before {
        if let Some(delay) = not_before.checked_duration_since(Instant::now()) {
            thread::sleep(delay);
        }
    }
}

fn record_request(
    origin: &str,
    retry_after: Option<Duration>,
    schedules: &mut HashMap<String, OriginSchedule>,
) {
    let now = Instant::now();
    let schedule = schedules.entry(origin.to_owned()).or_default();
    schedule.last_request = Some(now);
    if let Some(delay) = retry_after {
        let retry_until = now + delay;
        if schedule
            .retry_until
            .map_or(true, |current| retry_until > current)
        {
            schedule.retry_until = Some(retry_until);
        }
    }
}

fn retry_after(response: &reqwest::blocking::Response) -> Option<Duration> {
    parse_retry_after(response.headers().get(RETRY_AFTER)?)
}

fn parse_retry_after(header: &HeaderValue) -> Option<Duration> {
    let value = header.to_str().ok()?.trim();
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    DateTime::parse_from_rfc2822(value)
        .ok()
        .and_then(|date| (date.with_timezone(&Utc) - Utc::now()).to_std().ok())
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

    #[test]
    fn robots_handles_multiple_agents_in_one_group() {
        let policy = parse_robots(
            "User-agent: *\nUser-agent: knowledge-handoff-bundle\nDisallow: /archive\n\nUser-agent: other\nDisallow: /\n",
        );
        assert!(!policy.allows("/archive/item"));
        assert!(policy.allows("/current"));
    }

    #[test]
    fn retry_after_parses_seconds_and_http_dates() {
        let header = HeaderValue::from_static("2");
        assert_eq!(parse_retry_after(&header), Some(Duration::from_secs(2)));
        let future = (Utc::now() + chrono::Duration::seconds(3)).to_rfc2822();
        let parsed = parse_retry_after(&future.parse().unwrap()).unwrap();
        assert!(parsed >= Duration::from_secs(1));
        assert!(parsed <= Duration::from_secs(3));
    }
}
