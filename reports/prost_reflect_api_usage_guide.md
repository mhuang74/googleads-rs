# API Usage Guide - GoogleAdsRow Field Access

## Overview

The `GoogleAdsRow::get()` method now uses Protocol Buffer reflection to provide dynamic field access. This document shows how to use the new API.

## Basic Usage

### Single Field Access

```rust
use googleads_rs::google::ads::googleads::v23::services::GoogleAdsRow;

let field_mask = response.field_mask.unwrap();
for row in response.results {
    for path in &field_mask.paths {
        print!("{}: {}\t", path, row.get(&path));
    }
    print!("\n");
}
```

### Multiple Field Access (Optimized)

The new `get_many()` method is more efficient when accessing multiple fields:

```rust
let fields = vec![
    "campaign.id",
    "campaign.name",
    "campaign.status",
    "campaign.bidding_strategy_type"
];

let values = row.get_many(&fields);
// values[0] = campaign.id
// values[1] = campaign.name
// values[2] = campaign.status
// values[3] = campaign.bidding_strategy_type
```

## Supported Field Paths

### Direct Fields
```rust
row.get("customer.id")
row.get("customer.descriptive_name")
row.get("customer.currency_code")
```

### Nested Fields
```rust
row.get("campaign.id")
row.get("campaign.name")
row.get("campaign.status")
row.get("campaign.start_date")
row.get("campaign.end_date")
row.get("campaign.optimization_score")
```

### Deep Nesting
```rust
row.get("campaign_budget.id")
row.get("campaign_budget.amount_micros")
row.get("campaignBudget.status")
```

### Segment Fields
```rust
row.get("segments.date")
row.get("segments.ad_network_type")
row.get("segments.click_type")
row.get("segments.device")
row.get("segments.keyword.info.match_type")
```

### Metrics Fields
```rust
row.get("metrics.impressions")
row.get("metrics.clicks")
row.get("metrics.cost_micros")
row.get("metrics.conversions")
row.get("metrics.all_conversions")
row.get("metrics.conversions_value")
```

### Enum Fields (Now Proto-Canonical Names)
```rust
// Status fields return SCREAMING_SNAKE_CASE
row.get("campaign.status")           // "ENABLED", "PAUSED", "REMOVED"
row.get("ad_group.status")           // "ENABLED", "PAUSED", "REMOVED"
row.get("customer.status")           // "ENABLED", "PAUSED", "CLOSED"

// Type fields
row.get("campaign.advertising_channel_type")  // "SEARCH", "DISPLAY", "VIDEO", "PERFORMANCE_MAX"
row.get("campaign.bidding_strategy_type")    // "MANUAL_CPC", "MAXIMIZE_CONVERSIONS", "TARGET_ROAS"
row.get("ad_group_criterion.keyword.match_type") // "EXACT", "PHRASE", "BROAD"

// Device
row.get("segments.device")            // "MOBILE", "DESKTOP", "TABLET"

// Day of week
row.get("segments.day_of_week")       // "MONDAY", "TUESDAY", etc.
```

### Repeated Fields
```rust
// Lists are joined with appropriate separators
row.get("campaign.primary_status_reasons")       // "REASON_1, REASON_2, REASON_3"
row.get("ad_group_asset.primary_status_details")  // "Detail1; Detail2; Detail3"

// Asset automation settings (special format)
row.get("campaign.asset_automation_settings")
// Returns: "TEXT_ASSET_AUTOMATION:OPTED_IN, GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT"
```

### Oneof Fields
```rust
// Access oneof variant fields
row.get("ad_group_criterion.keyword.text")
row.get("ad_group_criterion.keyword.match_type")
row.get("campaign_criterion.location.geo_target_constant")
row.get("campaign_criterion.negative_keyword.text")
```

### Special Cases

#### Asset Automation Settings
```rust
// Special formatting: "TYPE:STATUS" pairs
row.get("campaign.asset_automation_settings")
// Example output:
// "TEXT_ASSET_AUTOMATION:OPTED_IN, GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED OUT"
```

#### Responsive Search Ad Content
```rust
// GAQL stops at the repeated message, but .text is extracted automatically
row.get("ad_group_ad.ad.responsive_search_ad.headlines")
// Returns comma-separated text values
row.get("ad_group_ad.ad.responsive_search_ad.descriptions")
// Returns comma-separated text values
```

## Return Value Behavior

### Present Fields
```rust
row.get("campaign.name")            // "Summer Sale 2024"
row.get("metrics.clicks")           // "1234"
row.get("campaign.status")          // "ENABLED"
```

### Absent Resources
```rust
// Returns empty string instead of panicking
row.get("campaign.id")              // "" (if campaign not present)
row.get("ad_group.name")            // "" (if ad_group not present)
row.get("metrics.impressions")      // "" (if metrics not selected)
```

### Unknown Fields
```rust
row.get("nonexistent.field")        // "not implemented by googleads-rs"
```

## Enum Values Reference

### Campaign Status
- `UNSPECIFIED`
- `UNKNOWN`
- `ENABLED`
- `PAUSED`
- `REMOVED`

