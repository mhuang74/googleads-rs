# Handoff: Replace `get()` match arms with prost-reflect - Remaining Work

## Current Status

**IN PROGRESS** - Core implementation complete, test compilation partially fixed, enum assertions need updating.

## What Has Been Done

### 1. Dependencies Added (Cargo.toml)
- Added `prost-reflect = "0.14"` to `[dependencies]`
- Added `prost-reflect-build = "0.14"` to `[build-dependencies]`

### 2. Build Script Updated (build.rs)
- Added file descriptor set generation using `prost_build::Config`
- Added `--experimental_allow_proto3_optional` flag for tonic compilation
- Protos re-downloaded with `optional` keywords preserved

### 3. Core Implementation (src/lib.rs)
Replaced 538 match arms + 10 macros with ~80 lines of reflection-based code:

```rust
// Key components:
- FILE_DESCRIPTOR_SET_BYTES: Static bytes from build output
- DESCRIPTOR_POOL: LazyLock<DescriptorPool> for runtime reflection
- get(&self, field_name: &str) -> String: Main entry point
- get_many(&self, field_names: &[&str]) -> Vec<String>: Efficient batch access
- walk_path(): Recursive GAQL path walker
- format_value(): Formats any prost-reflect Value as string
- is_message_empty(): Helper to detect unset parent messages
```

### 4. Test Helper Fixes (tests/test_helpers/mod.rs)
Fixed type mismatches for proto3 optional fields:
- Wrapped scalar fields in `Some()` where proto uses `optional`
- Removed `Some()` wrapping for fields that are NOT optional
- Updated builder methods to match new proto types

### 5. Phase1 Test Fixes (tests/google_ads_row_phase1_tests.rs)
- Updated struct literals to use `Some()` for optional fields
- Updated enum assertions to expect proto canonical names:
  - "Enabled" → "ENABLED"
  - "Paused" → "PAUSED"
  - "Eligible" → "ELIGIBLE"
  - etc.

## Current Issues

### 1. Test Compilation Errors (Outstanding)
Multiple test files still have type mismatch errors due to proto3 optional fields:

**Files needing updates:**
- `tests/integration_streaming_tests.rs` - ~20 errors
- `tests/google_ads_row_phase2_tests.rs` - ~15 errors
- `tests/google_ads_row_primary_status_details_tests.rs` - ~4 errors
- `tests/google_ads_row_primary_status_tests.rs` - unknown count
- `tests/google_ads_row_nested_tests.rs` - unknown count
- `tests/google_ads_row_oneof_tests.rs` - unknown count
- `tests/google_ads_row_error_tests.rs` - unknown count
- `tests/google_ads_row_phase3_tests.rs` - unknown count
- `tests/google_ads_row_phase4567_tests.rs` - unknown count
- `tests/google_ads_row_repeated_tests.rs` - unknown count
- `tests/google_ads_row_scalar_tests.rs` - partially fixed
- `tests/google_ads_row_enum_tests.rs` - needs enum assertion updates
- `tests/mock_integration_tests.rs` - unknown count
- `tests/property_based_tests.rs` - unknown count

**Common error patterns:**
```rust
// OLD (pre-optional):
campaign.id = 12345;
metrics.impressions = 10000;
segments.date = "2024-10-10".to_string();

// NEW (with optional):
campaign.id = Some(12345);
metrics.impressions = Some(10000);
segments.date = Some("2024-10-10".to_string());
```

### 2. Empty Message Handling Issue
**Problem:** When accessing a nested field path like `campaign_budget.amount_micros` and `campaign_budget` is not set, the current implementation may return "0" (default value) instead of "" (empty string).

**Expected behavior (per existing tests):**
- If parent message is absent → return ""
- If parent exists but child is default → return default value (e.g., "0")

**Solution attempted:** Added `is_message_empty()` helper in `src/lib.rs` to detect unset parent messages, but needs verification.

### 3. Enum Formatting Breaking Changes
**Confirmed behavior:** Enums now return proto canonical names (UPPER_SNAKE_CASE) instead of PascalCase:

