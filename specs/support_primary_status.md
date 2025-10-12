# Implementation Plan: primary_status and primary_status_reasons Support

## Overview
This spec describes the implementation plan to add missing match arms for `primary_status` and `primary_status_reasons` fields across Google Ads API v19 resources in GoogleAdsRow::get().

## Current Implementation Status

### Already Implemented (3 resources)
Based on src/lib.rs:272, 347, 387:
1. **ad_group_asset.primary_status** ✅ (line 272)
2. **campaign_asset.primary_status** ✅ (line 347)
3. **customer_asset.primary_status** ✅ (line 387)

Note: None of the `primary_status_reasons` fields are currently implemented.

## Resources Supporting primary_status Fields

Based on proto analysis (proto/google/ads/googleads/v19/resources/*.proto):

### 1. Campaign (proto: campaign.proto)
- **Fields:**
  - `campaign.primary_status` (line 390) - enum CampaignPrimaryStatus
  - `campaign.primary_status_reasons` (line 398) - repeated enum CampaignPrimaryStatusReason
- **Parent Optionality:** Non-optional (campaign is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: Use `method_str!([campaign], primary_status)`
  - `primary_status_reasons`: Requires special handling for repeated enum field

### 2. AdGroup (proto: ad_group.proto)
- **Fields:**
  - `ad_group.primary_status` (line 141) - enum AdGroupPrimaryStatus
  - `ad_group.primary_status_reasons` (line 146) - repeated enum AdGroupPrimaryStatusReason
- **Parent Optionality:** Non-optional (ad_group is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: Use `method_str!([ad_group], primary_status)`
  - `primary_status_reasons`: Requires special handling for repeated enum field

### 3. AdGroupAd (proto: ad_group_ad.proto)
- **Fields:**
  - `ad_group_ad.primary_status` (line 87) - enum AdGroupAdPrimaryStatus
  - `ad_group_ad.primary_status_reasons` (line 92) - repeated enum AdGroupAdPrimaryStatusReason
- **Parent Optionality:** Non-optional (ad_group_ad is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: Use `method_str!([ad_group_ad], primary_status)`
  - `primary_status_reasons`: Requires special handling for repeated enum field

### 4. AdGroupCriterion (proto: ad_group_criterion.proto)
- **Fields:**
  - `ad_group_criterion.primary_status` (line 197) - enum AdGroupCriterionPrimaryStatus
  - `ad_group_criterion.primary_status_reasons` (line 202) - repeated enum AdGroupCriterionPrimaryStatusReason
- **Parent Optionality:** Non-optional (ad_group_criterion is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: Use `method_str!([ad_group_criterion], primary_status)`
  - `primary_status_reasons`: Requires special handling for repeated enum field

### 5. AssetGroup (proto: asset_group.proto)
- **Fields:**
  - `asset_group.primary_status` (line 87) - enum AssetGroupPrimaryStatus
  - `asset_group.primary_status_reasons` (line 94) - repeated enum AssetGroupPrimaryStatusReason
- **Parent Optionality:** Non-optional (asset_group is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: Use `method_str!([asset_group], primary_status)`
  - `primary_status_reasons`: Requires special handling for repeated enum field

### 6. AdGroupAsset (proto: ad_group_asset.proto) ✅
- **Fields:**
  - `ad_group_asset.primary_status` (line 94) - enum AssetLinkPrimaryStatus ✅ ALREADY IMPLEMENTED
  - `ad_group_asset.primary_status_details` (line 100) - repeated AssetLinkPrimaryStatusDetails
  - `ad_group_asset.primary_status_reasons` (line 105) - repeated enum AssetLinkPrimaryStatusReason
- **Parent Optionality:** Non-optional (ad_group_asset is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: ✅ Already implemented (line 272)
  - `primary_status_details`: New field to implement (nested message with repeated)
  - `primary_status_reasons`: Requires special handling for repeated enum field

### 7. CampaignAsset (proto: campaign_asset.proto) ✅
- **Fields:**
  - `campaign_asset.primary_status` (line 95) - enum AssetLinkPrimaryStatus ✅ ALREADY IMPLEMENTED
  - `campaign_asset.primary_status_details` (line 101) - repeated AssetLinkPrimaryStatusDetails
  - `campaign_asset.primary_status_reasons` (line 106) - repeated enum AssetLinkPrimaryStatusReason
- **Parent Optionality:** Non-optional (campaign_asset is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: ✅ Already implemented (line 347)
  - `primary_status_details`: New field to implement (nested message with repeated)
  - `primary_status_reasons`: Requires special handling for repeated enum field

### 8. CustomerAsset (proto: customer_asset.proto) ✅
- **Fields:**
  - `customer_asset.primary_status` (line 85) - enum AssetLinkPrimaryStatus ✅ ALREADY IMPLEMENTED
  - `customer_asset.primary_status_details` (line 91) - repeated AssetLinkPrimaryStatusDetails
  - `customer_asset.primary_status_reasons` (line 96) - repeated enum AssetLinkPrimaryStatusReason
- **Parent Optionality:** Non-optional (customer_asset is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: ✅ Already implemented (line 387)
  - `primary_status_details`: New field to implement (nested message with repeated)
  - `primary_status_reasons`: Requires special handling for repeated enum field

### 9. AssetGroupAsset (proto: asset_group_asset.proto)
- **Fields:**
  - `asset_group_asset.primary_status` (line 87) - enum AssetLinkPrimaryStatus
  - `asset_group_asset.primary_status_details` (line 93) - repeated AssetLinkPrimaryStatusDetails
  - `asset_group_asset.primary_status_reasons` (line 98) - repeated enum AssetLinkPrimaryStatusReason
- **Parent Optionality:** Non-optional (asset_group_asset is guaranteed present)
- **Implementation Strategy:**
  - `primary_status`: Use `method_str!([asset_group_asset], primary_status)`
  - `primary_status_details`: New field to implement (nested message with repeated)
  - `primary_status_reasons`: Requires special handling for repeated enum field

## Implementation Details

### New Macro: repeated_enum_str!

Create a new macro to handle repeated enum fields that converts a Vec<i32> to a comma-separated string of enum debug representations.

**Macro Location:** Add after `optional_enum_match_str!` macro (around src/lib.rs:211)

**Macro Implementation:**
```rust
/// Macro to get repeated enum field as comma-separated debug string
/// Before:
///     "campaign.primary_status_reasons" => {
///         self.campaign.as_ref().unwrap()
///             .primary_status_reasons
///             .iter()
///             .map(|&v| format!("{:#?}",
///                 CampaignPrimaryStatusReason::from_i32(v).unwrap_or_default()))
///             .collect::<Vec<String>>()
///             .join(", ")
///     }
/// With Macro:
///     "campaign.primary_status_reasons" => repeated_enum_str!([campaign], primary_status_reasons, CampaignPrimaryStatusReason),
macro_rules! repeated_enum_str {
    ([$( $parent:ident ),+], $attr:ident, $enum_type:ty) => {
        self.$($parent.as_ref().unwrap().)+$attr
            .iter()
            .map(|&v| format!("{:#?}",
                <$enum_type>::from_i32(v).unwrap_or_default()))
            .collect::<Vec<String>>()
            .join(", ")
    };
}
```

**Key Design Decisions:**
1. **Supports multi-level parent chains**: Uses `[$( $parent:ident ),+]` pattern like existing macros to support nested resources (e.g., `[ad_group_ad, ad]`)
2. **Type parameter for enum**: The macro takes the enum type as a parameter to handle different enum types across resources
3. **Uses from_i32()**: Prost-generated enums use `from_i32()` (not `try_from()`) to convert from i32 values
4. **Fallback to default**: Uses `unwrap_or_default()` to handle invalid enum values gracefully
5. **Debug formatting**: Uses `{:#?}` format to produce human-readable enum variant names (e.g., "Paused" instead of numeric values)
6. **Consistent separator**: Uses `", "` to match existing patterns in the codebase (e.g., labels field)

**Why this approach:**
- Follows the established macro patterns in the codebase
- Type-safe: Requires explicit enum type specification
- Reusable: Can be used for any repeated enum field
- Consistent: Produces output format matching existing repeated field rendering

### Strategy for Repeated Enum Fields (primary_status_reasons)

All `primary_status_reasons` fields are `repeated` enum fields stored as `Vec<i32>` at the proto level. The new `repeated_enum_str!` macro will:

1. Access the repeated field on the parent resource
2. Iterate over the Vec<i32>
3. Convert each i32 to its typed enum using `from_i32()` (prost-generated method)
4. Format each enum value as a debug string
5. Join all values with ", " separator
6. Return empty string if the Vec is empty

### Strategy for primary_status_details Fields

The `primary_status_details` fields on asset link resources (ad_group_asset, campaign_asset, customer_asset, asset_group_asset) are repeated nested messages of type `AssetLinkPrimaryStatusDetails` from `proto/google/ads/googleads/v19/common/asset_policy.proto`.

These are complex nested structures that may require custom rendering logic or could be omitted from the initial implementation scope. Consider implementing these in a follow-up phase after basic primary_status and primary_status_reasons support.

### Macro Evaluation

Based on specs/how_to_implement_get_matcharms.md:
- **primary_status (enum accessor)**: Use `method_str!` macro - These are enum fields with generated accessor methods
- **primary_status_reasons (repeated enum)**: Requires inline implementation or new macro - Not covered by existing macros

## Implementation Guide

### Step 1: Add Enum Imports
Location: src/lib.rs, after line 28

Add the following import block:
```rust
use crate::google::ads::googleads::v19::enums::{
    ad_group_ad_primary_status_reason_enum::AdGroupAdPrimaryStatusReason,
    ad_group_criterion_primary_status_reason_enum::AdGroupCriterionPrimaryStatusReason,
    ad_group_primary_status_reason_enum::AdGroupPrimaryStatusReason,
    asset_group_primary_status_reason_enum::AssetGroupPrimaryStatusReason,
    asset_link_primary_status_reason_enum::AssetLinkPrimaryStatusReason,
    campaign_primary_status_reason_enum::CampaignPrimaryStatusReason,
};
```

### Step 2: Add repeated_enum_str! Macro
Location: src/lib.rs, after the `optional_enum_match_str!` macro (around line 211)

Add the new macro:
```rust
/// Macro to get repeated enum field as comma-separated debug string
/// Before:
///     "campaign.primary_status_reasons" => {
///         self.campaign.as_ref().unwrap()
///             .primary_status_reasons
///             .iter()
///             .map(|&v| format!("{:#?}",
///                 CampaignPrimaryStatusReason::from_i32(v).unwrap_or_default()))
///             .collect::<Vec<String>>()
///             .join(", ")
///     }
/// With Macro:
///     "campaign.primary_status_reasons" => repeated_enum_str!([campaign], primary_status_reasons, CampaignPrimaryStatusReason),
macro_rules! repeated_enum_str {
    ([$( $parent:ident ),+], $attr:ident, $enum_type:ty) => {
        self.$($parent.as_ref().unwrap().)+$attr
            .iter()
            .map(|&v| format!("{:#?}",
                <$enum_type>::from_i32(v).unwrap_or_default()))
            .collect::<Vec<String>>()
            .join(", ")
    };
}
```

### Step 3: Add Match Arms - Phase 1 (primary_status fields)
Location: src/lib.rs, in the match block starting at line 213

Add the following match arms in their appropriate sections:

**For ad_group (around line 231, after "ad_group.type"):**
```rust
"ad_group.type" => method_str!([ad_group], r#type),
"ad_group.primary_status" => method_str!([ad_group], primary_status),
"ad_group.labels" => self.ad_group.as_ref().unwrap().labels.join(", "),
```

**For ad_group_ad (around line 240, after "ad_group_ad.status"):**
```rust
"ad_group_ad.status" => method_str!([ad_group_ad], status),
"ad_group_ad.primary_status" => method_str!([ad_group_ad], primary_status),
"ad_group_ad.labels" => self.ad_group_ad.as_ref().unwrap().labels.join(", "),
```

**For ad_group_criterion (around line 263, after "ad_group_criterion.type"):**
```rust
"ad_group_criterion.type" => method_str!([ad_group_criterion], r#type),
"ad_group_criterion.primary_status" => method_str!([ad_group_criterion], primary_status),
"ad_group_criterion.labels" => self.ad_group_criterion.as_ref().unwrap().labels.join(", "),
```

**For asset_group (around line 289, after "asset_group.ad_strength"):**
```rust
"asset_group.ad_strength" => attr_str!([asset_group], ad_strength),
"asset_group.primary_status" => method_str!([asset_group], primary_status),
"audience.description" => attr_str!([audience], description),
```

**For campaign (around line 312, after the bidding_strategy_type block):**
```rust
},
"campaign.primary_status" => method_str!([campaign], primary_status),
"campaign_criterion.campaign" => optional_attr_str!(campaign_criterion, campaign),
```

**For asset_group_asset (around line 466, after "asset_group_asset.performance_label"):**
```rust
"asset_group_asset.performance_label" => method_str!([asset_group_asset], performance_label),
"asset_group_asset.primary_status" => method_str!([asset_group_asset], primary_status),
// ===== ASSET_GROUP_SIGNAL (Phase 5) =====
```

### Step 4: Add Match Arms - Phase 2 (primary_status_reasons fields)
Location: src/lib.rs, immediately after each corresponding primary_status field

**For ad_group:**
```rust
"ad_group.primary_status" => method_str!([ad_group], primary_status),
"ad_group.primary_status_reasons" => repeated_enum_str!([ad_group], primary_status_reasons, AdGroupPrimaryStatusReason),
```

**For ad_group_ad:**
```rust
"ad_group_ad.primary_status" => method_str!([ad_group_ad], primary_status),
"ad_group_ad.primary_status_reasons" => repeated_enum_str!([ad_group_ad], primary_status_reasons, AdGroupAdPrimaryStatusReason),
```

**For ad_group_criterion:**
```rust
"ad_group_criterion.primary_status" => method_str!([ad_group_criterion], primary_status),
"ad_group_criterion.primary_status_reasons" => repeated_enum_str!([ad_group_criterion], primary_status_reasons, AdGroupCriterionPrimaryStatusReason),
```

**For ad_group_asset (line 272, after existing primary_status):**
```rust
"ad_group_asset.primary_status" => method_str!([ad_group_asset], primary_status),
"ad_group_asset.primary_status_reasons" => repeated_enum_str!([ad_group_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
```

**For asset_group:**
```rust
"asset_group.primary_status" => method_str!([asset_group], primary_status),
"asset_group.primary_status_reasons" => repeated_enum_str!([asset_group], primary_status_reasons, AssetGroupPrimaryStatusReason),
```

**For campaign:**
```rust
"campaign.primary_status" => method_str!([campaign], primary_status),
"campaign.primary_status_reasons" => repeated_enum_str!([campaign], primary_status_reasons, CampaignPrimaryStatusReason),
```

**For campaign_asset (line 347, after existing primary_status):**
```rust
"campaign_asset.primary_status" => method_str!([campaign_asset], primary_status),
"campaign_asset.primary_status_reasons" => repeated_enum_str!([campaign_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
```

**For customer_asset (line 387, after existing primary_status):**
```rust
"customer_asset.primary_status" => method_str!([customer_asset], primary_status),
"customer_asset.primary_status_reasons" => repeated_enum_str!([customer_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
```

**For asset_group_asset:**
```rust
"asset_group_asset.primary_status" => method_str!([asset_group_asset], primary_status),
"asset_group_asset.primary_status_reasons" => repeated_enum_str!([asset_group_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
```

## Implementation Tasks Summary

### Phase 1: Implement Missing primary_status Fields (6 new fields)
1. `"campaign.primary_status"` → `method_str!([campaign], primary_status)`
2. `"ad_group.primary_status"` → `method_str!([ad_group], primary_status)`
3. `"ad_group_ad.primary_status"` → `method_str!([ad_group_ad], primary_status)`
4. `"ad_group_criterion.primary_status"` → `method_str!([ad_group_criterion], primary_status)`
5. `"asset_group.primary_status"` → `method_str!([asset_group], primary_status)`
6. `"asset_group_asset.primary_status"` → `method_str!([asset_group_asset], primary_status)`

### Phase 2: Implement primary_status_reasons Fields (9 fields)
1. `"campaign.primary_status_reasons"` → `repeated_enum_str!([campaign], primary_status_reasons, CampaignPrimaryStatusReason)`
2. `"ad_group.primary_status_reasons"` → `repeated_enum_str!([ad_group], primary_status_reasons, AdGroupPrimaryStatusReason)`
3. `"ad_group_ad.primary_status_reasons"` → `repeated_enum_str!([ad_group_ad], primary_status_reasons, AdGroupAdPrimaryStatusReason)`
4. `"ad_group_criterion.primary_status_reasons"` → `repeated_enum_str!([ad_group_criterion], primary_status_reasons, AdGroupCriterionPrimaryStatusReason)`
5. `"asset_group.primary_status_reasons"` → `repeated_enum_str!([asset_group], primary_status_reasons, AssetGroupPrimaryStatusReason)`
6. `"ad_group_asset.primary_status_reasons"` → `repeated_enum_str!([ad_group_asset], primary_status_reasons, AssetLinkPrimaryStatusReason)`
7. `"campaign_asset.primary_status_reasons"` → `repeated_enum_str!([campaign_asset], primary_status_reasons, AssetLinkPrimaryStatusReason)`
8. `"customer_asset.primary_status_reasons"` → `repeated_enum_str!([customer_asset], primary_status_reasons, AssetLinkPrimaryStatusReason)`
9. `"asset_group_asset.primary_status_reasons"` → `repeated_enum_str!([asset_group_asset], primary_status_reasons, AssetLinkPrimaryStatusReason)`

### Phase 3 (Optional): Implement primary_status_details Fields
Consider implementing for completeness (4 resources):

1. `"ad_group_asset.primary_status_details"`
2. `"campaign_asset.primary_status_details"`
3. `"customer_asset.primary_status_details"`
4. `"asset_group_asset.primary_status_details"`

These are repeated nested messages and may require special formatting logic.

## Required Enum Type Imports

Add to the use statements in src/lib.rs (after line 28, before line 30):
```rust
use crate::google::ads::googleads::v19::enums::{
    ad_group_ad_primary_status_reason_enum::AdGroupAdPrimaryStatusReason,
    ad_group_criterion_primary_status_reason_enum::AdGroupCriterionPrimaryStatusReason,
    ad_group_primary_status_reason_enum::AdGroupPrimaryStatusReason,
    asset_group_primary_status_reason_enum::AssetGroupPrimaryStatusReason,
    asset_link_primary_status_reason_enum::AssetLinkPrimaryStatusReason,
    campaign_primary_status_reason_enum::CampaignPrimaryStatusReason,
};
```

**Note:** These imports are sorted alphabetically to maintain consistency with the existing codebase style.

## Testing Considerations

After implementation:
1. Verify GAQL path spelling matches API documentation
2. Verify enum accessor methods exist on the generated structs
3. Test with GAQL queries that select these fields
4. Ensure empty repeated fields render as empty strings
5. Verify comma-separated output format for repeated enums matches user expectations

## Summary

**Total Fields to Implement:**
- 6 new `primary_status` fields (campaign, ad_group, ad_group_ad, ad_group_criterion, asset_group, asset_group_asset)
- 9 `primary_status_reasons` fields (all resources)
- 4 `primary_status_details` fields (optional, asset link resources only)

**Total: 19 new match arms** (15 required + 4 optional)

## Reference Locations
- Entry point: [impl GoogleAdsRow::get()](src/lib.rs:65)
- Match arms block: [match field_name](src/lib.rs:215)
- Existing primary_status implementations: src/lib.rs:272, 347, 387
- How-to spec: specs/how_to_implement_get_matcharms.md
- Proto files: proto/google/ads/googleads/v19/resources/*.proto
