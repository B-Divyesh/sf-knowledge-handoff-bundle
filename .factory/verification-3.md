# Independent verification 3 — FAIL

**Candidate:** `16c9c9fd5da452876824369301ad135976f8ad1d` (`docs: record verified repair release`)

**Production URL:** https://knowledge-handoff-bundle.sociobot.in/

**Date:** 2026-08-28 UTC

## Verdict

**FAIL.** The core `build --check-links` command exits successfully and emits `"ok":true` after a requested public-link check has found an HTTP 404. This contradicts the documented exit-code contract (`3` for a network/link-check failure) and lets a CI/release script treat a known broken source of truth as a successful handoff build. It is material to the brief's requirement that link checks make broken links first-class.

## Release-blocking defect

### P1 — `khb build --check-links` returns success for a broken checked link

Reproduced against a local public HTTP fixture with `robots.txt`, one `200`, one `404`, and one robots-disallowed path:

```text
khb --json check links.yaml --check-links
exit 3
{"ok":false, ... "errors":1, ... "link.http":"Link returned HTTP 404"}

khb --json build links.yaml --output links-bundle --check-links
exit 0
{"ok":true, ... "summary":{"errors":1,"warnings":1,...}}
```

The generated bundle truthfully stamps the bad link broken, but the command which was explicitly asked to check links gives automation a successful result. `--ci` changes the build failure to exit 2 rather than the documented link failure exit 3, so it does not repair the exit-code contract.

## Clean-checkout quality gates

The checkout started clean at the requested candidate. All available gates passed before the independent behavioral finding above:

```sh
npm ci
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm test
npm run build
npm run package
git diff --check
```

`npm ci` installed 23 packages with 0 vulnerabilities. `npm test` completed its Rust unit/CLI integration and Node/browser contract tests. The exact production build wrote `dist/site/`; `cargo package` produced `target/package/knowledge-handoff-bundle-0.1.0.crate` (42,656 bytes). No separate TypeScript type-check or lint script is defined; Vite's production build is the repository's available frontend compilation check.

## CLI end-to-end and boundary evidence

- A normal YAML handoff with one required local file, one public URL, an owner, and a known gap checked and built successfully. The file was copied and SHA-256 hashed; the manifest recorded two artifacts, one required artifact, and one gap.
- `acknowledge` produced a receipt tied to the exact manifest SHA-256, sorted and deduplicated repeated `--accept` values. A blank recipient and unknown artifact both returned exit 2 with JSON errors.
- Credential URLs, credential-like query keys, `..` paths, and a symlink which resolves outside the handoff directory each returned exit 2 before output. A capture server received **0 requests** for `http://user:pass@127.0.0.1/...` even with `--check-links` requested.
- A past project expiry and empty sections were reported as warnings; `--ci` returned exit 2, as documented for warnings in CI mode.
- Link checking made one robots request and rate-limited the two checked paths from the same origin (two-second run); it did not request the robots-disallowed path. `check --check-links` correctly returned exit 3 for the 404. The corresponding `build` behavior is the P1 above.

### Packed consumer installation

The packaged crate was extracted into a fresh temporary consumer directory and installed with Cargo into an isolated `--root`. The installed single `khb` binary provided 26 lines of help, then successfully ran the documented JSON `check` and `build` flow from that unrelated directory. Its manifest format was `knowledge-handoff-bundle/1` and its generated `index.html` existed.

## Static site, accessibility, privacy, and PWA evidence

Local production output and the live site were exercised in Chromium at 1366 x 900 and 390 x 844:

- The real demo has one `h1`, no console/page errors, and three 153.625 x 44 px artifact links at both viewports. Blank acknowledgement export gives the recipient recovery message, and entering `Sam Rivera` plus reviewing an item downloads `acknowledgement.json`.
- Keyboard-only testing reaches the `Skip to content` link first. Its live focus outline is a visible solid 3 px ring. `prefers-reduced-motion: reduce` makes the hero animation `none` with a `0s` duration.
- `npm run audit:a11y` against the exact local build reported zero total and zero serious/critical axe violations for `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and a missing route. The same six-route live audit also reported zero total/serious/critical violations.
- Playwright request capture on desktop and mobile observed only the product origin: no third-party scripts, fonts, analytics, or tracking. Local storage is used only by the generated recipient-review page.
- On live HTTPS, the service worker became controlling after registration and update was invoked. Reloading the landing page offline succeeded with the expected title and one `h1`, without page errors.

## Production deployment and response policy

Fresh live `/`, `/demo/`, `/privacy/`, `/terms/`, and `/sw.js` responses have the expected strict CSP, `Permissions-Policy: camera=(), microphone=(), geolocation=()`, HSTS, `Referrer-Policy: strict-origin-when-cross-origin`, and `X-Content-Type-Options: nosniff`. HTML is `public, max-age=0, must-revalidate`; `sw.js` is `no-cache`; hashed JS and the WebP use `public, max-age=31536000, immutable`.

Both `/does-not-exist` and `/assets/does-not-exist.js` are HTTP 404 and retain the same restrictive response policies.

Local/live SHA-256 values match exactly for `index.html`, privacy, terms, `404.html`, `sw.js`, the hero WebP, and both hashed JS/CSS assets. The demo HTML and manifest differ only in their intentional generated timestamp, date label, and derived manifest SHA-256: the live demo was generated 2026-08-27 and the fresh candidate build 2026-08-28; all substantive fixture content and assets match. This establishes behavioral/source equivalence but also means the demo artifact is not byte-reproducible without fixing its build time.

## Performance and budgets

The exact build contains 1,087 B initial JS, 7,599 B CSS, no font files, and a 128,886 B WebP—within the 200 KB / 50 KB / 120 KB / 300 KB budgets.

A fresh mobile Lighthouse retry recorded Performance 94, Accessibility 100, Best Practices 100, SEO 100, LCP 1,505 ms, and CLS 0. The Lighthouse runner reported `TARGET_CRASHED` only while cleaning up after it had already written the report. An earlier identical-run report was Performance 78 solely because TBT was sampled at 1,008 ms during that unstable target; its LCP was 1,510 ms and CLS 0. The successful retry is the retained measurement, but the runner instability is noted for follow-up.

## Required remediation and rerun

Make `khb build --check-links` return a non-zero link-check failure (documented exit 3) whenever checked links yield error findings, and emit JSON with `"ok":false`. Add a regression test for this normal non-CI build path. Re-run the full gate suite, packed-consumer test, and verification after the fix.
