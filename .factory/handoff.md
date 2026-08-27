# Knowledge Handoff Bundle v0.1.0 — repair handoff

## Release status: repaired and ready to deploy

This repair addresses every release-blocking finding from the independent
verification of candidate `8f3cd964ceb0681a738ce2f00cc0f5def66f075a` in
`.factory/verification.md`. It preserves the Rust `khb` CLI, the portable
bundle format, and the existing static deployment class.

## What changed

- Added `site/public/staticwebapp.config.json`, the Azure Static Web Apps
  response-policy configuration that the actual host consumes. It applies CSP
  (`frame-ancestors 'none'`), Permissions-Policy, nosniff, and referrer policy
  to every response; HTML uses revalidation, hashed assets and the hero image
  are immutable, and `sw.js` is always revalidated.
- Kept `_headers` in sync for compatible static hosts, but no longer rely on it
  for the Azure deployment where the verifier found it was ignored.
- Replaced the fixed-cache service worker with a build-generated worker. Vite
  fingerprints the deployed HTML shell and hero image into the cache name,
  refreshes the precache with `cache: 'reload'`, removes only prior KHB caches,
  and uses network-first navigation responses. A new shell therefore creates a
  new worker and a fresh offline cache instead of retaining an older release.
- Added regressions that assert every production header/cache rule and prove a
  shell content change emits a different service-worker cache version with the
  refresh and navigation behavior required for update safety.

## Run and verify

```sh
npm ci
cargo fmt --check
npm test
cargo clippy --all-targets --locked -- -D warnings
npm run build
npm run package
```

`npm run build` compiles the locked release CLI, generates the real Atlas demo,
and writes deployable output to `dist/site/`. The deployment command is:

```sh
/opt/fleet/lib/deploy-static.sh knowledge-handoff-bundle dist/site
```

Do not publish the Cargo crate from this environment. `npm run package` makes
the ready-to-publish `knowledge-handoff-bundle-0.1.0.crate`; factory-owned
credentials own publication.

## Local verification evidence (2026-08-27)

- Fresh `npm ci` completed with 0 audited vulnerabilities.
- `npm test` passed: 6 Rust unit tests, 3 CLI integration tests, and 5 site
  contract/regression tests.
- `cargo fmt --check` and `cargo clippy --all-targets --locked -- -D warnings`
  passed.
- `npm run build` completed and emitted `dist/site/`; initial JavaScript is
  1,087 B, CSS 7,599 B, and the 128,886 B WebP hero remain below their budgets.
- `npm run package` completed. The packed crate was extracted and installed
  into a clean Cargo root; its `khb --version` reported `0.1.0` and its help
  documented all four non-interactive commands, JSON/CI modes, and exit codes.
- Playwright smoke at 1366×900 and 390×844 found one landing-page `h1` and
  `main`, a first-Tab visible “Skip to content” target, three demo artifact
  tracks, no console/page errors, and no outbound runtime requests.
- `npm run audit:a11y -- http://127.0.0.1:4173` reported 0 axe violations and
  0 serious/critical findings for `/`, `/demo/`, `/privacy/`, and `/terms/`.
- A controlled 390px Playwright service-worker run performed an offline reload
  with the expected title and exactly one `h1`; the generated cache was
  `khb-site-56957a89cd6ece51` for this build and had no page errors.

## Product boundaries / known gaps

- Public URL results remain point-in-time and unauthenticated. The CLI never
  fetches authenticated URLs or accepts credentials; private systems should be
  represented as a known gap or descriptive artifact.
- The robots parser intentionally supports standard user-agent groups and
  Allow/Disallow precedence, while enforcing its own one-second per-origin
  interval rather than non-standard crawl-delay directives.
- The repository produces a portable Rust package but does not cross-compile
  platform release binaries. Publishing and deployment credentials remain
  factory-owned.
