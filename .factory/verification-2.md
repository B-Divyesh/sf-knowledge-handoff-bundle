# Independent verification 2 — FAIL

**Work order:** `knowledge-handoff-bundle-verify-2`  
**Candidate:** `8e219d351f8f4a0a9e9d7fc5f57542a0d74b4347`  
**Verified:** 2026-08-27  
**Live URL:** https://knowledge-handoff-bundle.sociobot.in/

## Decision

**FAIL.** The Rust CLI, packed consumer installation, static handoff, and live
deployment all work for the core job-to-be-done. The prior deployment-only
findings are fixed: fresh live responses carry the intended policies and the
release-specific service worker successfully supports offline reload. However,
the generated handoff's primary artifact links are only **24.8 CSS px tall**
at the required 390 px mobile viewport. Opening an artifact is a core
recipient action, and this fails the supplied 44 px touch-target requirement
and the factory's mobile end-to-end definition of done.

## Defects

### Medium — generated handoff artifact links miss the 44 px mobile target

At 390 × 844 on the live `/demo/`, each `Open copied file` and `Open public
link` control measured 153.625 × **24.796875** CSS px. The controls are the
recipient's primary way to open the declared source-of-truth artifacts. They
are keyboard-operable, but are materially undersized for touch. The generated
bundle stylesheet needs to give these links a 44 px-or-larger hit area before
release.

### Low — platform-generated 404s do not receive declared global headers

Fresh HTTPS `GET` responses for `/does-not-exist` and
`/assets/does-not-exist.js` were `404 text/html` with only `date` and
`content-type`; they omitted the declared CSP, Permissions-Policy,
Referrer-Policy, nosniff, and Cache-Control. Known deployed product routes do
have the policies below. This is host-generated 404 hardening rather than a
core product-flow defect, but the deployment claim that global headers protect
every response is not true for those paths.

## Evidence that passed

### Clean install, quality gates, and exact build

The clean candidate worktree started at the requested SHA with no changes.

- `npm ci` passed: 23 packages audited, 0 vulnerabilities.
- `cargo fmt --check` passed.
- `npm test` passed: 6 Rust unit tests, 3 CLI integration tests, and 5 site
  contract/regression tests.
- `cargo clippy --all-targets --locked -- -D warnings` passed.
- Exact `npm run build` passed: locked release CLI build, Atlas demo build,
  and Vite production output in `dist/site/`.
- `npm run package` passed: Cargo packaged and verified 27 files, 143.9 KiB
  uncompressed / 40.6 KiB compressed. No publication occurred.
- The package was installed into a clean Cargo root from the packaged source;
  its single `khb 0.1.0` binary exposed the documented `init`, `check`,
  `build`, and `acknowledge` commands, JSON/CI modes, and exit codes. That
  installed binary built the Atlas packet successfully.

### CLI end-to-end and recovery checks

- Normal Atlas `check` returned 0 findings; `build` created 3 artifacts, 3
  required items, 2 SHA-256-verified local files, and 1 known gap. The copied
  `architecture-decisions.md` hash exactly matched its source
  (`8dbbfc8900c2aedd5e959d80a9ce1cb8190cbc60f3c4afbb2a7e5d048d382d9c`).
- `acknowledge` for `Sam Rivera`, accepting `architecture` and `runbook`,
  produced a receipt with a 64-character manifest SHA-256.
- Re-running `init` returned exit 4 without overwriting its existing YAML.
- A credential-bearing URL returned exit 2 with
  `artifact.url_credentials`; JSON output did not include the test secret.
- An invalid date plus `../` file path returned exit 2 with
  `project.expires_at` and `artifact.parent_path`.
- A 2026-08-28 expiry warning exits 0 normally and exit 2 with `--ci`.
- The representative public 404 link check returned exit 3 and `link.http`.
- Empty acknowledgement recipient returned exit 2 with `Recipient name cannot
  be empty`.

### Browser, accessibility, privacy, and PWA

- Production output was served locally and tested at 1366 × 900 and 390 ×
  844. The first Tab reaches the visibly outlined skip link; landing pages
  have one `h1` and `main`; reduced motion makes the hero animation `none`;
  no horizontal overflow occurred at 390 px.
- On local and live demos: 3 tracks render, `Needs attention` filters to 1,
  blank recipient export reports recovery text, one review exports
  `acknowledgement.json`, and the success text reports 1 reviewed artifact.
  No page or console errors occurred.
- Local `npm run audit:a11y` found 0 total axe violations across `/`,
  `/demo/`, `/privacy/`, and `/terms/`. Fresh live `node scripts/a11y.mjs
  https://knowledge-handoff-bundle.sociobot.in` also found 0 total and 0
  serious/critical violations on all four routes.
- Browser request capture at both viewports found no runtime requests to a
  third-party origin. Source inspection found no analytics, remote fonts, or
  remote scripts; acknowledgement state and export remain local.
- A fresh live 390 px session was controlled by `/sw.js`, created cache
  `khb-site-56957a89cd6ece51`, then reloaded offline with HTTP 200, the
  expected title, one `h1`, and no browser errors. The passing regression test
  also proves a changed shell generates a new worker cache version and reloads
  the precache before control.

### Deployment identity, policies, caching, and budgets

- Local and live landing `index.html` SHA-256 are identical:
  `022cad3ee501be6b06b3170aed594d392bb9331a978c22a53f6333ed7e66e126`.
  Local/live JS, CSS, hero image, and service worker hashes also matched.
  The demo manifest matched after excluding its expected generated timestamp;
  it has the same summary and copied-file hashes.
- Fresh HTTPS `200` responses for `/`, `/demo/`, `/privacy/`, `/terms/`,
  `/sw.js`, the hero, and both hashed assets include the restrictive CSP,
  `Permissions-Policy: camera=(), microphone=(), geolocation=()`, HSTS,
  `Referrer-Policy: strict-origin-when-cross-origin`, and nosniff. HTML is
  `public, max-age=0, must-revalidate`; JS/CSS and the hero are
  `public, max-age=31536000, immutable`; `/sw.js` is `no-cache`.
- Production sizes: JS 1,087 B, CSS 7,599 B, and hero WebP 128,886 B, below
  the 200 KB, 50 KB, and 300 KB budgets. No font assets are shipped.
- A fresh Lighthouse CLI attempt could not connect to the supplied Chromium
  despite successful Playwright and axe runs, so no new Lighthouse score is
  claimed in this report.

## Required next steps

1. Make artifact links in `templates/bundle.css` meet the 44 px touch-target
   minimum and retest the generated bundle at 390 px.
2. Configure the static host's platform-generated 404 responses to emit the
   same baseline security and caching headers, or explicitly document the
   host limitation and serve a controlled 404 page.
3. Re-run this verification after the deployment updates; do not change CLI
   behavior or publish the Cargo package as part of the repair.
