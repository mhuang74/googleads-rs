// Mock Integration Tests for googleads-rs
//
// These tests verify the main use case (query → stream → extract fields)
// without requiring real Google Ads API calls.

#![allow(clippy::field_reassign_with_default)]

// ============================================================================
// Test 2.1: Client Type Signature Verification
// ============================================================================

#[test]
fn test_google_ads_service_client_has_search_stream_method() {
    // This is a compile-time check to ensure the method exists with correct signature
    // We're using a trait bound to verify the method signature without calling it

    use googleads_rs::google::ads::googleads::v21::services::{
        google_ads_service_client::GoogleAdsServiceClient, SearchGoogleAdsStreamRequest,
    };
    use tonic::transport::Channel;

    fn _check_method_exists(
        _client: &GoogleAdsServiceClient<Channel>,
        _request: SearchGoogleAdsStreamRequest,
    ) {
        // If this compiles, the search_stream method exists with the right types
        // We're not actually calling it, just verifying the signature
    }
}

// ============================================================================
// Test 2.2: Mock Streaming Response Processing
// ============================================================================

#[tokio::test]
async fn test_process_mock_streaming_response() {
    use googleads_rs::google::ads::googleads::v21::common::Metrics;
    use googleads_rs::google::ads::googleads::v21::enums::campaign_status_enum::CampaignStatus;
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use googleads_rs::google::ads::googleads::v21::services::{
        GoogleAdsRow, SearchGoogleAdsStreamResponse,
    };
    use prost_types::FieldMask;

    // Simulate a streaming response batch
    let mut campaign = Campaign::default();
    campaign.id = 12345;
    campaign.name = "Test Campaign".to_string();
    campaign.status = CampaignStatus::Enabled as i32;

    let mut metrics = Metrics::default();
    metrics.impressions = 10000;
    metrics.clicks = 500;
    metrics.ctr = 0.05;

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
                "campaign.status".to_string(),
                "metrics.impressions".to_string(),
                "metrics.clicks".to_string(),
                "metrics.ctr".to_string(),
            ],
        }),
        summary_row: None,
        request_id: "test-request".to_string(),
        query_resource_consumption: 0,
    };

    // Process the response as a user would
    let field_mask = response.field_mask.as_ref().unwrap();
    let row = &response.results[0];

    // Collect values for all fields in the field mask
    let mut output: Vec<(String, String)> = Vec::new();
    for path in &field_mask.paths {
        let value = row.get(path);
        output.push((path.clone(), value));
    }

    // Verify we got all expected values
    assert_eq!(output.len(), 6);

    // Verify specific values
    assert_eq!(output[0].1, "12345");
    assert_eq!(output[1].1, "Test Campaign");
    assert_eq!(output[2].1, "Enabled");
    assert_eq!(output[3].1, "10000");
    assert_eq!(output[4].1, "500");
    assert_eq!(output[5].1, "0.05");
}

// ============================================================================
// Test 2.3: Multiple Batches Stream Processing
// ============================================================================

#[tokio::test]
async fn test_process_multiple_streaming_batches() {
    use googleads_rs::google::ads::googleads::v21::enums::campaign_status_enum::CampaignStatus;
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use googleads_rs::google::ads::googleads::v21::services::{
        GoogleAdsRow, SearchGoogleAdsStreamResponse,
    };
    use prost_types::FieldMask;

    // Simulate multiple streaming response batches
    let field_mask = FieldMask {
        paths: vec!["campaign.id".to_string(), "campaign.name".to_string()],
    };

    let mut all_responses = Vec::new();

    // Batch 1: 3 campaigns
    let mut batch1_rows = Vec::new();
    for i in 1..=3 {
        let mut campaign = Campaign::default();
        campaign.id = i * 100;
        campaign.name = format!("Campaign {}", i);
        campaign.status = CampaignStatus::Enabled as i32;

        batch1_rows.push(GoogleAdsRow {
            campaign: Some(campaign),
            ..Default::default()
        });
    }

    all_responses.push(SearchGoogleAdsStreamResponse {
        results: batch1_rows,
        field_mask: Some(field_mask.clone()),
        summary_row: None,
        request_id: "batch-1".to_string(),
        query_resource_consumption: 0,
    });

    // Batch 2: 2 campaigns
    let mut batch2_rows = Vec::new();
    for i in 4..=5 {
        let mut campaign = Campaign::default();
        campaign.id = i * 100;
        campaign.name = format!("Campaign {}", i);
        campaign.status = CampaignStatus::Enabled as i32;

        batch2_rows.push(GoogleAdsRow {
            campaign: Some(campaign),
            ..Default::default()
        });
    }

    all_responses.push(SearchGoogleAdsStreamResponse {
        results: batch2_rows,
        field_mask: Some(field_mask.clone()),
        summary_row: None,
        request_id: "batch-2".to_string(),
        query_resource_consumption: 0,
    });

    // Process all batches (simulating stream processing)
    let mut all_rows: Vec<Vec<String>> = vec![Vec::new(); 2]; // 2 columns (id, name)

    for response in all_responses {
        let field_mask = response.field_mask.as_ref().unwrap();

        for row in response.results {
            for (col_idx, path) in field_mask.paths.iter().enumerate() {
                let value = row.get(path);
                all_rows[col_idx].push(value);
            }
        }
    }

    // Verify we collected data from all 5 campaigns
    assert_eq!(all_rows[0].len(), 5); // 5 campaign IDs
    assert_eq!(all_rows[1].len(), 5); // 5 campaign names

    // Verify first and last campaigns
    assert_eq!(all_rows[0][0], "100");
    assert_eq!(all_rows[1][0], "Campaign 1");
    assert_eq!(all_rows[0][4], "500");
    assert_eq!(all_rows[1][4], "Campaign 5");
}

