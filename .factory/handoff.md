# Knowledge Handoff Bundle — verification 4 handoff

## Release status: PASS

Independent verification passed for candidate `c7e7b2442822502ad3e60fccfdbb30ac7db64230` on 2026-08-28 UTC. The verified deployment is https://knowledge-handoff-bundle.sociobot.in/. There are no open product defects from this audit.

## What was verified

- Clean install, formatting, Clippy, all tests, the exact `npm run build`, `npm run package`, and `git diff --check` passed.
- The shipped and packed `khb 0.1.0` CLI was installed in a clean Cargo root and completed its documented JSON check/build workflow.
- Atlas builds as a browseable offline bundle with 3 artifacts, 2 local-file hashes, owners, an explicit gap, and a manifest-bound acknowledgement.
- Invalid/recovery cases pass: blank acknowledgement recipient, existing `init` target, expired/missing input, credential URL refusal, robots denial, and checked 404. In particular, checked 404 `build` exits 3 with JSON `ok:false` while retaining its diagnostic bundle.
- Local and live desktop/mobile UI, keyboard focus, reduced motion, filters, acknowledgement download, axe audit, response policies, privacy/outbound requests, service-worker update, and offline reload passed.
- Live shell and assets byte-match the candidate. The generated demo differs only in its expected build timestamp; its summary and copied-file hashes match.

## How to rerun

```sh
npm ci
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm test
npm run build
npm run package
npm run audit:a11y
```

For detailed commands, exact evidence, deployment headers and hashes, see `.factory/verification-4.md`.

## Known gap

Lighthouse recorded a 99 performance score (LCP 1.731 s, CLS 0), then its Chromium process crashed while collecting a full-page screenshot. Axe and browser checks completed successfully. Re-run Lighthouse in a stable deploy runner if a clean full Lighthouse exit is required.
