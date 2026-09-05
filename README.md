# Build a project handoff

Knowledge Handoff Bundle is a Rust CLI for departing project owners and small teams. It turns a YAML checklist, local files, and public links into one portable static bundle for the next owner.

The bundle shows copied-file SHA-256 hashes, owners, link results, expiry warnings, and known gaps. The recipient can review artifacts and export a local acknowledgement tied to the manifest hash.

## Try the sample

Run the bundled Atlas sample without setup:

```sh
khb demo
```

The command writes a sample bundle in a temporary directory and prints its path. The sample includes copied files, a recorded reachable link, a recorded broken link, an expiry warning, and a missing-access gap.

The website demo is at <https://knowledge-handoff-bundle.sociobot.in/demo/>. It uses the `demo:` browser storage namespace. Reset demo clears that sample state. Start for real clears it before returning home.

## Install

Requires Rust 1.78 or newer.

```sh
git clone https://github.com/B-Divyesh/sf-knowledge-handoff-bundle
cd sf-knowledge-handoff-bundle
cargo install --path .
khb --help
```

## Use

Start a checklist, edit it, validate it, and build the bundle:

```sh
khb init handoff.yaml
khb check handoff.yaml
khb build handoff.yaml --output ./handoff-bundle --check-links
```

Open `handoff-bundle/index.html` in any browser. The bundle works from disk and needs no hosted runtime or database.

For automation, every command supports `--json`. `--ci` makes warnings return a failing exit code.

```sh
khb --json check handoff.yaml
khb --json --ci build handoff.yaml --output ./handoff-bundle
```

Export an acknowledgement from the immutable manifest:

```sh
khb acknowledge ./handoff-bundle/manifest.json \
  --recipient "Sam Rivera" \
  --accept architecture --accept runbook \
  --note "Access to production is still pending" \
  --output acknowledgement.json
```

The recipient page stores review state only in that browser. Export acknowledgement creates a local JSON file and does not upload it.

## Checklist format

```yaml
project:
  title: Atlas migration
  summary: Move the reporting pipeline to the new warehouse.
  owner:
    name: Priya Shah
    contact: priya@example.test
  prepared_at: 2026-09-05
  expires_at: 2026-12-05
sections:
  - title: Start here
    artifacts:
      - id: architecture
        title: Architecture decision log
        kind: file
        path: docs/decisions.md
        owner: Platform team
        required: true
        note: Read decisions 14 and 18 first.
      - id: dashboard
        title: Delivery dashboard
        kind: url
        url: https://www.rfc-editor.org/rfc/rfc9110.html
        owner: Priya Shah
gaps:
  - id: prod-access
    title: Production access has not transferred
    owner: Operations
    next_step: Add the recipient to the deploy group.
```

Paths resolve relative to the YAML file. The CLI copies listed local files and records SHA-256 hashes.

Public links are checked only with `--check-links`. Checks use a named user agent, wait at least one second between requests to one origin, honor `Retry-After`, and respect `robots.txt`.

URLs with embedded credentials or credential-like query keys are refused before fetch or output. The checker does not crawl beyond listed URLs and each origin’s `robots.txt`.

## Commands and exit codes

- `khb init [FILE]` writes an annotated starter file and refuses to overwrite an existing file.
- `khb check <FILE>` validates schema, files, dates, duplicate IDs, and optional public links.
- `khb build <FILE> --output <DIR>` writes `index.html`, `manifest.json`, `assets/`, and copied local files.
- `khb demo` builds the shipped sample in a temporary directory.
- `khb acknowledge <MANIFEST>` exports recipient, accepted IDs, note, UTC timestamp, and manifest SHA-256.
- Exit `0`: success; `2`: invalid input or warnings in CI mode; `3`: network or link-check failure; `4`: filesystem or build failure.

## Develop and verify

```sh
npm ci
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm test
npm run build       # writes the landing site to dist/site/
npm run audit:a11y  # builds no server prerequisite; audits dist/site/
npm run package     # verifies a publishable crate; does not publish
```

Each public claim is listed in [.factory/claims.json](.factory/claims.json). Run one claim from a clean setup with its command in that registry.

Publishing credentials belong to the factory. Do not publish this crate from a worker environment.

## Privacy and security

The CLI has no telemetry. It reads listed files and writes to the output path you choose.

The landing site has no analytics, advertising, cookies, remote scripts, or remote fonts. See the [privacy page](https://knowledge-handoff-bundle.sociobot.in/privacy/) and [terms page](https://knowledge-handoff-bundle.sociobot.in/terms/).

## License

MIT © 2026 Sociobot (Param Factory). See [LICENSE](LICENSE).
