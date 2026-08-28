# Knowledge Handoff Bundle — independent QA handoff

## Release status: FAIL

Independent verification on 2026-08-28 tested candidate `16c9c9fd5da452876824369301ad135976f8ad1d` and https://knowledge-handoff-bundle.sociobot.in/.

The release is blocked by a P1 CLI contract failure: with a public URL that returns HTTP 404, `khb --json build handoff.yaml --output bundle --check-links` returns exit **0** and `"ok":true`, even though its generated manifest reports one link-check error. `khb check --check-links` correctly returns exit 3 for the same input. This allows automation to claim a successful verified handoff with a known broken source of truth, contrary to the documented exit codes and the core product brief.

All other measured gates passed: clean install; formatter; Clippy with warnings denied; tests; production build; package; isolated packed-consumer install; normal/invalid CLI flows; 390 px and desktop browser flows; keyboard focus; reduced motion; local and live axe audits; no observed third-party traffic; service-worker offline reload; response policies; production asset budgets; and live/source equivalence (the generated demo timestamp/manifest hash naturally varies by build time). See `.factory/verification-3.md` for commands, exact results, policies, hashes, and the full defect reproduction.

## How to rerun after remediation

```sh
npm ci
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
npm test
npm run build
npm run package
```

Then test a `--check-links` build against an HTTP 404 fixture and require exit 3/JSON `ok:false`, repeat the clean consumer install of the `.crate`, and repeat the live verification suite in `.factory/verification-3.md`.

No product code was modified by this verification. The only intended changes are this handoff and verification report.
