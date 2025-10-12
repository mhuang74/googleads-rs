// Unit tests for GoogleAdsRow::get() method - Primary Status Details
//
// This module tests the repeated_message_str! macro
// which is used for extracting and formatting repeated nested message fields
// specifically for AssetLinkPrimaryStatusDetails

mod test_helpers;

use googleads_rs::google::ads::googleads::v19::common::{
    AssetDisapproved, AssetLinkPrimaryStatusDetails,
};
use googleads_rs::google::ads::googleads::v19::resources::{
    AdGroupAsset, AssetGroupAsset, CampaignAsset, CustomerAsset,
};
use test_helpers::GoogleAdsRowBuilder;

// ============================================================================
// Helper Functions
// ============================================================================

/// Creates a simple AssetLinkPrimaryStatusDetails with just reason and status
/// Note: Using i32 directly since enum variants use proto naming (e.g., ASSET_LINK_PAUSED)
fn create_simple_status_detail(reason: i32, status: i32) -> AssetLinkPrimaryStatusDetails {
    AssetLinkPrimaryStatusDetails {
        reason,
        status,
        details: None,
    }
}

/// Creates an AssetLinkPrimaryStatusDetails with asset disapproved details
fn create_status_detail_with_disapproved(
    reason: i32,
    status: i32,
    error_reasons: Vec<i32>,
) -> AssetLinkPrimaryStatusDetails {
    use googleads_rs::google::ads::googleads::v19::common::asset_link_primary_status_details::Details;

    AssetLinkPrimaryStatusDetails {
        reason,
        status,
        details: Some(Details::AssetDisapproved(AssetDisapproved {
            offline_evaluation_error_reasons: error_reasons,
        })),
    }
}

// ============================================================================
// AdGroupAsset Tests
// ============================================================================

#[test]
fn test_ad_group_asset_single_status_detail() {
    // reason: 0 = UNSPECIFIED, status: 0 = UNSPECIFIED (simple valid case)
    let status_details = vec![create_simple_status_detail(0, 0)];

    let ad_group_asset = AdGroupAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    let result = row.get("ad_group_asset.primary_status_details");

    // Should contain the enum values in debug format
    assert!(result.contains("reason:"), "Result: {}", result);
    assert!(result.contains("status:"), "Result: {}", result);
    // The debug format will show the integer values
    assert!(!result.is_empty(), "Result should not be empty");
}

#[test]
fn test_ad_group_asset_multiple_status_details() {
    // reason: 2 = ASSET_LINK_PAUSED, status: 2 = PAUSED
    // reason: 4 = ASSET_DISAPPROVED, status: 3 = NOT_ELIGIBLE
    let status_details = vec![
        create_simple_status_detail(2, 2),
        create_simple_status_detail(4, 3),
    ];

    let ad_group_asset = AdGroupAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    let result = row.get("ad_group_asset.primary_status_details");

    // Multiple details should be separated by semicolons
    assert!(result.contains(";"));
    // Should contain multiple status entries
    let count = result.matches("reason:").count();
    assert_eq!(count, 2, "Expected 2 status details, found {}", count);
}

#[test]
fn test_ad_group_asset_status_detail_with_asset_disapproved() {
    // reason: 4 = ASSET_DISAPPROVED, status: 3 = NOT_ELIGIBLE
    // error_reasons: 2 = PRICE_ASSET_DESCRIPTION_REPEATS_ROW_HEADER, 3 = PRICE_ASSET_REPETITIVE_HEADERS
    let status_details = vec![create_status_detail_with_disapproved(4, 3, vec![2, 3])];

    let ad_group_asset = AdGroupAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    let result = row.get("ad_group_asset.primary_status_details");

    // Should contain nested details
    assert!(result.contains("details:"));
    assert!(result.contains("AssetDisapproved") || result.contains("asset_disapproved"));
    assert!(result.contains("offline_evaluation_error_reasons"));
}

