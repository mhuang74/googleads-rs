# Handoff: Fix Proto3 Optional Field Compilation Errors - Remaining Work

## Current Status

**IN PROGRESS** - 7 test files fixed, approximately 7 more files remaining.

## Background

The build.rs now uses `--experimental_allow_proto3_optional` flag, which causes proto3 fields marked with `optional` to generate as `Option<T>` types instead of plain `T` types.

Test code was written before this change, so scalar field assignments and struct initializations need to be updated to wrap values in `Some()`.

## What Has Been Done

### Fixed Files (7 test files)

1. **tests/integration_streaming_tests.rs** (~10 fixes)
   - Fixed `campaign.id`, `campaign.name`, `metrics.*` fields to use `Some()`
   - Fixed NetworkSettings boolean fields to use `Some()`
   - Fixed Segments fields to use `Some()`

2. **tests/mock_integration_tests.rs** (~20 fixes)
   - Fixed Campaign, AdGroup, Customer scalar fields
   - Fixed Metrics fields: impressions, clicks, cost_micros
   - Fixed Segments.date
   - Note: `segments.device` kept as direct i32 (not optional in proto)

3. **tests/google_ads_row_asset_automation_settings_tests.rs** (~3 fixes)
   - Fixed Campaign struct literal: `id` and `name` wrapped in `Some()`
   - Fixed `create_asset_automation_setting()` to wrap enum fields in `Some()`

4. **tests/google_ads_row_phase2_tests.rs** (~36 fixes)
   - Fixed all E-commerce metrics fields to use `Some()`
   - Fixed Location Asset metrics fields to use `Some()`
   - Fixed Customer Acquisition metrics fields to use `Some()`

5. **tests/google_ads_row_phase4567_tests.rs** (~50+ fixes)
   - Fixed CampaignLabel, AdGroupLabel, AdGroupAdLabel string fields
   - Fixed SharedSet: `type` and `status` kept as plain i32 (not optional)
   - Fixed SharedCriterion string fields
   - Fixed KeywordInfo: `match_type` kept as plain i32 (not optional)
   - Fixed Segments: `product_condition` kept as plain i32 (not optional)
   - Fixed Metrics: non-optional fields kept as plain values

6. **tests/google_ads_row_primary_status_tests.rs** (~8 fixes)
   - Fixed Campaign fields: `id`, `name`, `resource_name` wrapped in `Some()`
   - AssetGroupAsset `resource_name` wrapped in `Some()`
   - Note: `primary_status` fields kept as plain i32 (not optional in proto)

7. **tests/google_ads_row_primary_status_details_tests.rs** (~6 fixes)
   - Fixed `create_simple_status_detail()` to wrap `reason` and `status` in `Some()`
   - Fixed AdGroupAsset and AssetGroupAsset field assignments

## Current Compilation Status

Approximately **32 errors remaining** (down from 97+ errors initially).

## Files Remaining to Fix

Based on the original handoff document:

1. **tests/google_ads_row_nested_tests.rs** - Tests for nested message paths
2. **tests/google_ads_row_oneof_tests.rs** - Tests for oneof fields
3. **tests/google_ads_row_error_tests.rs** - Tests for error handling
4. **tests/google_ads_row_phase3_tests.rs** - Additional phase 3 fields
5. **tests/google_ads_row_repeated_tests.rs** - Tests for repeated fields
6. **tests/google_ads_row_scalar_tests.rs** - Tests for scalar values (partially fixed?)
7. **tests/google_ads_row_enum_tests.rs** - Tests for enum values (needs enum assertion updates also)
8. **tests/property_based_tests.rs** - Property-based tests

## Fix Pattern

### Adding `Some()` wrappers

When a proto field has `optional` keyword, wrap the value:

```rust
// BEFORE (wrong):
campaign.id = 12345;
campaign.name = "Test Campaign".to_string();
metrics.impressions = 10000;
metrics.ctr = 0.05;
segments.date = "2024-10-10".to_string();

// AFTER (correct):
campaign.id = Some(12345);
campaign.name = Some("Test Campaign".to_string());
metrics.impressions = Some(10000);
metrics.ctr = Some(0.05);
segments.date = Some("2024-10-10".to_string());
```

### Struct initialization patterns

In struct literals, add `Some()` for optional fields:

```rust
// BEFORE:
let campaign = Campaign {
    id: 12345,
    name: "Test Campaign".to_string(),
    status: CampaignStatus::Enabled as i32,
    ..Default::default()
};

// AFTER:
let campaign = Campaign {
    id: Some(12345),
    name: Some("Test Campaign".to_string()),
    status: CampaignStatus::Enabled as i32,  // enums in proto3 without 'optional' stay as i32
    ..Default::default()
};
```

### Do NOT wrap:

1. **Enum fields without `optional` keyword in proto** - these are `i32` directly
2. **Non-optional scalar fields** in proto definitions
3. **Function call arguments** (unless the function parameter is Option<T>)
4. **Vector/ push expressions** - `vec![...]` or `.push()`

### Important: Check the proto first!

Before changing code, verify if the field is actually optional:

```bash
# Example: Check if campaign.id is optional
grep -A 2 "^.*id = " proto/google/ads/googleads/v23/resources/campaign.proto
```

- If line starts with `optional int64 id` → use `Some(value)`
- If line is just `int64 id` → use `value` directly

## Key Differences by Field Type

| Proto Declaration | Rust Type | Test Fix Pattern |
|-------------------|-----------|------------------|
| `optional string name` | `Option<String>` | `name: Some("value".to_string())` |
| `optional int64 id` | `Option<i64>` | `id: Some(12345)` |
| `optional int32 count` | `Option<i32>` | `count: Some(123)` |
| `optional double value` | `Option<f64>` | `value: Some(3.14)` |
| `optional bool flag` | `Option<bool>` | `flag: Some(true)` |
| `string resource_name` (no optional) | `String` | `resource_name: "value".to_string()` |
| `int64 impressions` (no optional) | `i64` | `impressions: 1000` |
| `EnumType status` (no optional) | `i32` | `status: Status::Enabled as i32` |

## Additional Issues (NOT compilation errors)

### Enum Formatting Breaking Changes

Enums now return proto canonical names (UPPER_SNAKE_CASE) instead of PascalCase:

| Old Value | New Value |
|-----------|-----------|
| "Enabled" | "ENABLED" |
| "Paused" | "PAUSED" |
| "Eligible" | "ELIGIBLE" |

**After fixing compilation errors, update enum assertions in:**
- tests/google_ads_row_enum_tests.rs
- Any other files with enum value assertions

### Special Fields Need Verification

1. **campaign.bidding_strategy_type** - Previously returned custom camelCase names
2. **change_event.changed_fields** - Previously returned single-quoted format

## Verification Commands

```bash
# Build all tests
cargo test --no-run

# Run specific failing test file
cargo test --test google_ads_row_nested_tests

# Check remaining errors
cargo test --no-run 2>&1 | grep "^error"

# Run all tests after fixes
cargo test
```

## Next Steps

1. Fix the remaining ~7 test files with compilation errors
2. Update enum assertions to expect UPPER_SNAKE_CASE values
3. Verify special fields (bidding_strategy_type, changed_fields) work correctly
4. Run full test suite to confirm all tests pass

## References

- Previous handoff: `reports/support_all_fields_handoff_2.md`
- Prost-reflect implementation: `specs/support_all_fields_via_prost_reflect.md`
- GAQL field paths: https://developers.google.com/google-ads/api/fields/v23/overview
