# Build a portable project handoff — review 2

## Verdict

**FAIL — 3 findings and 2 untested claims.**

The CLI, demo workflow, installed package, live routes, privacy isolation,
offline behavior, and performance all work. This review cannot pass because
one registered storage claim is false as written, one credential-safety claim
is not fully exercised by its declared command, and the desktop demo exposes
two banner landmarks.

- Review date: 2026-09-06 UTC
- Live URL: https://knowledge-handoff-bundle.sociobot.in
- Implementation reviewed: `788c736e922336f4078b5c0bb1242ef5f40a8801`
- Documentation reviewed: `cf9c00699bfb11f63132e7dfee0e8cfa106385ab`
- Review base: `abbcd8bc92ed58315d13e10bfa92fe54262e0652`

Commits after `788c736` change only `.factory` reports. The live landing,
legal pages, 404, scripts, styles, images, and demo assets byte-match the clean
build. The live demo manifest also matches after excluding its expected
generation timestamp.

## Job, audience, and first action

Before scrolling in fresh 1366 × 900 and 390 × 844 browsers:

- Job: build a portable project handoff from a checklist, files, and links.
- Audience: departing owners and small teams handing work to the next owner.
- First action: `Try it with sample data`.

The job, audience, action, three product facts, and the result of the action are
visible in both viewports.

## Findings

### F-01 — Medium — `landing-stores-nothing` is false and incompletely tested

`.factory/claims.json` says, “The landing site stores nothing.” A fresh live
browser registers the service worker and creates Cache Storage entry
`khb-site-1655993f0bc1a4d3`. The declared test checks only `localStorage` and
cookies, so it exits 0 without checking Cache Storage, session storage,
IndexedDB, or service-worker registration.

The live site does keep user data stores and cookies empty, and the cache holds
only public site files. That narrower privacy statement would be accurate.
The categorical registered claim is not.

Required fix: either remove site persistence or narrow the claim to the user
data stores that remain empty. Make the tagged test inspect every storage
mechanism named by the revised claim.

### F-02 — Medium — The desktop demo has duplicate banner landmarks

At desktop width, `/demo/` renders both `.demo-site-header` and `.tape-head` as
visible `<header>` elements. Both become unnamed `banner` landmarks. Live Axe
4.13 reports two moderate violations:

- `landmark-no-duplicate-banner`
- `landmark-unique`

The repository audit runs only at 390 px. Its media query hides `.tape-head` at
that width, so `npm run audit:a11y` reports zero violations and misses the
desktop defect. This also makes the earlier “0 total” Axe statement incorrect
for desktop `/demo/`.

Required fix: make the generated tape metadata a non-landmark element, or give
the regions valid distinct semantics. Add a desktop demo viewport to the Axe
audit and require zero retained violations.

### F-03 — Low — The credential URL claim does not test both promised cases

The registered `credential-urls-rejected` claim promises rejection of both
credential-bearing URLs and credential-like query URLs. Its tagged test uses
one URL containing both `user:password@` and `?token=value`, then asserts only
`artifact.url_credentials`. Validation stops on the user-info branch, so the
declared claim command never reaches or proves the query-key branch.

An untagged Rust unit test covers a query-only URL, and source inspection shows
the product currently rejects it. The claims contract still requires the
declared command itself to prove the whole public claim.

Required fix: make the tagged claim test exercise separate user-info-only and
credential-query-only URLs, including no-fetch and no-output assertions.

## Claim command results

The registry has 25 unique IDs, and every tag appears exactly once. All 25
declared commands were run individually from the clean checkout; every command
exited 0. Outcome review found 23 fully supported claims and the two incomplete
claims in F-01 and F-03.

| Claim | Command | Claim review |
| --- | --- | --- |
| `build-static-bundle` | Pass | Supported |
| `copied-file-hash` | Pass | Supported |
| `expiry-warning` | Pass | Supported |
| `static-expiry-current` | Pass | Supported |
| `opt-in-link-check` | Pass | Supported |
| `origin-rate-limit` | Pass | Supported |
| `robots-respected` | Pass | Supported |
| `credential-urls-rejected` | Pass | Incomplete; F-03 |
| `no-link-crawling` | Pass | Supported |
| `bundle-offline` | Pass | Supported |
| `static-self-contained` | Pass | Supported |
| `review-state-local` | Pass | Supported |
| `acknowledgement-local-json` | Pass | Supported |
| `no-cli-telemetry` | Pass | Supported |
| `scoped-file-access` | Pass | Supported |
| `landing-stores-nothing` | Pass | False/incomplete; F-01 |
| `landing-no-third-party` | Pass | Supported |
| `json-output` | Pass | Supported |
| `ci-warnings-fail` | Pass | Supported |
| `init-no-overwrite` | Pass | Supported |
| `exit-codes` | Pass | Supported |
| `mit-free` | Pass | Supported |
| `no-account` | Pass | Supported |
| `demo-sandbox` | Pass | Supported |
| `demo-sample-results` | Pass | Supported |

No additional unlisted public claim was found in the landing page, README,
privacy page, terms page, CLI help, or generated bundle copy.

## Live demo and browser evidence

The one-click sample opens a populated Atlas migration handoff with four
artifacts, two copied files, a recorded HTTP 200, a recorded HTTP 404 rendered
as `Link needs replacement`, an expiry warning, and a production-access gap.
Its health is `Action required`.

The persistent `Demo — sample data, nothing is saved` banner remains visible
after scrolling. Review state survives reload under a `demo:` key. Reset demo
clears review state and form fields. Start for real clears the demo key and
returns home. A seeded `khb:real-sentinel` key remained unchanged through both
actions, proving that the demo did not alter the simulated real namespace.

