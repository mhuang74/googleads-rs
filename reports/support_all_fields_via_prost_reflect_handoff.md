# Handoff: Replace `get()` match arms with prost-reflect dynamic field access

## Current Status

**IN PROGRESS** - Core implementation complete, needs testing and refinement.

## What Has Been Done

### 1. Dependencies Added (Cargo.toml)
- Added `prost-reflect = "0.14"` to `[dependencies]`
- Added `prost-reflect-build = "0.14"` to `[build-dependencies]`

### 2. Build Script Updated (build.rs)
- Added file descriptor set generation using `prost_build::Config`
- Added `--experimental_allow_proto3_optional` flag for tonic compilation
- Protos re-downloaded with `optional` keywords preserved (was previously stripped)

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
```

### 4. Key Implementation Details

**Enum Handling:**
- Enum values are formatted as proto canonical names (e.g., "ENABLED" not "Enabled")
- This is an intentional breaking change per user direction

**Field Path Walking:**
- Splits dotted paths (e.g., "campaign.status")
- Uses `DynamicMessage::get_field()` to traverse nested messages
- Returns "not implemented by googleads-rs" for invalid paths

**Value Formatting:**
- Scalars: Standard Display formatting
- Enums: Resolved via `EnumDescriptor::values()` iteration
- Messages: `format!("{:#?}", msg)` debug format
- Lists: Joined with ", " for scalars/enums, "; " for messages

## Current Issues

### 1. Build Status
```bash
$ cargo build
# Should compile successfully now
# Last known: 30 warnings, 0 errors (need to verify)
```

### 2. Test Status
- **NOT YET RUN** - Tests need to be updated for enum format changes
- Enum assertions expect "Enabled" but now get "ENABLED"
- Need to update ~80 test assertions

### 3. Known Breaking Changes
1. **Enum formatting**: "Enabled" → "ENABLED", "Paused" → "PAUSED", etc.
2. **Custom enum names**: `campaign.bidding_strategy_type` used custom camelCase names (ManualCPC, TargetCPA) but now returns proto names (MANUAL_CPC, TARGET_CPA)
3. **changed_fields**: Single-quoted format `'path1, path2'` now returns comma-joined without quotes

## Next Steps for Completing

### Step 1: Verify Build
```bash
cargo build 2>&1 | tail -50
```

### Step 2: Run Tests to See Failures
```bash
cargo test 2>&1 | tee /tmp/test_output.txt
```

### Step 3: Update Enum Tests
Files likely needing updates (search for enum assertions):
- `tests/google_ads_row_enum_tests.rs`
- `tests/google_ads_row_phase1_tests.rs`
- `tests/google_ads_row_phase2_tests.rs`
- etc.

Change patterns like:
```rust
assert_eq!(row.get("campaign.status"), "Enabled");
// to:
assert_eq!(row.get("campaign.status"), "ENABLED");
```

### Step 4: Handle Special Cases (if needed)
Check if these need custom handling:
1. `campaign.bidding_strategy_type` - was returning custom camelCase
2. `change_event.changed_fields` - was single-quoted

### Step 5: Add New Tests
Add tests for previously unsupported fields to validate reflection coverage:
```rust
// Example: test account_link which wasn't in the 538 match arms
let row = GoogleAdsRowBuilder::new()
    .with_account_link(/* ... */)
    .build();
assert_eq!(row.get("account_link.resource_name"), "...");
```

### Step 6: Performance Check (Optional)
Benchmark `get()` vs `get_many()` on a row with 20 fields.

## Files Modified

| File | Status |
|------|--------|
| Cargo.toml | ✅ Updated with prost-reflect deps |
| build.rs | ✅ Added file descriptor set generation |
| utils/update.sh | ✅ Removed optional stripping, added --force flag |
| src/lib.rs | ✅ Replaced get() implementation |
| tests/*.rs | ⏳ Need enum assertion updates |

## User Decisions Confirmed

1. ✅ **Accept breaking change** for enum formatting (use proto names like "ENABLED")
2. ✅ **Use reflection default** for change_event.changed_fields (no single quotes)
3. ✅ **Re-download protos** with optional keywords preserved

## Key Code Locations

### Reflection Setup (src/lib.rs:36-45)
```rust
static FILE_DESCRIPTOR_SET_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));

static DESCRIPTOR_POOL: LazyLock<prost_reflect::DescriptorPool> = LazyLock::new(|| {
    prost_reflect::DescriptorPool::decode(FILE_DESCRIPTOR_SET_BYTES).expect(...)
});
```

### Main get() Method (src/lib.rs:68-89)
Encodes row → DynamicMessage → walk_path()

### Path Walker (src/lib.rs:114-149)
Splits dotted path, traverses message tree via get_field()

### Value Formatter (src/lib.rs:151-218)
Matches Value types, handles enums via EnumDescriptor iteration

## Verification Commands

```bash
# Build
cargo build

# Run tests (expect failures due to enum format changes)
cargo test

# Check specific test
cargo test test_campaign_status_enabled -- --nocapture

# Build with release (check for any additional issues)
cargo build --release
```

## Risks / Watch Out For

1. **File descriptor set size**: ~4.1MB embedded in binary - verify this is acceptable
2. **Performance**: Each `get()` call encodes/decodes the row - acceptable for current use case but monitor
3. **Oneof fields**: GAQL paths like `ad_group_criterion.keyword.text` should work via normal path walking
4. **Optional field presence**: With `optional` restored, unset fields return "" (empty string)

## References

- Original spec: `specs/support_all_fields_via_prost_reflect.md`
- Prost-reflect docs: https://docs.rs/prost-reflect/0.14/
- GAQL field paths: https://developers.google.com/google-ads/api/fields/v23/overview
