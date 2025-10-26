// Unit tests for GoogleAdsRow::get() method - Asset Automation Settings
//
// This module tests the repeated_nested_enum_pair_str! macro
// which is used for extracting repeated nested messages with two enum accessor methods
//
// Resource tested:
// - Campaign.asset_automation_settings (repeated AssetAutomationSetting messages)
//   Each AssetAutomationSetting has:
//   - asset_automation_type() -> AssetAutomationType enum
//   - asset_automation_status() -> AssetAutomationStatus enum

mod test_helpers;

use googleads_rs::google::ads::googleads::v22::resources::campaign::AssetAutomationSetting;
use googleads_rs::google::ads::googleads::v22::resources::Campaign;
use test_helpers::GoogleAdsRowBuilder;

// ============================================================================
// Helper Functions
// ============================================================================

fn create_asset_automation_setting(
    automation_type: i32,
    automation_status: i32,
) -> AssetAutomationSetting {
    AssetAutomationSetting {
        asset_automation_type: automation_type,
        asset_automation_status: automation_status,
    }
}

// ============================================================================
// Basic Tests - Empty and Single Setting
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_empty() {
    let campaign = Campaign {
        asset_automation_settings: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    assert_eq!(result, "");
}

#[test]
fn test_campaign_asset_automation_settings_single_opted_in() {
    // AssetAutomationType::TEXT_ASSET_AUTOMATION = 2
    // AssetAutomationStatus::OPTED_IN = 2
    let campaign = Campaign {
        asset_automation_settings: vec![create_asset_automation_setting(2, 2)],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    assert_eq!(result, "TextAssetAutomation:OptedIn");
}

#[test]
fn test_campaign_asset_automation_settings_single_opted_out() {
    // AssetAutomationType::GENERATE_VERTICAL_YOUTUBE_VIDEOS = 3
    // AssetAutomationStatus::OPTED_OUT = 3
    let campaign = Campaign {
        asset_automation_settings: vec![create_asset_automation_setting(3, 3)],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    assert_eq!(result, "GenerateVerticalYoutubeVideos:OptedOut");
}

// ============================================================================
// Multiple Settings Tests
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_multiple() {
    // Multiple settings with different combinations
    let campaign = Campaign {
        asset_automation_settings: vec![
            create_asset_automation_setting(2, 2), // TEXT_ASSET_AUTOMATION:OPTED_IN
            create_asset_automation_setting(3, 3), // GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT
            create_asset_automation_setting(4, 2), // GENERATE_SHORTER_YOUTUBE_VIDEOS:OPTED_IN
        ],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    // Should be comma-space separated
    assert!(result.contains(", "));

    // Split and verify count
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);

    // Each part should have the format "Type:Status"
    for part in &parts {
        assert!(part.contains(':'), "Part '{}' should contain ':'", part);
        let type_status: Vec<&str> = part.split(':').collect();
        assert_eq!(
            type_status.len(),
            2,
            "Part '{}' should have exactly one ':'",
            part
        );
    }
}

#[test]
fn test_campaign_asset_automation_settings_realistic_pmax_campaign() {
    // Realistic Performance Max campaign with multiple automation settings
    let campaign = Campaign {
        id: 123456,
        name: "Performance Max Campaign".to_string(),
        asset_automation_settings: vec![
            create_asset_automation_setting(2, 2), // TEXT_ASSET_AUTOMATION:OPTED_IN
            create_asset_automation_setting(6, 2), // GENERATE_ENHANCED_YOUTUBE_VIDEOS:OPTED_IN
            create_asset_automation_setting(7, 2), // GENERATE_IMAGE_ENHANCEMENT:OPTED_IN
            create_asset_automation_setting(9, 2), // GENERATE_IMAGE_EXTRACTION:OPTED_IN
            create_asset_automation_setting(11, 2), // FINAL_URL_EXPANSION_TEXT_ASSET_AUTOMATION:OPTED_IN
        ],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    // Verify it's not empty
    assert!(!result.is_empty());

    // Verify count
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 5);

    // Verify all are opted in
    for part in &parts {
        assert!(
            part.ends_with(":OptedIn"),
            "Part '{}' should end with :OptedIn",
            part
        );
    }
}

#[test]
fn test_campaign_asset_automation_settings_realistic_search_campaign() {
    // Realistic Search campaign with selective automation (some opted out)
    let campaign = Campaign {
        id: 789012,
        name: "Search Campaign".to_string(),
        asset_automation_settings: vec![
            create_asset_automation_setting(2, 3), // TEXT_ASSET_AUTOMATION:OPTED_OUT
            create_asset_automation_setting(11, 3), // FINAL_URL_EXPANSION_TEXT_ASSET_AUTOMATION:OPTED_OUT
        ],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    // Verify it's not empty
    assert!(!result.is_empty());

    // Verify count
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 2);

    // Verify all are opted out
    for part in &parts {
        assert!(
            part.ends_with(":OptedOut"),
            "Part '{}' should end with :OptedOut",
            part
        );
    }
}

// ============================================================================
// Enum Value Tests - All AssetAutomationType Values
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_all_automation_types() {
    // Test all known AssetAutomationType enum values (excluding UNSPECIFIED=0, UNKNOWN=1)
    let campaign = Campaign {
        asset_automation_settings: vec![
            create_asset_automation_setting(2, 2),  // TEXT_ASSET_AUTOMATION
            create_asset_automation_setting(3, 2),  // GENERATE_VERTICAL_YOUTUBE_VIDEOS
            create_asset_automation_setting(4, 2),  // GENERATE_SHORTER_YOUTUBE_VIDEOS
            create_asset_automation_setting(5, 2),  // GENERATE_LANDING_PAGE_PREVIEW
            create_asset_automation_setting(6, 2),  // GENERATE_ENHANCED_YOUTUBE_VIDEOS
            create_asset_automation_setting(7, 2),  // GENERATE_IMAGE_ENHANCEMENT
            create_asset_automation_setting(9, 2),  // GENERATE_IMAGE_EXTRACTION
            create_asset_automation_setting(10, 2), // GENERATE_DESIGN_VERSIONS_FOR_IMAGES
            create_asset_automation_setting(11, 2), // FINAL_URL_EXPANSION_TEXT_ASSET_AUTOMATION
            create_asset_automation_setting(12, 2), // GENERATE_VIDEOS_FROM_OTHER_ASSETS
        ],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    // Should have 10 settings
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 10);

    // All should be opted in
    for part in &parts {
        assert!(part.ends_with(":OptedIn"));
    }
}

// ============================================================================
// Edge Cases - UNSPECIFIED and UNKNOWN
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_unspecified_type() {
    // AssetAutomationType::UNSPECIFIED = 0
    // AssetAutomationStatus::OPTED_IN = 2
    let campaign = Campaign {
        asset_automation_settings: vec![create_asset_automation_setting(0, 2)],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    assert_eq!(result, "Unspecified:OptedIn");
}

#[test]
fn test_campaign_asset_automation_settings_unknown_type() {
    // AssetAutomationType::UNKNOWN = 1
    // AssetAutomationStatus::OPTED_IN = 2
    let campaign = Campaign {
        asset_automation_settings: vec![create_asset_automation_setting(1, 2)],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    assert_eq!(result, "Unknown:OptedIn");
}

#[test]
fn test_campaign_asset_automation_settings_unspecified_status() {
    // AssetAutomationType::TEXT_ASSET_AUTOMATION = 2
    // AssetAutomationStatus::UNSPECIFIED = 0
    let campaign = Campaign {
        asset_automation_settings: vec![create_asset_automation_setting(2, 0)],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    assert_eq!(result, "TextAssetAutomation:Unspecified");
}

#[test]
fn test_campaign_asset_automation_settings_unknown_status() {
    // AssetAutomationType::TEXT_ASSET_AUTOMATION = 2
    // AssetAutomationStatus::UNKNOWN = 1
    let campaign = Campaign {
        asset_automation_settings: vec![create_asset_automation_setting(2, 1)],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    assert_eq!(result, "TextAssetAutomation:Unknown");
}

#[test]
fn test_campaign_asset_automation_settings_both_unspecified() {
    // Both UNSPECIFIED
    let campaign = Campaign {
        asset_automation_settings: vec![create_asset_automation_setting(0, 0)],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    assert_eq!(result, "Unspecified:Unspecified");
}

// ============================================================================
// Mixed Status Tests
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_mixed_statuses() {
    // Mix of opted in and opted out
    let campaign = Campaign {
        asset_automation_settings: vec![
            create_asset_automation_setting(2, 2), // TEXT_ASSET_AUTOMATION:OPTED_IN
            create_asset_automation_setting(3, 3), // GENERATE_VERTICAL_YOUTUBE_VIDEOS:OPTED_OUT
            create_asset_automation_setting(4, 2), // GENERATE_SHORTER_YOUTUBE_VIDEOS:OPTED_IN
            create_asset_automation_setting(5, 3), // GENERATE_LANDING_PAGE_PREVIEW:OPTED_OUT
            create_asset_automation_setting(6, 2), // GENERATE_ENHANCED_YOUTUBE_VIDEOS:OPTED_IN
        ],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 5);

    // Count opted in vs opted out
    let opted_in_count = parts.iter().filter(|p| p.ends_with(":OptedIn")).count();
    let opted_out_count = parts.iter().filter(|p| p.ends_with(":OptedOut")).count();

    assert_eq!(opted_in_count, 3);
    assert_eq!(opted_out_count, 2);
}

// ============================================================================
// Large Scale Tests
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_many_settings() {
    // Test with many settings (stress test)
    let mut settings = vec![];
    for i in 2..=12 {
        // Skip 8 as it's not defined in the enum
        if i == 8 {
            continue;
        }
        // Alternate between opted in and opted out
        let status = if i % 2 == 0 { 2 } else { 3 };
        settings.push(create_asset_automation_setting(i, status));
    }

    let campaign = Campaign {
        asset_automation_settings: settings,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    // Should have 10 settings (2-12 excluding 8)
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 10);

    // All should have the colon format
    for part in &parts {
        assert!(part.contains(':'));
    }
}

// ============================================================================
// Integration Tests - Verify with Other Campaign Fields
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_with_other_fields() {
    let campaign = Campaign {
        id: 999888,
        name: "Multi-field Test Campaign".to_string(),
        status: 2, // ENABLED
        asset_automation_settings: vec![
            create_asset_automation_setting(2, 2),
            create_asset_automation_setting(6, 2),
        ],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Test multiple fields
    assert_eq!(row.get("campaign.id"), "999888");
    assert_eq!(row.get("campaign.name"), "Multi-field Test Campaign");
    assert_eq!(row.get("campaign.status"), "Enabled");

    let settings_result = row.get("campaign.asset_automation_settings");
    assert!(settings_result.contains(", "));
    let parts: Vec<&str> = settings_result.split(", ").collect();
    assert_eq!(parts.len(), 2);
}

// ============================================================================
// Format Verification Tests
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_format_consistency() {
    let campaign = Campaign {
        asset_automation_settings: vec![
            create_asset_automation_setting(2, 2),
            create_asset_automation_setting(3, 3),
            create_asset_automation_setting(4, 2),
        ],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    // Verify separator is comma-space (not just comma)
    assert!(result.contains(", "));
    assert!(!result.contains(",,"));

    // Verify no leading or trailing commas
    assert!(!result.starts_with(','));
    assert!(!result.ends_with(','));

    // Verify each part has exactly one colon
    let parts: Vec<&str> = result.split(", ").collect();
    for part in &parts {
        assert_eq!(
            part.matches(':').count(),
            1,
            "Part '{}' should have exactly one colon",
            part
        );
    }
}

#[test]
fn test_campaign_asset_automation_settings_no_extra_whitespace() {
    let campaign = Campaign {
        asset_automation_settings: vec![create_asset_automation_setting(2, 2)],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.asset_automation_settings");

    // Should not have leading or trailing whitespace
    assert_eq!(result.trim(), result);

    // Should not have double spaces
    assert!(!result.contains("  "));
}

// ============================================================================
// Negative Tests - Absent Campaign Resource
// ============================================================================

#[test]
fn test_campaign_asset_automation_settings_campaign_absent() {
    // Create a row without a campaign resource
    let row = GoogleAdsRowBuilder::new().build();

    // Should panic because campaign is required (not optional)
    // This is expected behavior based on the non-optional macro pattern
    let result = std::panic::catch_unwind(|| row.get("campaign.asset_automation_settings"));

    assert!(result.is_err(), "Should panic when campaign is absent");
}