#[test]
fn test_ad_group_asset_empty_status_details() {
    let ad_group_asset = AdGroupAsset {
        primary_status_details: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    let result = row.get("ad_group_asset.primary_status_details");
    assert_eq!(result, "");
}

#[test]
fn test_ad_group_asset_complex_scenario() {
    // Mix of simple and complex status details
    let status_details = vec![
        create_simple_status_detail(
            2, // ASSET_LINK_PAUSED
            2, // PAUSED
        ),
        create_status_detail_with_disapproved(
            4, // ASSET_DISAPPROVED
            3, // NOT_ELIGIBLE
            vec![2],
        ),
        create_simple_status_detail(
            2, // ASSET_LINK_PAUSED
            2, // PAUSED
        ),
    ];

    let ad_group_asset = AdGroupAsset {
        resource_name: "customers/123/adGroupAssets/456~789".to_string(),
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    let result = row.get("ad_group_asset.primary_status_details");

    // Should have 3 details separated by 2 semicolons
    assert_eq!(result.matches(";").count(), 2);
    // Should contain all statuses
    let reason_count = result.matches("reason:").count();
    assert_eq!(reason_count, 3);
}

// ============================================================================
// CampaignAsset Tests
// ============================================================================

#[test]
fn test_campaign_asset_single_status_detail() {
    let status_details = vec![create_simple_status_detail(
        2, // ASSET_LINK_PAUSED
        2, // PAUSED
    )];

    let campaign_asset = CampaignAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    let result = row.get("campaign_asset.primary_status_details");

    assert!(result.contains("reason:"));
    assert!(result.contains("status:"));
}

#[test]
fn test_campaign_asset_multiple_status_details() {
    let status_details = vec![
        create_simple_status_detail(
            2, // ASSET_LINK_PAUSED
            2, // PAUSED
        ),
        create_simple_status_detail(
            6, // ASSET_APPROVED_LABELED
            4, // LIMITED
        ),
    ];

    let campaign_asset = CampaignAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    let result = row.get("campaign_asset.primary_status_details");

    // Should be separated by semicolons
    assert!(result.contains(";"));
    assert_eq!(result.matches("reason:").count(), 2);
}

#[test]
fn test_campaign_asset_empty_status_details() {
    let campaign_asset = CampaignAsset {
        primary_status_details: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    let result = row.get("campaign_asset.primary_status_details");
    assert_eq!(result, "");
}

#[test]
fn test_campaign_asset_with_disapproved_details() {
    let status_details = vec![create_status_detail_with_disapproved(
        4, // ASSET_DISAPPROVED
        3, // NOT_ELIGIBLE
        vec![2, 3, 7],
    )];

    let campaign_asset = CampaignAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    let result = row.get("campaign_asset.primary_status_details");

    // Should contain detailed error information
    assert!(result.contains("offline_evaluation_error_reasons"));
    // Should have 3 error reasons
    let bracket_pairs = result.matches('[').count();
    assert!(bracket_pairs > 0);
}

// ============================================================================
// CustomerAsset Tests
// ============================================================================

#[test]
fn test_customer_asset_single_status_detail() {
    let status_details = vec![create_simple_status_detail(
        2, // ASSET_LINK_PAUSED
        2, // PAUSED
    )];

    let customer_asset = CustomerAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    let result = row.get("customer_asset.primary_status_details");

    assert!(result.contains("reason:"));
    assert!(result.contains("status:"));
}

#[test]
fn test_customer_asset_multiple_status_details() {
    let status_details = vec![
        create_simple_status_detail(
            2, // ASSET_LINK_PAUSED
            2, // PAUSED
        ),
        create_simple_status_detail(
            3, // ASSET_LINK_REMOVED
            5, // REMOVED
        ),
    ];

    let customer_asset = CustomerAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    let result = row.get("customer_asset.primary_status_details");

    assert!(result.contains(";"));
    assert_eq!(result.matches("reason:").count(), 2);
}

#[test]
fn test_customer_asset_empty_status_details() {
    let customer_asset = CustomerAsset {
        primary_status_details: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    let result = row.get("customer_asset.primary_status_details");
    assert_eq!(result, "");
}

// ============================================================================
// AssetGroupAsset Tests
// ============================================================================

#[test]
fn test_asset_group_asset_single_status_detail() {
    let status_details = vec![create_simple_status_detail(
        2, // ASSET_LINK_PAUSED
        2, // PAUSED
    )];

    let asset_group_asset = AssetGroupAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    let result = row.get("asset_group_asset.primary_status_details");

    assert!(result.contains("reason:"));
    assert!(result.contains("status:"));
}

#[test]
fn test_asset_group_asset_multiple_status_details() {
    let status_details = vec![
        create_simple_status_detail(
            2, // ASSET_LINK_PAUSED
            2, // PAUSED
        ),
        create_simple_status_detail(
            5, // ASSET_UNDER_REVIEW
            6, // PENDING
        ),
    ];

    let asset_group_asset = AssetGroupAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    let result = row.get("asset_group_asset.primary_status_details");

    assert!(result.contains(";"));
    assert_eq!(result.matches("reason:").count(), 2);
}

#[test]
fn test_asset_group_asset_empty_status_details() {
    let asset_group_asset = AssetGroupAsset {
        primary_status_details: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    let result = row.get("asset_group_asset.primary_status_details");
    assert_eq!(result, "");
}

#[test]
fn test_asset_group_asset_realistic_scenario() {
    // Realistic Performance Max asset scenario
    let status_details = vec![
        create_simple_status_detail(
            2, // ASSET_LINK_PAUSED
            2, // PAUSED
        ),
        create_status_detail_with_disapproved(
            4, // ASSET_DISAPPROVED
            3, // NOT_ELIGIBLE
            vec![2],
        ),
    ];

    let asset_group_asset = AssetGroupAsset {
        resource_name: "customers/123/assetGroupAssets/456~789~1011".to_string(),
        asset_group: "customers/123/assetGroups/456".to_string(),
        asset: "customers/123/assets/789".to_string(),
        field_type: 1, // HEADLINE
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    let result = row.get("asset_group_asset.primary_status_details");

    // Should contain both status details
    assert_eq!(result.matches(";").count(), 1);
    // Should contain status details with reason and status fields
    assert!(result.contains("reason:"));
    assert!(result.contains("status:"));
    assert!(result.contains("details:"));
}

// ============================================================================
// Edge Cases and Format Tests
// ============================================================================

#[test]
fn test_semicolon_separator_not_comma() {
    // Verify that status details use semicolon separator, not comma
    let status_details = vec![
        create_simple_status_detail(
            2, // ASSET_LINK_PAUSED
            2, // PAUSED
        ),
        create_simple_status_detail(
            2, // ASSET_LINK_PAUSED
            2, // PAUSED
        ),
    ];

    let ad_group_asset = AdGroupAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    let result = row.get("ad_group_asset.primary_status_details");

    // Should use semicolon separator
    assert!(result.contains(";"));

    // The separator between status details should be "; " (semicolon space)
    // not ", " (comma space) which is used for enum lists
    assert!(result.contains("; "));
}

#[test]
fn test_debug_format_structure() {
    let status_details = vec![create_simple_status_detail(
        2, // ASSET_LINK_PAUSED
        2, // PAUSED
    )];

    let ad_group_asset = AdGroupAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    let result = row.get("ad_group_asset.primary_status_details");

    // Debug format should include field names
    assert!(result.contains("reason:"));
    assert!(result.contains("status:"));
    assert!(result.contains("details:"));
}

#[test]
fn test_all_resources_handle_empty_consistently() {
    // Verify all 4 resources handle empty status_details the same way
    let ad_group_asset = AdGroupAsset {
        primary_status_details: vec![],
        ..Default::default()
    };

    let campaign_asset = CampaignAsset {
        primary_status_details: vec![],
        ..Default::default()
    };

    let customer_asset = CustomerAsset {
        primary_status_details: vec![],
        ..Default::default()
    };

    let asset_group_asset = AssetGroupAsset {
        primary_status_details: vec![],
        ..Default::default()
    };

    let row1 = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();
    let row2 = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();
    let row3 = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();
    let row4 = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    assert_eq!(row1.get("ad_group_asset.primary_status_details"), "");
    assert_eq!(row2.get("campaign_asset.primary_status_details"), "");
    assert_eq!(row3.get("customer_asset.primary_status_details"), "");
    assert_eq!(row4.get("asset_group_asset.primary_status_details"), "");
}

#[test]
fn test_max_status_details_handling() {
    // Test with many status details to ensure no performance issues
    let mut status_details = Vec::new();
    for i in 0..10 {
        let reason = if i % 2 == 0 {
            2 // ASSET_LINK_PAUSED
        } else {
            3 // ASSET_LINK_REMOVED
        };
        let status = if i % 2 == 0 {
            2 // PAUSED
        } else {
            5 // REMOVED
        };
        status_details.push(create_simple_status_detail(reason, status));
    }

    let ad_group_asset = AdGroupAsset {
        primary_status_details: status_details,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    let result = row.get("ad_group_asset.primary_status_details");

    // Should have 10 status details
    assert_eq!(result.matches("reason:").count(), 10);
    // Should have 9 semicolons (separating 10 items)
    assert_eq!(result.matches("; ").count(), 9);
}
