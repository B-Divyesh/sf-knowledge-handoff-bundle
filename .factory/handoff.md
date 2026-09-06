# Build a portable project handoff — repair 5 handoff

## Status

**PASS — 0 known findings and 0 untested claims.**

Repair 5 resolves all three strict-review findings. The free Rust CLI, static
site, demo sandbox, offline behavior, and installed consumer artifact work end
to end. No paid offer or external integration applies to this product.

## Release identity

- Implementation and deployed artifact: `55a75c27f029c3315831d9bf22d078b41672ac51`
- Previous failed review report: `22710a553be3ed1e2d7fa4351d804419974b399b`
- Documentation: the report-only commit containing this handoff; its exact SHA
  is recorded in `/work/.evidence/verdict.json` after commit.
- Live URL: https://knowledge-handoff-bundle.sociobot.in
- Deployment target: existing Azure Static Web App
  `sf-knowledge-handoff-bundle`, production environment, eastus2

The live landing, demo, privacy, terms, 404, and service-worker files match the
local build byte for byte. Later report-only commits do not require a new site
image.

## What changed

1. Replaced the false “landing stores nothing” claim with a precise claim. On
   a fresh visit, cookies, localStorage, sessionStorage, and IndexedDB remain
   empty. The service worker's same-origin file cache is now disclosed and
   tested after activation and reload.
2. Changed generated bundle metadata from a second `<header>` to a named
   complementary landmark. The desktop demo now has one banner landmark.
3. Expanded the credential claim test into separate user-info and
   credential-query cases. Both assert exit 2, the correct finding, no network
   request, no generated output, and no secret value in process output.
4. Made the accessibility audit cover phone and desktop sizes and fail on any
   Axe violation, including moderate landmark defects.
5. Fixed two additional accessibility issues found during cold verification:
   skip links now move focus into `<main>`, and every site route reflows at
   200% text size without horizontal scrolling.

## Clean-checkout verification

The final implementation was checked from a clean clone at
`55a75c27f029c3315831d9bf22d078b41672ac51` after `npm ci`:

| Check | Result |
| --- | --- |
| `cargo fmt --check` | Pass |
| `cargo clippy --all-targets --locked -- -D warnings` | Pass |
| `npm test` | Pass: 7 Rust unit, 6 Rust integration, 6 browser/site, and 25 claim tests |
| Every command in `.factory/claims.json`, run separately | Pass: 25/25 |
| `npm run build` | Pass; `dist/site/` produced |
| `npm run audit:a11y` | Pass; zero Axe violations across 6 routes at phone and desktop widths |
| `npm run package` | Pass; 32 files, 52,853 bytes compressed |
| `git diff --check` | Pass |

There are 25 unique claim IDs and exactly one matching test tag for each. The
revised storage and credential tests inspect observable browser, filesystem,
process, and request outcomes rather than source strings.

## Installed CLI and generated bundle

The final `.crate` was installed into a new Cargo root. With a process
environment containing only `PATH`, installed `khb 0.1.0`:

- built the four-artifact Atlas sample with one known gap, one expiry warning,
  and recorded HTTP 200 and HTTP 404 outcomes;
- exported a local acknowledgement for `Sam Rivera`, with one reviewed ID and
  a 64-character manifest SHA-256;
- opened its generated bundle from `file://` while the browser was offline;
- filtered to the one attention item, moved skip-link focus into the main
  content, and reflowed at 200% text size without horizontal scrolling.

The clean automated suites also cover malformed YAML and dates, missing and
escaping files, duplicate IDs, blank and unknown acknowledgement inputs,
output conflicts, robots denial, checked 404s, `Retry-After`, CI warnings, and
exit codes 0, 2, 3, and 4.

## Live HTTPS verification

Fresh Chromium contexts at 1366 × 900 and 390 × 844 verified the deployed
site. Before scrolling, both show the job (`Build a project handoff`), the
audience (departing owners and small teams), the `Try it with sample data`
action, what it opens, and three product facts.

The one-click demo opens four realistic artifacts, named owners, two copied
files, reachable and broken recorded link results, an expiry warning, and a
production-access gap. The sample banner stays visible while scrolling. Review
state survives reload under one `demo:` key. Reset clears sample state and
fields. Start for real clears sample state and returns home. A seeded
`khb:real-sentinel` remained unchanged throughout.

