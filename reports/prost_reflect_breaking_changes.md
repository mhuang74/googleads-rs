# Breaking Changes - Prost-Reflect Migration

## Summary

This document lists all breaking changes introduced by the prost-reflect migration. Changes fall into three categories:
1. **Enum value format changes** - PascalCase → SCREAMING_SNAKE_CASE
2. **Format changes** - Quotes, numeric values → Enum names
3. **Behavioral changes** - Panic → Empty string

## Enum Value Changes (PascalCase → SCREAMING_SNAKE_CASE)

### General Status Values
| Old | New |
|-----|-----|
| `"Enabled"` | `"ENABLED"` |
| `"Paused"` | `"PAUSED"` |
| `"Removed"` | `"REMOVED"` |
| `"Unspecified"` | `"UNSPECIFIED"` |
| `"Unknown"` | `"UNKNOWN"` |

### Advertising Channel Types
| Old | New |
|-----|-----|
| `"Search"` | `"SEARCH"` |
| `"Display"` | `"DISPLAY"` |
| `"Shopping"` | `"SHOPPING"` |
| `"Video"` | `"VIDEO"` |
| `"PerformanceMax"` | `"PERFORMANCE_MAX"` |

### Bidding Strategy Types
| Old | New |
|-----|-----|
| `"ManualCPC"` | `"MANUAL_CPC"` |
| `"MaximizeConversions"` | `"MAXIMIZE_CONVERSIONS"` |
| `"MaximizeConversionValue"` | `"MAXIMIZE_CONVERSION_VALUE"` |
| `"TargetCPA"` | `"TARGET_CPA"` |
| `"TargetROAS"` | `"TARGET_ROAS"` |
| `"TargetImpressionShare"` | `"TARGET_IMPRESSION_SHARE"` |
| `"EnhancedCpc"` (was `"Unsupported"`) | `"ENHANCED_CPC"` |

### Device Types
| Old | New |
|-----|-----|
| `"Mobile"` | `"MOBILE"` |
| `"Desktop"` | `"DESKTOP"` |
| `"Tablet"` | `"TABLET"` |

### Day of Week
| Old | New |
|-----|-----|
| `"Monday"` | `"MONDAY"` |
| `"Tuesday"` | `"TUESDAY"` |
| `"Wednesday"` | `"WEDNESDAY"` |
| `"Thursday"` | `"THURSDAY"` |
| `"Friday"` | `"FRIDAY"` |
| `"Saturday"` | `"SATURDAY"` |
| `"Sunday"` | `"SUNDAY"` |

### Campaign Serving Status
| Old | New |
|-----|-----|
| `"Canceled"` | `"CANCELED"` |
| `"Suspended"` | `"SUSPENDED"` |
| `"Eligible"` | `"ELIGIBLE"` |
| `"Pending"` | `"PENDING"` |

### Campaign Primary Status Reasons
| Old | New |
|-----|-----|
| `"CampaignPaused"` | `"CAMPAIGN_PAUSED"` |
| `"CampaignRemoved"` | `"CAMPAIGN_REMOVED"` |
| `"CampaignPending"` | `"CAMPAIGN_PENDING"` |

### Ad Group Criterion Types
| Old | New |
|-----|-----|
| `"Keyword"` | `"KEYWORD"` |

### Statuses
| Old | New |
|-----|-----|
| `"Approved"` | `"APPROVED"` |
| `"Reviewed"` | `"REVIEWED"` |

### Ad Types and Assets
| Old | New |
|-----|-----|
| `"Headline"` | `"HEADLINE"` |
| `"Headline1"` | `"HEADLINE_1"` |
| `"Description"` | `"DESCRIPTION"` |
| `"Sitelink"` | `"SITELINK"` |
| `"Image"` | `"IMAGE"` |
| `"Webpage"` | `"WEBPAGE"` |

### Other Values
| Old | New |
|-----|-----|
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

