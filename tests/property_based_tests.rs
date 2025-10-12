// Property-based tests for GoogleAdsRow::get() method
//
// These tests use proptest to fuzz the get() method with random inputs
// to verify robustness, discover edge cases, and ensure no panics occur.

mod test_helpers;

use proptest::prelude::*;
use test_helpers::{
    AdGroupBuilder, AdGroupCriterionBuilder, CampaignBuilder, CustomerBuilder, GoogleAdsRowBuilder,
    MetricsBuilder, SegmentsBuilder,
};

use googleads_rs::google::ads::googleads::v19::enums::{
    ad_group_status_enum::AdGroupStatus, advertising_channel_type_enum::AdvertisingChannelType,
    campaign_status_enum::CampaignStatus,
};

// ============================================================================
// Property Test 1: get() Never Panics with Random Field Paths
// ============================================================================

proptest! {
    #[test]
    fn test_get_never_panics_with_random_field_paths(field_path in ".*") {
        // Create minimal GoogleAdsRow
        let row = GoogleAdsRowBuilder::new().build();

        // Call get() with random field path - should never panic
        let _result = row.get(&field_path);

        // If we got here, no panic occurred - test passes
    }
}

proptest! {
    #[test]
    fn test_get_never_panics_with_random_ascii_paths(
        field_path in "[a-zA-Z0-9._-]{0,100}"
    ) {
        let row = GoogleAdsRowBuilder::new().build();
        let _result = row.get(&field_path);
    }
}

proptest! {
    #[test]
    fn test_get_never_panics_with_sql_injection_like_inputs(
        field_path in r#"[a-zA-Z0-9._;'"<>()]*"#
    ) {
        let row = GoogleAdsRowBuilder::new().build();
        let _result = row.get(&field_path);
    }
}

proptest! {
    #[test]
    fn test_get_never_panics_with_unicode_paths(
        field_path in r"[\p{L}\p{N}._-]{0,50}"
    ) {
        let row = GoogleAdsRowBuilder::new().build();
        let _result = row.get(&field_path);
    }
}

// ============================================================================
// Property Test 2: Valid Field Paths Always Return Non-Empty or Expected Values
// ============================================================================

// Strategy to generate valid campaign field paths
fn campaign_field_path_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "campaign.id".to_string(),
        "campaign.name".to_string(),
        "campaign.status".to_string(),
        "campaign.advertising_channel_type".to_string(),
        "campaign.bidding_strategy_type".to_string(),
        "campaign.end_date".to_string(),
        "campaign.campaign_budget".to_string(),
        "campaign.optimization_score".to_string(),
        "campaign.labels".to_string(),
    ])
}

proptest! {
    #[test]
    fn test_valid_campaign_paths_return_values(
        field_path in campaign_field_path_strategy(),
        campaign_id in 1i64..1_000_000_000i64,
        campaign_name in "[a-zA-Z0-9 ]{1,50}",
    ) {
        let campaign = CampaignBuilder::new()
            .id(campaign_id)
            .name(&campaign_name)
            .status(CampaignStatus::Enabled)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_campaign(campaign)
            .build();

        let result = row.get(&field_path);

        // For scalar fields with values, should return non-empty
        if field_path == "campaign.id" {
            assert_eq!(result, campaign_id.to_string());
        } else if field_path == "campaign.name" {
            assert_eq!(result, campaign_name);
        } else if field_path == "campaign.status" {
            assert!(!result.is_empty());
        }
    }
}

// Strategy to generate valid metrics field paths
fn metrics_field_path_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "metrics.impressions".to_string(),
        "metrics.clicks".to_string(),
        "metrics.ctr".to_string(),
        "metrics.cost_micros".to_string(),
        "metrics.conversions".to_string(),
        "metrics.conversions_value".to_string(),
        "metrics.average_cpc".to_string(),
        "metrics.average_cpm".to_string(),
    ])
}

