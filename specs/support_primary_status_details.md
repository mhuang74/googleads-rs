# Implementation Plan: Macro for primary_status_details Support

## Overview
Create a new macro called `repeated_message_str!` to handle `primary_status_details` fields. These are **repeated nested message types** (not enums), which is different from `primary_status_reasons`.

## Key Analysis

### Data Structure (from asset_policy.proto)
```proto
message AssetLinkPrimaryStatusDetails {
  AssetLinkPrimaryStatusReason reason = 1;      // enum field
  AssetLinkPrimaryStatus status = 2;             // enum field
  oneof details {
    AssetDisapproved asset_disapproved = 3;      // nested message
  }
}

message AssetDisapproved {
  repeated AssetOfflineEvaluationErrorReasons offline_evaluation_error_reasons = 1;  // repeated enum
}
```

### Generated Rust Type
- Field type: `Vec<AssetLinkPrimaryStatusDetails>`
- Each element is a **struct** (not an enum i32)
- Contains: `reason` (enum), `status` (enum), and optional `details` (oneof with nested message)

## Proposed Macro: `repeated_message_str!`

### Macro Design

```rust
/// Macro to format repeated nested message fields as comma-separated strings
/// Each message is formatted using Debug trait
/// Example:
///     "ad_group_asset.primary_status_details" =>
///       repeated_message_str!([ad_group_asset], primary_status_details),
macro_rules! repeated_message_str {
    ([$( $parent:ident ),+], $attr:ident) => {
        self.$($parent.as_ref().unwrap().)+$attr
            .iter()
            .map(|item| format!("{:#?}", item))
            .collect::<Vec<String>>()
            .join("; ")
    };
}
```

### Key Design Decisions

1. **Uses Debug formatting** (`{:#?}`): Provides readable struct representation with all fields
2. **Semicolon separator** (`;`): Different from comma to distinguish from simple lists
3. **Generic over message type**: Works with any repeated message field, not type-specific
4. **Supports multi-level parents**: Uses same `[$( $parent:ident ),+]` pattern as other macros
5. **No type parameter needed**: Unlike `repeated_enum_str!`, we don't need enum type since we're formatting structs directly

### Why This Approach

- **Simple & maintainable**: Leverages Rust's Debug trait instead of custom formatting
- **Consistent**: Follows the pattern of existing macros in the codebase
- **Future-proof**: Works with any repeated message field without modification
- **Readable output**: Pretty-print format shows all nested details

### Output Format Example
```
reason: Eligible, status: Eligible, details: None; reason: AssetDisapproved, status: NotEligible, details: Some(AssetDisapproved { offline_evaluation_error_reasons: [ImageTooSmall, ImageTooLarge] })
```

## Implementation Steps

### Step 1: Add Macro Definition
**Location**: src/lib.rs, after `repeated_enum_str!` macro (around line 244)

```rust
/// Macro to format repeated nested message fields as comma-separated strings
/// Before:
///     "ad_group_asset.primary_status_details" => {
///         self.ad_group_asset.as_ref().unwrap()
///             .primary_status_details
///             .iter()
///             .map(|item| format!("{:#?}", item))
///             .collect::<Vec<String>>()
///             .join("; ")
///     }
/// With Macro:
///     "ad_group_asset.primary_status_details" => repeated_message_str!([ad_group_asset], primary_status_details),
macro_rules! repeated_message_str {
    ([$( $parent:ident ),+], $attr:ident) => {
        self.$($parent.as_ref().unwrap().)+$attr
            .iter()
            .map(|item| format!("{:#?}", item))
            .collect::<Vec<String>>()
            .join("; ")
    };
}
```

### Step 2: Add Match Arms
**Location**: Add immediately after each `primary_status_reasons` field

#### For ad_group_asset (after line 113):
```rust
"ad_group_asset.primary_status_reasons" => repeated_enum_str!([ad_group_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
"ad_group_asset.primary_status_details" => repeated_message_str!([ad_group_asset], primary_status_details),
```

