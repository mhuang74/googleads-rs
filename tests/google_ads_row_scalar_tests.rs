// Unit tests for GoogleAdsRow::get() method - Scalar Fields
//
// This module tests the attr_str! and optional_attr_str! macros
// which are used for extracting scalar field values (string, int64, double, bool)

#![allow(clippy::excessive_precision)]

mod test_helpers;

use test_helpers::{
    GoogleAdsRowBuilder, CampaignBuilder, AdGroupBuilder, CampaignBudgetBuilder,
    CustomerBuilder, MetricsBuilder, SegmentsBuilder,
};

// ============================================================================
// Campaign Scalar Fields (attr_str!)
// ============================================================================

#[test]
fn test_campaign_id() {
    let campaign = CampaignBuilder::new()
        .id(123456789)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.id"), "123456789");
}

#[test]
fn test_campaign_name() {
    let campaign = CampaignBuilder::new()
        .name("Test Campaign 2024")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.name"), "Test Campaign 2024");
}

#[test]
fn test_campaign_name_with_special_characters() {
    let campaign = CampaignBuilder::new()
        .name("Test Campaign: Q4 2024 - \"Sale\"")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.name"), "Test Campaign: Q4 2024 - \"Sale\"");
}

#[test]
fn test_campaign_end_date() {
    let campaign = CampaignBuilder::new()
        .end_date("20241231")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.end_date"), "20241231");
}

#[test]
fn test_campaign_campaign_budget() {
    let campaign = CampaignBuilder::new()
        .campaign_budget("customers/123/campaignBudgets/456")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.campaign_budget"), "customers/123/campaignBudgets/456");
}

#[test]
fn test_campaign_optimization_score() {
    let campaign = CampaignBuilder::new()
        .optimization_score(0.85)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.optimization_score"), "0.85");
}

// ============================================================================
// AdGroup Scalar Fields (attr_str!)
// ============================================================================

#[test]
fn test_ad_group_id() {
    let ad_group = AdGroupBuilder::new()
        .id(987654321)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.id"), "987654321");
}

#[test]
fn test_ad_group_name() {
    let ad_group = AdGroupBuilder::new()
        .name("Brand Keywords")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.name"), "Brand Keywords");
}

#[test]
fn test_ad_group_cpc_bid_micros() {
    let ad_group = AdGroupBuilder::new()
        .cpc_bid_micros(5000000) // $5.00
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.cpc_bid_micros"), "5000000");
}

#[test]
fn test_ad_group_cpm_bid_micros() {
    let ad_group = AdGroupBuilder::new()
        .cpm_bid_micros(3000000) // $3.00
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.cpm_bid_micros"), "3000000");
}

#[test]
fn test_ad_group_target_cpa_micros() {
    let ad_group = AdGroupBuilder::new()
        .target_cpa_micros(25000000) // $25.00
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.target_cpa_micros"), "25000000");
}

// ============================================================================
// Customer Scalar Fields (attr_str!)
// ============================================================================

#[test]
fn test_customer_id() {
    let customer = CustomerBuilder::new()
        .id(1234567890)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer(customer)
        .build();

    assert_eq!(row.get("customer.id"), "1234567890");
}

#[test]
fn test_customer_descriptive_name() {
    let customer = CustomerBuilder::new()
        .descriptive_name("Acme Corp")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer(customer)
        .build();

    assert_eq!(row.get("customer.descriptive_name"), "Acme Corp");
}

#[test]
fn test_customer_currency_code() {
    let customer = CustomerBuilder::new()
        .currency_code("USD")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer(customer)
        .build();

    assert_eq!(row.get("customer.currency_code"), "USD");
}

#[test]
fn test_customer_time_zone() {
    let customer = CustomerBuilder::new()
        .time_zone("America/New_York")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer(customer)
        .build();

    assert_eq!(row.get("customer.time_zone"), "America/New_York");
}

// ============================================================================
// Optional Scalar Fields (optional_attr_str!)
// ============================================================================

#[test]
fn test_campaign_budget_amount_micros_present() {
    let budget = CampaignBudgetBuilder::new()
        .amount_micros(50000000) // $50.00
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_budget(budget)
        .build();

    assert_eq!(row.get("campaign_budget.amount_micros"), "50000000");
}

#[test]
fn test_campaign_budget_amount_micros_absent() {
    // Create row without campaign_budget
    let row = GoogleAdsRowBuilder::new().build();

    assert_eq!(row.get("campaign_budget.amount_micros"), "");
}

#[test]
fn test_campaign_budget_amount_micros_default_value() {
    let budget = CampaignBudgetBuilder::new()
        // Don't set amount_micros, defaults to 0
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_budget(budget)
        .build();

    // Default i64 value is 0
    assert_eq!(row.get("campaign_budget.amount_micros"), "0");
}

