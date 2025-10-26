# Implementation Spec: Support campaign.asset_automation_settings

## Overview
This document specifies how to implement support for the `campaign.asset_automation_settings` field in `GoogleAdsRow::get()`.

## Field Analysis

### Proto Definition
From [campaign.proto:791](../proto/google/ads/googleads/v22/resources/campaign.proto#L791):
```protobuf
repeated AssetAutomationSetting asset_automation_settings = 88;
```

From [campaign.proto:430-438](../proto/google/ads/googleads/v22/resources/campaign.proto#L430-438):
```protobuf
message AssetAutomationSetting {
  // The asset automation type advertiser would like to opt-in/out.
  google.ads.googleads.v22.enums.AssetAutomationTypeEnum
      .AssetAutomationType asset_automation_type = 1;

  // The opt-in/out status of asset automation type.
  google.ads.googleads.v22.enums.AssetAutomationStatusEnum
      .AssetAutomationStatus asset_automation_status = 2;
}
```

### Field Classification
- **Parent Resource**: `campaign` (non-optional, guaranteed present when selected)
- **Field Type**: Repeated nested message (`Vec<AssetAutomationSetting>`)
- **Nested Fields**: Two enum fields accessed via generated accessor methods:
  - `asset_automation_type()` → returns `AssetAutomationType` enum
  - `asset_automation_status()` → returns `AssetAutomationStatus` enum

### Enum Values

**AssetAutomationType** ([asset_automation_type.proto](../proto/google/ads/googleads/v22/enums/asset_automation_type.proto)):
- `UNSPECIFIED = 0`
- `UNKNOWN = 1`
- `TEXT_ASSET_AUTOMATION = 2`
- `GENERATE_VERTICAL_YOUTUBE_VIDEOS = 3`
- `GENERATE_SHORTER_YOUTUBE_VIDEOS = 4`
- `GENERATE_LANDING_PAGE_PREVIEW = 5`
- `GENERATE_ENHANCED_YOUTUBE_VIDEOS = 6`
- `GENERATE_IMAGE_ENHANCEMENT = 7`
- `GENERATE_IMAGE_EXTRACTION = 9`
- `GENERATE_DESIGN_VERSIONS_FOR_IMAGES = 10`
- `FINAL_URL_EXPANSION_TEXT_ASSET_AUTOMATION = 11`
- `GENERATE_VIDEOS_FROM_OTHER_ASSETS = 12`

**AssetAutomationStatus** ([asset_automation_status.proto](../proto/google/ads/googleads/v22/enums/asset_automation_status.proto)):
- `UNSPECIFIED = 0`
- `UNKNOWN = 1`
- `OPTED_IN = 2`
- `OPTED_OUT = 3`

## Implementation Decision

### Approach
Since `asset_automation_settings` is a **repeated nested message** where each message contains **two enum fields accessed via methods**, we need a pattern that:

1. Iterates over the repeated `Vec<AssetAutomationSetting>`
2. For each item, calls both accessor methods: `asset_automation_type()` and `asset_automation_status()`
3. Formats each pair as `"TYPE:STATUS"`
4. Joins all pairs with `", "`

### Output Format
Based on user preference, the output will be TYPE:STATUS pairs:
```
"TEXT_ASSET_AUTOMATION:OPTED_IN, GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT"
```

### Why Existing Macros Don't Work

1. **`repeated_enum_str!`** - Only handles repeated enum values (Vec<i32>), not repeated messages
2. **`repeated_message_str!`** - Uses debug formatting for entire structs, not selective field access
3. **`enum_match_iterator_str!`** - Only for iterating nested arrays inside a oneof variant, not top-level repeated fields
4. **`attr_str!`** - Only for scalar fields, not repeated

### New Macro Required: `repeated_nested_enum_pair_str!`

A new macro is needed to handle the specific pattern of:
- Repeated nested messages (not a oneof, just a direct Vec field)
- Accessing two enum methods on each item
- Formatting as colon-separated pairs

## Implementation Steps

### 1. Add Required Imports
At the top of `src/lib.rs`, add to the existing `use` statements around line 36-43:

```rust
use crate::google::ads::googleads::v22::enums::{
    // ... existing imports ...
    asset_automation_status_enum::AssetAutomationStatus,
    asset_automation_type_enum::AssetAutomationType,
};
```

### 2. Define New Macro
After the existing `repeated_message_str!` macro definition (around line 272), add:

```rust
/// Macro to format repeated nested messages with two enum method accessors as "ENUM1:ENUM2" pairs
/// Use case: campaign.asset_automation_settings where each item has asset_automation_type() and asset_automation_status() methods
/// Example output: "TEXT_ASSET_AUTOMATION:OPTED_IN, GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT"
macro_rules! repeated_nested_enum_pair_str {
    ([$( $parent:ident ),+], $attr:ident, $method1:ident, $method2:ident) => {
        self.$($parent.as_ref().unwrap().)+$attr
            .iter()
            .map(|item| format!("{:#?}:{:#?}", item.$method1(), item.$method2()))
            .collect::<Vec<String>>()
            .join(", ")
    };
}
```

### 3. Add Match Arm
In the `match field_name` block (around line 274+), add the following arm in the campaign section:

```rust
"campaign.asset_automation_settings" => repeated_nested_enum_pair_str!(
    [campaign],
    asset_automation_settings,
    asset_automation_type,
    asset_automation_status
),
```

**Recommended placement**: After the existing `campaign.primary_status_reasons` line (around line 385), since both are related to campaign-level repeated fields.

## Example GAQL Query
```sql
SELECT
  campaign.id,
  campaign.name,
  campaign.asset_automation_settings
FROM campaign
WHERE campaign.status = 'ENABLED'
```

## Expected Output Examples

### Single Setting
```
campaign.id: 12345
campaign.name: My Campaign
campaign.asset_automation_settings: TEXT_ASSET_AUTOMATION:OPTED_IN
```

### Multiple Settings
```
campaign.id: 12345
campaign.name: My Campaign
campaign.asset_automation_settings: TEXT_ASSET_AUTOMATION:OPTED_IN, GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT, GENERATE_IMAGE_ENHANCEMENT:OPTED_IN
```

### Empty Settings
```
campaign.id: 12345
campaign.name: My Campaign
campaign.asset_automation_settings:
```

## Macro Pattern Documentation

This establishes a new reusable pattern for future fields with similar structure:

**Pattern**: Repeated nested message with multiple enum accessor methods
**Use**: `repeated_nested_enum_pair_str!([parent], field_name, method1, method2)`
**Output**: Comma-separated list of "METHOD1:METHOD2" pairs using debug formatting

### When to Use This Pattern
- Field is `repeated MessageType` (not a oneof)
- MessageType has two or more enum fields accessed via methods (e.g., `field_type()`, `status()`)
- Want to show relationships between the enum values in a compact format
- Each message represents a "setting" or "configuration" pairing

### Similar Fields in the API
This pattern could also be used for:
- `ad_group_ad.ad_group_ad_asset_automation_settings` (similar structure but different message type)
- Any future repeated settings fields that pair enum values

## Test Implementation Plan

### Test File Location
Create new test file: `tests/google_ads_row_asset_automation_settings_tests.rs`

This follows the established pattern of organizing tests by feature/macro type (e.g., `google_ads_row_repeated_tests.rs`, `google_ads_row_primary_status_tests.rs`).

### Test Structure

Following the patterns from existing test files:
1. Use `test_helpers::GoogleAdsRowBuilder` for constructing test data
2. Create helper function to build `AssetAutomationSetting` instances
3. Organize tests into logical sections with clear comments
4. Test both positive cases and edge cases

### Required Test Cases

#### 1. Basic Tests
- **Empty settings** (`vec![]`)
  - Verify returns empty string `""`
- **Single setting - opted in**
  - AssetAutomationType::TEXT_ASSET_AUTOMATION + AssetAutomationStatus::OPTED_IN
  - Expected: `"TextAssetAutomation:OptedIn"`
- **Single setting - opted out**
  - AssetAutomationType::GENERATE_VERTICAL_YOUTUBE_VIDEOS + AssetAutomationStatus::OPTED_OUT
  - Expected: `"GenerateVerticalYoutubeVideos:OptedOut"`

#### 2. Multiple Settings Tests
- **Multiple settings with mixed statuses**
  - Test 3-5 settings with different type/status combinations
  - Verify comma-space separation: `", "`
  - Verify count of parts matches settings count
  - Verify each part has format `"Type:Status"`

#### 3. Realistic Scenario Tests
- **Performance Max campaign**
  - Multiple automation types all opted in
  - Types: TEXT_ASSET_AUTOMATION, GENERATE_ENHANCED_YOUTUBE_VIDEOS, GENERATE_IMAGE_ENHANCEMENT, GENERATE_IMAGE_EXTRACTION, FINAL_URL_EXPANSION_TEXT_ASSET_AUTOMATION
  - All with OPTED_IN status
- **Search campaign**
  - Selective automation, some opted out
  - Types: TEXT_ASSET_AUTOMATION (opted out), FINAL_URL_EXPANSION_TEXT_ASSET_AUTOMATION (opted out)

#### 4. Enum Coverage Tests
- **All AssetAutomationType values**
  - Test all 10 known enum values (2-12, excluding 8)
  - Verify all render correctly
- **UNSPECIFIED and UNKNOWN type values**
  - AssetAutomationType::UNSPECIFIED (0)
  - AssetAutomationType::UNKNOWN (1)
  - Verify they render as `"Unspecified"` and `"Unknown"`
- **UNSPECIFIED and UNKNOWN status values**
  - AssetAutomationStatus::UNSPECIFIED (0)
  - AssetAutomationStatus::UNKNOWN (1)
  - Test with TEXT_ASSET_AUTOMATION type

#### 5. Format Verification Tests
- **Separator consistency**
  - Verify separator is `", "` (comma-space, not just comma)
  - Verify no double commas
  - Verify no leading/trailing commas
- **Colon format**
  - Each part has exactly one colon
  - No extra whitespace around colons
- **No extra whitespace**
  - No leading or trailing whitespace on output
  - No double spaces

#### 6. Integration Tests
- **Combined with other campaign fields**
  - Test `campaign.id`, `campaign.name`, `campaign.status` alongside `campaign.asset_automation_settings`
  - Verify all fields work correctly together

#### 7. Edge Cases
- **Campaign resource absent**
  - Test that accessing `campaign.asset_automation_settings` when campaign is not present causes panic (expected behavior for non-optional macros)
  - Use `std::panic::catch_unwind`

#### 8. Large Scale Tests
- **Many settings (10+)**
  - Test with all available automation types
  - Verify performance and output correctness

### Helper Function Pattern

```rust
fn create_asset_automation_setting(
    automation_type: i32,
    automation_status: i32,
) -> AssetAutomationSetting {
    AssetAutomationSetting {
        asset_automation_type: automation_type,
        asset_automation_status: automation_status,
        ..Default::default()
    }
}
```

### Test Data Construction Pattern

```rust
use googleads_rs::google::ads::googleads::v22::resources::campaign::AssetAutomationSetting;
use googleads_rs::google::ads::googleads::v22::resources::Campaign;
use test_helpers::GoogleAdsRowBuilder;

let campaign = Campaign {
    asset_automation_settings: vec![
        create_asset_automation_setting(2, 2), // TEXT_ASSET_AUTOMATION:OPTED_IN
        create_asset_automation_setting(3, 3), // GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT
    ],
    ..Default::default()
};

let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
let result = row.get("campaign.asset_automation_settings");

assert_eq!(result, "TextAssetAutomation:OptedIn, GenerateVerticalYoutubeVideos:OptedOut");
```

### Test Organization

Organize tests into sections with clear headers:
1. Helper Functions
2. Basic Tests - Empty and Single Setting
3. Multiple Settings Tests
4. Enum Value Tests - All AssetAutomationType Values
5. Edge Cases - UNSPECIFIED and UNKNOWN
6. Mixed Status Tests
7. Large Scale Tests
8. Integration Tests - Verify with Other Campaign Fields
9. Format Verification Tests
10. Negative Tests - Absent Campaign Resource

### Expected Test Count
Approximately 20-25 tests covering all scenarios listed above.

### Running Tests

```bash
# Run all asset automation settings tests
cargo test --test google_ads_row_asset_automation_settings_tests

# Run specific test
cargo test test_campaign_asset_automation_settings_empty

# Run with output
cargo test --test google_ads_row_asset_automation_settings_tests -- --show-output
```

### Success Criteria
- All tests pass
- Coverage includes all enum values
- Edge cases are handled correctly
- Format is consistent with existing patterns
- No panics except for expected cases (absent required parent)

## References

### Proto Files
- [campaign.proto:791](../proto/google/ads/googleads/v22/resources/campaign.proto#L791) - field definition
- [campaign.proto:428-438](../proto/google/ads/googleads/v22/resources/campaign.proto#L428-438) - AssetAutomationSetting message
- [asset_automation_type.proto](../proto/google/ads/googleads/v22/enums/asset_automation_type.proto) - AssetAutomationType enum
- [asset_automation_status.proto](../proto/google/ads/googleads/v22/enums/asset_automation_status.proto) - AssetAutomationStatus enum

### Implementation Files
- [src/lib.rs](../src/lib.rs) - main implementation file
- [specs/how_to_implement_get_matcharms.md](./how_to_implement_get_matcharms.md) - general implementation guide

## Related Work
This implementation follows the established patterns documented in `specs/how_to_implement_get_matcharms.md`, specifically:
- Section 6: Repeated fields - join with ", "
- Macro definitions: lines 71-272
- Match arms block: starting at line 274

The new `repeated_nested_enum_pair_str!` macro extends the existing macro family to handle a new pattern not previously covered by the library.
