# Build a project handoff — review 3

## Verdict

**PASS — 0 findings and 0 untested claims.**

This independent review checked implementation `55a75c27f029c3315831d9bf22d078b41672ac51`, documentation candidate `5c10cdda2d8a665bc0cd07bf02a36bb004988179`, and the deployed site at <https://knowledge-handoff-bundle.sociobot.in>. Commit `d44a5e05cdfc7b3c66e3ffbf029aed50fd4274d2` contains reports only; it does not require another product image.

## Job, audience, and first action

Fresh desktop (1366 × 900) and phone (390 × 844) sessions showed this before scrolling:

- Job: **Build a project handoff**.
- Audience: departing owners and small teams giving work to the next owner.
- First action: **Try it with sample data**; it opens a realistic bundle with a broken link and a missing-access gap.

The first screen also shows the three checked facts: MIT-free, no account or telemetry, and bundles work offline.

## Clean checkout and claims

I cloned `d44a5e0` into a new temporary checkout and ran `npm ci` successfully (23 packages, zero vulnerabilities). All 25 exact commands declared in `.factory/claims.json` were run individually from that checkout; each passed. The complete `npm test` run independently passed all 25 tagged tests as well.

| Claim IDs | Result |
| --- | --- |
| `build-static-bundle`, `copied-file-hash`, `expiry-warning`, `static-expiry-current`, `opt-in-link-check` | Pass |
| `origin-rate-limit`, `robots-respected`, `credential-urls-rejected`, `no-link-crawling`, `bundle-offline` | Pass |
| `static-self-contained`, `review-state-local`, `acknowledgement-local-json`, `no-cli-telemetry`, `scoped-file-access` | Pass |
| `landing-storage-disclosed`, `landing-no-third-party`, `json-output`, `ci-warnings-fail`, `init-no-overwrite` | Pass |
| `exit-codes`, `mit-free`, `no-account`, `demo-sandbox`, `demo-sample-results` | Pass |

The direct clean gate results were:

| Command | Result |
| --- | --- |
| `cargo fmt --check` | Pass |
| `cargo clippy --all-targets --locked -- -D warnings` | Pass |
| `npm test` | Pass: 7 Rust unit, 6 Rust integration, 6 site, 25 claim tests |
| `npm run build` | Pass; produces `dist/site/` |
| `npm run audit:a11y` | Pass; Axe found zero violations on six routes at phone and desktop sizes |
| `npm run package` | Pass; package verification succeeds |
| `git diff --check` | Pass |

The claim checks cover normal, invalid, boundary, and recovery paths: malformed input, missing/escaping files, duplicate IDs, warning-as-failure CI, safe no-overwrite behavior, both credential URL forms, link failure, robots denial, per-origin `429`/`Retry-After`, and documented exit codes 0, 2, 3, and 4. No public claim in the landing page, README, privacy/terms pages, CLI help, or generated bundle was missing from the registry.

## Installed consumer flow

The packed crate was installed into a separate Cargo root. With an environment containing only `PATH`, installed `khb 0.1.0` ran `--json demo`, produced the bundled four-artifact Atlas handoff (one gap, one expiry warning, and recorded reachable/broken link outcomes), and exported an acknowledgement for `Sam Rivera`. The acknowledgement accepted `architecture` and had a 64-character manifest SHA-256.

The generated `file://` bundle opened in a browser already set offline: it had one `h1`, one `main`, four artifacts, no horizontal overflow, and no console errors.

## Live site, demo, accessibility, privacy, and routes

Fresh live phone and desktop sessions had no console errors, no third-party requests, and no horizontal overflow. Axe found zero violations on `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and `/does-not-exist` at both viewport sizes. The skip link moves focus to `main`; native filters and review controls are keyboard-operable; visible focus styling remains; and the local test suite verifies 200% text reflow and reduced-motion behavior.

The live one-click demo showed four populated artifacts, named owners, two copied files, a recorded HTTP 200, a recorded HTTP 404 marked for replacement, an expiry warning, and a production-access gap. Its persistent label reads `Demo — sample data, nothing is saved`. Reviewing an item used a `demo:` key only. Reset removed that key and cleared fields; Start for real returned home and left a seeded `khb:real-sentinel` unchanged.

A fresh landing visit left cookies, localStorage, sessionStorage, and IndexedDB empty. Its registered service worker cached only four same-origin site files. After worker activation and reload, the landing reloaded offline with HTTP 200, the correct title, and one `h1`.

Normal routes returned 200 with route-specific titles. The deliberate missing route returned the designed HTTP 404 with header, navigation, main content, footer, and a home link; this expected 404 is not a defect. Internal routes/assets and the product GitHub link returned successful responses. Normal and 404 responses include CSP, HSTS, Referrer-Policy, Permissions-Policy, and `X-Content-Type-Options: nosniff`.

The live landing, legal pages, 404, worker, images, robots file, sitemap, and hashed JS/CSS match the candidate build byte-for-byte. The generated demo has the same semantic manifest and assets; its `generated_at` value and derived manifest hash vary by build as expected.

This is a static CLI product. It has no backend tenant store, SQLite state, health endpoint, or server-side restart persistence. The applicable request-limit behavior is the CLI link checker, covered by the passing `origin-rate-limit` claim.

## Earlier findings

| Earlier finding | Current disposition and evidence |
| --- | --- |
| Missing claim registry/tagged tests | Fixed: 25 unique claim IDs and tags; all exact commands pass. |
| Sample lacked an isolated web/CLI demo | Fixed: `/demo/`, `demo:` storage, reset/exit controls, `khb demo`, and `.factory/demo.md` work. |
| Sample results were stale or unchecked | Fixed: recorded HTTP 200/404 results and browser-current expiry status are shown. |
| `Retry-After` was ignored | Fixed: deterministic 429 per-origin test passes. |
| First screen or copy was metaphorical/incomplete | Fixed: job, audience, action, result, and facts are visible before scrolling; copy audit passes. |
| Metadata, routing, legal pages, skeleton, or 404 were incomplete | Fixed: live route, title, metadata, nav/footer, robots, sitemap, and intentional-404 checks pass. |
| Landing or artifact actions were too small | Fixed: local phone/desktop touch-control checks pass. |
| Accessibility audit lacked its server or missed desktop landmarks | Fixed: documented command serves its own build and reports zero Axe violations at both sizes. |
| CSP/cache/404 response policies were absent | Fixed: live normal and intentional-404 header checks pass. |
| Service worker could retain stale shell files | Fixed: cache-fingerprint regression plus live offline reload pass. |
| Checked broken links returned success | Fixed: exit-code claim and integration test pass. |
| Lighthouse cleanup was unstable | No current product defect: local performance-size budget, browser, and Axe gates pass. |
| Landing storage claim was false/incomplete | Fixed: narrowed storage/cache disclosure and full browser-store test pass. |
| Desktop demo had duplicate banner landmarks | Fixed: desktop site test and Axe pass with one banner. |
| Credential claim skipped query-only URLs | Fixed: independent user-info and query-key cases pass with no fetch or output. |
| Skip links did not move focus | Fixed: site tests assert focus enters `main`. |
| Pages overflowed at 200% text size | Fixed: all reviewed routes pass the reflow test. |

## Evidence

- Fresh captures: `/work/.evidence/review-3-desktop-landing.png`, `/work/.evidence/review-3-phone-landing.png`, and `/work/.evidence/review-3-desktop-demo.png`.
- This report is copied to `/work/.evidence/qa-report.md`.
- `/work/.evidence/qa-result.json` records the matching PASS verdict.

No product code was modified during this review.
