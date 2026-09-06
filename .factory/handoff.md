# Build a portable project handoff — review 2 handoff

## Status

**FAIL — review 2 found 3 findings and 2 untested claims.**

The core CLI, installed package, live sample workflow, privacy isolation,
offline behavior, routes, links, and performance pass. Acceptance remains
blocked by two claim-scope defects and one desktop landmark defect. Full
evidence is in `.factory/review-2.md`.

## Release identity

- Implementation reviewed: `788c736e922336f4078b5c0bb1242ef5f40a8801`
- Documentation reviewed: `cf9c00699bfb11f63132e7dfee0e8cfa106385ab`
- Review base: `abbcd8bc92ed58315d13e10bfa92fe54262e0652`
- Live URL: https://knowledge-handoff-bundle.sociobot.in

Later commits contain only `.factory` documentation. The deployed shell,
assets, legal pages, and demo assets match the clean implementation build.

## What review 2 verified

- The job, audience, `Try it with sample data` action, and three facts appear
  before scrolling on fresh desktop and phone sessions.
- The demo contains four realistic artifacts, recorded reachable and broken
  outcomes, an expiry warning, a known gap, a persistent sample banner, reset,
  exit, and a separate `demo:` namespace.
- A seeded real-data sentinel survived review, reload, reset, and exit.
- Blank and valid acknowledgement paths work, including focus recovery and a
  local JSON receipt tied to the manifest hash.
- Mobile layout, keyboard navigation, focus, reduced motion, 200% text scale,
  legal routes, intentional 404, security headers, links, update, and offline
  reload work.
- All 25 declared claim commands exited 0 individually. Two do not fully prove
  their exact registered claims.
- A packaged crate installed in a clean Cargo root. Its demo, acknowledgement,
  offline bundle, malformed input, missing file, unknown ID, output conflict,
  and exit-code paths work.

## Findings to fix

1. Narrow or remove `landing-stores-nothing`. The service worker creates a
   Cache Storage entry, while the tagged test checks only localStorage and
   cookies.
2. Replace the desktop demo's second visible `<header>` landmark with suitable
   non-banner markup, then add desktop Axe coverage. Live Axe currently reports
   `landmark-no-duplicate-banner` and `landmark-unique`.
3. Expand the `credential-urls-rejected` tagged test to exercise separate
   user-info and credential-query URLs. Its current combined fixture reaches
   only the user-info validation branch.

No product code was changed by this reviewer.

## How to reproduce

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm run build
npm run audit:a11y
npm run package
```

Run every `test` command in `.factory/claims.json` separately. To reproduce the
desktop landmark issue, run Axe against the built or live `/demo/` at a width
above 700 px. To reproduce the storage claim issue, wait for
`navigator.serviceWorker.ready`, reload, and inspect `await caches.keys()`.

## Verification summary

`npm test`, format, Clippy, build, package, and diff checks passed. The
repository accessibility command passes because it tests a 390 px viewport.
Fresh desktop Axe exposes the finding above.

Fresh mobile Lighthouse scored 99 Performance and 100 for Accessibility, Best
Practices, and SEO. LCP was 1.502 s, TBT 144 ms, and CLS 0. Initial JavaScript
is 1.08 kB and CSS is 7.99 kB.

Evidence screenshots and Lighthouse JSON are under `/work/.evidence/`. The
required report copy and verdict JSON are written there at completion.
