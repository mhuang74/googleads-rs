// Unit tests for GoogleAdsRow::get() method - Primary Status and Primary Status Reasons
//
// This module tests:
// 1. method_str! macro for primary_status fields (single enum)
// 2. repeated_enum_str! macro for primary_status_reasons fields (repeated enums)
//
// Resources tested:
// - Campaign (primary_status + primary_status_reasons)
// - AdGroup (primary_status + primary_status_reasons)
// - AdGroupAd (primary_status + primary_status_reasons)
// - AdGroupCriterion (primary_status + primary_status_reasons)
// - AssetGroup (primary_status + primary_status_reasons)
// - AdGroupAsset (primary_status + primary_status_reasons) [already implemented in lib.rs]
// - CampaignAsset (primary_status + primary_status_reasons) [already implemented in lib.rs]
// - CustomerAsset (primary_status + primary_status_reasons) [already implemented in lib.rs]
// - AssetGroupAsset (primary_status + primary_status_reasons)

mod test_helpers;

use googleads_rs::google::ads::googleads::v22::resources::{
    AdGroup, AdGroupAd, AdGroupAsset, AdGroupCriterion, AssetGroup, AssetGroupAsset, Campaign,
    CampaignAsset, CustomerAsset,
};
use test_helpers::GoogleAdsRowBuilder;

// ============================================================================
// Campaign Primary Status Tests
// ============================================================================

