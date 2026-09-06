# Build a portable project handoff — verification 6

## Verdict

**PASS — 0 findings and 0 untested claims.**

- Work order: `knowledge-handoff-bundle-verify-6`
- Verified: 2026-09-06 UTC
- Live URL: https://knowledge-handoff-bundle.sociobot.in
- Implementation reviewed: `55a75c27f029c3315831d9bf22d078b41672ac51`
- Documentation reviewed: `5c10cdda2d8a665bc0cd07bf02a36bb004988179`

The documentation candidate differs from the implementation only in factory
handoff/report material. The product source and deployment candidate are the
implementation SHA above.

## Job, audience, and first action

Before scrolling in fresh 1366 × 900 desktop and 390 × 844 phone browsers, the
live page states:

- Job: **Build a project handoff**.
- Audience: departing owners and small teams giving the next owner files, link
  results, owners, and gaps.
- First action: **Try it with sample data**. It opens a realistic bundle with a
  broken link and a missing-access gap.

The same first screen shows the three concise facts: MIT-licensed, no account
or telemetry, and built bundles work offline. The landing title is
`Knowledge Handoff Bundle — Build project handoffs`.

## Findings

None. Finding count: **0**. Untested claim count: **0**.

The deliberate `/verify6-missing` request returned HTTP 404 with the designed
page, one `h1`, `main`, header, footer, and a route home. Its expected failed
resource console message is not a product defect; normal routes had no console
errors.

## Clean checkout and claims

A fresh clone of the documentation candidate was used. `npm ci` completed with
23 packages and zero vulnerabilities. These gates passed:

| Command | Result |
| --- | --- |
| `cargo fmt --check` | Pass |
| `cargo clippy --all-targets --locked -- -D warnings` | Pass |
| `npm test` | Pass: Rust unit/integration, site, and all claim tests |
| `npm run build` | Pass; produced `dist/site/` |
| `npm run audit:a11y` | Pass; zero Axe violations on six routes at phone and desktop widths |
| `npm run package` | Pass; verified 32-file crate, 51.6 KiB compressed |
| `git diff --check` | Pass |

All 25 commands declared in `.factory/claims.json` were run separately from
that clean checkout. Each exited 0 and ran its one matching tagged sandbox
test:

`build-static-bundle`, `copied-file-hash`, `expiry-warning`,
`static-expiry-current`, `opt-in-link-check`, `origin-rate-limit`,
`robots-respected`, `credential-urls-rejected`, `no-link-crawling`,
`bundle-offline`, `static-self-contained`, `review-state-local`,
`acknowledgement-local-json`, `no-cli-telemetry`, `scoped-file-access`,
`landing-storage-disclosed`, `landing-no-third-party`, `json-output`,
`ci-warnings-fail`, `init-no-overwrite`, `exit-codes`, `mit-free`,
`no-account`, `demo-sandbox`, and `demo-sample-results`.

The claim review also cross-checked landing, README, legal pages, CLI help, and
generated-bundle copy. No false, incomplete, or unlisted public claim remains.

## Installed artifact and recovery paths

The packed crate was extracted and installed into a separate Cargo root. The
installed `khb 0.1.0`, in an environment containing only `PATH`, ran
`--json demo` without an account or network configuration. It produced the
four-artifact Atlas sample with one recorded broken-link error, one warning,
and one known gap.

The installed binary exported a local acknowledgement for `Sam Rivera` with a
64-character manifest SHA-256. An empty recipient returned the clear JSON
error and documented exit 2. Its generated `file://` bundle opened while the
browser was offline with one main landmark and four artifacts.

Normal, invalid, boundary, and recovery behavior is covered by the individual
claims and the clean suite: malformed dates/YAML, missing and escaping files,
duplicate IDs, no-overwrite and output conflicts, blank and unknown
acknowledgements, CI warnings, robots denial, checked 404 links, credential
user-info and query strings, per-origin 429/`Retry-After`, and JSON output.

This is a static CLI product. It has no backend, tenant store, health route,
shared database, or server-side restart persistence. Tenant-isolation and
backend health checks do not apply. The applicable rate-limit behavior is the
CLI's public-link checker, which the exact `origin-rate-limit` claim verified.

