# Build a portable project handoff — verification 5

## Verdict

**PASS — 0 findings and 0 untested claims.**

The live site, clean source checkout, packaged CLI, installed CLI, generated
bundle, and all 25 registered public claims passed independent verification.
No Critical, High, Medium, or Low findings remain.

- Work order: `knowledge-handoff-bundle-verify-5`
- Verified: 2026-09-06 UTC
- Live URL: https://knowledge-handoff-bundle.sociobot.in
- Implementation reviewed: `788c736e922336f4078b5c0bb1242ef5f40a8801`
- Documentation reviewed: `cf9c00699bfb11f63132e7dfee0e8cfa106385ab`

Only `.factory/handoff.md` changed between those two candidate SHAs. Product
code at the documentation SHA is identical to the implementation SHA.

## Job, audience, and first action

Before scrolling on fresh desktop and 390 px phone sessions, the live page
states:

- Job: **Build a project handoff**.
- Audience: departing owners and small teams handing files, link results,
  owners, and gaps to the next owner.
- First action: **Try it with sample data**. The adjacent text says it opens a
  realistic bundle with a broken link and a missing-access gap.

The first screen also states that the product is MIT-licensed, needs no account
or telemetry, and creates bundles that work offline. The title is `Knowledge
Handoff Bundle — Build project handoffs`.

## Findings

None. Finding count: **0**. Untested claim count: **0**.

The deliberate missing-route request returned HTTP 404 with the designed page,
one `h1`, a `main`, site navigation, footer, and a route home. That expected
404 is not a defect. Its browser resource message is the only console error in
the combined route crawl; normal routes and flows produced none.

## Deployment identity and CI

The deployment is current. The live origin reports `Last-Modified: Sun, 06 Sep
2026 00:06:56 GMT`. The live landing, privacy, terms, 404, robots, sitemap,
service worker, not-found CSS, images, hashed JavaScript/CSS, and demo
JavaScript/CSS byte-match the clean candidate build. The live landing SHA-256
is `e9cba1da11c7ff64a41401ddeb4372150ed540a5004f8a1edd9292d8486d9fbf`.

The live and local demo manifests differ only in `generated_at`; after removing
that field, both hash to
`1a1f031bd4fe209a0f4a1ef28883158743a2d97d6682edc25a20c5ae79092e38`.
Both contain 4 artifacts, 4 required items, 3 verified items, 1 warning, 1
recorded broken link, and 1 known gap.

Repository CI completed successfully for both candidates:

- Implementation run `34000043661`, SHA `788c736`, completed 2026-09-06
  00:00:14 UTC.
- Documentation run `34000385119`, SHA `cf9c006`, completed 2026-09-06
  00:07:58 UTC.

## Live demo and browser checks

Fresh Chromium contexts at 1366 × 900 and 390 × 844 covered the first screen,
full demo, reset, recovery, export, legal routes, missing route, offline reload,
and reduced motion.

- One keyboard-activated click opened `/demo/`, already populated with two
  copied files, a recorded HTTP 200 reference, a recorded HTTP 404 reference,
  an expiry warning, named owners, and a production-access gap.
- `Demo — sample data, nothing is saved`, `Reset demo`, and `Start for real`
  remain in a sticky header while scrolling.
- A review writes only a `demo:<manifest-hash>:reviewed` key. Reload restores
  it. Reset removes the key and clears all checks. Start for real removes demo
  state and returns to `/`; no `khb:` real-data key was read or written.
- `Needs attention` is keyboard-operable with Space and shows exactly one
  artifact. All restores four. Review checkboxes are keyboard-operable.
- Blank acknowledgement export announces `Enter the recipient name before
  exporting.` and returns focus to the recipient input. A valid export contains
  the receipt format, `Sam Rivera`, the selected artifact, and a 64-character
  manifest hash.
- Every effective interactive target measured at least 44 × 44 CSS px at both
  widths, including the 44 px checkbox labels and artifact actions. Neither
  page overflows horizontally.
- The first Tab reaches `Skip to content`; its focus ring computes to a 3 px
  solid `#174a5b` outline. Labels, native roles, pressed states, and the status
  live region are present.
- `prefers-reduced-motion: reduce` computes `animation-name: none` and a 0 s
  transition. The viewport metadata does not disable zoom.
