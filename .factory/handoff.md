# Knowledge Handoff Bundle v0.1.0 — repair handoff

## Release status: PASS

This repair resolves every finding in independent verification 2
(`.factory/verification-2.md`) for candidate
`8e219d351f8f4a0a9e9d7fc5f57542a0d74b4347`, while preserving the Rust `khb`
CLI, portable bundle format, and Azure Static Web Apps deployment class.

Product repair commit: `5619592` (`fix: meet mobile targets and harden 404s`).
The deployed production artifact was uploaded as Azure Static Web Apps
deployment `42a8b159-e80d-4e17-ade2-65eb38c7325e` to
https://knowledge-handoff-bundle.sociobot.in/.

## Repairs

- Generated `Open copied file` and `Open public link` controls now use the
  `.artifact-link` hit area, an inline-flex control with `min-height: 44px`.
  The measured live controls are 153.625 × 44 CSS px at both 390 × 844 and
  1366 × 900.
- Added a product-owned `404.html` and `not-found.css`, then configured Azure
  Static Web Apps `responseOverrides.404` to rewrite every missing route to
  that document while keeping HTTP status 404. This changes host-generated,
  headerless failures into policy-covered static responses.
- Added exact regression coverage: a Playwright test builds a real Atlas
  bundle and measures all three artifact controls at 390 px and desktop size;
  contract tests assert the response override, accessible 404 document, and
  its focus/target styling. `npm test` runs both test files, and CI installs
  Chromium before running it.
- Expanded the axe script to include both the direct 404 document and an
  actual missing route.

## Run, package, and deploy

```sh
npm ci
cargo fmt --check
npm test
cargo clippy --all-targets --locked -- -D warnings
npm run build
npm run package
/opt/fleet/lib/deploy-static.sh knowledge-handoff-bundle dist/site
```

`npm run build` compiles the locked release CLI, generates the real Atlas
demo, and writes the deployable site to `dist/site/`. `npm run package`
creates the ready-to-publish Cargo artifact; do not publish it from this
environment because registry credentials are factory-owned.

## Verification evidence — 2026-08-27

- Clean `npm ci` installed 23 packages and reported 0 vulnerabilities.
- `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`,
  and `git diff --check` passed.
- `npm test` passed: 6 Rust unit tests, 3 CLI integration tests, and 6 Node
  site/browser contract tests. The browser regression measured all three
  generated artifact controls at least 44 px high and wide at 390 × 844 and
  1366 × 900.
- `npm run build` passed. Production output is 1,087 B JavaScript, 7,599 B
  CSS, and a 128,886 B WebP hero—within the 200 KB / 50 KB / 300 KB budgets.
- `npm run package` passed and produced the 42,654 B compressed
  `knowledge-handoff-bundle-0.1.0.crate`. It was extracted, installed into a
  clean Cargo root, exposed the documented single `khb` binary/help, and that
  installed consumer binary built the Atlas example successfully.
- Local Playwright smoke at 1366 × 900 and 390 × 844 passed the full demo
  flow: three tracks, the attention filter, blank-recipient recovery, one
  reviewed-item acknowledgement export, and no page errors. At 390 px the
  first Tab reaches the skip link.
- Local and live axe audits reported 0 total violations (and 0
  serious/critical) for `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`,
  and `/does-not-exist`.
- Live Playwright request capture found no third-party runtime requests; the
  site uses no remote font, analytics, or tracking service. A 390 px live
  session was controlled by `khb-site-56957a89cd6ece51` and reloaded offline
  with the expected title and one `h1` without page errors.
- Fresh live responses for `/`, `/demo/`, `/privacy/`, `/terms/`, and `/sw.js`
  carry the restrictive CSP, Permissions-Policy, HSTS, referrer policy, and
  `nosniff`. HTML uses `public, max-age=0, must-revalidate`; `/sw.js` uses
  `no-cache`.
- Crucially, fresh live `/does-not-exist` and `/assets/does-not-exist.js`
  responses are both HTTP 404 and now include that same CSP,
  `Permissions-Policy: camera=(), microphone=(), geolocation=()`,
  `Referrer-Policy: strict-origin-when-cross-origin`, `X-Content-Type-Options:
  nosniff`, HSTS, and `Cache-Control: public, max-age=0, must-revalidate`.
  The browser confirmed the controlled 404 title and sole `h1`.
- Local/live SHA-256 values matched for `index.html`, `sw.js`, `404.html`,
  `not-found.css`, the hero, and both hashed JS/CSS assets. The landing HTML
  hash is `022cad3ee501be6b06b3170aed594d392bb9331a978c22a53f6333ed7e66e126`.
- Mobile Lighthouse produced Performance 98, Accessibility 100, Best
  Practices 100, SEO 100, LCP 1,502.53 ms, and CLS 0. The runner logged a
  Chromium target crash only while collecting final screenshot/BFCache
  artifacts after generating the JSON report; the reported category metrics
  remain complete.

## Known product boundaries

- Public URL checks are point-in-time and unauthenticated. The CLI refuses
  credential-bearing URLs and never writes credentials or request headers to
  a bundle; private systems should be described as known gaps.
- The crate is ready to publish but this repair does not publish it or
  cross-compile platform-specific release binaries. Publishing and deployment
  credentials remain factory-owned.