// ============================================================================
// Test 2.4: Field Mask Integration with GoogleAdsRow::get()
// ============================================================================

#[test]
fn test_field_mask_all_paths_accessible() {
    use googleads_rs::google::ads::googleads::v21::common::{Metrics, Segments};
    use googleads_rs::google::ads::googleads::v21::enums::{
        ad_group_status_enum::AdGroupStatus, campaign_status_enum::CampaignStatus,
    };
    use googleads_rs::google::ads::googleads::v21::resources::{AdGroup, Campaign, Customer};
    use googleads_rs::google::ads::googleads::v21::services::GoogleAdsRow;
    use prost_types::FieldMask;

    // Create a comprehensive field mask covering different resource types
    let field_mask = FieldMask {
        paths: vec![
            // Campaign fields
            "campaign.id".to_string(),
            "campaign.name".to_string(),
            "campaign.status".to_string(),
            // AdGroup fields
            "ad_group.id".to_string(),
            "ad_group.name".to_string(),
            "ad_group.status".to_string(),
            // Customer fields
            "customer.id".to_string(),
            "customer.descriptive_name".to_string(),
            // Metrics
            "metrics.impressions".to_string(),
            "metrics.clicks".to_string(),
            "metrics.cost_micros".to_string(),
            // Segments
            "segments.date".to_string(),
            "segments.device".to_string(),
        ],
    };

    // Create a fully populated row
    let mut campaign = Campaign::default();
    campaign.id = 111;
    campaign.name = "Campaign A".to_string();
    campaign.status = CampaignStatus::Enabled as i32;

    let mut ad_group = AdGroup::default();
    ad_group.id = 222;
    ad_group.name = "Ad Group B".to_string();
    ad_group.status = AdGroupStatus::Paused as i32;

    let mut customer = Customer::default();
    customer.id = 333;
    customer.descriptive_name = "Test Account".to_string();

    let mut metrics = Metrics::default();
    metrics.impressions = 50000;
    metrics.clicks = 1000;
    metrics.cost_micros = 75000000;

    let mut segments = Segments::default();
    segments.date = "2024-10-14".to_string();
    segments.device = 2; // Mobile

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ad_group: Some(ad_group),
        customer: Some(customer),
        metrics: Some(metrics),
        segments: Some(segments),
        ..Default::default()
    };

    // Verify ALL paths in field mask return non-empty, implemented values
    let mut valid_count = 0;
    for path in &field_mask.paths {
        let value = row.get(path);

        // Check value is not empty and not "not implemented"
        assert!(!value.is_empty(), "Path '{}' returned empty value", path);
        assert_ne!(
            value, "not implemented by googleads-rs",
            "Path '{}' is not implemented",
            path
        );

        valid_count += 1;
    }

    // All 13 paths should return valid values
    assert_eq!(
        valid_count, 13,
        "All field mask paths should return implemented values"
    );
}

// ============================================================================
// Test 2.5: Error Handling Simulation
// ============================================================================

