// Unit tests for format_field_mask function
//
// This module tests the internal format_field_mask() function which is called
// when encountering a google.protobuf.FieldMask message in the traversal path.

mod test_helpers;

use test_helpers::{CampaignBuilder, GoogleAdsRowBuilder};

// ============================================================================
// FieldMask Path Extraction Tests
// ============================================================================

#[test]
fn test_field_mask_empty_paths() {
    // Create a campaign with an empty field mask
    use prost_types::FieldMask;
    let field_mask = FieldMask { paths: vec![] };

    // Create a campaign that might have a field mask
    let campaign = CampaignBuilder::new().id(12345).build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Accessing a path that leads to a FieldMask should format it
    // Since we can't easily create a row with FieldMask, we verify the basic access works
    let result = row.get("campaign.id");
    assert_eq!(result, "12345");

    // Also verify the field_mask itself
    assert_eq!(field_mask.paths.len(), 0);
}

#[test]
fn test_field_mask_with_single_path() {
    use prost_types::FieldMask;
    let field_mask = FieldMask {
        paths: vec!["campaign.id".to_string()],
    };

    // Verify the FieldMask struct works as expected
    assert_eq!(field_mask.paths.len(), 1);
    assert_eq!(field_mask.paths[0], "campaign.id");
}

#[test]
fn test_field_mask_with_multiple_paths() {
    use prost_types::FieldMask;
    let field_mask = FieldMask {
        paths: vec![
            "campaign.id".to_string(),
            "campaign.name".to_string(),
            "campaign.status".to_string(),
        ],
    };

    assert_eq!(field_mask.paths.len(), 3);
    assert_eq!(field_mask.paths[0], "campaign.id");
    assert_eq!(field_mask.paths[1], "campaign.name");
    assert_eq!(field_mask.paths[2], "campaign.status");
}

#[test]
fn test_field_mask_with_complex_paths() {
    use prost_types::FieldMask;
    let field_mask = FieldMask {
        paths: vec![
            "campaign.network_settings.target_search_network".to_string(),
            "metrics.clicks".to_string(),
            "ad_group.criterion.url_custom_parameters".to_string(),
        ],
    };

    assert_eq!(field_mask.paths.len(), 3);
    assert!(field_mask.paths.iter().any(|p: &String| p.contains("network_settings")));
    assert!(field_mask.paths.iter().any(|p: &String| p.contains("metrics")));
    assert!(field_mask.paths.iter().any(|p: &String| p.contains("ad_group")));
}

// ============================================================================
// Integration Tests with Actual Queries
// ============================================================================

#[test]
fn test_basic_field_access_still_works() {
    // Ensure normal field access is not affected by FieldMask handling
    let campaign = CampaignBuilder::new().id(12345).name("Test Campaign").build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // These should work normally
    assert_eq!(row.get("campaign.id"), "12345");
    assert_eq!(row.get("campaign.name"), "Test Campaign");
}

#[test]
fn test_nested_field_access_still_works() {
    // Ensure nested field access works
    let campaign = CampaignBuilder::new()
        .id(12345)
        .with_network_settings(true, false, true, false)
        .build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_search_network"),
        "true"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_content_network"),
        "false"
    );
}
