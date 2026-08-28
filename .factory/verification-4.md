# Independent verification 4 — PASS

**Work order:** `knowledge-handoff-bundle-verify-4`  
**Candidate:** `c7e7b2442822502ad3e60fccfdbb30ac7db64230` (`docs: record production deployment`)  
**Verified:** 2026-08-28 UTC  
**Live URL:** https://knowledge-handoff-bundle.sociobot.in/

## Decision

**PASS.** From a clean candidate checkout, the Rust CLI builds a portable, static handoff with copied-file SHA-256 values, owners, expiry/gap visibility, public-link checking, and local recipient acknowledgement export. The repaired `build --check-links` behavior now gives its required non-zero result for a broken checked link. The live static/PWA site is byte-identical to the candidate shell and assets, and its demo has the same substantive manifest content and copied-file hashes.

No product defects were found in this verification.

## Clean checkout and release gates

The checkout was clean at the candidate before installation. These commands passed:

```sh
npm ci
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm test
npm run build
npm run package
git diff --check
```

`npm ci` installed 22 packages (23 audited, 0 vulnerabilities). `npm test` passed all 6 Rust unit tests, 4 Rust CLI integration tests, and 6 Node/Playwright site tests. There is no separate TypeScript type-check or lint script; Vite's production compilation is the supplied frontend check. The exact production build completed its locked release Rust build, generated the Atlas demo, and emitted `dist/site/`. `cargo package --locked --allow-dirty` successfully created and verified `target/package/knowledge-handoff-bundle-0.1.0.crate` (149.7 KiB unpacked, 42.2 KiB compressed). No publishing was attempted.

## CLI and package end-to-end evidence

- `target/release/khb --json check examples/atlas/handoff.yaml` returned `ok:true`; a build produced 3 artifacts, 3 required artifacts, 2 copied and hashed files, and 1 explicit known gap. The copied architecture file SHA-256 exactly matched its source: `8dbbfc8900c2aedd5e959d80a9ce1cb8190cbc60f3c4afbb2a7e5d048d382d9c`.
- `acknowledge` for `Sam Rivera`, with repeated `--accept architecture`, wrote the documented receipt format, de-duplicated and sorted accepted IDs, and tied the receipt to a 64-character manifest SHA-256. An empty recipient returned JSON `ok:false`, a clear recovery message, and exit 2.
- Invalid input recovery was exercised: a past expiry warns, missing local files and a credential-bearing URL fail validation with exit 2, and the JSON message does not disclose the credential. Re-running `init` on an existing file preserved it and returned exit 4.
- A local public HTTP fixture proved the link contract: an allowed 200 URL returned success; a checked 404 made `build --check-links` write a diagnostic bundle but return exit **3** with JSON `ok:false`; a robots-disallowed path emitted `link.robots_denied` and was not requested; a `user:pass@` URL was rejected before any request. Captured fixture requests had the named user agent and no `Authorization` header. The fresh CLI integration suite also covers the repaired 404 regression.
- The produced `.crate` was extracted into a fresh temporary consumer, installed with `cargo install --debug --path … --root … --locked`, and its isolated `khb 0.1.0` binary successfully ran the documented JSON `check` and `build` flow, producing both `manifest.json` and `index.html`.

## Site, accessibility, privacy, and PWA

The exact built output was served locally and the production URL was exercised with Chromium at 1366 × 900 and 390 × 844.

- Landing pages have the expected title, exactly one `h1`, and `main`. The first Tab lands on the visible `Skip to content` link with a 3 px focus outline. At 390 px neither landing nor demo horizontally overflows.
- Reduced-motion contexts compute no hero animation. The demo renders all 3 tracks at both sizes; `Needs attention` filters to 1 track; empty acknowledgement export gives the recovery text; entering `Sam Rivera` and reviewing one item downloads `acknowledgement.json` and reports one reviewed artifact. Artifact actions measure 153.625 × 44 CSS px.
- No page errors or console errors occurred. Browser request capture on local desktop/mobile and live mobile observed only the relevant product origin: no runtime analytics, trackers, remote fonts, scripts, or other third-party requests. Acknowledgements are a local download and review state is local to the browser.
- `npm run audit:a11y` and the same fresh live audit reported **0 total** and **0 serious/critical** axe findings for `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and a missing route.
- On live HTTPS, a service worker became controlling after reload; invoking its update check and then reloading offline returned the expected landing title and one `h1`, with no page/console errors.

## Deployment identity, response policy, and performance

Fresh live SHA-256 values exactly match the candidate for `index.html`, privacy, terms, `404.html`, `sw.js`, the hero WebP, and the hashed JS/CSS assets. The generated demo intentionally has a different `generated_at` timestamp, but live and local both report 3 artifacts, 3 required items, 1 gap, and the same two copied-file hashes.

Live `/`, `/demo/`, `/privacy/`, `/terms/`, `/sw.js`, both controlled 404 paths, the hashed JS/CSS, and the WebP were checked directly. All have the strict CSP, `Permissions-Policy: camera=(), microphone=(), geolocation=()`, HSTS, `Referrer-Policy: strict-origin-when-cross-origin`, and `nosniff`. HTML has `public, max-age=0, must-revalidate`; `sw.js` is `no-cache`; hashed assets and the hero use `public, max-age=31536000, immutable`.

The production initial JS is 1,087 B, CSS is 7,599 B, hero WebP is 128,886 B, and there are no shipped font assets: all are within the stated budgets. A fresh mobile Lighthouse report recorded Performance **99**, LCP **1,731 ms**, CLS **0**, and TBT **1 ms**. Its runner then reported `TARGET_CRASHED` while collecting the full-page screenshot, so it did not emit final non-performance category scores; independent axe and browser checks above passed. This is an environmental Lighthouse cleanup limitation, not an observed product failure.

## Defects and follow-up

No Critical, High, Medium, or Low product defects found.

The only follow-up is optional: rerun Lighthouse in a stable deploy runner if a clean Lighthouse process exit and all category scores are required as a release artifact.

