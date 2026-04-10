# Plan: Consumer Surface Validation for googleads-rs

## Context

After each Google Ads API version upgrade, googleads-rs is validated by building the downstream project mcc-gaql. Git history shows recurring post-upgrade breakage patterns:

1. **Missing `GoogleAdsRow::get()` match arms** (most frequent) — new fields silently return `"not implemented by googleads-rs"`
2. **gRPC client generation failures** — proto build producing message structs but not service clients
3. **Request/response type shape changes** — fields renamed/removed across API versions
4. **Version path breakage** — module paths embed API version (e.g., `v23`)

The goal is to add a test file in googleads-rs that replicates how mcc-gaql uses the library, catching these issues without building mcc-gaql.

## Approach: Single Integration Test File

Create `tests/consumer_surface_tests.rs` — no CI changes needed since `cargo test` already picks up all test files.

### What's Already Covered (no need to duplicate)
- `GoogleAdsServiceClient<Channel>` type exists (`build_verification_tests.rs`)
- `SearchGoogleAdsStreamRequest/Response`, `GoogleAdsRow` types exist
- Core resource types compile and default-construct
- Many `GoogleAdsRow::get()` fields tested across phase1-4567 tests

### Gaps to Fill

**Section 1: gRPC Client Construction with Interceptor**
- Define a `DummyInterceptor` implementing `tonic::service::Interceptor`
- Compile-check `GoogleAdsServiceClient::with_interceptor(channel, interceptor)` — this is the actual construction pattern mcc-gaql uses
- Compile-check `GoogleAdsFieldServiceClient::with_interceptor()` — **zero coverage today**
- Verify client method signatures exist: `.search_stream()`, `.search()`, `.search_google_ads_fields()`

**Section 2: Missing Request/Response Types**
- `SearchGoogleAdsRequest` — construct with `customer_id`, `query`, `validate_only`, `..Default::default()`
- `SearchGoogleAdsFieldsRequest` — construct with `query`, `page_token`, `page_size`
- `SearchGoogleAdsFieldsResponse` — verify `.results` field exists
- `SearchGoogleAdsStreamResponse` — verify `.results`, `.field_mask`, `.query_resource_consumption` fields

**Section 3: Consumer-Critical `get()` Fields**
- Test a curated list of fields mcc-gaql actually queries (from `SUB_ACCOUNTS_QUERY`):
  - `customer_client.id`, `customer_client.level`, `customer_client.currency_code`, `customer_client.time_zone`, `customer_client.descriptive_name`
- Assert none return `"not implemented by googleads-rs"`

**Section 4: End-to-End Consumer Pattern**
- Simulate the mcc-gaql streaming flow: build a `SearchGoogleAdsStreamResponse` with populated `GoogleAdsRow` and `FieldMask`, iterate results, call `row.get()` for each field path, assert no `"not implemented"` values

## Files to Create/Modify

| File | Action | Purpose |
|------|--------|---------|
| `tests/consumer_surface_tests.rs` | **Create** | All validation tests |

No changes to CI workflow — existing `cargo test` picks this up automatically.

## Key Reference Files
- `tests/build_verification_tests.rs` — existing compile-check pattern to follow
- `tests/mock_integration_tests.rs` — existing mock streaming pattern
- `tests/test_helpers/mod.rs` — existing builders for `GoogleAdsRow`, `CustomerClient`, etc.
- `src/lib.rs` — the `get()` match arms (lines 288-889)
- `mcc-gaql/crates/mcc-gaql/src/googleads.rs` — the consumer API surface to replicate

## Verification
1. `cargo test --test consumer_surface_tests` — all tests pass
2. `cargo clippy --all-targets` — no warnings
3. Manually verify: temporarily break a type import or remove a `get()` match arm → test fails
