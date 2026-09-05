# Build a portable project handoff — review 1

## Verdict

**FAIL — 8 findings and 22 untested public claims.**

The CLI completes its main build and acknowledgement job, and the earlier implementation defects remain fixed. This review cannot pass because the required claim registry is absent, the sample is not a controlled demo, the live sample does not show current link and expiry results, and the link checker ignores `Retry-After` before requesting the same origin again.

- Review date: 2026-09-05 UTC
- Live URL: https://knowledge-handoff-bundle.sociobot.in
- Implementation reviewed: `5143b6cf534f0b00ae3cca4a23b0f2eeede053cc`
- Deployment record: `c7e7b2442822502ad3e60fccfdbb30ac7db64230`
- Documentation reviewed: `f1c585d65b429433e52c081c445c98987fe17177`

The commits after `5143b6c` change only `.factory` reports. A clean build at `f1c585d` byte-matched the live landing page, privacy page, terms page, 404 page, service worker, hero image, JavaScript, and CSS. The live landing SHA-256 is `022cad3ee501be6b06b3170aed594d392bb9331a978c22a53f6333ed7e66e126`.

## Job, audience, and first action

- Job: turn a YAML checklist, local files, and public links into a portable project handoff.
- Audience: a departing project owner or small team and the next owner receiving the work.
- First action shown before scrolling: `Build your bundle`. A secondary `Open the live demo` link also appears.

The first screen does not state the audience. Its heading, `Leave the project. Not the context.`, does not name the job. The required `Try it with sample data` action is absent.

## Findings

### F-01 — High — Public claims have no claim registry or tagged tests

`.factory/claims.json` is missing, and `rg '@claim:'` finds no tagged test. There are therefore no declared claim commands to run. Existing general tests cover some behavior but do not meet the contract that each public claim has exactly one sandbox test.

This review found 22 distinct public claims without required claim tests:

1. The CLI turns YAML, files, and URLs into a portable static handoff.
2. Local files are copied and SHA-256 hashed.
3. Expiry warnings are shown.
4. Public links are checked only when requested.
5. Link checks are limited to one request per origin per second.
6. Link checks respect `robots.txt`.
7. Credential-bearing and credential-like query URLs are rejected before fetch or output.
8. Link checking does not crawl beyond the listed URLs and `robots.txt`.
9. Generated bundles work offline and from disk.
10. Generated bundles are self-contained and need no hosted runtime or database.
11. Review state stays in the recipient browser.
12. Acknowledgement export is local JSON tied to the manifest hash.
13. The CLI has no telemetry.
14. The CLI reads only listed files and writes only to the chosen output.
15. The landing site stores nothing.
16. The landing site has no analytics, advertising, cookies, remote scripts, or remote fonts.
17. Every command supports JSON output.
18. CI mode makes warnings fail validation.
19. `init` refuses to overwrite an existing file.
20. The documented `0`, `2`, `3`, and `4` exit-code meanings hold.
21. The product is free and open source under MIT.
22. The product needs no account.

Required fix: add `.factory/claims.json`, one `@claim:<id>` test for each retained claim, and remove or narrow claims that cannot be proved in the clean demo sandbox.

### F-02 — High — The sample is not the required demo sandbox

The live sample is reachable in one click, but it has no persistent `Demo — sample data, nothing is saved` label, no `Reset demo`, and no `Start for real`. Reviewing an artifact writes `khb:<manifest-hash>:reviewed` to local storage and the value survives reload. A fresh context had no prior keys, no third-party requests occurred, and this site has no separate real workspace to change, but the required explicit demo namespace and exit/reset controls are absent.

The installed `khb 0.1.0` artifact also rejects `khb demo` as an unknown command. The landing page shows command text rather than a recording of the real binary running bundled sample input. `.factory/demo.md` is missing.

Required fix: add `khb demo` that builds shipped sample data in a temporary directory and prints the output path. Make `/demo` an explicit sample mode with the required label, reset, and start-for-real actions, and document it in `.factory/demo.md`.

### F-03 — Medium — The live sample does not show current link and expiry results