// ============================================================================
// Metrics Scalar Fields (attr_str!)
// ============================================================================

#[test]
fn test_metrics_impressions() {
    let metrics = MetricsBuilder::new()
        .impressions(10000)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.impressions"), "10000");
}

#[test]
fn test_metrics_clicks() {
    let metrics = MetricsBuilder::new()
        .clicks(500)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.clicks"), "500");
}

#[test]
fn test_metrics_ctr() {
    let metrics = MetricsBuilder::new()
        .ctr(0.05)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.ctr"), "0.05");
}

#[test]
fn test_metrics_cost_micros() {
    let metrics = MetricsBuilder::new()
        .cost_micros(1500000) // $1.50
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.cost_micros"), "1500000");
}

#[test]
fn test_metrics_conversions() {
    let metrics = MetricsBuilder::new()
        .conversions(25.5)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.conversions"), "25.5");
}

#[test]
fn test_metrics_conversions_value() {
    let metrics = MetricsBuilder::new()
        .conversions_value(1234.56)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.conversions_value"), "1234.56");
}

#[test]
fn test_metrics_average_cpc() {
    let metrics = MetricsBuilder::new()
        .average_cpc(3.45)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.average_cpc"), "3.45");
}

#[test]
fn test_metrics_average_cpm() {
    let metrics = MetricsBuilder::new()
        .average_cpm(12.75)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.average_cpm"), "12.75");
}

// ============================================================================
// Segments Scalar Fields (attr_str!)
// ============================================================================

#[test]
fn test_segments_date() {
    let segments = SegmentsBuilder::new()
        .date("2024-10-10")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.date"), "2024-10-10");
}

#[test]
fn test_segments_hour() {
    let segments = SegmentsBuilder::new()
        .hour(14)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.hour"), "14");
}

#[test]
fn test_segments_month() {
    let segments = SegmentsBuilder::new()
        .month("2024-10")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.month"), "2024-10");
}

#[test]
fn test_segments_year() {
    let segments = SegmentsBuilder::new()
        .year(2024)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.year"), "2024");
}

// ============================================================================
// Repeated Scalar Fields (direct join)
// ============================================================================

#[test]
fn test_campaign_labels_multiple() {
    let campaign = CampaignBuilder::new()
        .labels(vec!["label1".to_string(), "label2".to_string(), "label3".to_string()])
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.labels"), "label1, label2, label3");
}

#[test]
fn test_campaign_labels_single() {
    let campaign = CampaignBuilder::new()
        .labels(vec!["single_label".to_string()])
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.labels"), "single_label");
}

#[test]
fn test_campaign_labels_empty() {
    let campaign = CampaignBuilder::new()
        .labels(vec![])
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.labels"), "");
}

#[test]
fn test_ad_group_labels_multiple() {
    let ad_group = AdGroupBuilder::new()
        .labels(vec![
            "customers/123/labels/1".to_string(),
            "customers/123/labels/2".to_string(),
        ])
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.labels"), "customers/123/labels/1, customers/123/labels/2");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_zero_values() {
    let campaign = CampaignBuilder::new()
        .id(0)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.id"), "0");
}

#[test]
fn test_negative_values() {
    let ad_group = AdGroupBuilder::new()
        .cpc_bid_micros(-100)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.cpc_bid_micros"), "-100");
}

#[test]
fn test_empty_string_values() {
    let campaign = CampaignBuilder::new()
        .name("")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.name"), "");
}

#[test]
fn test_very_large_numbers() {
    let metrics = MetricsBuilder::new()
        .impressions(9_223_372_036_854_775_807) // i64::MAX
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.impressions"), "9223372036854775807");
}

#[test]
fn test_floating_point_precision() {
    let metrics = MetricsBuilder::new()
        .ctr(0.12345678901234567890)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    // Rust's format! will handle precision
    let result = row.get("metrics.ctr");
    assert!(result.starts_with("0.123456"));
}

// ============================================================================
// Multiple Resources in Same Row
// ============================================================================

#[test]
fn test_multiple_resources_in_row() {
    let campaign = CampaignBuilder::new()
        .id(111)
        .name("Campaign A")
        .build();

    let ad_group = AdGroupBuilder::new()
        .id(222)
        .name("AdGroup B")
        .build();

    let metrics = MetricsBuilder::new()
        .impressions(1000)
        .clicks(50)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .with_ad_group(ad_group)
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("campaign.id"), "111");
    assert_eq!(row.get("campaign.name"), "Campaign A");
    assert_eq!(row.get("ad_group.id"), "222");
    assert_eq!(row.get("ad_group.name"), "AdGroup B");
    assert_eq!(row.get("metrics.impressions"), "1000");
    assert_eq!(row.get("metrics.clicks"), "50");
}
