# Build a portable project handoff — repair 4 handoff

## Status

The repair implementation is ready and its repository CI passed. The HTTPS
site was still serving the previous August build at the final check, so live
deployment verification remains outstanding.

## Release identity

- Implementation: `788c736e922336f4078b5c0bb1242ef5f40a8801`
  (`fix: complete demo sandbox and claim coverage`)
- Earlier review documentation: `a2b8b13ce26d37f7ac2bb2d0c844e59db5cc66ed`
- Verification documentation: `ae3d0a4c9c29a98fa81b9c02d34562a7b42acb5e`
  (`docs: record repair verification`), separate from the implementation.

## What changed

- Added `.factory/claims.json` with 25 public claims and one runnable,
  outcome-based sandbox test for each claim.
- Added `khb demo`, bundled Atlas sample data, `examples/atlas/`, and
  `.factory/demo.md`. The generated demo shows a persistent sample-data banner,
  has Reset demo and Start for real controls, and uses the `demo:` browser
  storage namespace only.
- Replaced stale sample output with recorded reachable and broken-link results.
  Recipient bundles now recalculate expiry status when opened.
- Honored `Retry-After` per origin before a later public-link request.
- Rewrote the landing page around the job, audience, and first action; added
  the copy audit and a verb-first catalog description. The catalog description
  was copied to `/work/.evidence/catalog-description.txt`.
- Completed route titles, canonical and social metadata, robots, sitemap,
  headers, standard site skeleton, designed 404, legal pages, touch targets,
  and the self-contained accessibility audit.
- Added original derivative social and touch assets from the project’s existing
  generated cassette art; provenance is in `.factory/design.md`.

## Disposition of review 1 findings

| Finding | Current disposition |
| --- | --- |
| Missing claims registry and tests | Fixed: 25 registered claims pass from a clean setup. |
| Incomplete CLI/demo sandbox | Fixed: `khb demo` and `/demo/` use shipped, isolated sample data. |
| Stale sample results | Fixed: recorded link results and dynamic expiry states. |
| Ignored `Retry-After` | Fixed and regression-tested with a local 429 server. |
| First-screen metaphor copy | Fixed: job, audience, and Try it with sample data are explicit. |
| Route, metadata, and skeleton gaps | Fixed in the static build. |
| Landing touch targets below 44 px | Fixed and browser-tested at phone and desktop widths. |
| Accessibility audit required an external server | Fixed: `npm run audit:a11y` serves and audits the build itself. |

## How to run and verify

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm run build
npm run audit:a11y
npm run package
```

Run the actual CLI sample without setup:

```sh
cargo run -- demo
```

Run any declared claim exactly as listed in `.factory/claims.json`, for
example:

```sh
npm run test:claims -- --test-name-pattern @claim:demo-sandbox
```

`npm test` passed 7 Rust unit tests, 6 CLI integration tests, 4 site/bundle
browser tests, and all 25 claim tests. Format and Clippy passed. `npm run
build` produced `dist/site`; its entry JavaScript is 1.08 kB (0.59 kB gzip)
and its CSS is 7.99 kB (2.59 kB gzip). `npm run audit:a11y` found zero issues
on the landing, demo, legal, 404, and unknown-route pages. `npm run package`
created and verified the publishable Cargo package. An isolated Cargo install
of that package successfully ran `khb --json demo`.

## Deployment and remaining work

Repository CI for the implementation succeeded. At the final HTTPS check,
`https://knowledge-handoff-bundle.sociobot.in` returned the old title
“Knowledge Handoff Bundle — leave the project, not the context” and a
28 August 2026 `Last-Modified` value. This does not match implementation
`788c736`. No deploy workflow or durable product deployment command exists in
this repository, and no infrastructure was changed.

Release the already-pushed implementation through the factory’s normal static
deployment path, then re-run a fresh desktop and phone browser check against
the HTTPS origin. The expected first screen states the job (build a project
handoff), audience (departing owners and small teams handing work to the next
owner), and first action (Try it with sample data). The product is free under
MIT; no billing offer or external provider is required.