### Asset Automation Settings
| Old | New |
|-----|-----|
| `"TextAssetAutomation:OptedIn"` | `"TEXT_ASSET_AUTOMATION:OPTED_IN"` |
| `"GenerateVerticalYoutubeVideos:OptedOut"` | `"GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT"` |
| ... (all asset_automation pairs follow proto-canonical naming) |

## Format Changes

### `change_event.changed_fields`
**Old**: `"'field1, field2'"` (values wrapped in single quotes)
**New**: `"field1, field2"` (no quotes)

### `keyword.match_type` / similar enum fields
**Old**: Numeric string values
- `"2"` → Exact
- `"3"` → Phrase
- `"4"` → Broad

**New**: Proto enum names
- `"EXACT"`
- `"PHRASE"`
- `"BROAD"`

### Debug Format for Messages
**Old**: Prost-generated struct debug format
**New**: `DynamicMessage` debug format

Note: Exact output may differ slightly for complex nested messages.

## Behavioral Changes

### Absent Resources
**Old**: Panics
**New**: Returns empty string `""`

Example:
```rust
// Old behavior - would panic
row.get("campaign.status") // panics if campaign is None

// New behavior - returns empty string
row.get("campaign.status") // returns ""
```

### Repeated Message Fields
Separator behavior is maintained:
- Message items: `"; "` (semicolon-space)
- Scalar items: `", "` (comma-space)

## Migration Guide

### Updating Test Assertions

```rust
// Before
assert_eq!(row.get("campaign.status"), "Enabled");
assert_eq!(row.get("campaign.bidding_strategy_type"), "ManualCPC");
assert_eq!(row.get("ad_group_criterion.keyword.match_type"), "3");
assert_eq!(row.get("change_event.changed_fields"), "'id, status'");

// After
assert_eq!(row.get("campaign.status"), "ENABLED");
assert_eq!(row.get("campaign.bidding_strategy_type"), "MANUAL_CPC");
assert_eq!(row.get("ad_group_criterion.keyword.match_type"), "PHRASE");
assert_eq!(row.get("change_event.changed_fields"), "id, status");
```

### Updating Application Code

If you have code that parses or compares enum values:

```rust
// Old
if row.get("campaign.status") == "Enabled" { ... }

// New
if row.get("campaign.status") == "ENABLED" { ... }
```

### Keyword Match Type Handling

```rust
// Old - if you were mapping numeric values manually
let match_type = row.get("keyword.match_type");
if match_type == "3" { /* phrase */ }

// New - direct enum name comparison
let match_type = row.get("keyword.match_type");
if match_type == "PHRASE" { /* phrase */ }
```

## Testing Checklist

- [ ] Campaign status changes (`ENABLED`, `PAUSED`, `REMOVED`)
- [ ] Campaign bidding strategy types (`MANUAL_CPC`, etc.)
- [ ] Advertising channel types (`SEARCH`, `DISPLAY`, etc.)
- [ ] Device types (`MOBILE`, `DESKTOP`, `TABLET`)
- [ ] Day of week values (`MONDAY`, `TUESDAY`, etc.)
- [ ] Keyword match types (`EXACT`, `PHRASE`, `BROAD`)
- [ ] Change event changed_fields (no quotes)
- [ ] All primary status reasons
- [ ] Asset automation settings format
- [ ] Absent resource behavior (empty string, not panic)
- [ ] Repeated field separators maintained
- [ ] Integration tests with real GAQL queries

## Rollback Plan

If issues arise during migration:

1. Revert `src/lib.rs` to the previous version (commit hash: `f9dad39`)
2. Optionally revert `build.rs`, `Cargo.toml`, and `utils/update.sh`
3. Note: May need to remove `optional` keyword stripping from protos again

However, rollback is not recommended long-term as the old implementation has:
- Limited resource support (~35 of ~178)
- High maintenance burden per API version
- Manual match arms for each field path
