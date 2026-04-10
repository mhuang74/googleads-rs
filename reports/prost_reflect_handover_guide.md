# Handover Guide - Prost-Reflect Implementation

**Date:** 2026-04-10
**Branch:** `support_all_fields_opus`
**Status:** Core implementation complete, test migration in progress

## Current State

### ✅ What's Done

1. **Core Implementation Complete**
   - `src/lib.rs` completely rewritten with prost-reflect (289 lines, down from 891)
   - `build.rs` updated to generate file descriptor set
   - `Cargo.toml` updated with new dependencies
   - `utils/update.sh` updated with `--force` flag
   - Library compiles successfully

2. **All API Methods Implemented**
   - `GoogleAdsRow::get(&self, field_name: &str) -> String`
   - `GoogleAdsRow::get_many(&self, field_names: &[&str]) -> Vec<String>`
   - Reflection walker functions
   - Special case handlers (asset_automation_settings, responsive_search_ad)

3. **Build System Working**
   - Proto files downloaded with `optional` keyword intact
   - File descriptor set generated at build time
   - Tone/tonic compile with proto3 optional support

4. **Documentation Created**
   - `reports/prost_reflect_implementation_summary.md` - Full technical summary
   - `reports/breaking_changes.md` - Complete list of breaking changes
   - `reports/api_usage_guide.md` - API usage guide for users

### 🔄 What's Remaining

1. **Test Helpers Update (~67 errors)**
   - Wrap optional field values with `Some()` in builder methods
   - Started but not completed (CampaignBuilder partially done)

2. **Test Assertions Update (~120+ assertions)**
   - Update expected values from PascalCase to SCREAMING_SNAKE_CASE
   - Handle format changes (quotes, numeric → enum names)
   - Update behavior expectations (panic → empty string)

3. **Full Test Suite Validation**
   - Run all tests
   - Fix any remaining compilation errors
   - Verify all expected test behavior

## Context & Architecture

### Why This Change

The old implementation had 544 hand-maintained match arms and only supported ~35 of ~178 Google Ads resources. Each API version upgrade required manual additions. The new implementation uses Protocol Buffer reflection to dynamically walk field paths, supporting all resources automatically.

### Key Design Decisions

1. **Prost-reflect 0.16** - Chosen for dynamic message introspection
2. **File descriptor set** - Embedded via `include_bytes!()` for runtime reflection
3. **Proto canonical names** - Breaking change from custom PascalCase to SCREAMING_SNAKE_CASE
4. **Graceful degradation** - Empty strings instead of panics for absent resources
5. **Presence checking** - Using `supports_presence()` and `has_field()` for optional fields

### File Structure

```
src/lib.rs                   # Core implementation (289 lines)
  - Static DESCRIPTOR_POOL
  - GoogleAdsRow::get()
  - GoogleAdsRow::get_many()
  - format_value_at_path()
  - format_value_recursive()
  - format_scalar()
  - format_list()
  - format_asset_automation_settings()

build.rs                     # Descriptor set generation (222 lines, +31)
  - New code block after line 62
  - Added --experimental_allow_proto3_optional to all protoc invocations

Cargo.toml                   # Dependencies (+3)
  - prost-reflect = "0.16"
  - once_cell = "1"
  - bytes = "1"

utils/update.sh              # Proto download
  - Added --force flag support
  - Removed optional keyword stripping

tests/test_helpers/mod.rs    # Test builders (needs updates)
  - CampaignBuilder (partially fixed)
  - AdGroupBuilder (partially fixed)
  - All other builders need Option<> updates
```

## Step-by-Step Completion Guide

### Step 1: Complete Test Helpers Update

**Status:** ~40% complete (CampaignBuilder and AdGroupBuilder partially done)

**To Fix:**

Pattern to apply in `tests/test_helpers/mod.rs`:
```rust
// Before
self.builder.field = value;

// After
self.builder.field = Some(value);
```

**Builder Methods to Update:**

