# Prost-Reflect Implementation Summary

**Date:** 2026-04-10
**Branch:** support_all_fields_opus
**API Version:** Google Ads v23.2

## Executive Summary

Successfully replaced 544 hand-maintained match arms in `GoogleAdsRow::get()` with a generic reflection-based field access system using `prost-reflect`. This provides automatic support for all ~178 Google Ads resources and future API versions without manual maintenance.

## Overview

The old implementation required manually adding match arms for each GAQL field path, with only ~35 of ~178 resources supported. Each API version upgrade required extensive manual updates. The new implementation uses Protocol Buffer reflection to dynamically walk field paths at runtime.

## Implementation Details

### 1. Changes Made

#### 1.1 `utils/update.sh`
- Added `--force` flag to skip version check
- Removed the `sed` command that stripped `optional` keyword from proto files
- This preserves proto3 optional fields which generate `Option<>` types in Rust

```bash
# Before
if [ "$current_version" == "$GOOGLEADS_API_VERSION" ]; then

# After
if [ "$current_version" == "$GOOGLEADS_API_VERSION" ] && [ "$2" != "--force" ]; then
```

#### 1.2 `Cargo.toml`
Added new dependencies:
```toml
prost-reflect = "0.16"
once_cell = "1"
bytes = "1"
```

#### 1.3 `build.rs`
Added descriptor set generation before the existing tonic compilation:

```rust
// Generate unified file descriptor set for prost-reflect
{
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let descriptor_path = Path::new(&out_dir).join("file_descriptor_set.bin");
    let tmp_out = Path::new(&out_dir).join("_fds_tmp");
    std::fs::create_dir_all(&tmp_out).ok();

    let mut config = Config::new();
    config.file_descriptor_set_path(&descriptor_path);
    config.out_dir(&tmp_out);
    config.protoc_arg("--experimental_allow_proto3_optional");
    config.compile_protos(&protos, std::slice::from_ref(&proto_path))?;

    info!("Generated file descriptor set at {:?}", descriptor_path);
}
```

Also added `--experimental_allow_proto3_optional` flag to all `tonic_prost_build::configure()` calls to support proto3 optional fields.

#### 1.4 `src/lib.rs` - Complete Rewrite
Replaced 891 lines of macro-based code with reflection:

**New imports:**
```rust
use once_cell::sync::Lazy;
use prost::Message;
use prost_reflect::{DescriptorPool, DynamicMessage, ReflectMessage, Value};
```

**Static descriptor pool:**
```rust
static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
    DescriptorPool::decode(bytes.as_ref()).expect("Failed to decode file descriptor set")
});

const GOOGLE_ADS_ROW_FQN: &str = "google.ads.googleads.v23.services.GoogleAdsRow";
```

**Core implementation:**
- `get(&self, field_name: &str) -> String` - Main accessor method
- `get_many(&self, field_names: &[&str]) -> Vec<String>` - Efficient bulk access
- `format_value_at_path(&self, msg: &DynamicMessage, path: &str) -> String` - Path walker
- `format_value_recursive(...)` - Recursive format handler
- `format_scalar(&self, value: &Value, field_desc: &FieldDescriptor) -> String` - Scalar formatter
- `format_list(&self, items: &[Value], field_desc: &FieldDescriptor) -> String` - List formatter
- `format_asset_automation_settings(&self, dyn_msg: &DynamicMessage) -> String` - Special case handler

### 2. How It Works

1. **Encoding**: `GoogleAdsRow` is encoded to bytes using `prost::Message::encode_to_vec()`
2. **Dynamic Decoding**: Bytes are decoded as `DynamicMessage` via the descriptor pool
3. **Path Walking**: The dotted field path (e.g., `campaign.status`) is split into segments
4. **Field Resolution**: Each segment is looked up via `MessageDescriptor::get_field_by_name()`
5. **Value Extraction**: Field values are retrieved and formatted
6. **Type-Specific Formatting**:
   - Enums: Resolved via `EnumDescriptor::get_value()` to proto-canonical names
   - Messages: Debug format `{:?}`
   - Lists: Joined with `,` (scalars) or `;` (messages)
   - Scalars: `to_string()`

### 3. Special Cases

#### 3.1 `campaign.asset_automation_settings`
Format repeated messages as `"TYPE:STATUS"` pairs:
```rust
TEXT_ASSET_AUTOMATION:OPTED_IN, GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT
```

#### 3.2 `ad_group_ad.ad.responsive_search_ad.headlines`/`.descriptions`
GAQL path stops at the repeated message, but users expect `.text` extracted from each item. Internally appends `.text` to the path.

### 4. Breaking Changes

