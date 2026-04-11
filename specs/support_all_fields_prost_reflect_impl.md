# Plan: Replace `get()` match arms with prost-reflect dynamic field access

## Context

`GoogleAdsRow::get(field_name: &str)` in `src/lib.rs` has **544 hand-maintained match arms** and **9 macros** to convert GAQL field paths to strings. Only ~35 of ~178 resources are supported. Each API version upgrade requires manually adding match arms.

**Goal:** Replace everything with a generic reflection-based field walker using `prost-reflect`. Every valid GAQL field path works automatically, including new fields in future API versions.

**Breaking changes accepted:**
- Enum values change from PascalCase (`Enabled`) to proto-canonical SCREAMING_SNAKE_CASE (`ENABLED`)
- `campaign.bidding_strategy_type` loses custom names (`ManualCPC` → `MANUAL_CPC`)
- `change_event.changed_fields` drops single-quote wrapping
- `keyword.match_type` changes from raw i32 (`"3"`) to enum name (`"PHRASE"`)
- Absent resources return `""` instead of panicking

---

## Step 1: Update `utils/update.sh` — stop stripping `optional`

**File:** `utils/update.sh`

1. Add `--force` flag support at line 30:
   ```bash
   if [ "$current_version" == "$GOOGLEADS_API_VERSION" ] && [ "$2" != "--force" ]; then
   ```

2. Delete lines 62-64 (the `sed` that strips `optional`):
   ```bash
   # DELETE these 3 lines:
   safe_run find proto -type f | while read -r file; do
     sed_inplace -e 's/^ *optional//g' "$file" || echo "Failed to process $file"
   done
   ```

3. Re-run: `./utils/update.sh v23 --force` to re-download fresh protos with `optional` intact.

---

## Step 2: Add dependencies to `Cargo.toml`

**File:** `Cargo.toml`

Add to `[dependencies]`:
```toml
prost-reflect = "0.16"
once_cell = "1"
```

No build-dep needed — we generate the descriptor set via `prost_build::Config` directly (already a build-dep).

---

## Step 3: Generate file descriptor set in `build.rs`

**File:** `build.rs`

Insert after line 55 (after the proto walk loop), before the chunked compilation. Use a separate `prost_build::Config` pass that generates ONLY the descriptor set, writing throwaway Rust code to a temp dir:

```rust
// Generate unified file descriptor set for prost-reflect
{
    let out_dir = env::var("OUT_DIR").unwrap();
    let descriptor_path = Path::new(&out_dir).join("file_descriptor_set.bin");
    let tmp_out = Path::new(&out_dir).join("_fds_tmp");
    std::fs::create_dir_all(&tmp_out).ok();

    let mut config = prost_build::Config::new();
    config.file_descriptor_set_path(&descriptor_path);
    config.out_dir(&tmp_out);
    config.compile_protos(&protos, std::slice::from_ref(&proto_path))?;

    info!("Generated file descriptor set at {:?}", descriptor_path);
}
```

This runs protoc once with ALL protos to produce a single unified descriptor. The chunked tonic compilation continues as before for code generation.

---

## Step 4: Rewrite `src/lib.rs` — the core change

**File:** `src/lib.rs` (replace lines 30-891)

### 4a. Remove all old imports (lines 30-50)

Delete the `use` statements for `BiddingStrategyType::*`, `*PrimaryStatusReason`, `InteractionEventType`, and oneof `Criterion::*` variants.

### 4b. Add new imports

```rust
use once_cell::sync::Lazy;
use prost::Message;
use prost_reflect::{DescriptorPool, DynamicMessage, Value};
```

### 4c. Static descriptor pool

```rust
static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
    DescriptorPool::decode(bytes.as_ref()).expect("Failed to decode file descriptor set")
});

const GOOGLE_ADS_ROW_FQN: &str = "google.ads.googleads.v23.services.GoogleAdsRow";
```

### 4d. Reflection walker functions

Three core functions:

1. **`format_value_at_path(msg, path_segments)`** — walks dotted path segments on a `DynamicMessage`:
   - Looks up field by name via `msg.descriptor().get_field_by_name(segment)`
   - For unset message fields with presence: returns `""`
   - For set fields: delegates to `format_value_recursive`
   - Unknown field name → `"not implemented by googleads-rs"`

2. **`format_value_recursive(value, remaining_path, field_desc)`** — pattern matches on `Value`:
   - `Value::Message(sub_msg)` + more path → recurse into sub-msg
   - `Value::Message(sub_msg)` terminal → `format!("{:?}", sub_msg)`
   - `Value::List(items)` + more path → walk into each message item, join with `", "`
   - `Value::List(items)` terminal → delegate to `format_list`
   - Scalar terminal → delegate to `format_scalar`