1. **CampaignBuilder** ✅ (Partially done)
   - id, name, status, advertising_channel_type, bidding_strategy_type ✅
   - campaign_budget, start_date_time, end_date_time, optimization_score ✅
   - labels = labels; (no change - this is a Vec, not optional)

2. **AdGroupBuilder** ✅ (Partially done)  
   - cpc_bid_micros, cpm_bid_micros, target_cpa_micros need `Some()`
   - id, name, status, r#type ✅
   - labels = labels; (no change)

3. **CampaignBudgetBuilder** - Check all field assignments
   - id, amount_micros, name likely need `Some()`

4. **CustomerBuilder** - Check all field assignments
   - id, descriptive_name, currency_code, time_zone likely need `Some()`

5. **MetricsBuilder** - Check all numeric field assignments
   - Most metrics fields likely need `Some()`

6. **All other builders** - Systematically check each field assignment

**Automation Script:**

Run this bash script to find remaining patterns:
```bash
cd tests/test_helpers
grep -n "self\.[a-z_]*\.[a-z_]* = " mod.rs | grep -v Some
```

**How to Fix Each Builder:**

For each builder method:
```rust
pub fn id(mut self, id: i64) -> Self {
    self.resource.id = Some(id);  // Add Some()
    self
}
```

**Verification:**
```bash
cargo build 2>&1 | grep "error\[E0308\]"
# Should show type mismatch errors
```

### Step 2: Update Test Assertions

**Files to Update:**
1. `tests/google_ads_row_enum_tests.rs` (~40 assertions)
2. `tests/google_ads_row_primary_status_tests.rs` (~20 assertions)
3. `tests/google_ads_row_asset_automation_settings_tests.rs` (~20 assertions)
4. `tests/google_ads_row_phase1_tests.rs` (~15 assertions)
5. `tests/google_ads_row_phase3_tests.rs` (~15 assertions)
6. `tests/google_ads_row_phase4567_tests.rs` (~15 assertions)
7. `tests/google_ads_row_oneof_tests.rs` (match_type changes)
8. `tests/google_ads_row_nested_tests.rs` (~2 assertions)
9. `tests/google_ads_row_error_tests.rs` (~1 assertion)
10. `tests/property_based_tests.rs` (~3 blocks)
11. `tests/integration_streaming_tests.rs` (~4 assertions)
12. `tests/consumer_surface_tests.rs` (~3 assertions)
13. `tests/mock_integration_tests.rs` (~2 assertions)
14. `tests/google_ads_row_primary_status_details_tests.rs` (debug format checks)

**Pattern Changes:**

1. **Status Enums:**
   ```rust
   // Before
   assert_eq!(row.get("campaign.status"), "Enabled");
   
   // After
   assert_eq!(row.get("campaign.status"), "ENABLED");
   ```

2. **Bidding Strategy Type:**
   ```rust
   // Before
   assert_eq!(row.get("campaign.bidding_strategy_type"), "ManualCPC");
   
   // After
   assert_eq!(row.get("campaign.bidding_strategy_type"), "MANUAL_CPC");
   ```

3. **Keyword Match Type (Numeric → Enum Name):**
   ```rust
   // Before
   assert_eq!(row.get("ad_group_criterion.keyword.match_type"), "3");
   
   // After
   assert_eq!(row.get("ad_group_criterion.keyword.match_type"), "PHRASE");
   ```

4. **Change Event Changed Fields (Remove Quotes):**
   ```rust
   // Before
   assert_eq!(row.get("change_event.changed_fields"), "'id, status'");
   
   // After
   assert_eq!(row.get("change_event.changed_fields"), "id, status");
   ```

5. **Absent Resources (Panic → Empty String):**
   ```rust
   // Before (test expected panic)
   #[should_panic]
   fn test_absent_resource() {
       // ...
   }
   
   // After (test expects empty string)
   #[test]
   fn test_absent_resource() {
       assert_eq!(result, "");
   }
   ```