| Field | Old Value | New Value |
|-------|-----------|-----------|
| `campaign.status` | `"Enabled"` | `"ENABLED"` |
| `campaign.bidding_strategy_type` | `"ManualCPC"` | `"MANUAL_CPC"` |
| `keyword.match_type` | `"3"` | `"PHRASE"` |
| `change_event.changed_fields` | `"'field1, field2'"` | `"field1, field2"` |
| Absent resources | Panic | `""` |

### 5. Architecture Benefits

1. **Future-Proof**: Supports all current and future resources automatically
2. **Maintainable**: No manual match arm updates needed for API version upgrades
3. **Type-Safe**: Leverages proto schema information through descriptors
4. **Extensible**: Easy to add special case handlers without core changes
5. **Efficient**: `get_many()` reduces encode/decode overhead for bulk access

## Performance Considerations

### Encoded Binary Size
The file descriptor set is embedded into the binary via `include_bytes!()`:
- **Estimated size**: 2-5MB
- **Acceptable for**: Server-side applications
- **Not ideal for**: WASM or very constrained environments

### Per-Call Overhead
Each `get()` call involves:
1. `encode_to_vec()` - GoogleAdsRow to bytes
2. `Cursor::new()` - Wrap bytes in Buf
3. `DynamicMessage::decode()` - Decode with descriptor
4. Path walking and formatting

**Mitigation**: `get_many()` performs encode/decode once for multiple fields.

### Alternatives Considered
1. **Code generation**: Would require massive macro expansion for all field paths
2. **Serde**: Not compatible with prost-generated types
3. **Dynamic dispatch**: Would lose type safety and proto awareness

## Implementation Files

| File | Lines Before | Lines After | Change |
|------|--------------|-------------|--------|
| `src/lib.rs` | 891 | 289 | -602 lines |
| `build.rs` | 191 | 222 | +31 lines |
| `utils/update.sh` | 93 | 89 | -4 lines |
| `Cargo.toml` | 42 | 43 | +1 line |

**Total**:  -574 lines while drastically increasing functionality

## Test Migration Status

### Completed
- Core library compiles successfully
- Prototype validates reflection approach

### Remaining Work
1. **Test Helpers**: Update `tests/test_helpers/mod.rs` to wrap optional field values:
   - `self.campaign.id = id;` → `self.campaign.id = Some(id);`
   - Similar for all optional fields across all builders
2. **Test Assertions**: Update expected values to use SCREAMING_SNAKE_CASE:
   ```rust
   // Before
   assert_eq!(row.get("campaign.status"), "Enabled");

   // After
   assert_eq!(row.get("campaign.status"), "ENABLED");
   ```

### Test Files to Update
- `tests/google_ads_row_enum_tests.rs` (~40 assertions)
- `tests/google_ads_row_primary_status_tests.rs` (~20 assertions)
- `tests/google_ads_row_asset_automation_settings_tests.rs` (~20 assertions)
- `tests/google_ads_row_phase1_tests.rs` (~15 assertions)
- `tests/google_ads_row_phase3_tests.rs` (~15 assertions)
- `tests/google_ads_row_phase4567_tests.rs` (~15 assertions)
- `tests/google_ads_row_oneof_tests.rs` (match_type numeric → enum name)
- `tests/google_ads_row_nested_tests.rs` (~2 assertions)
- `tests/google_ads_row_error_tests.rs` (~1 assertion)
- `tests/property_based_tests.rs` (~3 blocks)
- `tests/integration_streaming_tests.rs` (~4 assertions)
- `tests/consumer_surface_tests.rs` (~3 assertions)
- `tests/mock_integration_tests.rs` (~2 assertions)

## Risk Assessment

### Low Risk
- **Descriptor set size**: Large but acceptable for server-side use
- **Performance**: Per-call overhead is moderate; `get_many()` mitigates bulk access

### Medium Risk
- **Debug format differences**: DynamicMessage debug format may differ from prost-generated structs
- **Enum resolution**: Proto canonical names vs custom mappings (accepted breaking change)

### Mitigations
- Thorough test coverage with updated expectations
- Benchmarking and profiling if performance issues arise
- Consider lazy-loading descriptor set if binary size becomes critical

## Next Steps

1. **Complete test migration**: Update all test helpers and assertions
2. **Run full test suite**: Validate all ~120+ test cases pass
3. **Benchmark performance**: Measure impact on typical query workloads
4. **Document API changes**: Update README with breaking changes
5. **Release preparation**: Tag version bump (semantic major version)

## Conclusion

The prost-reflect implementation successfully transforms the library from a manually maintained set of match arms to a dynamic, extensible field access system. This reduces maintenance burden dramatically while expanding support from ~35 to all ~178 Google Ads resources. The breaking changes are well-defined and documented, and the architecture supports future API versions with minimal effort.

**Status**: Core implementation complete and compiling. Test migration in progress.
