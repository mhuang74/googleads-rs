# Handover Guide - Prost-Reflect Implementation (Part 2)

**Date:** 2026-04-10
**Branch:** `support_all_fields_opus`
**Status:** Test helpers partially fixed, primary status tests passing, asset automation settings tests in progress

## Progress Since Handover 1

### ✅ Completed Work

1. **Primary Status Tests (100% complete)**
   - All 63 tests passing
   - Fixed ad_group_criterion.primary_status optional field wrapping (needed `Some()`)
   - Updated all assertions from PascalCase → SCREAMING_SNAKE_CASE

2. **Test Helpers - Partial Migration (~75% complete)**
   - Re-reset test_helpers to original state (files had been modified)
   - Fixed many enum fields correctly (removed `Some()` where not needed):
     - `campaign.status`, `advertising_channel_type`, `bidding_strategy_type`
     - `ad_group.status`, `r#type`
     - `ad_group_ad.status`
     - Various segment enums (device, day_of_week)
   - Fixed many optional fields with `Some()`:
     - `campaign.network_settings` inner fields (all bools now wrapped)
     - `customer.descriptive_name`, `currency_code`, `time_zone`
     - `metrics` fields (impressions, clicks, ctr, cost_micros, etc.)
     - `segments.date`, `segments.month`, `segments.hour`, `segments.year`
     - `Criterion::KeywordInfo.text`
     - `ad.id`, `ad.name`
     - `AdTextAsset.text`
     - `ResponsiveSearchAdInfo.path1`, `path2`
     - `criterion.criterion_id`, `criterion.cpc_bid_micros`
     - `account_budget.id`, `account_budget.name`
     - `audience.name`
     - `bidding_strategy.id`, `bidding_strategy.name`
     - `label.id`, `label.name`
     - `customer_client` fields (id, client_customer, currency_code, descriptive_name, time_zone)
     - `search_term_view.ad_group`, `search_term_view.search_term`
     - `ad_group_ad_asset_view.asset`

3. **Asset Automation Settings Tests - Preparation**
   - Fixed `create_asset_automation_setting()` helper to wrap enums in `Some()`
   - Fixed campaign.id and campaign.name assignments in tests

### 🔄 Remaining Work

1. **Asset Automation Settings Tests**
   - Need to fix remaining test_helpers compilation errors
   - Then verify tests pass
   - Expected changes: enum value SCREAMING_SNAKE_CASE format

2. **Complete Test Helpers Migration**
   - Approximately 16 more compilation errors remaining
   - Patterns to look for:
     - Non-optional fields incorrectly wrapped in `Some()`
     - Optional fields missing `Some()`

3. **Phase Tests (1, 3, 4, 5, 6, 7)**
   - Files: `tests/google_ads_row_phase*.rs`
   - Expect: Enum value format changes + optional field fixes

4. **Oneof Tests**
   - File: `tests/google_ads_row_oneof_tests.rs`
   - Changes: match_type numeric → enum names (e.g. "3" → "PHRASE")

5. **Other Test Files**
   - `tests/google_ads_row_asset_automation_settings_tests.rs`
   - `tests/integration_streaming_tests.rs`
   - `tests/consumer_surface_tests.rs`
   - `tests/google_ads_row_primary_status_details_tests.rs`
   - `tests/google_ads_row_error_tests.rs`
   - `tests/property_based_tests.rs`
   - `tests/google_ads_row_nested_tests.rs`

## Current Issues

### Test Helpers Compilation Errors (~16 remaining)

From latest log (`1775824660_cargo_test.log`):

**Line numbers with errors (check proto definitions):**
- Lines around: 1000-1300 region
- Fields that likely need `Some()`:
  - `customer_client.level`, `customer_client.manager` (check if optional)
  - `smart_campaign_search_term_view.campaign`, `search_term` (check proto)
  - `change_event.change_date_time` - this is non-optional per previous report
  - `change_event.change_resource_name` (check if optional)
  - `change_event.user_email` (check if optional)
  - `change_event.campaign` (check if optional)

**Non-optional fields that may be incorrectly wrapped:**
- `search_term_view.status` - enum field, should NOT have `Some()`
- `smart_campaign_search_term_view.status` - enum field, should NOT have `Some()`
- `change_event.change_resource_type` - enum field, should NOT have `Some()`
- `change_event.resource_change_operation` - enum field, should NOT have `Some()`
- `customer_client.status` - enum field, should NOT have `Some()`
- Other enum-type fields

### Proto Files to Check for Field Optionality

When fixing compilation errors:

```bash
# Check if field is optional in proto
grep "field_name = " proto/google/ads/googleads/v23/resources/<resource>.proto
# Look for "optional" keyword before the type

# Example: check smart_campaign_search_term_view
cat proto/google/ads/googleads/v23/resources/smart_campaign_search_term_view.proto | grep -E "(campaign|search_term|status) = "

# Example: check change_event
cat proto/google/ads/googleads/v23/resources/change_event.proto | grep -E "(change_date_time|change_resource_name|user_email|campaign) = "
```

## Key Patterns for Updates

### Pattern 1: Enum Fields (NON-optional)
```rust
// Proto definition:
// google.ads.googleads.v23.enums.SomeEnum.SomeEnum status = 10 [(google.api.field_behavior) = OUTPUT_ONLY];

// Correct:
self.resource.status = status_value;  // NO Some()

// Wrong:
self.resource.status = Some(status_value);  // Will error
```

