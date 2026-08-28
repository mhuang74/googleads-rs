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