3. **`format_scalar(value, field_desc)`** — formats individual values:
   - `Value::EnumNumber(n)` → resolve via `EnumDescriptor::get_value(n)` → `"ENABLED"`, `"PAUSED"` etc.
   - `Value::String(s)` → return clone
   - `Value::Bool/I32/I64/U32/U64/F32/F64` → `.to_string()`
   - `Value::Bytes` → debug format

4. **`format_list(items, field_desc)`** — joins values:
   - Message items: join with `"; "` (matches current `repeated_message_str!` behavior)
   - All others: join with `", "`

### 4e. Special-case handlers

Only two special cases needed:

1. **`campaign.asset_automation_settings`** — walk to the campaign message, then for each repeated `AssetAutomationSetting` message, extract two enum fields and format as `"TYPE:STATUS"` (proto-canonical names). E.g., `"TEXT_ASSET_AUTOMATION:OPTED_IN"`.

2. **`ad_group_ad.ad.responsive_search_ad.headlines`** and **`.descriptions`** — the GAQL path stops at the repeated message, but users expect the `.text` sub-field extracted from each. Internally append `.text` to the path and let the walker handle it via List+remaining_path logic.

### 4f. New `get()` and `get_many()` methods

```rust
impl GoogleAdsRow {
    pub fn get(&self, field_name: &str) -> String {
        // encode self → bytes → decode as DynamicMessage → walk path
    }

    pub fn get_many(&self, field_names: &[&str]) -> Vec<String> {
        // encode once, walk multiple paths
    }

    fn get_field_from_dynamic(dyn_msg: &DynamicMessage, field_name: &str) -> String {
        // check special cases first, then general walk
    }
}
```

**Key API notes (verified against prost-reflect 0.16.3 source):**
- `DescriptorPool::decode(bytes: impl Buf) -> Result<Self, DescriptorError>`
- `DynamicMessage::decode(desc: MessageDescriptor, buf: &[u8]) -> Result<Self, DecodeError>`
- `msg.get_field(&field_desc) -> Cow<'_, Value>` — returns borrowed or owned
- `msg.has_field(&field_desc) -> bool`
- `field_desc.supports_presence() -> bool`
- `field_desc.is_list() -> bool`
- `field_desc.kind() -> Kind` — `Kind::Enum(EnumDescriptor)` for enum fields
- `enum_desc.get_value(number: i32) -> Option<EnumValueDescriptor>`
- `enum_value_desc.name() -> &str` — returns proto-canonical name like `"ENABLED"`
- Oneof variant fields are accessed by their field name (e.g., `get_field_by_name("keyword")` on `AdGroupCriterion`). If a different variant is active, `has_field` returns false.

---

## Step 5: Migrate test assertions

~120+ assertions across 14 test files need updating. Key changes:

### Enum value mappings (PascalCase → SCREAMING_SNAKE_CASE)

