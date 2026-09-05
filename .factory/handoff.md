# Build a portable project handoff — review 1 handoff

## Status

**FAIL — 8 findings and 22 untested public claims.**

Review 1 audited implementation `5143b6cf534f0b00ae3cca4a23b0f2eeede053cc`, deployment record `c7e7b2442822502ad3e60fccfdbb30ac7db64230`, and documentation `f1c585d65b429433e52c081c445c98987fe17177`. Later commits change reports only. The live shell and assets byte-match the clean build.

## What was done

- Opened the live landing page, sample, legal pages, and 404 in fresh desktop and phone contexts.
- Checked first-screen wording, sample state, reset and exit controls, keyboard use, focus, touch targets, reduced motion, accessibility, privacy, outbound requests, links, route titles, offline reload, service-worker update, and response headers.
- Built and packaged a clean checkout, installed the crate in an isolated Cargo root, and exercised normal, invalid, boundary, and recovery paths.
- Compared every earlier finding with current live or regression evidence.
- Made no product-code changes.

## Verification

Passed: `npm ci`, format, Clippy, `npm test`, `npm run build`, `npm run package`, `git diff --check`, live axe, the factory URL verifier, installed CLI build and acknowledgement, direct-file offline use, and live service-worker offline reload.

`npm run audit:a11y` fails from the documented clean setup because it expects an unstated server on port 4173. A direct live audit reports zero axe violations. Lighthouse wrote 100 scores for all four categories, LCP 1.516 s, CLS 0, and TBT 60 ms, then its Chromium tab crashed.

## Work left

1. Add the required claim registry and tagged sandbox tests for all retained public claims.
2. Add `khb demo`, the documented sample sandbox, persistent sample label, reset, and start-for-real controls.
3. Replace the dead sample URL, show current expiry/link results, and address status ageing.
4. Honor server `Retry-After` before making another request to that origin.
5. Replace metaphor copy with the required job, audience, and first action; add the copy audit.
6. Add route metadata, discovery files, standard headers and footers, and 44 px landing targets.
7. Make the documented accessibility audit command self-contained.

Full evidence and exact commands are in `.factory/review-1.md`. Evidence is also copied to `/work/.evidence/qa-report.md`, with the machine result in `/work/.evidence/qa-result.json`.
