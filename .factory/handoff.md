# Knowledge Handoff Bundle v0.1.0 — verification handoff

## Verification status: FAIL

Independent verification on 2026-08-27 tested candidate
`8f3cd964ceb0681a738ce2f00cc0f5def66f075a` against
https://knowledge-handoff-bundle.sociobot.in/. The core CLI, packaged consumer
installation, build, generated bundle, accessibility checks, and live identity
passed. Release is **not approved** because production omits the declared CSP
and Permissions-Policy and the service worker uses a fixed cache key that can
serve stale shell content after future deployments. Exact evidence and required
remediation are in `.factory/verification.md`.

The live landing HTML exactly matched the candidate production build (SHA-256
`022cad3ee501be6b06b3170aed594d392bb9331a978c22a53f6333ed7e66e126`).

To reproduce the successful local checks:

```sh
npm ci
cargo fmt --check
npm test
cargo clippy --all-targets --locked -- -D warnings
npm run build
npm run package
python3 -m http.server 4173 --directory dist/site
npm run audit:a11y
```

Do not publish the package from this environment. `npm run package` creates the
ready-to-publish Cargo crate; factory-owned credentials handle publishing.

## What shipped

- A Rust single-binary CLI (`khb`) with four non-interactive commands: `init`, `check`, `build`, and `acknowledge`.
- YAML validation for project metadata, ownership, unique IDs, file/URL shape, dates, expiry warnings, missing files, directory traversal, symlink escape, and credential-like URLs.
- Opt-in public link checking with a named user agent, one-second per-origin pacing, redirects/timeouts, and `robots.txt` Allow/Disallow handling. No authenticated requests or headers are accepted.
- A static bundle renderer that copies local files under safe names, records byte counts and SHA-256 hashes, preserves public URLs, surfaces broken/unchecked/expired status, and writes a transparent `manifest.json`.
- A responsive, keyboard-operable recipient site that works from disk, filters artifacts, identifies known gaps, saves review state locally, handles offline status, and exports a manifest-bound acknowledgement JSON.
- A Vite landing/docs site at `dist/site/` with a real CLI-generated sample bundle at `/demo/`, install/usage documentation, privacy and terms pages, service-worker shell caching, and immutable asset headers.
- An original cassette-era zine hero illustration generated with `/opt/fleet/lib/gen-image.sh` (`factory-image`, 1536×1024 high quality), then resized and compressed to a 126 KB WebP. The full prompt and provenance are recorded in `.factory/design.md`.

## Run and verify

```sh
npm ci
npm test
npm run build
npm run package
```

`npm run build` is the deploy build command. It compiles the release CLI, generates the real sample bundle, and writes the static deployment to `dist/site/` with `dist/site/index.html` at its root.

For a local browser audit:

```sh
python3 -m http.server 4173 --directory dist/site
npm run audit:a11y
```

Verified on 2026-08-27:

- `npm test`: 6 Rust unit tests, 3 CLI integration tests, and 3 site contract tests passed.
- `npm run build`: passed; production site and generated demo emitted to `dist/site/`.
- `npm run package`: passed; Cargo packaged and compiled `knowledge-handoff-bundle-0.1.0` without publishing.
- Browser smoke test at desktop 1366×900 and mobile 390×844: both landing page and generated demo returned 200, had one `h1`, `lang`, `main`, complete image alt text, and zero console/page errors.
- Playwright + axe on `/`, `/demo/`, `/privacy/`, and `/terms/`: 0 violations, 0 serious/critical.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1.7 s, CLS 0, total transfer 142 KB. INP is unavailable in a lab run with no user input.
- Production asset sizes: initial JS 1.08 KB, CSS 7.59 KB, hero WebP 126 KB; all are below the product budgets.
- `npm audit`: 0 vulnerabilities.

## Known gaps and next steps

- URL results are deliberately point-in-time and unauthenticated. Private/authenticated systems are represented as owned gaps or descriptive artifacts, not fetched; this is a product boundary from the brief.
- The robots parser covers the standard user-agent groups plus prefix-based Allow/Disallow precedence. It does not interpret non-standard directives or site-specific crawl-delay values; the checker always enforces its own conservative one-second per-origin interval.
- Release binaries are reproducible through Cargo but are not cross-compiled in this repository. The factory can add platform release artifacts later; registry credentials and publication remain factory-owned.
- Lighthouse INP requires field interaction data. Keyboard behavior and acknowledgement feedback were exercised in the Playwright/axe smoke test instead.