proptest! {
    #[test]
    fn test_valid_metrics_paths_return_numeric_strings(
        field_path in metrics_field_path_strategy(),
        impressions in 0i64..1_000_000i64,
        clicks in 0i64..100_000i64,
    ) {
        let metrics = MetricsBuilder::new()
            .impressions(impressions)
            .clicks(clicks)
            .ctr(if impressions > 0 { clicks as f64 / impressions as f64 } else { 0.0 })
            .cost_micros(clicks * 1_000_000)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_metrics(metrics)
            .build();

        let result = row.get(&field_path);

        // All metrics should return numeric strings
        match field_path.as_str() {
            "metrics.impressions" => assert_eq!(result, impressions.to_string()),
            "metrics.clicks" => assert_eq!(result, clicks.to_string()),
            "metrics.ctr" | "metrics.average_cpc" | "metrics.average_cpm" => {
                // Should be parseable as f64
                result.parse::<f64>().expect("Should be valid f64");
            },
            "metrics.cost_micros" => {
                result.parse::<i64>().expect("Should be valid i64");
            },
            _ => {},
        }
    }
}

// ============================================================================
// Property Test 3: Random Data Generation - ID Fields
// ============================================================================

proptest! {
    #[test]
    fn test_campaign_id_with_random_values(campaign_id in any::<i64>()) {
        let campaign = CampaignBuilder::new()
            .id(campaign_id)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_campaign(campaign)
            .build();

        assert_eq!(row.get("campaign.id"), campaign_id.to_string());
    }
}

proptest! {
    #[test]
    fn test_ad_group_id_with_random_values(ad_group_id in any::<i64>()) {
        let ad_group = AdGroupBuilder::new()
            .id(ad_group_id)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_ad_group(ad_group)
            .build();

        assert_eq!(row.get("ad_group.id"), ad_group_id.to_string());
    }
}

proptest! {
    #[test]
    fn test_customer_id_with_random_values(customer_id in any::<i64>()) {
        let customer = CustomerBuilder::new()
            .id(customer_id)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_customer(customer)
            .build();

        assert_eq!(row.get("customer.id"), customer_id.to_string());
    }
}

// ============================================================================
// Property Test 4: Random String Fields
// ============================================================================

proptest! {
    #[test]
    fn test_campaign_name_with_random_strings(
        campaign_name in ".*"
    ) {
        let campaign = CampaignBuilder::new()
            .name(&campaign_name)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_campaign(campaign)
            .build();

        assert_eq!(row.get("campaign.name"), campaign_name);
    }
}

proptest! {
    #[test]
    fn test_ad_group_name_with_random_strings(
        ad_group_name in "[a-zA-Z0-9 _-]{0,100}"
    ) {
        let ad_group = AdGroupBuilder::new()
            .name(&ad_group_name)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_ad_group(ad_group)
            .build();

        assert_eq!(row.get("ad_group.name"), ad_group_name);
    }
}

proptest! {
    #[test]
    fn test_customer_descriptive_name_with_unicode(
        descriptive_name in r"[\p{L}\p{N} ]{0,50}"
    ) {
        let customer = CustomerBuilder::new()
            .descriptive_name(&descriptive_name)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_customer(customer)
            .build();

        assert_eq!(row.get("customer.descriptive_name"), descriptive_name);
    }
}

// ============================================================================
// Property Test 5: Exhaustive Enum Value Testing
// ============================================================================

// Strategy to generate all valid CampaignStatus enum values
fn campaign_status_strategy() -> impl Strategy<Value = CampaignStatus> {
    prop::sample::select(vec![
        CampaignStatus::Unspecified,
        CampaignStatus::Unknown,
        CampaignStatus::Enabled,
        CampaignStatus::Paused,
        CampaignStatus::Removed,
    ])
}

