// Integration tests for GoogleAdsRow streaming and field mask functionality
//
// This module tests the integration between gRPC streaming responses,
// field masks, and the GoogleAdsRow::get() method.

#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::useless_vec)]

use googleads_rs::google::ads::googleads::v19::services::{
    GoogleAdsRow, SearchGoogleAdsStreamResponse,
};
use googleads_rs::google::ads::googleads::v19::resources::{Campaign, AdGroup, Customer};
use googleads_rs::google::ads::googleads::v19::common::{Metrics, Segments};
use googleads_rs::google::ads::googleads::v19::enums::{
    campaign_status_enum::CampaignStatus,
    ad_group_status_enum::AdGroupStatus,
};
use prost_types::FieldMask;

// ============================================================================
// Field Mask Integration Tests
// ============================================================================

#[test]
fn test_field_mask_with_campaign_fields() {
    // Create a field mask requesting specific campaign fields
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
            "campaign.status".to_string(),
        ],
    };

    // Create a row with campaign data
    let mut campaign = Campaign::default();
    campaign.id = 12345;
    campaign.name = "Test Campaign".to_string();
    campaign.status = CampaignStatus::Enabled as i32;

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ..Default::default()
    };

    // Verify all field mask paths return correct values
    for path in &field_mask.paths {
        let value = row.get(path);
        assert!(!value.is_empty(), "Field {} should return a value", path);
        assert_ne!(value, "not implemented by googleads-rs", "Field {} should be implemented", path);
    }

    // Verify specific values
    assert_eq!(row.get("campaign.id"), "12345");
    assert_eq!(row.get("campaign.name"), "Test Campaign");
    assert_eq!(row.get("campaign.status"), "Enabled");
}

#[test]
fn test_field_mask_with_metrics_and_segments() {
    // Create a field mask requesting metrics and segments
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "metrics.impressions".to_string(),
            "metrics.clicks".to_string(),
            "metrics.ctr".to_string(),
            "segments.date".to_string(),
        ],
    };

    // Create a row with campaign, metrics, and segments
    let mut campaign = Campaign::default();
    campaign.id = 99999;

    let mut metrics = Metrics::default();
    metrics.impressions = 10000;
    metrics.clicks = 500;
    metrics.ctr = 0.05;

    let mut segments = Segments::default();
    segments.date = "2024-10-10".to_string();

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        metrics: Some(metrics),
        segments: Some(segments),
        ..Default::default()
    };

    // Verify all paths in field mask work
    for path in &field_mask.paths {
        let value = row.get(path);
        assert!(!value.is_empty(), "Field {} should return a value", path);
    }

    assert_eq!(row.get("campaign.id"), "99999");
    assert_eq!(row.get("metrics.impressions"), "10000");
    assert_eq!(row.get("metrics.clicks"), "500");
    assert_eq!(row.get("metrics.ctr"), "0.05");
    assert_eq!(row.get("segments.date"), "2024-10-10");
}

#[test]
fn test_field_mask_with_nested_fields() {
    // Test field mask with nested message fields
    let _field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.network_settings.target_search_network".to_string(),
            "campaign.network_settings.target_content_network".to_string(),
        ],
    };

    // Create campaign with network settings
    let mut campaign = Campaign::default();
    campaign.id = 55555;
    campaign.network_settings = Some(
        googleads_rs::google::ads::googleads::v19::resources::campaign::NetworkSettings {
            target_search_network: true,
            target_content_network: false,
            target_partner_search_network: false,
            target_google_search: true,
            ..Default::default()
        },
    );

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ..Default::default()
    };

    // Verify nested paths work
    assert_eq!(row.get("campaign.id"), "55555");
    assert_eq!(row.get("campaign.network_settings.target_search_network"), "true");
    assert_eq!(row.get("campaign.network_settings.target_content_network"), "false");
}