#### For campaign_asset (after line 193):
```rust
"campaign_asset.primary_status_reasons" => repeated_enum_str!([campaign_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
"campaign_asset.primary_status_details" => repeated_message_str!([campaign_asset], primary_status_details),
```

#### For customer_asset (after line 234):
```rust
"customer_asset.primary_status_reasons" => repeated_enum_str!([customer_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
"customer_asset.primary_status_details" => repeated_message_str!([customer_asset], primary_status_details),
```

#### For asset_group_asset (after line 528):
```rust
"asset_group_asset.primary_status_reasons" => repeated_enum_str!([asset_group_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
"asset_group_asset.primary_status_details" => repeated_message_str!([asset_group_asset], primary_status_details),
```

## Comparison with repeated_enum_str!

### repeated_enum_str! (for enums)
```rust
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
- **Input**: `Vec<i32>` (proto enum values)
- **Type param**: Required (`$enum_type:ty`)
- **Conversion**: `from_i32()` + `unwrap_or_default()`
- **Separator**: Comma (`, `)

### repeated_message_str! (for messages)
```rust
macro_rules! repeated_message_str {
    ([$( $parent:ident ),+], $attr:ident) => {
        self.$($parent.as_ref().unwrap().)+$attr
            .iter()
            .map(|item| format!("{:#?}", item))
            .collect::<Vec<String>>()
            .join("; ")
    };
}
```
- **Input**: `Vec<MessageType>` (proto message structs)
- **Type param**: Not needed (generic)
- **Conversion**: Direct Debug formatting
- **Separator**: Semicolon (`; `)

## Resources Affected

### Asset Link Resources (4 resources)
All resources with `primary_status_details` field:

1. **AdGroupAsset** (proto: ad_group_asset.proto, line 100)
   - Field: `repeated AssetLinkPrimaryStatusDetails primary_status_details`
   - Match arm: `"ad_group_asset.primary_status_details"`

2. **CampaignAsset** (proto: campaign_asset.proto, line 101)
   - Field: `repeated AssetLinkPrimaryStatusDetails primary_status_details`
   - Match arm: `"campaign_asset.primary_status_details"`

3. **CustomerAsset** (proto: customer_asset.proto, line 91)
   - Field: `repeated AssetLinkPrimaryStatusDetails primary_status_details`
   - Match arm: `"customer_asset.primary_status_details"`

4. **AssetGroupAsset** (proto: asset_group_asset.proto, line 93)
   - Field: `repeated AssetLinkPrimaryStatusDetails primary_status_details`
   - Match arm: `"asset_group_asset.primary_status_details"`

## Summary

- **New macro**: `repeated_message_str!` (generic, no type parameters)
- **4 new match arms** for `primary_status_details` fields
- **No additional imports needed** (uses Debug trait from std::fmt)
- **Separator**: Semicolon (`;`) to distinguish from enum lists
- **Compatible with existing patterns**: Follows same multi-level parent support

## Advantages of This Approach

1. **Simpler than enum version**: No type parameter required, works with any message type
2. **Leverages Debug trait**: No manual field extraction or conversion needed
3. **Rich nested detail**: Automatically includes all nested fields and their values
4. **Maintainable**: If proto adds new fields to AssetLinkPrimaryStatusDetails, they appear automatically
5. **Reusable**: Can be used for any future repeated message fields

## Testing Considerations

After implementation:
1. Verify GAQL path spelling matches API documentation (`*.primary_status_details`)
2. Test with GAQL queries that select these fields
3. Ensure empty repeated fields render as empty strings
4. Verify semicolon-separated output format is readable
5. Test with real data that has nested `asset_disapproved` details

## Reference Locations
- Proto definition: proto/google/ads/googleads/v19/common/asset_policy.proto:59-80
- Asset link resources: proto/google/ads/googleads/v19/resources/{ad_group_asset,campaign_asset,customer_asset,asset_group_asset}.proto
- Implementation location: src/lib.rs (macro after line 244, match arms in respective sections)
- Related spec: specs/support_primary_status.md