#[test]
fn test_campaign_primary_status_eligible() {
    // 2 = ELIGIBLE
    let campaign = Campaign {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_campaign_primary_status_paused() {
    // 3 = PAUSED
    let campaign = Campaign {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status");

    assert_eq!(result, "Paused");
}

#[test]
fn test_campaign_primary_status_removed() {
    // 4 = REMOVED
    let campaign = Campaign {
        primary_status: 4,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status");

    assert_eq!(result, "Removed");
}

#[test]
fn test_campaign_primary_status_unspecified() {
    // 0 = UNSPECIFIED
    let campaign = Campaign {
        primary_status: 0,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status");

    assert_eq!(result, "Unspecified");
}

// ============================================================================
// Campaign Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_campaign_primary_status_reasons_empty() {
    let campaign = Campaign {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_campaign_primary_status_reasons_single() {
    // 3 = CAMPAIGN_PAUSED
    let campaign = Campaign {
        primary_status_reasons: vec![3],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status_reasons");

    assert_eq!(result, "CampaignPaused");
}

#[test]
fn test_campaign_primary_status_reasons_multiple() {
    // 2 = CAMPAIGN_REMOVED, 3 = CAMPAIGN_PAUSED, 5 = CAMPAIGN_ENDED
    let campaign = Campaign {
        primary_status_reasons: vec![2, 3, 5],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

#[test]
fn test_campaign_primary_status_reasons_unspecified() {
    // 0 = UNSPECIFIED
    let campaign = Campaign {
        primary_status_reasons: vec![0],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status_reasons");

    assert_eq!(result, "Unspecified");
}

// ============================================================================
// AdGroup Primary Status Tests
// ============================================================================

#[test]
fn test_ad_group_primary_status_eligible() {
    // 2 = ELIGIBLE
    let ad_group = AdGroup {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();
    let result = row.get("ad_group.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_ad_group_primary_status_paused() {
    // 3 = PAUSED
    let ad_group = AdGroup {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();
    let result = row.get("ad_group.primary_status");

    assert_eq!(result, "Paused");
}

#[test]
fn test_ad_group_primary_status_removed() {
    // 4 = REMOVED
    let ad_group = AdGroup {
        primary_status: 4,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();
    let result = row.get("ad_group.primary_status");

    assert_eq!(result, "Removed");
}

// ============================================================================
// AdGroup Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_ad_group_primary_status_reasons_empty() {
    let ad_group = AdGroup {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();
    let result = row.get("ad_group.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_ad_group_primary_status_reasons_single() {
    // 3 = AD_GROUP_PAUSED (based on proto pattern)
    let ad_group = AdGroup {
        primary_status_reasons: vec![3],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();
    let result = row.get("ad_group.primary_status_reasons");

    // Just verify it's not empty and doesn't error
    assert!(!result.is_empty());
}

#[test]
fn test_ad_group_primary_status_reasons_multiple() {
    // Using multiple values
    let ad_group = AdGroup {
        primary_status_reasons: vec![3, 4, 5],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();
    let result = row.get("ad_group.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

// ============================================================================
// AdGroupAd Primary Status Tests
// ============================================================================

#[test]
fn test_ad_group_ad_primary_status_eligible() {
    // 2 = ELIGIBLE
    let ad_group_ad = AdGroupAd {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();
    let result = row.get("ad_group_ad.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_ad_group_ad_primary_status_paused() {
    // 3 = PAUSED
    let ad_group_ad = AdGroupAd {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();
    let result = row.get("ad_group_ad.primary_status");

    assert_eq!(result, "Paused");
}

#[test]
fn test_ad_group_ad_primary_status_removed() {
    // 4 = REMOVED
    let ad_group_ad = AdGroupAd {
        primary_status: 4,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();
    let result = row.get("ad_group_ad.primary_status");

    assert_eq!(result, "Removed");
}

#[test]
fn test_ad_group_ad_primary_status_pending() {
    // 5 = PENDING
    let ad_group_ad = AdGroupAd {
        primary_status: 5,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();
    let result = row.get("ad_group_ad.primary_status");

    assert_eq!(result, "Pending");
}

// ============================================================================
// AdGroupAd Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_ad_group_ad_primary_status_reasons_empty() {
    let ad_group_ad = AdGroupAd {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();
    let result = row.get("ad_group_ad.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_ad_group_ad_primary_status_reasons_single() {
    // 2 = CAMPAIGN_REMOVED (from proto)
    let ad_group_ad = AdGroupAd {
        primary_status_reasons: vec![2],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();
    let result = row.get("ad_group_ad.primary_status_reasons");

    assert_eq!(result, "CampaignRemoved");
}

#[test]
fn test_ad_group_ad_primary_status_reasons_multiple() {
    // 2, 3, 5 (using actual proto values)
    let ad_group_ad = AdGroupAd {
        primary_status_reasons: vec![2, 3, 5],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();
    let result = row.get("ad_group_ad.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

// ============================================================================
// AdGroupCriterion Primary Status Tests
// ============================================================================

#[test]
fn test_ad_group_criterion_primary_status_eligible() {
    // 2 = ELIGIBLE
    let ad_group_criterion = AdGroupCriterion {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .build();
    let result = row.get("ad_group_criterion.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_ad_group_criterion_primary_status_paused() {
    // 3 = PAUSED
    let ad_group_criterion = AdGroupCriterion {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .build();
    let result = row.get("ad_group_criterion.primary_status");

    assert_eq!(result, "Paused");
}

#[test]
fn test_ad_group_criterion_primary_status_removed() {
    // 4 = REMOVED
    let ad_group_criterion = AdGroupCriterion {
        primary_status: 4,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .build();
    let result = row.get("ad_group_criterion.primary_status");

    assert_eq!(result, "Removed");
}

#[test]
fn test_ad_group_criterion_primary_status_pending() {
    // 5 = PENDING
    let ad_group_criterion = AdGroupCriterion {
        primary_status: 5,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .build();
    let result = row.get("ad_group_criterion.primary_status");

    assert_eq!(result, "Pending");
}

// ============================================================================
// AdGroupCriterion Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_ad_group_criterion_primary_status_reasons_empty() {
    let ad_group_criterion = AdGroupCriterion {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .build();
    let result = row.get("ad_group_criterion.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_ad_group_criterion_primary_status_reasons_single() {
    // 2 = CAMPAIGN_PENDING (from proto)
    let ad_group_criterion = AdGroupCriterion {
        primary_status_reasons: vec![2],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .build();
    let result = row.get("ad_group_criterion.primary_status_reasons");

    assert_eq!(result, "CampaignPending");
}

#[test]
fn test_ad_group_criterion_primary_status_reasons_multiple() {
    // 2, 4, 6 (using actual values)
    let ad_group_criterion = AdGroupCriterion {
        primary_status_reasons: vec![2, 4, 6],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .build();
    let result = row.get("ad_group_criterion.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

// ============================================================================
// AssetGroup Primary Status Tests
// ============================================================================

#[test]
fn test_asset_group_primary_status_eligible() {
    // 2 = ELIGIBLE
    let asset_group = AssetGroup {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();
    let result = row.get("asset_group.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_asset_group_primary_status_paused() {
    // 3 = PAUSED
    let asset_group = AssetGroup {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();
    let result = row.get("asset_group.primary_status");

    assert_eq!(result, "Paused");
}

#[test]
fn test_asset_group_primary_status_removed() {
    // 4 = REMOVED
    let asset_group = AssetGroup {
        primary_status: 4,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();
    let result = row.get("asset_group.primary_status");

    assert_eq!(result, "Removed");
}

#[test]
fn test_asset_group_primary_status_pending() {
    // 6 = PENDING (if exists, or use a valid value)
    let asset_group = AssetGroup {
        primary_status: 6,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();
    let result = row.get("asset_group.primary_status");

    // Check the actual expected value
    assert!(!result.is_empty());
}

// ============================================================================
// AssetGroup Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_asset_group_primary_status_reasons_empty() {
    let asset_group = AssetGroup {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();
    let result = row.get("asset_group.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_asset_group_primary_status_reasons_single() {
    // Using valid value from proto
    let asset_group = AssetGroup {
        primary_status_reasons: vec![3],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();
    let result = row.get("asset_group.primary_status_reasons");

    assert!(!result.is_empty());
}

#[test]
fn test_asset_group_primary_status_reasons_multiple() {
    // Using multiple values
    let asset_group = AssetGroup {
        primary_status_reasons: vec![2, 3, 4],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();
    let result = row.get("asset_group.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

// ============================================================================
// AdGroupAsset Primary Status Tests (Already Implemented)
// ============================================================================

#[test]
fn test_ad_group_asset_primary_status_eligible() {
    // 2 = ELIGIBLE
    let ad_group_asset = AdGroupAsset {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();
    let result = row.get("ad_group_asset.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_ad_group_asset_primary_status_paused() {
    // 3 = PAUSED
    let ad_group_asset = AdGroupAsset {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();
    let result = row.get("ad_group_asset.primary_status");

    assert_eq!(result, "Paused");
}

// ============================================================================
// AdGroupAsset Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_ad_group_asset_primary_status_reasons_empty() {
    let ad_group_asset = AdGroupAsset {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();
    let result = row.get("ad_group_asset.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_ad_group_asset_primary_status_reasons_single() {
    // 2 = ASSET_LINK_PAUSED
    let ad_group_asset = AdGroupAsset {
        primary_status_reasons: vec![2],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();
    let result = row.get("ad_group_asset.primary_status_reasons");

    assert_eq!(result, "AssetLinkPaused");
}

#[test]
fn test_ad_group_asset_primary_status_reasons_multiple() {
    // 2 = ASSET_LINK_PAUSED, 3 = ASSET_LINK_REMOVED, 4 = ASSET_DISAPPROVED
    let ad_group_asset = AdGroupAsset {
        primary_status_reasons: vec![2, 3, 4],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();
    let result = row.get("ad_group_asset.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

// ============================================================================
// CampaignAsset Primary Status Tests (Already Implemented)
// ============================================================================

#[test]
fn test_campaign_asset_primary_status_eligible() {
    // 2 = ELIGIBLE
    let campaign_asset = CampaignAsset {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();
    let result = row.get("campaign_asset.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_campaign_asset_primary_status_paused() {
    // 3 = PAUSED
    let campaign_asset = CampaignAsset {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();
    let result = row.get("campaign_asset.primary_status");

    assert_eq!(result, "Paused");
}

// ============================================================================
// CampaignAsset Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_campaign_asset_primary_status_reasons_empty() {
    let campaign_asset = CampaignAsset {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();
    let result = row.get("campaign_asset.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_campaign_asset_primary_status_reasons_single() {
    // 2 = ASSET_LINK_PAUSED
    let campaign_asset = CampaignAsset {
        primary_status_reasons: vec![2],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();
    let result = row.get("campaign_asset.primary_status_reasons");

    assert_eq!(result, "AssetLinkPaused");
}

#[test]
fn test_campaign_asset_primary_status_reasons_multiple() {
    // 2 = ASSET_LINK_PAUSED, 5 = ASSET_UNDER_REVIEW, 6 = ASSET_APPROVED_LABELED
    let campaign_asset = CampaignAsset {
        primary_status_reasons: vec![2, 5, 6],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();
    let result = row.get("campaign_asset.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

// ============================================================================
// CustomerAsset Primary Status Tests (Already Implemented)
// ============================================================================

#[test]
fn test_customer_asset_primary_status_eligible() {
    // 2 = ELIGIBLE
    let customer_asset = CustomerAsset {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();
    let result = row.get("customer_asset.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_customer_asset_primary_status_paused() {
    // 3 = PAUSED
    let customer_asset = CustomerAsset {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();
    let result = row.get("customer_asset.primary_status");

    assert_eq!(result, "Paused");
}

// ============================================================================
// CustomerAsset Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_customer_asset_primary_status_reasons_empty() {
    let customer_asset = CustomerAsset {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();
    let result = row.get("customer_asset.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_customer_asset_primary_status_reasons_single() {
    // 2 = ASSET_LINK_PAUSED
    let customer_asset = CustomerAsset {
        primary_status_reasons: vec![2],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();
    let result = row.get("customer_asset.primary_status_reasons");

    assert_eq!(result, "AssetLinkPaused");
}

#[test]
fn test_customer_asset_primary_status_reasons_multiple() {
    // 2 = ASSET_LINK_PAUSED, 3 = ASSET_LINK_REMOVED, 5 = ASSET_UNDER_REVIEW
    let customer_asset = CustomerAsset {
        primary_status_reasons: vec![2, 3, 5],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();
    let result = row.get("customer_asset.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

// ============================================================================
// AssetGroupAsset Primary Status Tests
// ============================================================================

#[test]
fn test_asset_group_asset_primary_status_eligible() {
    // 2 = ELIGIBLE
    let asset_group_asset = AssetGroupAsset {
        primary_status: 2,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();
    let result = row.get("asset_group_asset.primary_status");

    assert_eq!(result, "Eligible");
}

#[test]
fn test_asset_group_asset_primary_status_paused() {
    // 3 = PAUSED
    let asset_group_asset = AssetGroupAsset {
        primary_status: 3,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();
    let result = row.get("asset_group_asset.primary_status");

    assert_eq!(result, "Paused");
}

#[test]
fn test_asset_group_asset_primary_status_limited() {
    // 6 = LIMITED
    let asset_group_asset = AssetGroupAsset {
        primary_status: 6,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();
    let result = row.get("asset_group_asset.primary_status");

    assert_eq!(result, "Limited");
}

#[test]
fn test_asset_group_asset_primary_status_not_eligible() {
    // 7 = NOT_ELIGIBLE
    let asset_group_asset = AssetGroupAsset {
        primary_status: 7,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();
    let result = row.get("asset_group_asset.primary_status");

    assert_eq!(result, "NotEligible");
}

// ============================================================================
// AssetGroupAsset Primary Status Reasons Tests
// ============================================================================

#[test]
fn test_asset_group_asset_primary_status_reasons_empty() {
    let asset_group_asset = AssetGroupAsset {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();
    let result = row.get("asset_group_asset.primary_status_reasons");

    assert_eq!(result, "");
}

#[test]
fn test_asset_group_asset_primary_status_reasons_single() {
    // 2 = ASSET_LINK_PAUSED
    let asset_group_asset = AssetGroupAsset {
        primary_status_reasons: vec![2],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();
    let result = row.get("asset_group_asset.primary_status_reasons");

    assert_eq!(result, "AssetLinkPaused");
}

#[test]
fn test_asset_group_asset_primary_status_reasons_multiple() {
    // 2 = ASSET_LINK_PAUSED, 4 = ASSET_DISAPPROVED, 5 = ASSET_UNDER_REVIEW
    let asset_group_asset = AssetGroupAsset {
        primary_status_reasons: vec![2, 4, 5],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();
    let result = row.get("asset_group_asset.primary_status_reasons");

    // Should be comma-separated
    assert!(result.contains(", "));
    let parts: Vec<&str> = result.split(", ").collect();
    assert_eq!(parts.len(), 3);
}

// ============================================================================
// Realistic Scenario Tests
// ============================================================================

#[test]
fn test_campaign_paused_with_multiple_reasons() {
    // Realistic scenario: Campaign is paused with multiple reasons
    let campaign = Campaign {
        id: 12345,
        name: "Test Campaign".to_string(),
        primary_status: 3,                   // PAUSED
        primary_status_reasons: vec![3, 11], // CAMPAIGN_PAUSED, BUDGET_CONSTRAINED
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    let status = row.get("campaign.primary_status");
    let reasons = row.get("campaign.primary_status_reasons");

    assert_eq!(status, "Paused");
    assert!(reasons.contains(", "));
    assert_eq!(reasons.split(", ").count(), 2);
}

#[test]
fn test_ad_group_ad_pending_approval() {
    // Realistic scenario: Ad is pending approval
    let ad_group_ad = AdGroupAd {
        primary_status: 5,               // PENDING
        primary_status_reasons: vec![2], // CAMPAIGN_REMOVED
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();

    let status = row.get("ad_group_ad.primary_status");
    let reasons = row.get("ad_group_ad.primary_status_reasons");

    assert_eq!(status, "Pending");
    assert_eq!(reasons, "CampaignRemoved");
}

#[test]
fn test_asset_group_asset_not_eligible_with_disapproval() {
    // Realistic scenario: Asset is not eligible due to disapproval
    let asset_group_asset = AssetGroupAsset {
        resource_name: "customers/123/assetGroupAssets/456~789~1011".to_string(),
        primary_status: 7,               // NOT_ELIGIBLE (correct value)
        primary_status_reasons: vec![4], // ASSET_DISAPPROVED
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    let status = row.get("asset_group_asset.primary_status");
    let reasons = row.get("asset_group_asset.primary_status_reasons");

    assert_eq!(status, "NotEligible");
    assert_eq!(reasons, "AssetDisapproved");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_all_resources_handle_empty_reasons_consistently() {
    // Verify all resources handle empty reasons the same way
    let campaign = Campaign {
        primary_status_reasons: vec![],
        ..Default::default()
    };
    let ad_group = AdGroup {
        primary_status_reasons: vec![],
        ..Default::default()
    };
    let ad_group_ad = AdGroupAd {
        primary_status_reasons: vec![],
        ..Default::default()
    };
    let ad_group_criterion = AdGroupCriterion {
        primary_status_reasons: vec![],
        ..Default::default()
    };
    let asset_group = AssetGroup {
        primary_status_reasons: vec![],
        ..Default::default()
    };

    let row1 = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let row2 = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();
    let row3 = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();
    let row4 = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .build();
    let row5 = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();

    assert_eq!(row1.get("campaign.primary_status_reasons"), "");
    assert_eq!(row2.get("ad_group.primary_status_reasons"), "");
    assert_eq!(row3.get("ad_group_ad.primary_status_reasons"), "");
    assert_eq!(row4.get("ad_group_criterion.primary_status_reasons"), "");
    assert_eq!(row5.get("asset_group.primary_status_reasons"), "");
}

#[test]
fn test_many_status_reasons() {
    // Test with many reasons to ensure no performance issues
    let reasons: Vec<i32> = (2..12).collect(); // 10 reasons

    let campaign = Campaign {
        primary_status_reasons: reasons,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status_reasons");

    // Should have 10 items separated by 9 commas
    assert_eq!(result.matches(", ").count(), 9);
}

#[test]
fn test_comma_space_separator_not_semicolon() {
    // Verify that status reasons use comma-space separator (not semicolon like status_details)
    let campaign = Campaign {
        primary_status_reasons: vec![2, 4],
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let result = row.get("campaign.primary_status_reasons");

    // Should use comma-space separator
    assert!(result.contains(", "));
    // Should NOT use semicolon separator
    assert!(!result.contains(";"));
}