The sample dashboard points to `https://example.com/status`, which returned HTTP 404 during the link crawl. The page labels it `unchecked · Link check was not requested`, so the sample does not demonstrate the product identifying a broken link.

The live sample was generated on 2026-08-28 and records zero warnings. On the review date, its 2026-09-30 expiry is within 30 days. The freshly installed CLI reports `expiry.soon`, while the live sample still says `Ready to review`. Review state and status do not refresh as the static bundle ages.

Required fix: use a realistic sample URL and generate the demo with checked reachable and broken examples. Make time-based status current when a recipient opens an older bundle, or clearly show that status is only valid at generation time.

### F-04 — Medium — Link checking ignores a server `Retry-After` window

A local origin returned `429` with `Retry-After: 5` for the first listed URL. The installed CLI requested a second URL on the same origin about 1.003 seconds later. The full run took 2.017 seconds and returned exit 3. Treating the first URL as an error is reasonable; sending the next request before the stated retry time is not.

Required fix: parse valid `Retry-After` seconds or dates, delay later requests to that origin, and add a deterministic regression test.

### F-05 — Medium — The first screen and section copy do not meet the plain-words contract

The heading does not name the job, the first sentence does not name the audience, and the main explanatory sentence has 24 words, above the 22-word cap. The title is 61 characters and uses the same slogan instead of the required plain description. Copy such as `The dropout`, `Three tracks. One handoff.`, `Hand over the tape`, `Side A`, and `Dub copy` uses metaphor or mood labels instead of section names. `.factory/copy-audit.md` is missing.

Required fix: use a job-naming heading of at most nine words, one audience sentence of at most 22 words, the required sample action, plain section headings, and a complete copy audit.

### F-06 — Medium — Required route metadata and the shared route structure are incomplete

The landing page has no canonical link, Open Graph fields, Twitter card fields, or Apple touch icon. Its meta description is 165 characters, above the 155-character limit. `/robots.txt` and `/sitemap.xml` return the designed 404 instead of the required files.

The demo title is `Atlas reporting migration — Knowledge handoff`, not `Demo — Knowledge Handoff Bundle`. The landing header omits Demo and Privacy, legal headers omit the standard navigation, and the designed 404 has no header, navigation, or footer. Footers omit `Built by Param Factory` and a version or build ID.

The HTTP 404 status itself is correct and is not a defect: `/does-not-exist` returns the styled page with one `h1`, a `main`, a home link, and the expected security headers.

### F-07 — Medium — Several landing links are shorter than the 44 px touch minimum

At 390 px, `Open the live demo`, `Read the source on GitHub`, Privacy, Terms, GitHub, and both KHB home links measure about 24.8 CSS px tall. The first-screen sample link is among the affected controls. The generated bundle's artifact links remain fixed at exactly 44 px, so the earlier artifact-link finding stays closed.

Required fix: give every visible link or its containing hit area a height and width of at least 44 CSS px with at least 8 px separation.

### F-08 — Low — A documented verification command fails from the stated setup

The current handoff tells a clean worker to run `npm run audit:a11y`. Running that exact command after `npm ci` and `npm run build` fails immediately with `net::ERR_CONNECTION_REFUSED` because no server is started on port 4173.

Running `node scripts/a11y.mjs https://knowledge-handoff-bundle.sociobot.in` directly does work and reports zero axe violations on all six routes. The documented command must start its own preview server or the instructions must include that prerequisite.

## Earlier finding status

| Earlier finding | Current evidence | Status |
| --- | --- | --- |
| Production omitted CSP, Permissions-Policy, and cache rules | Fresh 200 and controlled 404 responses include CSP, Permissions-Policy, HSTS, Referrer-Policy, and nosniff. Cache rules match the source configuration. | Fixed |
| Service-worker updates could keep stale shell files | The release-fingerprint regression test passes. The live worker controls a fresh context, updates, and reloads the landing page offline with HTTP 200. | Fixed |
| Generated artifact links were 24.8 px tall | All three live artifact actions measure 153.625 × 44 CSS px at 390 px. | Fixed |
| Platform 404 responses omitted security headers | `/does-not-exist` and `/assets/does-not-exist.js` return 404 with the same required headers. | Fixed |
| `build --check-links` returned success after a checked 404 | The installed artifact returns exit 3 and JSON `ok:false`; the Rust integration regression passes. | Fixed |
| Lighthouse runner crashed after recording performance | The current run again ended with a tab crash after writing a valid report. It recorded Performance 100, Accessibility 100, Best Practices 100, SEO 100, LCP 1.516 s, CLS 0, and TBT 60 ms. | Product result passes; runner remains unstable |