Blank acknowledgement export announces the error and focuses Recipient name.
A valid export contained the recipient, reviewed artifact ID, note, UTC time,
and a 64-character manifest hash.

Phone and desktop sessions had no horizontal overflow. Effective controls were
at least 44 CSS px, including the labels around 22 px checkboxes. The sample
filter reduced the view to the one attention item. A 200% forced text-scale run
kept all five tested routes within the 390 px viewport.

Keyboard checks reached the skip link, navigation, demo controls, filters,
artifact actions, and review checkboxes. Focus uses a 3 px teal outline. Reduced
motion removes transitions and animations. All tested live routes except the
demo have zero Axe violations; the demo result is F-02.

Landing request capture used only the product origin and left cookies,
`localStorage`, `sessionStorage`, and IndexedDB empty. Cache Storage is the
exception described by F-01. The service worker controlled a fresh context,
completed an update, and reloaded the landing page offline with HTTP 200.

## Routes, links, deployment, and performance

`/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` return 200 with their
route-specific titles. A random missing route returns the intentional HTTP 404
and the designed `Page not found — Knowledge Handoff Bundle` page with a home
link. The expected 404 console message is not a product defect.

All navigable product, copied-file, GitHub, and RFC links returned 200. The
recorded broken sample address is text rather than a dead anchor. Normal and
404 responses include CSP, Permissions-Policy, HSTS, Referrer-Policy, and
nosniff. The service worker uses `no-cache`; versioned assets use immutable
caching.

Fresh mobile Lighthouse results:

- Performance: 99
- Accessibility: 100
- Best Practices: 100
- SEO: 100
- FCP: 821 ms
- LCP: 1,502 ms
- TBT: 144 ms
- CLS: 0
- transferred bytes: 135,733

The build contains 1,087 bytes of initial JavaScript, 7,997 bytes of CSS, no
font files, and a 128,886-byte hero image.

## Clean checkout and installed artifact

Documented prerequisites were installed before runtime tests.

| Command | Result |
| --- | --- |
| `npm ci` | Pass; 23 packages audited, 0 vulnerabilities |
| `npm test` | Pass; 7 Rust unit, 6 CLI integration, 4 browser/site, and 25 claim tests |
| All 25 registry commands separately | Pass as commands; 2 claim scopes are incomplete |
| `cargo fmt --check` | Pass |
| `cargo clippy --all-targets --locked -- -D warnings` | Pass |
| `npm run build` | Pass; produced `dist/site/` |
| `npm run audit:a11y` | Pass at its phone viewport; desktop gap is F-02 |
| `npm run package` | Pass; 32 files, 50.3 KiB compressed |
| `git diff --check` | Pass |

The packaged crate was installed into a new Cargo root. With a process
environment containing only `PATH` and deliberately unusable HTTP proxies,
the installed `khb --json demo` produced the four-artifact sample without an
account or network. The generated bundle opened from `file://` while offline,
filtered correctly, made only local-file requests, and logged no error.

Installed-artifact checks also passed valid acknowledgement export, duplicate
ID normalization, blank recipient, unknown ID, malformed input, missing file,
existing output, and documented exit codes 0, 2, 3, and 4. The clean suites
cover malformed YAML and dates, duplicate IDs, escaping files, robots denial,
checked 404s, CI warnings, expiry boundaries, and `Retry-After`.

This product has no backend, tenant store, health route, shared database, or
server-side restart state. Backend isolation and persistence checks do not
apply. The applicable 429 behavior is the CLI link checker; its local
`Retry-After` test passed.

AI assistance is not missing leverage. The brief excludes AI summarization,
and deterministic, inspectable output is central to this handoff format.

## Earlier finding disposition

| Earlier finding | Current disposition |
| --- | --- |
| Missing claim registry and tagged tests | Fixed for presence and execution: 25 IDs and 25 passing commands. F-01 and F-03 identify two remaining scope defects. |
| Sample was not an isolated demo | Fixed: live review/reset/exit preserved a seeded real namespace. |
| Sample link and expiry results were stale | Fixed: current recorded 200/404 and expiry status are visible. |
| `Retry-After` was ignored | Fixed: local 429 regression and claim command pass. |
| First screen used metaphor and omitted audience/action | Fixed on live desktop and phone. |
| Route metadata and skeleton were incomplete | Earlier metadata and route gaps are fixed. F-02 is a new demo landmark defect. |
| Landing links were below 44 px | Fixed at phone and desktop widths. |
| Accessibility command needed an undocumented server | Fixed. Its missing desktop coverage is documented in F-02. |
| Production omitted security/cache policies | Fixed on normal and intentional 404 responses. |
| Service-worker update could retain stale shell | Fixed: fresh update and offline reload pass. |
| Generated artifact actions were below 44 px | Fixed at phone and desktop widths. |
| Platform 404 omitted security headers | Fixed. |
| Checked broken-link build returned success | Fixed: installed artifact returns exit 3. |
| Lighthouse runner was unstable | Fixed: both fresh desktop and mobile runs exited 0. |

## Evidence

- `/work/.evidence/review2-desktop-landing.png`
- `/work/.evidence/review2-phone-landing.png`
- `/work/.evidence/review2-desktop-demo.png`
- `/work/.evidence/review2-phone-demo.png`
- `/work/.evidence/review2-phone-text-200.png`
- `/work/.evidence/review2-lighthouse.json`
- `/work/.evidence/review2-lighthouse-mobile.json`

No product code was modified during this review.
