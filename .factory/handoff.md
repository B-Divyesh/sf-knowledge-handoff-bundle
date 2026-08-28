# Knowledge Handoff Bundle — repair handoff

## Release status: deployed

This repair resolves the release-blocking P1 from independent verification 3
(`b62ab2fec2ce0e3a2460624997386562a80e1a95`) for candidate
`16c9c9fd5da452876824369301ad135976f8ad1d`.

Repair commit: `5143b6cf534f0b00ae3cca4a23b0f2eeede053cc`
(`fix: fail checked-link builds`).

The final static artifact was deployed to production with Azure Static Web Apps
(`swa deploy dist/site --app-name sf-knowledge-handoff-bundle --resource-group
sociobot --env production`). The deployed source revision was
`8c9ecce17d6a57d3b0b89bccf31644d358b226ea`.

## What changed

`khb build --check-links` now treats an error emitted by the requested link
check as a link failure:

- returns exit code **3**, including outside `--ci`;
- emits JSON with `"ok": false`;
- still writes the bundle and its manifest so the recipient can see the exact
  broken artifact and remediation detail.

Schema/input failures still return 2 before a build starts; warnings in CI
still return 2. The change therefore preserves the documented command surface
and the passed behavior of the normal bundle flow.

`tests/cli.rs` now contains an end-to-end regression test with a local public
HTTP fixture. It serves `robots.txt` followed by a 404, runs the packaged
binary path with `--json build --check-links`, asserts exit 3 and `ok:false`,
and verifies that the emitted manifest contains `link.http`.

## Verification performed

All commands were run from this checkout after a clean `npm ci`:

```sh
npm ci
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm test
npm run build
npm run package
git diff --check
```

Results:

- `npm ci`: 23 packages audited, 0 vulnerabilities.
- `npm test`: passed — 6 Rust unit tests, 4 CLI integration tests (including
  the new 404 build regression), and 6 Node/browser/site contract tests.
- `npm run build`: passed — locked release CLI build, generated real Atlas
  demo, and Vite production output at `dist/site/`.
- `npm run package`: passed — produced
  `target/package/knowledge-handoff-bundle-0.1.0.crate` (about 43 KB). No
  registry publish was attempted.
- The `.crate` was extracted into `/tmp/tmp.gflwKghkUm`, installed from that
  extracted package into an isolated Cargo root with `cargo install --debug
  --path … --root … --locked`, and the installed `khb 0.1.0` completed the
  documented JSON `check` and `build` flow. Its resulting `manifest.json`
  existed.
- `cargo test --doc --locked` is not applicable: Cargo reports no library
  target for this binary-only crate.

## Browser, accessibility, privacy, and performance checks

The exact production output was served with Vite preview.

- `npm run audit:a11y` at 390 px passed with 0 total and 0 serious/critical
  axe findings for `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and a
  missing route.
- Playwright at 1366×900 and 390×844 passed: one `h1` and `main`, correct
  title, first Tab reaches the visible skip link, no page/console errors, no
  third-party requests, and the reduced-motion hero has no animation.
- The 390 px generated demo rendered all 3 tracks; an empty acknowledgement
  gave its recovery message; reviewing an artifact and entering `Sam Rivera`
  downloaded `acknowledgement.json` and reported one reviewed artifact.
- Live HTTPS service-worker control was established, then an offline reload
  returned the expected landing title and one `h1` without page errors.
- Fresh live header checks for `/`, `/demo/`, `/privacy/`, `/terms/`,
  `/sw.js`, and both controlled 404 paths found the CSP and
  Permissions-Policy everywhere. HTML is `max-age=0, must-revalidate`; the
  worker is `no-cache`; the policy remains present on 404 responses.
- Production assets are within budget: initial JS 1,087 B, CSS 7,599 B,
  hero WebP 128,886 B, and no font files.
- A mobile Lighthouse run against the built local site wrote its report before
  Chromium crashed during final screenshot/BFCache cleanup: Performance 99,
  Accessibility 100, Best Practices 100, SEO 92 (local HTTP), LCP 1,654 ms,
  CLS 0. The cleanup crash is an environment/browser-runner issue; the scored
  report is at `/tmp/khb-lighthouse.json` in this disposable worker.

## Deploy and rerun

The artifact remains a Rust `clap` CLI plus static Vite landing site.
`site/public/staticwebapp.config.json` supplies the strict headers, immutable
assets, 404 behavior, and service-worker cache policy. After deployment, live
and local SHA-256 values matched exactly for `index.html`,
`demo/manifest.json`, `sw.js`, and `cassette-handoff.webp`; the live demo
manifest hash was
`e497c091c5e5f6852d5af1390639287d09ea93b54158f0cfcf917ff0f35eafdd`.

```sh
npm ci
npm test
npm run build
npm run package
```

For the repaired contract specifically, run `npm test`; the named regression
test is `build_with_a_broken_checked_link_returns_link_failure`. A successful
checked-link build exits 0; a checked HTTP 404 now exits 3 with JSON
`"ok":false` while retaining the diagnostic bundle.

Known gap: Lighthouse's Chromium process reports `TARGET_CRASHED` during its
post-audit screenshot/BFCache cleanup in this environment even though it wrote
the scores above. Re-run it in the deploy environment if a fully clean runner
exit is required.
