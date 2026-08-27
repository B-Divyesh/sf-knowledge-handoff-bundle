# Knowledge Handoff Bundle

`khb` turns a plain YAML handoff checklist plus local files and public URLs into a portable, browseable, verifiable static bundle. It is for departing project owners and small teams who need the next owner to see what exists, who owns it, whether it is current, and what is still missing—without depending on the old owner's notes or a hosted knowledge base.

The format is transparent, the output works offline, and there is no telemetry. Link checks are opt-in, rate-limited per origin, respect `robots.txt`, and reject URLs containing credentials.

## Install

Requires Rust 1.78 or newer.

```sh
cargo install --path .
khb --help
```

## Usage

Start a handoff file, edit it, validate it, and build the packet:

```sh
khb init handoff.yaml
khb check handoff.yaml
khb build handoff.yaml --output ./handoff-bundle --check-links
```

For automation, every command supports `--json`. `--ci` disables decorative output and makes warnings fail validation:

```sh
khb --json check handoff.yaml
khb --json --ci build handoff.yaml --output ./handoff-bundle
```

To create a signed-by-content acknowledgement from the immutable manifest:

```sh
khb acknowledge ./handoff-bundle/manifest.json \
  --recipient "Sam Rivera" \
  --accept architecture --accept runbook \
  --note "Access to production is still pending" \
  --output acknowledgement.json
```

Open `handoff-bundle/index.html` in any browser. The recipient can also mark reviewed items in that page and export the same acknowledgement format without a server. Review state stays in that browser only.

### YAML format

```yaml
project:
  title: Atlas migration
  summary: Move the reporting pipeline to the new warehouse.
  owner:
    name: Priya Shah
    contact: priya@example.test
  prepared_at: 2026-08-27
  expires_at: 2026-11-27
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
        url: https://example.com/status
        owner: Priya Shah
        expires_at: 2026-09-30
gaps:
  - id: prod-access
    title: Production access has not transferred
    owner: Operations
    next_step: Add the recipient to the deploy group.
```

Paths are resolved relative to the YAML file, copied into `files/`, and SHA-256 hashed. URLs with embedded usernames/passwords are refused. Credentials and secret headers are never accepted or written to output.

## Commands and exit codes

- `khb init [FILE]` writes an annotated starter file and refuses to overwrite an existing file.
- `khb check <FILE>` validates schema, files, dates, duplicate IDs, and optionally public links.
- `khb build <FILE> --output <DIR>` writes `index.html`, `manifest.json`, `assets/`, and copied local files.
- `khb acknowledge <MANIFEST>` exports recipient, accepted IDs, note, UTC timestamp, and manifest SHA-256.
- Exit `0`: success; `2`: invalid input or warnings in CI mode; `3`: network/link-check failure; `4`: filesystem/build failure.

## Develop and verify

```sh
cargo fmt --check
cargo test
npm install
npm test
npm run build       # Rust release build + landing site at dist/site/
npm run package     # verifies cargo package can be created (does not publish)
```

Run the docs site locally with `npm run dev`. Publishing credentials belong to the factory; do not publish this crate from a worker environment.

## Privacy and security

Generated bundles are static and self-contained. Browser acknowledgement state uses local storage in that browser and is never transmitted. The landing site stores nothing and loads no remote scripts, fonts, analytics, or trackers. See the site's privacy and terms pages for plain-language details.

## License

MIT © 2026 Sociobot (Param Factory). See [LICENSE](LICENSE).