#[test]
fn test_field_mask_all_paths_accessible() {
    // Test that all paths in field mask are accessible
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
            "ad_group.id".to_string(),
            "ad_group.name".to_string(),
            "customer.id".to_string(),
        ],
    };

    let mut campaign = Campaign::default();
    campaign.id = 111;
    campaign.name = "Campaign 1".to_string();

    let mut ad_group = AdGroup::default();
    ad_group.id = 222;
    ad_group.name = "Ad Group 1".to_string();

    let mut customer = Customer::default();
    customer.id = 333;

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ad_group: Some(ad_group),
        customer: Some(customer),
        ..Default::default()
    };

    // Count how many paths return valid values
    let mut valid_count = 0;
    for path in &field_mask.paths {
        let value = row.get(path);
        if !value.is_empty() && value != "not implemented by googleads-rs" {
            valid_count += 1;
        }
    }

    // All 5 paths should return valid values
    assert_eq!(valid_count, 5, "All field mask paths should return values");
}

// ============================================================================
// Streaming Response Tests
// ============================================================================

#[test]
fn test_streaming_response_single_batch() {
    // Simulate a single batch streaming response
    let mut campaign = Campaign::default();
    campaign.id = 11111;
    campaign.name = "Campaign A".to_string();
    campaign.status = CampaignStatus::Enabled as i32;

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ..Default::default()
    };

    let response = SearchGoogleAdsStreamResponse {
        results: vec![row],
        field_mask: Some(FieldMask {
            paths: vec![
                "campaign.id".to_string(),
                "campaign.name".to_string(),
                "campaign.status".to_string(),
            ],
        }),
        summary_row: None,
        request_id: "test-request-123".to_string(),
        query_resource_consumption: 0,
    };

    // Process the response
    assert_eq!(response.results.len(), 1);
    assert!(response.field_mask.is_some());

    let row = &response.results[0];
    assert_eq!(row.get("campaign.id"), "11111");
    assert_eq!(row.get("campaign.name"), "Campaign A");
    assert_eq!(row.get("campaign.status"), "Enabled");
}

#[test]
fn test_streaming_response_multiple_rows() {
    // Simulate a streaming response with multiple rows
    let mut rows = Vec::new();

    for i in 1..=5 {
        let mut campaign = Campaign::default();
        campaign.id = i * 1000;
        campaign.name = format!("Campaign {}", i);
        campaign.status = if i % 2 == 0 {
            CampaignStatus::Paused as i32
        } else {
            CampaignStatus::Enabled as i32
        };

        rows.push(GoogleAdsRow {
            campaign: Some(campaign),
            ..Default::default()
        });
    }

    let response = SearchGoogleAdsStreamResponse {
        results: rows,
        field_mask: Some(FieldMask {
            paths: vec![
                "campaign.id".to_string(),
                "campaign.name".to_string(),
                "campaign.status".to_string(),
            ],
        }),
        summary_row: None,
        request_id: "test-request-456".to_string(),
        query_resource_consumption: 0,
    };

    // Verify we got 5 rows
    assert_eq!(response.results.len(), 5);

    // Verify each row
    for (idx, row) in response.results.iter().enumerate() {
        let i = idx + 1;
        assert_eq!(row.get("campaign.id"), format!("{}", i * 1000));
        assert_eq!(row.get("campaign.name"), format!("Campaign {}", i));

        let expected_status = if i % 2 == 0 { "Paused" } else { "Enabled" };
        assert_eq!(row.get("campaign.status"), expected_status);
    }
}