Blank export announces its recovery message and focuses Recipient name. A
valid download contains the recipient, reviewed item, timestamp, and manifest
hash. Phone and desktop have no horizontal overflow, all checked controls are
at least 44 × 44 CSS px, skip links move focus to main content, reduced motion
removes animation and transitions, and all five page types fit at 200% text.

Other live results:

- Factory URL verifier: HTTPS 200, correct title and language, one `h1`, one
  main landmark, complete image alternatives, labelled buttons, zero console
  errors; measured load 560 ms.
- Axe: zero violations on `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`,
  and a missing route at both phone and desktop widths.
- Routes: correct titles and status codes; the deliberate missing route returns
  the designed HTTP 404 with header, main, footer, and route home.
- Links: all nine unique navigable product, copied-file, GitHub, and RFC links
  return a successful response. The sample 404 result is non-clickable text.
- Privacy: a fresh landing leaves cookies, localStorage, sessionStorage, and
  IndexedDB empty. One service-worker cache contains four same-origin site
  files. No third-party request occurred.
- Offline/update: the worker controls a fresh context, completes an update
  check, and reloads the landing page offline with HTTP 200.
- Headers: CSP, Permissions-Policy, HSTS, Referrer-Policy, and nosniff are
  present on normal and intentional 404 responses.

Final mobile Lighthouse 13.4.1 scores: Performance 100, Accessibility 100,
Best Practices 100, and SEO 100. FCP was 831 ms, LCP 1,511 ms, TBT 0 ms, CLS
0, and transferred bytes 135,851. The build contains 1,081 bytes of initial
JavaScript, 8,329 bytes of CSS, no fonts, and a 128,886-byte hero image.

## Earlier finding disposition

| Finding | Current disposition |
| --- | --- |
| Landing “stores nothing” claim | Fixed: narrowed disclosure and full storage/cache browser test pass |
| Duplicate desktop banner landmarks | Fixed: one banner plus named bundle-metadata landmark; desktop Axe passes |
| Credential claim skipped query-only case | Fixed: two independent fixtures prove both rejection branches, no fetch, and no output |
| Missing claim registry/tagged checks | Fixed: 25 unique claims, 25 unique tags, every command passes separately |
| Sample was not isolated | Fixed: demo namespace, persistent label, reset/exit, and real-sentinel checks pass |
| Sample link and expiry results were stale | Fixed: current expiry and recorded 200/404 outcomes are visible |
| `Retry-After` was ignored | Fixed: deterministic 429 delay test passes |
| First screen was metaphorical or incomplete | Fixed: job, audience, action, result, and facts appear before scrolling |
| Route metadata and skeleton were incomplete | Fixed: route titles, metadata, navigation, footers, sitemap, robots, and 404 pass |
| Landing links were below 44 px | Fixed at phone and desktop widths |
| Accessibility command needed a server | Fixed: command starts its own server and now checks both viewports |
| Production response policies were absent | Fixed on normal and deliberate 404 responses |
| Service-worker updates retained stale shell files | Fixed: release fingerprint and live offline update pass |
| Generated artifact actions were below 44 px | Fixed at phone and desktop widths |
| Platform 404 omitted security headers | Fixed on the designed HTTP 404 response |
| Checked broken-link build returned success | Fixed: exit 3 and `ok:false` regression pass |
| Lighthouse runner was unstable | Fixed in this run: Lighthouse exited 0 with all four scores |
| Skip links did not move focus | Fixed and covered on landing, legal, 404, and demo output |
| Pages overflowed at 200% text | Fixed and covered on landing, demo, privacy, terms, and 404 |

## Evidence and remaining work

Evidence is in `/work/.evidence/repair5-*`, including desktop and phone
screenshots, browser results, URL verification, and Lighthouse JSON. The
catalog description is 80 characters before its newline, starts with a verb,
and is copied to `/work/.evidence/catalog-description.txt`.

No known product defect remains. This static CLI product has no backend,
tenant store, health endpoint, server-side state, shared database, paid tier,
or billing registration. Registry publication remains a factory-owner action;
no package was published during this repair.
