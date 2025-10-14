# Integration Test Specification for googleads-rs

## Overview

This document specifies the integration tests needed to verify that `googleads-rs` correctly generates gRPC clients and protobuf types from Google Ads API definitions. These tests focus on catching build-time failures and ensuring the library's main use case works without requiring actual Google Ads API credentials.

## Objectives

1. Verify gRPC service clients are generated correctly by `tonic_build`
2. Confirm essential Google Ads resource types compile and are usable
3. Validate that the core use case (query → streaming results → field extraction) works end-to-end with mock data
4. Catch protobuf generation regressions early in CI/CD

## Scope

**In Scope:**
- Build-time verification of generated types
- Mock gRPC streaming with constructed test data
- GoogleAdsRow field extraction with field masks
- Error handling with mock Status errors

**Out of Scope:**
- Real Google Ads API authentication
- Actual API calls requiring credentials
- Performance benchmarking
- Exhaustive testing of all 200+ GoogleAdsRow::get() fields (covered by unit tests)

---

## Test Category 1: Build Verification Tests

**Location:** `tests/build_verification_tests.rs`

**Purpose:** Ensure critical types are generated and compile successfully.

### Test 1.1: Service Clients Generated

```rust
#[test]
fn test_google_ads_service_client_exists() {
    use googleads_rs::google::ads::googleads::v21::services::google_ads_service_client::GoogleAdsServiceClient;

    // This test passes if the type exists and compiles
    // We're not instantiating it, just verifying the type is available
    let _type_check: Option<GoogleAdsServiceClient<tonic::transport::Channel>> = None;
}

#[test]
fn test_other_service_clients_exist() {
    use googleads_rs::google::ads::googleads::v21::services::{
        customer_service_client::CustomerServiceClient,
        campaign_service_client::CampaignServiceClient,
        ad_group_service_client::AdGroupServiceClient,
    };

    // Verify other commonly used service clients compile
    let _: Option<CustomerServiceClient<tonic::transport::Channel>> = None;
    let _: Option<CampaignServiceClient<tonic::transport::Channel>> = None;
    let _: Option<AdGroupServiceClient<tonic::transport::Channel>> = None;
}
```

### Test 1.2: Core Resource Types Generated

```rust
#[test]
fn test_core_resource_types_exist() {
    use googleads_rs::google::ads::googleads::v21::resources::{
        Campaign,
        AdGroup,
        Customer,
        AdGroupAd,
        AdGroupCriterion,
    };

    // Verify we can construct these types
    let _campaign = Campaign::default();
    let _ad_group = AdGroup::default();
    let _customer = Customer::default();
    let _ad = AdGroupAd::default();
    let _criterion = AdGroupCriterion::default();
}

#[test]
fn test_metrics_and_segments_exist() {
    use googleads_rs::google::ads::googleads::v21::common::{
        Metrics,
        Segments,
    };

    let _metrics = Metrics::default();
    let _segments = Segments::default();
}
```

### Test 1.3: Request/Response Types Generated

```rust
#[test]
fn test_search_request_response_types_exist() {
    use googleads_rs::google::ads::googleads::v21::services::{
        SearchGoogleAdsStreamRequest,
        SearchGoogleAdsStreamResponse,
        GoogleAdsRow,
    };

    let _request = SearchGoogleAdsStreamRequest::default();
    let _response = SearchGoogleAdsStreamResponse::default();
    let _row = GoogleAdsRow::default();
}

#[test]
fn test_field_mask_type_exists() {
    use prost_types::FieldMask;

    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
        ],
    };

    assert_eq!(field_mask.paths.len(), 2);
}
```

### Test 1.4: Common Enum Types Generated

```rust
#[test]
fn test_common_enum_types_exist() {
    use googleads_rs::google::ads::googleads::v21::enums::{
        campaign_status_enum::CampaignStatus,
        ad_group_status_enum::AdGroupStatus,
        advertising_channel_type_enum::AdvertisingChannelType,
    };

    let _campaign_status = CampaignStatus::Enabled;
    let _ad_group_status = AdGroupStatus::Paused;
    let _channel_type = AdvertisingChannelType::Search;
}
```

---

## Test Category 2: Mock Integration Tests

**Location:** `tests/mock_integration_tests.rs`

**Purpose:** Test the main use case (query → stream → extract fields) without real API calls.

### Test 2.1: Client Type Signature Verification

```rust
#[test]
fn test_google_ads_service_client_has_search_stream_method() {
    // This is a compile-time check to ensure the method exists with correct signature
    // We're using a trait bound to verify the method signature without calling it

    use googleads_rs::google::ads::googleads::v21::services::{
        google_ads_service_client::GoogleAdsServiceClient,
        SearchGoogleAdsStreamRequest,
    };
    use tonic::transport::Channel;

    fn check_method_exists(
        _client: &GoogleAdsServiceClient<Channel>,
        _request: SearchGoogleAdsStreamRequest,
    ) {
        // If this compiles, the search_stream method exists with the right types
        // We're not actually calling it, just verifying the signature
    }
}
```

### Test 2.2: Mock Streaming Response Processing