## Live site, demo, privacy, and accessibility

Fresh desktop and phone contexts checked `/`, `/demo/`, `/privacy/`,
`/terms/`, `/404.html`, and a missing route. Every normal route returned 200,
its route-specific title, `lang=en`, exactly one `h1`, exactly one `main`, no
horizontal overflow, no third-party runtime request, and no console error.
The missing route returned the expected designed 404.

Fresh live Axe checks found zero violations on all six routes at both viewport
sizes. The local audit independently found the same result. The site has a
working skip link, visible focus, 44 px controls, keyboard-operable native
controls, readable legal pages, and reduced-motion styles. The clean site
suite verifies 200% text reflow on the landing, demo, privacy, terms, and 404
pages.

The one-click demo directly opens a populated Atlas handoff: four artifacts,
two copied files, recorded reachable and broken link outcomes, a current
expiry warning, named owners, and a production-access gap. The persistent
`Demo — sample data, nothing is saved` label, Reset demo, and Start for real
are present. The exact demo claim verifies review persistence only in the
`demo:` namespace, reset of sample state and fields, exit to home, and that a
normal `khb:` sentinel is unchanged. Blank acknowledgement export announces
its recovery message and focuses Recipient name; a valid export is local.

On a fresh live landing visit cookies, localStorage, sessionStorage, and
IndexedDB were empty. The service worker controlled the page and placed only
same-origin site files in its disclosed cache. The live landing then reloaded
offline with HTTP 200, the expected title, and one `h1`.

Normal and intentional-404 responses carry CSP, Permissions-Policy, HSTS,
Referrer-Policy, and `X-Content-Type-Options: nosniff`. The live HTML, legal
pages, 404, worker, hero image, and hashed JS/CSS assets byte-match the
candidate build. No user-data or payment feature exists beyond browser-local
review state described on the privacy page.

## Earlier finding disposition

| Earlier finding | Current evidence | Status |
| --- | --- | --- |
| Missing claim registry and tagged tests | 25 entries, one tag each, and all 25 exact commands pass | Fixed |
| Sample lacked isolated web/CLI demo controls | `/demo`, `demo:` namespace, reset/exit controls, `khb demo`, and demo documentation pass | Fixed |
| Sample results were stale or unchecked | Four-artifact recorded 200/404 sample and current expiry test pass | Fixed |
| `Retry-After` was ignored | Exact 429 per-origin claim passes | Fixed |
| Plain-language first screen and copy audit were incomplete | Live first screen names job, audience, action, and facts; audit is present | Fixed |
| Metadata, routes, skeleton, legal pages, or 404 were incomplete | Route/title/landmark/404 checks pass | Fixed |
| Landing or artifact actions were below 44 px | Desktop/phone site tests pass target checks | Fixed |
| Documented accessibility audit needed a server | Exact clean `npm run audit:a11y` starts its own server and passes | Fixed |
| CSP, cache, or platform-404 headers were absent | Fresh normal and missing-route response checks pass | Fixed |
| Worker could retain stale shell | Worker cache-fingerprint regression and live offline reload pass | Fixed |
| Checked-link build reported success | Exact exit-code claim and integration test return exit 3 and `ok:false` | Fixed |
| Lighthouse cleanup was unstable | Previous completed production measurement is recorded; current independent axe, budget, and browser checks pass | No product defect |
| Landing “stores nothing” claim omitted Cache Storage | Claim narrowed to disclosed user-data stores and same-origin worker cache; browser test passes | Fixed |
| Desktop demo had duplicate banner landmarks | Desktop and phone Axe checks have zero violations | Fixed |
| Credential claim skipped query-only URLs | Separate user-info and credential-query cases pass with no fetch or output | Fixed |

## Evidence

- `/work/.evidence/verification-6-desktop.png` and
  `/work/.evidence/verification-6-phone.png` are fresh landing-page captures.
- Live response, accessibility, storage, offline, and route checks were run
  directly against the URL above during this verification.

No product code was modified during verification.
