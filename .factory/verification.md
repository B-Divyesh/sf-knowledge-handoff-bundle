# Independent verification — FAIL

**Work order:** `knowledge-handoff-bundle-verify-1`  
**Candidate:** `8f3cd964ceb0681a738ce2f00cc0f5def66f075a`  
**Verified:** 2026-08-27  
**Live URL:** https://knowledge-handoff-bundle.sociobot.in/

## Decision

**FAIL.** The CLI and generated bundle meet the core job-to-be-done in the
checks below, and the landing page at `/` is byte-identical to this candidate's
production build. However, the deployed response policy does not apply the
declared CSP/Permissions-Policy and the shipped service worker cannot safely
update cached shell content after a future deployment. These fail the requested
deployment response-policy and service-worker-update checks.

## Release-blocking defects

### Medium — intended browser security policy is absent in production

`site/public/_headers` declares a restrictive CSP, `frame-ancestors 'none'`,
and `Permissions-Policy: camera=(), microphone=(), geolocation=()`. Fresh
HTTPS `HEAD` responses for `/`, `/assets/main-BNa109us.js`,
`/assets/style-CnFK-bcj.css`, `/sw.js`, `/demo/`, `/privacy/`, and `/terms/`
returned no `Content-Security-Policy` and no `Permissions-Policy`. They did
return HSTS, `Referrer-Policy: strict-origin-when-cross-origin`, and
`X-Content-Type-Options: nosniff`.

All of those deployed assets instead use `Cache-Control: public,
must-revalidate, max-age=30`; the intended immutable cache directives are not
being applied either. This is a deployment/configuration failure, not a
candidate document mismatch: `/` exactly matched the locally built candidate
SHA-256 `022cad3ee501be6b06b3170aed594d392bb9331a978c22a53f6333ed7e66e126`.

### Medium — service-worker update leaves the shell stale

`site/public/sw.js` uses the fixed cache name `khb-site-v1` and cache-first
responses. On an update, a changed worker runs `cache.addAll(SHELL)` against
the same existing keys, which resolves from that cache rather than refreshing
`/`, `/privacy/`, `/terms/`, or the hero image. Consequently an already
controlled client can retain the old shell across a deployment. `skipWaiting`
and `clients.claim` do not fix the stale entries because the cache version is
unchanged. Offline reload works for the currently cached release, but a
versioned cache/precache manifest (or a network-first update strategy for the
HTML shell) is required before release.

## Evidence that passed

### Clean install, build, tests, and package

A fresh detached clone at the candidate SHA was used at
`/tmp/khb-verify.QedPDi`.

- `npm ci`: passed; 23 audited packages, 0 vulnerabilities.
- `cargo fmt --check`: passed.
- `npm test`: passed: 6 Rust unit tests, 3 CLI integration tests, and 3 site
  contract tests.
- `cargo clippy --all-targets --locked -- -D warnings`: passed.
- `npm run build`: passed: locked release CLI build, real demo generation, and
  Vite site build to `dist/site/`.
- `npm run package`: passed; produced
  `knowledge-handoff-bundle-0.1.0.crate` (40,019 bytes). No publishing was
  performed.
- `cargo test --doc --locked` is not applicable: Cargo correctly reports that
  this binary-only package has no library target/doctests.

### CLI consumer and recovery paths

The packed crate was extracted into a clean consumer directory, installed with
`cargo install --path <unpacked-crate> --root <clean-root> --locked`, and ran
as `khb 0.1.0`. `khb --help` documents the single binary, four commands,
global `--json`/`--ci`, and exit codes.

- A normal Atlas handoff checked successfully, built a portable bundle with 3
  artifacts and 1 gap, copied and SHA-256 hashed local files, and exported an
  acknowledgement for `Sam Rivera` with accepted `architecture` and a 64-char
  manifest hash.
- The representative public-link failure (`https://example.com/status`) with
  `--check-links` returned JSON `link.http` / HTTP 404 and exit `3`.
- Invalid data returned exit `2` and JSON findings for malformed date, expired
  date warning, `..` path, duplicate id, and credential-bearing URL. The JSON
  did not leak the test password/token value.
- Re-running `init` on an existing YAML returned exit `4` and left it intact.
- Empty acknowledgement recipient returned the clear JSON error `Recipient
  name cannot be empty`.

### Generated bundle and browser QA

Locally served production output was exercised at desktop 1366x900 and mobile
390x844.

- Landing page: one `h1`, `main`, title, first keyboard Tab reaches the visible
  “Skip to content” link; reduced-motion computed animation is `none`.
- Demo: 3 artifact tracks and 1 known gap render; the empty-recipient recovery
  message appears; “Needs attention” filters to one track; a reviewed checkbox
  persists during the session; export downloads `acknowledgement.json` and
  reports the reviewed count.
- No console errors, page errors, or outbound runtime requests occurred in the
  local desktop/mobile browser runs. Generated acknowledgement state is local
  storage and the export is a local download.
- `npm run audit:a11y` passed at 390px: `/`, `/demo/`, `/privacy/`, and
  `/terms/` each had 0 axe violations and 0 serious/critical findings.
- The live HTTPS page was also browser-tested: no console/page errors or
  outbound requests; service-worker control was established and an offline
  reload returned 200 with the expected title and one `h1`.

### Budget and deployment identity

- Candidate build landing initial JS: 1,087 B; CSS: 7,599 B; hero WebP:
  128,886 B. All are below the 200 KB JS, 50 KB CSS, and 300 KB image budgets;
  no remote fonts are loaded.
- Local Lighthouse could not complete because the supplied Chromium tab crashed
  during final screenshot/BFCache collection. It therefore supplies no
  independent score; axe, Playwright interaction, and static budget evidence
  above were completed.
- Live `/` is byte-identical to the candidate output. Live `/demo/` has the
  same candidate content and copied-file hashes; its generated timestamp and
  manifest hash differ, as expected for a newly generated demo.

## Required next steps

1. Configure the actual static host to emit the CSP, Permissions-Policy,
   frame protection, and immutable hashed-asset caching declared by the
   product, then verify with fresh HTTPS headers.
2. Version service-worker caches per release and ensure the install path
   refreshes the HTML shell; verify an existing client receives a changed
   release before offline reload.
3. Re-run this verification after deployment and capture a successful
   Lighthouse mobile audit if the verifier environment permits it.