- Fresh live Axe checks report 0 total, serious, or critical violations on `/`,
  `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and a missing route.
- The factory URL verifier reports HTTPS 200, title, `lang=en`, one `h1`, a
  `main`, no missing image alt, no unlabeled button, and no console error.
- Landing and the complete demo flow made no third-party request. The landing
  left browser storage empty. The privacy page explains browser-only review
  state, local deletion/export, optional public-link checks, and ordinary host
  logs.
- The service worker controlled a fresh context, completed an update check, and
  reloaded the landing page offline with HTTP 200, the correct title, and one
  `h1`.
- `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` return 200 with their
  route-specific titles. A random missing route returns the intentional 404 and
  the designed `Page not found — Knowledge Handoff Bundle` page.
- All navigable product, copied-file, GitHub, and RFC reference links returned
  200. The recorded broken sample URL is deliberately rendered as `Link needs
  replacement`, not as a clickable dead link.

Response checks found CSP, Permissions-Policy, HSTS, Referrer-Policy, and
nosniff on normal and 404 responses. HTML revalidates, the service worker uses
`no-cache`, and versioned assets use one-year immutable caching.

## Claims

`.factory/claims.json` has 25 entries. Each tag occurs exactly once in
`tests/claims.test.mjs`. From a clean checkout after `npm ci`, every exact
`test` command in the registry was run separately; all 25 exited 0.

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `build-static-bundle` | Pass | Fresh YAML produced HTML, manifest, assets, and copied files. |
| `copied-file-hash` | Pass | Copied bytes and manifest SHA-256 matched. |
| `expiry-warning` | Pass | A five-day expiry produced the warning. |
| `static-expiry-current` | Pass | An older bundle recalculated status from the recipient date. |
| `opt-in-link-check` | Pass | No request without the flag; a request occurred with it. |
| `origin-rate-limit` | Pass | A local 429/`Retry-After` origin delayed the later request. |
| `robots-respected` | Pass | A disallowed path was reported and not fetched. |
| `credential-urls-rejected` | Pass | Credential URLs were rejected before request or output. |
| `no-link-crawling` | Pass | Only listed URLs and `robots.txt` were requested. |
| `bundle-offline` | Pass | A generated `file://` bundle worked in an offline context. |
| `static-self-contained` | Pass | The bundle needed no hosted runtime or database. |
| `review-state-local` | Pass | Review state persisted only in browser storage. |
| `acknowledgement-local-json` | Pass | Local JSON export used the exact manifest hash. |
| `no-cli-telemetry` | Pass | The demo made no request through the recording proxy. |
| `scoped-file-access` | Pass | Only the listed file was copied to the selected output. |
| `landing-stores-nothing` | Pass | A fresh landing context retained no browser storage. |
| `landing-no-third-party` | Pass | Request capture found no analytics, ads, remote scripts, or fonts. |
| `json-output` | Pass | Init, check, build, demo, and acknowledge emitted valid JSON. |
| `ci-warnings-fail` | Pass | A warning returned exit 2 in CI mode. |
| `init-no-overwrite` | Pass | A second init preserved the file and returned exit 4. |
| `exit-codes` | Pass | Normal, invalid, broken-link, and filesystem paths returned 0, 2, 3, and 4. |
| `mit-free` | Pass | Cargo metadata and the shipped license both identify MIT. |
| `no-account` | Pass | The sample ran with an environment containing only `PATH`. |
| `demo-sandbox` | Pass | Sample review, reload, reset, exit, and namespace isolation passed. |
| `demo-sample-results` | Pass | The sample displays recorded reachable and broken outcomes. |

The landing, README, privacy page, terms page, CLI help, and generated-bundle
copy were cross-checked against the registry. No false, incomplete, unlisted,
or untested public claim was found.

## Clean checkout and installed artifact

The clean checkout was `cf9c006`, whose product tree matches implementation
`788c736`. Documented prerequisites were installed before runtime tests.

| Command | Result |
| --- | --- |
| `npm ci` | Pass; 23 packages audited, 0 vulnerabilities |
| `npm test` | Pass; 7 Rust unit, 6 CLI integration, 4 browser/site, and 25 claim tests |
| All 25 registry `test` commands, separately | Pass; 25/25, no skipped command |
| `cargo fmt --check` | Pass |
| `cargo clippy --all-targets --locked -- -D warnings` | Pass |
| `npm run build` | Pass; produced `dist/site/` |
| `npm run audit:a11y` | Pass; 0 violations on all six audited routes |
| `npm run package` | Pass; 32 files, 50.3 KiB compressed |
| `git diff --check` | Pass |