### Pattern 2: Optional Fields
```rust
// Proto definition:
// optional string name = 5 [(google.api.field_behavior) = REQUIRED|OUTPUT_ONLY|IMMUTABLE];
// optional int64 id = 10 [(google.api.field_behavior) = OUTPUT_ONLY];

// Correct:
self.resource.name = Some(value.to_string());
self.resource.id = Some(value);

// Wrong:
self.resource.name = value.to_string();  // Will error
self.resource.id = value;  // Will error
```

### Pattern 3: Proto3 Default Non-optional
```rust
// Proto definition:
// string resource_name = 1 [(google.api.field_behavior) = OUTPUT_ONLY];
// Note: No "optional" keyword, so field is non-optional

// Correct:
self.resource.resource_name = value.to_string();  // NO Some()

// Wrong:
self.resource.resource_name = Some(value.to_string());  // Will error
```

## Quick Fix Commands

### 1. Check Current Compilation Errors
```bash
cargo build --test google_ads_row_asset_automation_settings_tests 2>&1 | grep "error\[E"
```

### 2. Get Error Line Numbers
```bash
cat ~/.local/share/rtk/tee/$(ls -t ~/.local/share/rtk/tee/ 2>/dev/null | head -1) | grep "^   --> tests/test_helpers"
```

### 3. Check Proto Definition for a Field
```bash
# Example: check smart_campaign_search_term_view.campaign
cat proto/google/ads/googleads/v23/resources/smart_campaign_search_term_view.proto | grep "campaign = "
```

### 4. Bulk Fix Pattern (if many similar fields)
```bash
# Fix optional i64 fields
sed -i 's/self\.([a-z_]+)\.([a-z_]+) = \([a-z_]*\);/self.\1.\2 = Some(\3);/g' tests/test_helpers/mod.rs

# Then manually review/correct enum fields
```

## Known Non-Optional String Fields (DO NOT wrap in Some())

From handover 1 and proto review:
- `dynamic_search_ads_setting.domain_name`
- `dynamic_search_ads_setting.language_code`
- `asset_group.id` - wait, this is `int64`, check if optional
- `asset_group.name`
- `asset_group.resource_name`
- `asset_group.campaign`
- `change_event.change_date_time`
- `ad_group_ad_asset_view.resource_name`
- `smart_campaign_search_term_view.campaign`

## Status of Test Files

| Test File | Status | Notes |
|-----------|--------|-------|
| `google_ads_row_primary_status_tests.rs` | ✅ 63/63 passing | Complete |
| `google_ads_row_asset_automation_settings_tests.rs` | 🔄 In progress | test_helpers errors blocking |
| `google_ads_row_enum_tests.rs` | ❓ Unknown | May have been reset - verify |
| `mock_integration_tests.rs` | ❓ Unknown | May have been reset - verify |
| All other test files | ⏳ Not started | Pending |

## Files Modified This Session

```bash
# Check current changes
git status --short

# Current modified files:
M tests/google_ads_row_asset_automation_settings_tests.rs
M tests/google_ads_row_primary_status_tests.rs
M tests/mock_integration_tests.rs
M tests/test_helpers/mod.rs
```

## Recommended Next Steps

1. **Fix remaining test_helpers compilation errors**
   - Use `cargo build --test google_ads_row_asset_automation_settings_tests`
   - For each error:
     1. Get line number and field name
     2. Check proto definition for `optional` keyword
     3. Add or remove `Some()` accordingly
   - Priority: Focus on lines 1000-1300 region first

2. **Run all test_helpers-dependent tests to verify**
   ```bash
   cargo test --test google_ads_row_asset_automation_settings_tests
   cargo test --test google_ads_row_primary_status_tests  # Should still pass
   cargo test --test google_ads_row_enum_tests  # Verify status
   ```

3. **Update asset automation settings test assertions**
   - Enum values likely need SCREAMING_SNAKE_CASE format
   - Check test output for expected vs actual values

4. **Move to next test file**
   - Once asset automation tests pass, continue with phase tests
   - Follow same pattern: check compilation → fix test_helpers → update assertions

## Reference Documents

- **Handover 1**: `reports/prost_reflect_fix_tests_handover_1.md`
- **Handover 2**: `reports/prost_reflect_fix_tests_handover_2.md` (this file)
- **Implementation Summary**: `reports/prost_reflect_implementation_summary.md`
- **Breaking Changes**: `reports/prost_reflect_breaking_changes.md`
- **API Usage Guide**: `reports/prost_reflect_api_usage_guide.md`

## Debugging Tips

### When stuck on compilation error:

1. **Identify the field and resource**:
```rust
// Error at line 1234: expected `Option<i64>`, found `i64`
self.resource.id = id;  // Field: id, Resource: resource
```

2. **Check proto definition**:
```bash
grep "id = " proto/google/ads/googleads/v23/resources/resource.proto
```

3. **Determine fix pattern**:
   - If `optional int64 id` → add `Some()`
   - If `int64 id` (no optional) → remove `Some()`
   - If enum field → should NOT have `Some()`

4. **Apply fix**:
```bash
# Fix single line in test_helpers/mod.rs
sed -i '1234s/self\.resource\.id = id;/self.resource.id = Some(id);/' tests/test_helpers/mod.rs
```

### Common Gotchas

- **Enum fields look optional but aren't**: Proto3 enum fields are never optional
- **`resource_name` fields**: Usually non-optional string
- **Nested fields**: Check the inner message proto, not the parent
- **Repeated fields**: These are `Vec<>`, don't need `Some()` for the vector itself

## Estimated Time to Complete

- Fix remaining test_helpers errors: 30-60 mins
- Asset automation settings tests: 30 mins
- Phase tests (5 files): 1-2 hours
- Other test files: 2-3 hours
- **Total: ~4-6 hours remaining**

Good luck continuing the work! The patterns should be clearer now - always check the proto definition before adding/removing `Some()`.