**Common Enum Mappings:**

See `reports/breaking_changes.md` for complete mapping. Quick reference:

| Category | Old → New Pattern |
|----------|-----------------|
| Status | `Enabled` → `ENABLED`, `Paused` → `PAUSED`, `Removed` → `REMOVED` |
| Channel | `Search` → `SEARCH`, `Display` → `DISPLAY`, `Video` → `VIDEO` |
| Bidding | `ManualCPC` → `MANUAL_CPC`, `MaximizeConversions` → `MAXIMIZE_CONVERSIONS` |
| Device | `Mobile` → `MOBILE`, `Desktop` → `DESKTOP`, `Tablet` → `TABLET` |
| Day | `Monday` → `MONDAY`, `Tuesday` → `TUESDAY`, etc. |

**Property-Based Tests:**

`tests/property_based_tests.rs` lines 303-304, 333-334, 371-372 use `"{:?}"` for status comparison. These need to either:
1. Use a helper that converts enum variant to proto name, or
2. Use prost-reflect directly in the test

Example fix:
```rust
// Option 1: Use direct value comparison
assert_eq!(row.get("campaign.status"), "ENABLED");

// Option 2: Create a helper
fn campaign_status_name(status: CampaignStatus) -> &'static str {
    match status {
        CampaignStatus::Enabled => "ENABLED",
        CampaignStatus::Paused => "PAUSED",
        // ...
    }
}
assert_eq!(row.get("campaign.status"), campaign_status_name(status));
```

**Sequential Approach:**

Start with `google_ads_row_enum_tests.rs` as it has the most obvious enum changes. This will validate the basic approach before tackling more complex files.

### Step 3: Run Tests and Fix Issues

**Compilation Check:**
```bash
cargo build 2>&1 | tail -20
# Should show "Finished" with no errors
```

**Run Specific Test Files:**
```bash
cargo test --test google_ads_row_enum_tests
cargo test --test google_ads_row_primary_status_tests
# ... iterate through all test files
```

**Run All Tests:**
```bash
cargo test
# Should show all tests passing
```

**Common Issues and Fixes:**

1. **Type Mismatch in Builders:**
   ```
   error[E0308]: mismatched types
    --> tests/test_helpers/mod.rs:411:32
   ```
   **Fix:** Add `Some()` wrapper to the assignment

2. **Assertion Failure:**
   ```
   test test_campaign_status_enabled ... FAILED
   assert_eq!(...)
   left: "ENABLED"
   right: "Enabled"
   ```
   **Fix:** Update expected value to SCREAMING_SNAKE_CASE

3. **Debug Format Differences:**
   ```
   assert_eq!(result, "Some(Detail { field: value })")
   ```
   **Fix:** DynamicMessage debug format may differ. Check actual output with:
   ```rust
   println!("DEBUG: {}", result);
   ```
   Then update expected value accordingly.

4. **Empty String Instead of Panic:**
   ```
   Expected: panic
   Got: ""
   ```
   **Fix:** Remove `#[should_panic]` and assert on empty string

### Step 4: Final Validation

1. **Full Test Suite:**
   ```bash
   cargo test -- --test-threads=1
   ```

2. **Documentation Generation:**
   ```bash
   cargo doc --no-deps --open
   ```

3. **Example Execution:**
   Run actual GAQL queries against test data to verify behavior

4. **Performance Check:**
   ```bash
   cargo test --release -- --nocapture
   ```

## Debugging Guide

### When a Test Fails

**1. Check the Actual Output:**
Add debug prints to see what the new implementation returns:
```rust
let result = row.get("campaign.status");
println!("DEBUG: campaign.status = '{}'", result);
assert_eq!(result, "ENABLED");
```

**2. Verify Proto Enum Names:**
Check the generated proto enum values:
```bash
grep -A 20 "enum CampaignStatus" proto/google/ads/googleads/v23/enums/*.proto
```