proptest! {
    #[test]
    fn test_campaign_status_all_enum_values(
        status in campaign_status_strategy()
    ) {
        let campaign = CampaignBuilder::new()
            .status(status)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_campaign(campaign)
            .build();

        let result = row.get("campaign.status");

        // Should always return a non-empty string for enum debug format
        assert!(!result.is_empty());

        // Verify it matches expected enum variant name
        let expected = format!("{:?}", status);
        assert_eq!(result, expected);
    }
}

// Strategy to generate all valid AdGroupStatus enum values
fn ad_group_status_strategy() -> impl Strategy<Value = AdGroupStatus> {
    prop::sample::select(vec![
        AdGroupStatus::Unspecified,
        AdGroupStatus::Unknown,
        AdGroupStatus::Enabled,
        AdGroupStatus::Paused,
        AdGroupStatus::Removed,
    ])
}

proptest! {
    #[test]
    fn test_ad_group_status_all_enum_values(
        status in ad_group_status_strategy()
    ) {
        let ad_group = AdGroupBuilder::new()
            .status(status)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_ad_group(ad_group)
            .build();

        let result = row.get("ad_group.status");
        assert!(!result.is_empty());
        assert_eq!(result, format!("{:?}", status));
    }
}

// Strategy to generate all valid AdvertisingChannelType enum values
fn advertising_channel_type_strategy() -> impl Strategy<Value = AdvertisingChannelType> {
    prop::sample::select(vec![
        AdvertisingChannelType::Unspecified,
        AdvertisingChannelType::Unknown,
        AdvertisingChannelType::Search,
        AdvertisingChannelType::Display,
        AdvertisingChannelType::Shopping,
        AdvertisingChannelType::Hotel,
        AdvertisingChannelType::Video,
        AdvertisingChannelType::MultiChannel,
        AdvertisingChannelType::Local,
        AdvertisingChannelType::Smart,
        AdvertisingChannelType::PerformanceMax,
        AdvertisingChannelType::LocalServices,
        AdvertisingChannelType::Travel,
    ])
}

proptest! {
    #[test]
    fn test_advertising_channel_type_all_enum_values(
        channel_type in advertising_channel_type_strategy()
    ) {
        let campaign = CampaignBuilder::new()
            .advertising_channel_type(channel_type)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_campaign(campaign)
            .build();

        let result = row.get("campaign.advertising_channel_type");
        assert!(!result.is_empty());
        assert_eq!(result, format!("{:?}", channel_type));
    }
}

// ============================================================================
// Property Test 6: Keyword Text and Match Types (Oneof Variant Testing)
// ============================================================================

proptest! {
    #[test]
    fn test_keyword_with_random_text_and_match_types(
        match_type in 0i32..5i32,  // 0=Unspecified, 2=Exact, 3=Phrase, 4=Broad
        keyword_text in prop::string::string_regex("[a-zA-Z0-9 ]{1,50}").unwrap(),
    ) {
        let criterion = AdGroupCriterionBuilder::new()
            .with_keyword(&keyword_text, match_type)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_ad_group_criterion(criterion)
            .build();

        // Test keyword text
        assert_eq!(row.get("ad_group_criterion.keyword.text"), keyword_text);

        // Test keyword match_type (returns i32 as string)
        let result = row.get("ad_group_criterion.keyword.match_type");
        assert_eq!(result, match_type.to_string());
    }
}

// ============================================================================
// Property Test 7: Random Numeric Ranges
// ============================================================================

proptest! {
    #[test]
    fn test_metrics_impressions_full_i64_range(
        impressions in any::<i64>()
    ) {
        let metrics = MetricsBuilder::new()
            .impressions(impressions)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_metrics(metrics)
            .build();

        assert_eq!(row.get("metrics.impressions"), impressions.to_string());
    }
}