#[test]
fn test_streaming_response_with_metrics() {
    // Test streaming response that includes metrics
    let mut campaign = Campaign::default();
    campaign.id = 77777;
    campaign.name = "Performance Campaign".to_string();

    let mut metrics = Metrics::default();
    metrics.impressions = 50000;
    metrics.clicks = 2500;
    metrics.ctr = 0.05;
    metrics.cost_micros = 125000000; // $125 in micros

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        metrics: Some(metrics),
        ..Default::default()
    };

    let response = SearchGoogleAdsStreamResponse {
        results: vec![row],
        field_mask: Some(FieldMask {
            paths: vec![
                "campaign.id".to_string(),
                "campaign.name".to_string(),
                "metrics.impressions".to_string(),
                "metrics.clicks".to_string(),
                "metrics.ctr".to_string(),
                "metrics.cost_micros".to_string(),
            ],
        }),
        summary_row: None,
        request_id: "test-request-789".to_string(),
        query_resource_consumption: 0,
    };

    let row = &response.results[0];
    assert_eq!(row.get("campaign.id"), "77777");
    assert_eq!(row.get("campaign.name"), "Performance Campaign");
    assert_eq!(row.get("metrics.impressions"), "50000");
    assert_eq!(row.get("metrics.clicks"), "2500");
    assert_eq!(row.get("metrics.ctr"), "0.05");
    assert_eq!(row.get("metrics.cost_micros"), "125000000");
}

#[test]
fn test_streaming_response_empty_results() {
    // Test handling of empty response
    let response = SearchGoogleAdsStreamResponse {
        results: vec![],
        field_mask: Some(FieldMask {
            paths: vec!["campaign.id".to_string()],
        }),
        summary_row: None,
        request_id: "test-request-empty".to_string(),
        query_resource_consumption: 0,
    };

    assert_eq!(response.results.len(), 0);
    assert!(response.field_mask.is_some());
}

// ============================================================================
// Field Mask Validation Tests
// ============================================================================

#[test]
fn test_invalid_field_path_returns_not_implemented() {
    let _field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.invalid_field".to_string(),
            "unknown.resource.field".to_string(),
        ],
    };

    let mut campaign = Campaign::default();
    campaign.id = 12345;

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ..Default::default()
    };

    // Valid path should work
    assert_eq!(row.get("campaign.id"), "12345");

    // Invalid paths should return "not implemented"
    assert_eq!(row.get("campaign.invalid_field"), "not implemented by googleads-rs");
    assert_eq!(row.get("unknown.resource.field"), "not implemented by googleads-rs");
}

#[test]
fn test_field_mask_with_optional_resources() {
    // Test field mask with paths to optional resources
    // Note: Some resources use optional_attr_str! and return "" when not present,
    // while others use attr_str! and will panic if accessed when not present.
    // This test focuses on optional_attr_str! fields.
    let _field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign_budget.amount_micros".to_string(), // Uses optional_attr_str!
        ],
    };

    let mut campaign = Campaign::default();
    campaign.id = 99999;

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        // campaign_budget not set
        ..Default::default()
    };

    // Campaign field should work
    assert_eq!(row.get("campaign.id"), "99999");

    // Optional resource using optional_attr_str! should return empty string when not present
    assert_eq!(row.get("campaign_budget.amount_micros"), "");
}

// ============================================================================
// Realistic Integration Scenarios
// ============================================================================