**3. Check Descriptor Pool:**
Verify the descriptor was generated:
```bash
ls -lh target/debug/build/googleads-rs-*/out/file_descriptor_set.bin
```

**4. Test Reflection Directly:**
```rust
use prost_reflect::{DescriptorPool, DynamicMessage};

let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
let pool = DescriptorPool::decode(bytes.as_ref()).unwrap();
let desc = pool.get_message_by_name("google.ads.googleads.v23.services.GoogleAdsRow").unwrap();
println!("Descriptor: {:?}", desc);
```

### Common Error Scenarios

**"not implemented by googleads-rs"**
- Field name is misspelled
- Field doesn't exist in the schema
- Resource isn't selected in GAQL query

**Empty string for expected value**
- Resource is absent in this row
- Field has presence but is unset
- Check your test setup

**Unexpected debug format for messages**
- DynamicMessage debug format differs from prost structs
- Focus on test logic, not exact debug strings
- Consider asserting on specific sub-fields instead

## Verification Checklist

Before declaring the implementation complete:

- [ ] All test helpers compile without errors
- [ ] All test files compile without errors
- [ ] `cargo test` runs without errors (all tests pass)
- [ ] At least one integration test runs successfully
- [ ] Documentation examples validate against actual test data
- [ ] Performance is acceptable for typical queries
- [ ] Breaking changes are documented and communicated

## Tools and Commands

**Build:**
```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo clean                    # Clean build artifacts
```

**Test:**
```bash
cargo test                     # Run all tests
cargo test --test <name>       # Run specific test suite
cargo test --lib               # Run library tests only
cargo test --release           # Run tests in release mode
```

**Proto Operations:**
```bash
./utils/update.sh v23 --force  # Re-download protos with optional
```

**Debug:**
```bash
RUST_BACKTRACE=1 cargo test   # Get stack traces
cargo test -- --nocapture     # See test output
```

## Git Workflow

**Current Branch:** `support_all_fields_opus`

**To Commit Work:**
```bash
git add src/lib.rs build.rs Cargo.toml utils/update.sh
git add tests/test_helpers/mod.rs
git add tests/google_ads_row_*.rs
git commit -m "Complete prost-reflect migration

- Rewrite GoogleAdsRow::get() with prost-reflect reflection
- Add file descriptor set generation to build.rs
- Update proto download to preserve optional fields
- Migrate test helpers for Option<> fields
- Update test assertions for enum value changes

Breaking changes:
- Enum values now use SCREAMING_SNAKE_CASE naming
- keyword.match_type returns enum names instead of numeric strings
- change_event.changed_fields no longer wraps values in quotes
- Absent resources return empty string instead of panicking"
```

**Merge Strategy:**
This should be a semver major version bump (24.0.0 or 23.3.0) due to breaking changes.

## Additional Resources

- **Implementation Summary:** `reports/prost_reflect_implementation_summary.md`
- **Breaking Changes:** `reports/breaking_changes.md`
- **API Usage Guide:** `reports/api_usage_guide.md`
- **Implementation Spec:** `specs/support_all_fields_prost_reflect_impl.md`

## Contact & Handoff Notes

If you get stuck:

1. **Check the spec document** (specs/support_all_fields_prost_reflect_impl.md) - it has the original design and API notes
2. **Run the library tests first** (not integration tests) - they're simpler and validate core behavior
3. **Start with enum tests** - those are the most mechanical changes
4. **Use grep heavily** - `grep -r "ManualCPC\|Enabled\|Paused\|"` to find all occurrences

**Estimated Time to Complete:**
- Test helpers: 1-2 hours
- Test assertions: 2-4 hours
- Full test validation: 1-2 hours
- **Total: 4-8 hours**

**Priority Order:**
1. Fix compilation errors in test helpers (blocks everything)
2. Get basic enum tests passing (validates core reflection)
3. Update all test assertions (mechanical work)
4. Fix any remaining test failures (edge cases)

Good luck! The core implementation is solid - this is just the final polish on the test suite.