The packaged crate was installed into a new Cargo root from its verified
package directory. The installed `khb 0.1.0` exposed helpful command, JSON,
CI, and exit-code documentation. With an environment containing only `PATH`,
`khb --json demo` returned success and a temporary sample path.

Additional installed-artifact checks passed:

- An explicit sample output contained the realistic 4-artifact summary.
- Acknowledgement de-duplicated and sorted IDs and recorded a 64-character
  manifest hash.
- Blank recipient and unknown ID returned exit 2 with clear JSON recovery
  messages.
- Repeated init and a non-empty output returned exit 4 without overwrite.
- The installed demo opened from `file://` while offline, filtered to the one
  attention item, made only local file requests, and logged no error.

Normal, invalid, boundary, and recovery paths are also covered by the clean
claim and integration runs: malformed YAML and dates, missing and escaping
files, duplicate IDs, secret-like URLs, CI warnings, robots denial, checked
404s, local-storage reload/reset, and output conflicts.

This is a CLI plus static site. It has no backend, tenant store, health route,
shared database, or server-side restart persistence to test. SQLite tenant
isolation therefore does not apply. The applicable 429 behavior is the CLI's
per-origin public-link checker; its local 429/`Retry-After` claim and integration
tests passed.

AI assistance is not missing leverage here: the brief explicitly excludes AI
summarization, and the product's value is a deterministic, inspectable handoff.
It already provides the relevant export and portable-file path; hosted sync is
outside scope.

## Accessibility and performance

The exact build contains 1,087 bytes of initial JavaScript, 7,997 bytes of CSS,
no font assets, and a 128,886-byte hero WebP. The social image is 1200 × 630 and
the touch icon is 180 × 180.

A fresh live mobile Lighthouse run completed successfully:

- Performance 96
- Accessibility 100
- Best Practices 100
- SEO 100
- FCP 1,036 ms
- LCP 1,505 ms
- TBT 221 ms
- CLS 0

## Earlier finding disposition

| Earlier finding | Verification 5 evidence | Status |
| --- | --- | --- |
| Missing claim registry and tagged tests | 25 entries, exactly one tag each, 25 exact commands pass | Fixed |
| Sample was not an isolated demo | Web and installed CLI sandbox lifecycle passes | Fixed |
| Sample link and expiry results were stale | Recorded 200/404, current expiry logic, and claim pass | Fixed |
| `Retry-After` was ignored | Local 429 claim and CLI integration pass | Fixed |
| First screen used metaphor and omitted audience/action | Live first screen names job, audience, and sample action | Fixed |
| Route metadata and skeleton were incomplete | Live titles, metadata, robots, sitemap, headers, footers, and 404 pass | Fixed |
| Landing links were below 44 px | No effective desktop or phone target is below 44 px | Fixed |
| Accessibility command needed an undocumented server | Exact clean command starts its server and passes | Fixed |
| Production omitted security/cache policies | Normal and 404 live responses carry the declared policies | Fixed |
| Service-worker update could retain stale shell | Fingerprint test, live update, and offline reload pass | Fixed |
| Generated artifact actions were below 44 px | Live desktop and phone effective targets pass | Fixed |
| Platform 404 omitted security headers | Intentional live 404 retains all required headers | Fixed |
| Checked broken-link build returned success | Regression and claim return documented exit 3 / `ok:false` | Fixed |
| Lighthouse runner was unstable | Fresh verification 5 run exited 0 and wrote all four category scores | Fixed |

## Evidence

- `/work/.evidence/live-browser-audit.json`
- `/work/.evidence/live-desktop-landing.png`
- `/work/.evidence/live-phone-landing.png`
- `/work/.evidence/live-desktop-demo.png`
- `/work/.evidence/live-phone-demo.png`
- `/work/.evidence/verify-url/verify.json`
- `/work/.evidence/verify-url/screenshot-desktop.png`
- `/work/.evidence/verify-url/screenshot-mobile.png`
- `/work/.evidence/lighthouse-v5.json`

No product code was modified during verification.