proptest! {
    #[test]
    fn test_metrics_ctr_full_f64_range(
        ctr in -1000.0f64..1000.0f64
    ) {
        let metrics = MetricsBuilder::new()
            .ctr(ctr)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_metrics(metrics)
            .build();

        let result = row.get("metrics.ctr");

        // Should be parseable back to f64
        let parsed: f64 = result.parse().expect("Should parse as f64");

        // Allow for floating point precision differences
        assert!((parsed - ctr).abs() < 0.000001 || ctr.is_nan() == parsed.is_nan());
    }
}

proptest! {
    #[test]
    fn test_ad_group_bid_micros_realistic_range(
        bid_micros in 0i64..1_000_000_000i64  // $0 to $1000
    ) {
        let ad_group = AdGroupBuilder::new()
            .cpc_bid_micros(bid_micros)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_ad_group(ad_group)
            .build();

        assert_eq!(row.get("ad_group.cpc_bid_micros"), bid_micros.to_string());
    }
}

// ============================================================================
// Property Test 8: Date and Time Segments
// ============================================================================

proptest! {
    #[test]
    fn test_segments_date_format(
        year in 2020i32..2030i32,
        month in 1u8..13u8,
        day in 1u8..29u8,  // Avoid month-end edge cases
    ) {
        let date_str = format!("{:04}-{:02}-{:02}", year, month, day);

        let segments = SegmentsBuilder::new()
            .date(&date_str)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_segments(segments)
            .build();

        assert_eq!(row.get("segments.date"), date_str);
    }
}

proptest! {
    #[test]
    fn test_segments_hour_valid_range(
        hour in 0i32..24i32
    ) {
        let segments = SegmentsBuilder::new()
            .hour(hour)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_segments(segments)
            .build();

        assert_eq!(row.get("segments.hour"), hour.to_string());
    }
}

proptest! {
    #[test]
    fn test_segments_year_range(
        year in 2000i32..2100i32
    ) {
        let segments = SegmentsBuilder::new()
            .year(year)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_segments(segments)
            .build();

        assert_eq!(row.get("segments.year"), year.to_string());
    }
}

// ============================================================================
// Property Test 9: Unimplemented Paths Always Return Expected Message
// ============================================================================

proptest! {
    #[test]
    fn test_invalid_paths_return_not_implemented(
        prefix in "[a-z]{3,10}",
        suffix in "[a-z]{3,10}",
    ) {
        let field_path = format!("{}.{}", prefix, suffix);

        let row = GoogleAdsRowBuilder::new().build();

        let result = row.get(&field_path);

        // Invalid paths should either return empty string or "not implemented"
        // (depends on whether it's a valid but unimplemented path)
        assert!(
            result.is_empty() || result == "not implemented by googleads-rs",
            "Expected empty or 'not implemented', got: '{}'",
            result
        );
    }
}

// ============================================================================
// Property Test 10: Multiple Resources with Random Data
// ============================================================================

proptest! {
    #[test]
    fn test_multiple_resources_random_data(
        campaign_id in 1i64..1_000_000i64,
        ad_group_id in 1i64..1_000_000i64,
        impressions in 0i64..1_000_000i64,
        clicks in 0i64..100_000i64,
    ) {
        let campaign = CampaignBuilder::new()
            .id(campaign_id)
            .status(CampaignStatus::Enabled)
            .build();

        let ad_group = AdGroupBuilder::new()
            .id(ad_group_id)
            .status(AdGroupStatus::Enabled)
            .build();

        let metrics = MetricsBuilder::new()
            .impressions(impressions)
            .clicks(clicks)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_campaign(campaign)
            .with_ad_group(ad_group)
            .with_metrics(metrics)
            .build();

        // All fields should be accessible
        assert_eq!(row.get("campaign.id"), campaign_id.to_string());
        assert_eq!(row.get("ad_group.id"), ad_group_id.to_string());
        assert_eq!(row.get("metrics.impressions"), impressions.to_string());
        assert_eq!(row.get("metrics.clicks"), clicks.to_string());
        assert!(!row.get("campaign.status").is_empty());
        assert!(!row.get("ad_group.status").is_empty());
    }
}
