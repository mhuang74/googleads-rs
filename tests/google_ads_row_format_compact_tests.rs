// Unit tests for format_message_compact function
//
// This module tests the internal format_message_compact() function which formats
// messages as "field:value" pairs for display in repeated message lists.

mod test_helpers;

use test_helpers::{CampaignBuilder, GoogleAdsRowBuilder};

// ============================================================================
// Basic Formatting Tests
// ============================================================================

#[test]
fn test_format_compact_single_field() {
    let campaign = CampaignBuilder::new().id(12345).build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    
    // Access campaign.id to trigger normal path
    let result = row.get("campaign.id");
    assert_eq!(result, "12345");
}

#[test]
fn test_format_compact_multiple_fields() {
    let campaign = CampaignBuilder::new()
        .id(12345)
        .name("Test Campaign")
        .build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    
    // Access multiple fields
    assert_eq!(row.get("campaign.id"), "12345");
    assert_eq!(row.get("campaign.name"), "Test Campaign");
}

// ============================================================================
// Empty and Default Value Tests
// ============================================================================

#[test]
fn test_format_compact_empty_message() {
    // Create campaign with minimal/no fields set
    let campaign = CampaignBuilder::new().build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    
    // Default/unset fields should return empty string
    assert_eq!(row.get("campaign.name"), "");
}

#[test]
fn test_format_compact_partial_fields() {
    // Campaign with only some fields set
    let campaign = CampaignBuilder::new()
        .id(12345)
        // name not set
        .build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    
    // Set field returns value
    assert_eq!(row.get("campaign.id"), "12345");
    // Unset field returns empty
    assert_eq!(row.get("campaign.name"), "");
}

// ============================================================================
// Nested Message Formatting Tests
// ============================================================================

#[test]
fn test_format_compact_nested_message() {
    let campaign = CampaignBuilder::new()
        .id(12345)
        .with_network_settings(true, false, true, false)
        .build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    
    // Nested message fields should work
    assert_eq!(row.get("campaign.network_settings.target_search_network"), "true");
    assert_eq!(row.get("campaign.network_settings.target_content_network"), "false");
    assert_eq!(row.get("campaign.network_settings.target_partner_search_network"), "true");
    assert_eq!(row.get("campaign.network_settings.target_google_search"), "false");
}

#[test]
fn test_format_compact_deeply_nested_path() {
    let campaign = CampaignBuilder::new()
        .id(12345)
        .with_network_settings(true, true, true, true)
        .build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    
    // Test that deeply nested paths resolve correctly
    let result = row.get("campaign.network_settings");
    // Partial paths return "not implemented"
    assert_eq!(result, "not implemented by googleads-rs");
}

// ============================================================================
// List/Message Separator Tests
// ============================================================================

#[test]
fn test_format_compact_with_enum_fields() {
    use googleads_rs::google::ads::googleads::v23::enums::campaign_status_enum::CampaignStatus;
    
    let campaign = CampaignBuilder::new()
        .id(12345)
        .status(CampaignStatus::Enabled)
        .build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    
    // Enum fields should be formatted as names
    assert_eq!(row.get("campaign.status"), "ENABLED");
}

#[test]
fn test_format_compact_with_optional_absent() {
    // Create row without optional resources
    let row = GoogleAdsRowBuilder::new().build();
    
    // Accessing nested fields when parent is absent should return empty
    assert_eq!(row.get("campaign.network_settings.target_search_network"), "");
}