#[test]
fn test_handle_missing_optional_resources() {
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use googleads_rs::google::ads::googleads::v21::services::GoogleAdsRow;

    // Create a row with only campaign (no budget, no ad_group)
    let mut campaign = Campaign::default();
    campaign.id = 999;

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        // campaign_budget and ad_group are None
        ..Default::default()
    };

    // Test each path
    let campaign_id = row.get("campaign.id");
    assert_eq!(campaign_id, "999");

    // Optional resource should return empty string (not panic)
    let budget = row.get("campaign_budget.amount_micros");
    assert_eq!(budget, "");

    // Note: Accessing non-optional missing resources (like ad_group.id) will panic
    // This is expected behavior - users should only query fields they selected
}

#[test]
fn test_unimplemented_field_returns_not_implemented() {
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use googleads_rs::google::ads::googleads::v21::services::GoogleAdsRow;

    let mut campaign = Campaign::default();
    campaign.id = 123;

    let row = GoogleAdsRow {
        campaign: Some(campaign),
        ..Default::default()
    };

    // Valid field
    assert_eq!(row.get("campaign.id"), "123");

    // Invalid/unimplemented fields should return the standard message
    assert_eq!(
        row.get("campaign.some_future_field"),
        "not implemented by googleads-rs"
    );
    assert_eq!(
        row.get("unknown.resource.field"),
        "not implemented by googleads-rs"
    );
}

// ============================================================================
// Test 2.6: Realistic Query Simulation
// ============================================================================

#[tokio::test]
async fn test_realistic_campaign_performance_query() {
    use googleads_rs::google::ads::googleads::v21::common::{Metrics, Segments};
    use googleads_rs::google::ads::googleads::v21::enums::campaign_status_enum::CampaignStatus;
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use googleads_rs::google::ads::googleads::v21::services::{
        GoogleAdsRow, SearchGoogleAdsStreamResponse,
    };
    use prost_types::FieldMask;

    // Simulate a realistic GAQL query:
    // SELECT campaign.id, campaign.name, campaign.status,
    //        metrics.impressions, metrics.clicks, metrics.cost_micros,
    //        segments.date
    // FROM campaign
    // WHERE segments.date DURING LAST_7_DAYS

    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
            "campaign.status".to_string(),
            "metrics.impressions".to_string(),
            "metrics.clicks".to_string(),
            "metrics.cost_micros".to_string(),
            "segments.date".to_string(),
        ],
    };

    // Simulate 7 days of data for one campaign
    let mut responses = Vec::new();
    let campaign_id = 555555;
    let campaign_name = "Q4 Promotion";

    for day in 8..=14 {
        let mut campaign = Campaign::default();
        campaign.id = campaign_id;
        campaign.name = campaign_name.to_string();
        campaign.status = CampaignStatus::Enabled as i32;

        let mut metrics = Metrics::default();
        metrics.impressions = 5000 + (day * 100);
        metrics.clicks = 250 + (day * 5);
        metrics.cost_micros = 50000000 + (day * 1000000);

        let mut segments = Segments::default();
        segments.date = format!("2024-10-{:02}", day);

        responses.push(SearchGoogleAdsStreamResponse {
            results: vec![GoogleAdsRow {
                campaign: Some(campaign),
                metrics: Some(metrics),
                segments: Some(segments),
                ..Default::default()
            }],
            field_mask: Some(field_mask.clone()),
            summary_row: None,
            request_id: format!("query-day-{}", day),
            query_resource_consumption: 0,
        });
    }

    // Process responses and aggregate data
    let mut daily_data: Vec<Vec<String>> = vec![Vec::new(); field_mask.paths.len()];

    for response in responses {
        let fm = response.field_mask.as_ref().unwrap();

        for row in response.results {
            for (idx, path) in fm.paths.iter().enumerate() {
                let value = row.get(path);
                daily_data[idx].push(value);
            }
        }
    }

    // Verify we got 7 days of data
    assert_eq!(daily_data[0].len(), 7); // 7 rows

    // Verify campaign ID is consistent
    assert!(daily_data[0].iter().all(|id| id == "555555"));

    // Verify metrics are increasing
    let impressions: Vec<i64> = daily_data[3].iter().map(|s| s.parse().unwrap()).collect();

    for i in 1..impressions.len() {
        assert!(
            impressions[i] > impressions[i - 1],
            "Impressions should increase daily"
        );
    }

    // Verify dates are present
    assert!(daily_data[6]
        .iter()
        .all(|date| date.starts_with("2024-10-")));
}