### Advertising Channel Type
- `UNSPECIFIED`
- `UNKNOWN`
- `SEARCH`
- `DISPLAY`
- `SHOPPING`
- `HOTEL`
- `VIDEO`
- `MULTI_CHANNEL`
- `LOCAL`
- `SMART`
- `PERFORMANCE_MAX`
- `TRAVEL`
- `APP`
- `LOCAL_SERVICES`

### Bidding Strategy Type
- `UNSPECIFIED`
- `UNKNOWN`
- `COMMISSION`
- `MANUAL_CPC`
- `MANUAL_CPM`
- `MANUAL_CPV`
- `MAXIMIZE_CONVERSIONS`
- `MAXIMIZE_CONVERSION_VALUE`
- `MAXIMIZE_CLICKS`
- `MAXIMIZE_IMPRESSION_SHARE`
- `MAXIMIZE_TARGET_CPA`
- `TARGET_CPA`
- `TARGET_CPM`
- `TARGET_RETURN_ON_AD_SPEND`
- `TARGET_ROAS`
- `TARGET_IMPRESSION_SHARE`
- `TARGET_SPEND`
- `ENHANCED_CPC`
- `MANUAL_MERCHANT_CENTER_INCENTIVES`

### Keyword Match Type
- `UNSPECIFIED`
- `UNKNOWN`
- `EXACT`
- `PHRASE`
- `BROAD`

### Device
- `UNSPECIFIED`
- `UNKNOWN`
- `MOBILE`
- `DESKTOP`
- `TABLET`
- `CONNECTED_TV`
- `OTHER`

### Day of Week
- `UNSPECIFIED`
- `UNKNOWN`
- `MONDAY`
- `TUESDAY`
- `WEDNESDAY`
- `THURSDAY`
- `FRIDAY`
- `SATURDAY`
- `SUNDAY`

## Error Handling

### Graceful Degradation
The API is designed to never panic in production use:
- Unknown fields return "not implemented by googleads-rs"
- Absent resources return empty string
- Corrupted data returns some representation

### Error Codes
If you need error codes (optional future enhancement):
```rust
let result = row.get_opt("campaign.status");
match result {
    Ok(value) => println!("Value: {}", value),
    Err(e) => match e {
        GetError::FieldNotFound => { /* handle */ },
        GetError::ResourceAbsent => { /* handle */ },
    }
}
```

## Performance Tips

1. **Use `get_many()` for multiple fields**
   - Encodes/decodes once instead of multiple times
   - Significant performance improvement for 3+ fields

2. **Cache repeated lookups**
   - If you need the same field multiple times, store it
   - Avoid redundant reflection work

3. **Batch by resource**
   - Group `get()` calls by the same parent resource
   - Reduces path traversal overhead

## Integration Examples

### GAQL Query Processing
```rust
// Execute GAQL query
let response = client.search_google_ads_stream(&request);

// Process each row
for row in response.results {
    let field_mask = response.field_mask.unwrap();

    for path in &field_mask.paths {
        let value = row.get(&path);
        println!("{} = {}", path, value);
    }
}
```

### Report Generation
```rust
// Build report with multiple fields
let fields = vec![
    "campaign.id",
    "campaign.name",
    "campaign.status",
    "campaign.bidding_strategy_type",
    "metrics.impressions",
    "metrics.clicks",
    "metrics.conversions",
    "segments.date"
];

writeln!(writer, "Campaign ID,Name,Status,Bidding,Impressions,Clicks,Conversions,Date")?;

for row in response.results {
    let values = row.get_many(&fields);
    writeln!(
        writer,
        "{},{},{},{},{},{},{},{}",
        values[0], values[1], values[2], values[3],
        values[4], values[5], values[6], values[7]
    )?;
}
```

### Filtering and Aggregation
```rust
let mut campaign_stats: HashMap<String, (i64, i64)> = HashMap::new();

for row in response.results {
    let status = row.get("campaign.status");
    let impressions = row.get("metrics.impressions").parse::<i64>().unwrap_or(0);
    let clicks = row.get("metrics.clicks").parse::<i64>().unwrap_or(0);

    if status == "ENABLED" {
        campaign_stats
            .entry(status)
            .or_insert((0, 0))
            .0 += impressions;
        let (_, click_ref) = campaign_stats.get_mut(&status).unwrap();
        *click_ref += clicks;
    }
}
```

## Troubleshooting

### Field Not Found
```
value = "not implemented by googleads-rs"
```
- Check spelling of field path
- Verify the field exists in the current API version
- Ensure the resource is selected in your GAQL query

### Empty String Returned
```
value = ""
```
- The resource may not be present in this row
- Check your GAQL query includes the resource
- The field may have presence but is unset

### Unexpected Format
```
value = "Some(DebugFormat(...))"
```
- You're accessing a message field directly
- For lists, check if you need a specific sub-field
- For enums, verify proto-canonical naming

## Resources

- [Prost-Reflect Documentation](https://docs.rs/prost-reflect/0.16.0/prost_reflect/)
- [Google Ads API v23 Field Reference](https://developers.google.com/google-ads/api/fields/v23)
- [GAQL Query Language Guide](https://developers.google.com/google-ads/api/fields/query/overview)