| Current | New |
|---------|-----|
| `"Enabled"` | `"ENABLED"` |
| `"Paused"` | `"PAUSED"` |
| `"Removed"` | `"REMOVED"` |
| `"Unspecified"` | `"UNSPECIFIED"` |
| `"Unknown"` | `"UNKNOWN"` |
| `"Search"` | `"SEARCH"` |
| `"Display"` | `"DISPLAY"` |
| `"Shopping"` | `"SHOPPING"` |
| `"Video"` | `"VIDEO"` |
| `"PerformanceMax"` | `"PERFORMANCE_MAX"` |
| `"ManualCPC"` | `"MANUAL_CPC"` |
| `"MaximizeConversions"` | `"MAXIMIZE_CONVERSIONS"` |
| `"MaximizeConversionValue"` | `"MAXIMIZE_CONVERSION_VALUE"` |
| `"TargetCPA"` | `"TARGET_CPA"` |
| `"TargetROAS"` | `"TARGET_ROAS"` |
| `"TargetImpShare"` | `"TARGET_IMPRESSION_SHARE"` |
| `"Unsupported"` (EnhancedCpc) | `"ENHANCED_CPC"` |
| `"SearchStandard"` | `"SEARCH_STANDARD"` |
| `"DisplayStandard"` | `"DISPLAY_STANDARD"` |
| `"ShoppingProductAds"` | `"SHOPPING_PRODUCT_ADS"` |
| `"VideoTrueViewInStream"` | `"VIDEO_TRUE_VIEW_IN_STREAM"` |
| `"Mobile"` | `"MOBILE"` |
| `"Desktop"` | `"DESKTOP"` |
| `"Tablet"` | `"TABLET"` |
| `"Monday"` | `"MONDAY"` |
| `"Friday"` | `"FRIDAY"` |
| `"Sunday"` | `"SUNDAY"` |
| `"Canceled"` | `"CANCELED"` |
| `"Suspended"` | `"SUSPENDED"` |
| `"Eligible"` | `"ELIGIBLE"` |
| `"Pending"` | `"PENDING"` |
| `"CampaignPaused"` | `"CAMPAIGN_PAUSED"` |
| `"CampaignRemoved"` | `"CAMPAIGN_REMOVED"` |
| `"CampaignPending"` | `"CAMPAIGN_PENDING"` |
| `"Keyword"` | `"KEYWORD"` |
| `"Approved"` | `"APPROVED"` |
| `"Reviewed"` | `"REVIEWED"` |
| `"Headline"` | `"HEADLINE"` |
| `"Headline1"` | `"HEADLINE_1"` |
| `"Description"` | `"DESCRIPTION"` |
| `"Sitelink"` | `"SITELINK"` |
| `"Image"` | `"IMAGE"` |
| `"Webpage"` | `"WEBPAGE"` |
| `"Purchase"` | `"PURCHASE"` |
| `"Open"` | `"OPEN"` |
| `"Remarketing"` | `"REMARKETING"` |
| `"NegativeKeywords"` | `"NEGATIVE_KEYWORDS"` |
| `"Added"` | `"ADDED"` |
| `"Campaign"` | `"CAMPAIGN"` |
| `"GoogleAdsWebClient"` | `"GOOGLE_ADS_WEB_CLIENT"` |
| `"Update"` | `"UPDATE"` |
| `"Best"` | `"BEST"` |
| `"TenThousandToFiftyThousand"` | `"TEN_THOUSAND_TO_FIFTY_THOUSAND"` |
| `"LocationOfPresence"` | `"LOCATION_OF_PRESENCE"` |
| `"TextAssetAutomation:OptedIn"` | `"TEXT_ASSET_AUTOMATION:OPTED_IN"` |
| (etc. for all asset_automation pairs) | |

### Special format changes

- `change_event.changed_fields`: `"'field1, field2'"` → `"field1, field2"` (drop quotes)
- `keyword.match_type`: `"3"` → `"PHRASE"`, `"4"` → `"BROAD"`, `"2"` → `"EXACT"`
- `ad_group.ad_strength` (if enum): `"7"` → proto name (need to check)

### Property-based tests (`tests/property_based_tests.rs`)

Lines 303-304, 333-334, 371-372 use `format!("{:?}", status)` to compare. Change to use a helper that converts enum variant to proto name, or use `prost_reflect` itself in the test to resolve the expected name.

### Panic → empty string tests

Tests expecting panics on absent resources (e.g., `test_campaign_asset_automation_settings_campaign_absent` in `tests/google_ads_row_asset_automation_settings_tests.rs`) now expect `""` instead.

### Test files to modify

1. `tests/google_ads_row_enum_tests.rs` (~40 assertions)
2. `tests/google_ads_row_primary_status_tests.rs` (~20 assertions)
3. `tests/google_ads_row_asset_automation_settings_tests.rs` (~20 assertions)
4. `tests/google_ads_row_phase1_tests.rs` (~15 assertions)
5. `tests/google_ads_row_phase3_tests.rs` (~15 assertions)
6. `tests/google_ads_row_phase4567_tests.rs` (~15 assertions)
7. `tests/google_ads_row_oneof_tests.rs` (match_type i32 → enum name)
8. `tests/google_ads_row_nested_tests.rs` (~2 assertions)
9. `tests/google_ads_row_error_tests.rs` (~1 assertion)
10. `tests/property_based_tests.rs` (~3 blocks)
11. `tests/integration_streaming_tests.rs` (~4 assertions)
12. `tests/consumer_surface_tests.rs` (~3 assertions)
13. `tests/mock_integration_tests.rs` (~2 assertions)
14. `tests/google_ads_row_primary_status_details_tests.rs` (debug format may change)

---

## Step 6: Build and verify

```bash
./utils/update.sh v23 --force   # restore protos with optional
cargo build                      # verify descriptor set + compilation
cargo test                       # verify all tests pass
```

---

## Risks

1. **Descriptor set size**: ~2-5MB embedded in binary via `include_bytes!`. Acceptable for server-side crate.
2. **Performance**: encode+decode on every `get()` call. `get_many()` mitigates for bulk access.
3. **`DynamicMessage` Debug format**: differs from prost-generated struct Debug. Affects tests checking `repeated_message_str!` output and `primary_status_details`. Will need to adjust assertions during test iteration.
4. **Proto download version drift**: `update.sh` fetches `master` branch of googleapis. Current checked-in protos should match since we're pinning `v23`.