## Verification results

### Clean checkout and package

The clean checkout was `f1c585d` with product code unchanged from `5143b6c`.

| Command | Result |
| --- | --- |
| `npm ci` | Pass; 23 packages audited, 0 vulnerabilities |
| `cargo fmt --check` | Pass |
| `cargo clippy --all-targets --locked -- -D warnings` | Pass |
| `npm test` | Pass; 6 unit, 4 CLI integration, and 6 Node/browser tests |
| `npm run build` | Pass; `dist/site/` produced |
| `npm run package` | Pass; 42.2 KiB crate produced and verified |
| `git diff --check` | Pass |
| `npm run audit:a11y` | Fail; documented server prerequisite is missing (F-08) |

The packaged crate was extracted and installed into an isolated Cargo root. The installed binary reports `khb 0.1.0` and provides `init`, `check`, `build`, and `acknowledge`.

### CLI behavior

- Normal Atlas check and build complete with 3 artifacts, 3 required items, 2 verified copied files, 1 known gap, and the now-current expiry warning.
- The copied architecture file hash exactly matches the source hash.
- Acknowledgement export de-duplicates accepted IDs, sorts them, and records a 64-character manifest hash.
- Blank recipient and unknown artifact ID return exit 2 with clear JSON errors.
- A warning returns exit 2 in CI mode.
- Repeated `init` and a non-empty output directory return exit 4 without overwrite.
- A duplicate ID, a file symlink escaping the input directory, and a credential-bearing URL each return exit 2. The credential case is rejected before link checking.
- The public sample link returns exit 3 and JSON `link.http` for HTTP 404.
- A built bundle opened directly from `file://` with no network. Filtering, review, and acknowledgement download worked without console errors.

### Live browser, accessibility, privacy, and links

- Fresh Chromium contexts covered 1366 × 900 and 390 × 844.
- Landing, demo, privacy, terms, and 404 pages have one `h1` and `main`; legal route titles are correct.
- Keyboard order reaches all demo controls, Space operates filters and checkboxes, focus rings are visible, and blank export returns focus to the recipient field.
- Reduced motion removes hero animation and transition.
- The demo filters from 3 artifacts to 1 attention item and exports a valid acknowledgement.
- No console errors, page errors, analytics, trackers, remote fonts, or third-party runtime requests appeared.
- The privacy page explains local review storage, deletion, hosting logs, and where to raise a question.
- The live axe audit reports 0 total violations on `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and `/does-not-exist`.
- The factory URL verifier passes title, language, one `h1`, `main`, image alt text, and console checks.
- All crawled links return 200 except the sample dashboard URL in F-03.
- The expected missing route returns 404 and a usable page.
- There is no backend, tenant store, health endpoint, or server-side rate limit to test. SQLite and restart persistence do not apply.

### Size and performance

- Initial JavaScript: 1,087 bytes.
- CSS: 7,599 bytes.
- Hero WebP: 128,886 bytes.
- Fonts: none.
- Lighthouse report: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.938 s, LCP 1.516 s, CLS 0, TBT 60 ms. The process reported a Chromium tab crash after writing the report.

## Evidence files

- `/work/.evidence/live-browser-audit.json`
- `/work/.evidence/live-desktop-landing.png`
- `/work/.evidence/live-phone-landing.png`
- `/work/.evidence/live-desktop-demo.png`
- `/work/.evidence/live-phone-demo.png`
- `/work/.evidence/verify.json`
- `/work/.evidence/lighthouse-live.json`

No product code was changed during this review.
