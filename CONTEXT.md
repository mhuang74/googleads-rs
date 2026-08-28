# googleads-rs

Rust gRPC client library for the Google Ads API, regenerated from Google's proto definitions on every API upgrade.

## Language

**Google Ads API major version**:
The major version (e.g. v23) of the Google Ads API that the checked-in proto definitions and generated client code support. Proto directory layout carries major only.
_Avoid_: API level, release train

**Minor version**:
The minor release of the Google Ads API (e.g. 23.2). Not represented in the proto directory layout; can only be inferred by comparing the proto download against Google's release notes.
_Avoid_: patch, point release

**Version convention**:
Crate major.minor mirrors the Google Ads API major.minor; the patch component is reserved for library-only fixes.

**current_gads_version**:
The public alias at the crate root (`pub use google::ads::googleads::v23 as current_gads_version;`) naming the generated module of the currently supported Google Ads API major version; the single hand-edit anchor for upgrades.
_Avoid_: vNN path, pinned version, current_version

**Deterministic migration**:
The scripted, non-AI part of an upgrade: download protos, copy infrastructure definitions, rewrite version references.
_Avoid_: bot run, repair

**best_effort step**:
A migration step permitted to fail with a warning without aborting the migration; reserved for cosmetic cleanup only.
_Avoid_: safe_run

**Upgrade**:
The end-to-end process of moving the crate to a newer Google Ads API major version: release detection → upgrade issue → deterministic migration → validation → AI repair (if needed) → PR → merge → release → publish.
_Avoid_: migration (reserve for the deterministic scripted part), bot run

**AI repair**:
The bounded coding-agent loop (max 5 attempts) that fixes residual compile/test/clippy failures after deterministic migration and validation. Runs only when validation fails; never weakens tests or checks.
_Avoid_: agent run, repair loop (in prose use "AI repair")

**Upgrade issue**:
The GitHub Issue that is the queue item for one upgrade: labeled `api upgrade bot`, carries body marker `google-ads-api-upgrade: vNN`, and is created already `ready-for-agent` so execution starts without human intervention. Success keeps it `in-progress`; the PR's `Closes #N` auto-closes it on merge. Failure flips it to `api-upgrade-failed`; retry is human branch GC plus relabeling.
_Avoid_: detector issue, release ticket, needs-triage

**Validation**:
The canonical set of checks for one change: fmt, check, test, clippy — each an `xtask` subcommand, invoked by humans, CI, and the upgrade workflow alike. Fails fast: the first failing check stops the run.
_Avoid_: CI run, full build

**External consumer check**:
A mcc-gaql-based validation job (builds the mcc-gaql workspace with googleads-rs patched to the branch under test) — currently **deferred**, because mcc-gaql's source imports `google::ads::googleads::vNN` paths and needs an import migration before the check can run against a new major. Revisit trigger: an upgrade merge that breaks mcc-gaql's CI on `main`.
_Avoid_: consumer test (reserve for in-repo integration tests like consumer_surface_tests.rs, which cargo test already runs)

**In-progress**:
Label state of an upgrade issue claimed by the worker; paired with `ready-for-agent` removal. Prevents duplicate execution. On success the issue stays `in-progress` until the PR merge auto-closes it; the only other exit is `api-upgrade-failed`.
_Avoid_: running label, claimed label (new label names), ready-for-human