#[test]
fn test_realistic_campaign_report_query() {
    // Simulate a realistic campaign performance report query
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
            "campaign.status".to_string(),
            "metrics.impressions".to_string(),
            "metrics.clicks".to_string(),
            "metrics.ctr".to_string(),
            "metrics.cost_micros".to_string(),
            "metrics.conversions".to_string(),
            "segments.date".to_string(),
        ],
    };

    // Create multiple rows simulating a date range report
    let mut rows = Vec::new();
    let dates = vec!["2024-10-08", "2024-10-09", "2024-10-10"];

    for (idx, date) in dates.iter().enumerate() {
        let mut campaign = Campaign::default();
        campaign.id = 555555;
        campaign.name = "Q4 Campaign".to_string();
        campaign.status = CampaignStatus::Enabled as i32;

        let mut metrics = Metrics::default();
        metrics.impressions = 5000 + (idx as i64 * 500);
        metrics.clicks = 250 + (idx as i64 * 25);
        metrics.ctr = 0.05;
        metrics.cost_micros = 50000000;
        metrics.conversions = 10.0 + (idx as f64 * 2.0);

        let mut segments = Segments::default();
        segments.date = date.to_string();

        rows.push(GoogleAdsRow {
            campaign: Some(campaign),
            metrics: Some(metrics),
            segments: Some(segments),
            ..Default::default()
        });
    }

    let response = SearchGoogleAdsStreamResponse {
        results: rows,
        field_mask: Some(field_mask),
        summary_row: None,
        request_id: "campaign-report-123".to_string(),
        query_resource_consumption: 0,
    };

    // Verify response structure
    assert_eq!(response.results.len(), 3);

    // Verify each day's data
    for (idx, row) in response.results.iter().enumerate() {
        assert_eq!(row.get("campaign.id"), "555555");
        assert_eq!(row.get("campaign.name"), "Q4 Campaign");
        assert_eq!(row.get("campaign.status"), "Enabled");
        assert_eq!(row.get("segments.date"), dates[idx]);

        // Verify metrics progression
        let expected_impressions = 5000 + (idx as i64 * 500);
        let expected_clicks = 250 + (idx as i64 * 25);
        let expected_conversions = 10.0 + (idx as f64 * 2.0);

        assert_eq!(row.get("metrics.impressions"), format!("{}", expected_impressions));
        assert_eq!(row.get("metrics.clicks"), format!("{}", expected_clicks));
        assert_eq!(row.get("metrics.conversions"), format!("{}", expected_conversions));
    }
}

#[test]
fn test_realistic_ad_group_query() {
    // Simulate an ad group query with parent campaign data
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
            "ad_group.id".to_string(),
            "ad_group.name".to_string(),
            "ad_group.status".to_string(),
            "ad_group.cpc_bid_micros".to_string(),
        ],
    };

    let mut campaign = Campaign::default();
    campaign.id = 123456;
    campaign.name = "Parent Campaign".to_string();

    let mut ad_group = AdGroup::default();
    ad_group.id = 789012;
    ad_group.name = "Brand Keywords".to_string();
    ad_group.status = AdGroupStatus::Enabled as i32;
    ad_group.cpc_bid_micros = 5000000; // $5 CPC bid

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ad_group: Some(ad_group),
        ..Default::default()
    };

    let response = SearchGoogleAdsStreamResponse {
        results: vec![row],
        field_mask: Some(field_mask),
        summary_row: None,
        request_id: "ad-group-query-456".to_string(),
        query_resource_consumption: 0,
    };

    let row = &response.results[0];
    assert_eq!(row.get("campaign.id"), "123456");
    assert_eq!(row.get("campaign.name"), "Parent Campaign");
    assert_eq!(row.get("ad_group.id"), "789012");
    assert_eq!(row.get("ad_group.name"), "Brand Keywords");
    assert_eq!(row.get("ad_group.status"), "Enabled");
    assert_eq!(row.get("ad_group.cpc_bid_micros"), "5000000");
}

#[test]
fn test_field_mask_iteration_pattern() {
    // Test the typical usage pattern: iterate field mask paths and call get()
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
            "campaign.status".to_string(),
        ],
    };

    let mut campaign = Campaign::default();
    campaign.id = 111111;
    campaign.name = "Test Campaign".to_string();
    campaign.status = CampaignStatus::Enabled as i32;

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ..Default::default()
    };

    // Simulate typical usage: iterate and collect values
    let mut output = Vec::new();
    for path in &field_mask.paths {
        let value = row.get(path);
        output.push(format!("{}: {}", path, value));
    }

    assert_eq!(output.len(), 3);
    assert_eq!(output[0], "campaign.id: 111111");
    assert_eq!(output[1], "campaign.name: Test Campaign");
    assert_eq!(output[2], "campaign.status: Enabled");
}