| Old Value | New Value |
|-----------|-----------|
| "Enabled" | "ENABLED" |
| "Paused" | "PAUSED" |
| "Eligible" | "ELIGIBLE" |
| "Headline" | "HEADLINE" |
| "Description" | "DESCRIPTION" |
| "Sitelink" | "SITELINK" |
| "Webpage" | "WEBPAGE" |
| "Purchase" | "PURCHASE" |
| "Open" | "OPEN" |
| "Remarketing" | "REMARKETING" |
| "TenThousandToFiftyThousand" | "TEN_THOUSAND_TO_FIFTY_THOUSAND" |
| "Approved" | "APPROVED" |
| "Reviewed" | "REVIEWED" |

### 4. Special Fields Not Yet Verified
- `campaign.bidding_strategy_type` - previously returned custom camelCase names
- `change_event.changed_fields` - previously returned single-quoted format

## Remaining TODOs

### Phase 1: Fix Test Compilation
For each test file with errors:

1. **Add `Some()` wrapping for proto3 optional scalar fields:**
   ```rust
   // Integers
   id: 123 → id: Some(123)
   
   // Strings
   name: "foo".to_string() → name: Some("foo".to_string())
   
   // Booleans
   eligible: true → eligible: Some(true)
   
   // Floats
   ctr: 0.05 → ctr: Some(0.05)
   ```

2. **Fix nested message construction:**
   ```rust
   // NetworkSettings fields need Some() wrapping
   target_search_network: true → target_search_network: Some(true)
   ```

### Phase 2: Fix Enum Assertions
Update test assertions to expect UPPER_SNAKE_CASE enum values.

Search pattern: `assert_eq!(row.get("..."), "...");` where value is PascalCase

### Phase 3: Verify Special Fields
- Check if `campaign.bidding_strategy_type` needs custom handling
- Check if `change_event.changed_fields` format has changed

### Phase 4: Verify Empty Message Handling
Ensure `is_message_empty()` correctly returns "" for unset parent messages:
- `test_campaign_budget_amount_micros_absent` expects "" when budget not set
- Verify behavior with other nested paths

### Phase 5: Add New Tests for Reflection Coverage
Add tests for fields not in original 538 match arms:
- `account_link` fields
- Other resources that were unsupported

## Key Implementation Details

### src/lib.rs Structure
```rust
// Lines 36-45: File descriptor set
static FILE_DESCRIPTOR_SET_BYTES: &[u8] = ...
static DESCRIPTOR_POOL: LazyLock<prost_reflect::DescriptorPool> = ...

// Lines 68-89: get() method
pub fn get(&self, field_name: &str) -> String

// Lines 91-112: get_many() method
pub fn get_many(&self, field_names: &[&str]) -> Vec<String>

// Lines 114-180: walk_path() - GAQL path traversal
fn walk_path(msg: &DynamicMessage, path: &str) -> String

// Lines 182-232: format_value() - value formatting
fn format_value(value: &Value, field_desc: Option<&prost_reflect::FieldDescriptor>) -> String

// Lines 234-258: is_message_empty() - empty message detection
fn is_message_empty(msg: &DynamicMessage) -> bool
```

### Enum Handling
Enums resolved via `EnumDescriptor::values()` iteration:
```rust
Value::EnumNumber(num) => {
    if let Some(field) = field_desc {
        if let prost_reflect::Kind::Enum(enum_desc) = field.kind() {
            for value in enum_desc.values() {
                if value.number() == *num {
                    return value.name().to_string(); // Returns "ENABLED"
                }
            }
        }
    }
    num.to_string()
}
```

## Verification Commands

```bash
# Build library
cargo build

# Run specific test file
cargo test --test google_ads_row_phase1_tests

# Check for compilation errors (faster than full test)
cargo test --no-run

# Run all tests
cargo test
```

## Risks / Watch Out For

1. **File descriptor set size**: ~4.1MB embedded in binary - verify this is acceptable
2. **Performance**: Each `get()` call encodes/decodes the row - acceptable for current use case but monitor
3. **Oneof fields**: GAQL paths like `ad_group_criterion.keyword.text` should work via normal path walking
4. **Optional field presence**: With `optional` restored, unset fields return "" (empty string) or default values depending on nesting level

## References

- Original spec: `specs/support_all_fields_via_prost_reflect.md`
- Prost-reflect docs: https://docs.rs/prost-reflect/0.14/
- GAQL field paths: https://developers.google.com/google-ads/api/fields/v23/overview