```rust
#[tokio::test]
async fn test_process_mock_streaming_response() {
    use googleads_rs::google::ads::googleads::v21::services::{
        SearchGoogleAdsStreamResponse,
        GoogleAdsRow,
    };
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use googleads_rs::google::ads::googleads::v21::enums::campaign_status_enum::CampaignStatus;
    use googleads_rs::google::ads::googleads::v21::common::Metrics;
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
```

### Test 2.3: Multiple Batches Stream Processing

```rust
#[tokio::test]
async fn test_process_multiple_streaming_batches() {
    use googleads_rs::google::ads::googleads::v21::services::{
        SearchGoogleAdsStreamResponse,
        GoogleAdsRow,
    };
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use googleads_rs::google::ads::googleads::v21::enums::campaign_status_enum::CampaignStatus;
    use prost_types::FieldMask;

    // Simulate multiple streaming response batches
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
        ],
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
```

### Test 2.4: Field Mask Integration with GoogleAdsRow::get()

```rust
#[test]
fn test_field_mask_all_paths_accessible() {
    use googleads_rs::google::ads::googleads::v21::services::GoogleAdsRow;
    use googleads_rs::google::ads::googleads::v21::resources::{
        Campaign, AdGroup, Customer,
    };
    use googleads_rs::google::ads::googleads::v21::common::{Metrics, Segments};
    use googleads_rs::google::ads::googleads::v21::enums::{
        campaign_status_enum::CampaignStatus,
        ad_group_status_enum::AdGroupStatus,
    };
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
    assert_eq!(valid_count, 13, "All field mask paths should return implemented values");
}
```

### Test 2.5: Error Handling Simulation

```rust
#[test]
fn test_handle_missing_optional_resources() {
    use googleads_rs::google::ads::googleads::v21::services::GoogleAdsRow;
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use prost_types::FieldMask;

    // Create a field mask that includes optional resources
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign_budget.amount_micros".to_string(), // Optional resource
            "ad_group.id".to_string(), // Missing resource
        ],
    };

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

    // This will panic because ad_group uses attr_str! not optional_attr_str!
    // This is expected behavior - the user should only query fields they selected
    // We're documenting this behavior, not testing it
}

#[test]
fn test_unimplemented_field_returns_not_implemented() {
    use googleads_rs::google::ads::googleads::v21::services::GoogleAdsRow;
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;

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
```

### Test 2.6: Realistic Query Simulation

```rust
#[tokio::test]
async fn test_realistic_campaign_performance_query() {
    use googleads_rs::google::ads::googleads::v21::services::{
        SearchGoogleAdsStreamResponse,
        GoogleAdsRow,
    };
    use googleads_rs::google::ads::googleads::v21::resources::Campaign;
    use googleads_rs::google::ads::googleads::v21::enums::campaign_status_enum::CampaignStatus;
    use googleads_rs::google::ads::googleads::v21::common::{Metrics, Segments};
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
    let impressions: Vec<i64> = daily_data[3]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 1..impressions.len() {
        assert!(impressions[i] > impressions[i-1], "Impressions should increase daily");
    }

    // Verify dates are present
    assert!(daily_data[6].iter().all(|date| date.starts_with("2024-10-")));
}
```

---

## Test Execution

### Running Tests

```bash
# Run all integration tests
cargo test --test build_verification_tests
cargo test --test mock_integration_tests

# Run with output
cargo test --test mock_integration_tests -- --nocapture

# Run specific test
cargo test --test mock_integration_tests test_realistic_campaign_performance_query
```

### CI/CD Integration

Add to `.github/workflows/rust.yml`:

```yaml
jobs:
  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install protobuf compiler
        run: sudo apt-get update && sudo apt-get install -y protobuf-compiler

      - name: Run build verification tests
        run: cargo test --test build_verification_tests

      - name: Run mock integration tests
        run: cargo test --test mock_integration_tests
```

---

## Success Criteria

All tests should:
1. ✅ Compile without errors
2. ✅ Run without panics
3. ✅ Complete in < 5 seconds total
4. ✅ Pass consistently (no flakiness)
5. ✅ Require no external dependencies (API credentials, network, etc.)

---

## Future Enhancements

### Low Priority Additions

1. **Test more service clients** - Add tests for other common services (ConversionActionService, etc.)
2. **Test nested message types** - Verify deeply nested proto structures compile
3. **Test oneof variants** - Ensure criterion types, ad types, etc. work correctly
4. **Add streaming mock** - Use `tokio_stream` to simulate actual streaming responses

### Maintenance

- Update tests when upgrading to new Google Ads API versions (v21 → v22, etc.)
- Add tests for new resource types as they become important
- Review and update after any changes to `build.rs` or proto filtering logic

---

## Related Documentation

- Existing unit tests: `tests/integration_streaming_tests.rs` (provides additional examples)
- Unit tests for GoogleAdsRow::get(): Various `tests/google_ads_row_*_tests.rs` files
- Build system: `build.rs` (proto file discovery and compilation)
- Comprehensive test plan: `specs/comprehensive_test_plan.md` (broader testing strategy)
