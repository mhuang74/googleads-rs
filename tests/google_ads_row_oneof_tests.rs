// Unit tests for GoogleAdsRow::get() method - Oneof Fields
//
// This module tests the enum_match_str! and optional_enum_match_str! macros
// which are used for extracting fields from oneof union types

mod test_helpers;

use test_helpers::{
    GoogleAdsRowBuilder, AdGroupCriterionBuilder, CampaignCriterionBuilder,
    AdBuilder, AdGroupAdBuilder,
};

// ============================================================================
// AdGroupCriterion Oneof Fields (enum_match_str!)
// ============================================================================

#[test]
fn test_ad_group_criterion_keyword_text() {
    let criterion = AdGroupCriterionBuilder::new()
        .criterion_id(12345)
        .with_keyword("brand shoes", 2) // 2 = EXACT match type
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    assert_eq!(row.get("ad_group_criterion.keyword.text"), "brand shoes");
}

#[test]
fn test_ad_group_criterion_keyword_match_type() {
    let criterion = AdGroupCriterionBuilder::new()
        .criterion_id(12345)
        .with_keyword("running shoes", 3) // 3 = PHRASE match type
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    // Match type is returned as numeric string
    assert_eq!(row.get("ad_group_criterion.keyword.match_type"), "3");
}

#[test]
fn test_ad_group_criterion_keyword_broad_match() {
    let criterion = AdGroupCriterionBuilder::new()
        .with_keyword("shoes", 4) // 4 = BROAD match type
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    assert_eq!(row.get("ad_group_criterion.keyword.text"), "shoes");
    assert_eq!(row.get("ad_group_criterion.keyword.match_type"), "4");
}

#[test]
fn test_ad_group_criterion_with_special_characters() {
    let criterion = AdGroupCriterionBuilder::new()
        .with_keyword("[brand \"shoes\"]", 2)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    assert_eq!(row.get("ad_group_criterion.keyword.text"), "[brand \"shoes\"]");
}

#[test]
fn test_ad_group_criterion_with_unicode() {
    let criterion = AdGroupCriterionBuilder::new()
        .with_keyword("café ☕", 2)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    assert_eq!(row.get("ad_group_criterion.keyword.text"), "café ☕");
}

#[test]
fn test_ad_group_criterion_empty_keyword() {
    let criterion = AdGroupCriterionBuilder::new()
        .with_keyword("", 2)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    assert_eq!(row.get("ad_group_criterion.keyword.text"), "");
}

// ============================================================================
// CampaignCriterion Oneof Fields (optional_enum_match_str!)
// ============================================================================

#[test]
fn test_campaign_criterion_keyword_text() {
    let criterion = CampaignCriterionBuilder::new()
        .criterion_id(54321)
        .with_keyword("negative keyword")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(criterion)
        .build();

    assert_eq!(row.get("campaign_criterion.keyword.text"), "negative keyword");
}

#[test]
fn test_campaign_criterion_location_geo_target() {
    let criterion = CampaignCriterionBuilder::new()
        .criterion_id(99999)
        .with_location("geoTargetConstants/2840") // United States
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(criterion)
        .build();

    assert_eq!(row.get("campaign_criterion.location.geo_target_constant"), "geoTargetConstants/2840");
}

#[test]
fn test_campaign_criterion_absent() {
    // Test optional_enum_match_str! when parent is absent
    let row = GoogleAdsRowBuilder::new().build();

    assert_eq!(row.get("campaign_criterion.keyword.text"), "");
    assert_eq!(row.get("campaign_criterion.location.geo_target_constant"), "");
}

#[test]
fn test_campaign_criterion_with_different_variant() {
    // Test accessing location when keyword variant is set
    let criterion = CampaignCriterionBuilder::new()
        .with_keyword("test")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(criterion)
        .build();

    // Accessing location fields when keyword is set should return empty
    assert_eq!(row.get("campaign_criterion.location.geo_target_constant"), "");
}

#[test]
fn test_campaign_criterion_location_with_keyword_variant() {
    // Test accessing keyword when location variant is set
    let criterion = CampaignCriterionBuilder::new()
        .with_location("geoTargetConstants/1234")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(criterion)
        .build();

    // Accessing keyword fields when location is set should return empty
    assert_eq!(row.get("campaign_criterion.keyword.text"), "");
}

// ============================================================================
// Ad oneof Fields - ResponsiveSearchAd
// ============================================================================

#[test]
fn test_ad_responsive_search_ad_path1() {
    let ad = AdBuilder::new()
        .id(111)
        .with_responsive_search_ad(
            vec!["Headline 1", "Headline 2"],
            vec!["Description 1"],
            Some("path1"),
            Some("path2"),
        )
        .build();

    let ad_group_ad = AdGroupAdBuilder::new()
        .with_ad(ad)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();

    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.path1"), "path1");
}

#[test]
fn test_ad_responsive_search_ad_path2() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline"],
            vec!["Description"],
            Some("sale"),
            Some("2024"),
        )
        .build();

    let ad_group_ad = AdGroupAdBuilder::new()
        .with_ad(ad)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();

    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.path2"), "2024");
}

#[test]
fn test_ad_responsive_search_ad_no_paths() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline"],
            vec!["Description"],
            None,
            None,
        )
        .build();

    let ad_group_ad = AdGroupAdBuilder::new()
        .with_ad(ad)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();

    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.path1"), "");
    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.path2"), "");
}

// ============================================================================
// Complex Scenarios
// ============================================================================

#[test]
fn test_multiple_oneof_fields_in_row() {
    let ad_group_criterion = AdGroupCriterionBuilder::new()
        .criterion_id(123)
        .with_keyword("keyword1", 2)
        .build();

    let campaign_criterion = CampaignCriterionBuilder::new()
        .criterion_id(456)
        .with_location("geoTargetConstants/2840")
        .build();

    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline"],
            vec!["Description"],
            Some("offers"),
            None,
        )
        .build();

    let ad_group_ad = AdGroupAdBuilder::new()
        .with_ad(ad)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(ad_group_criterion)
        .with_campaign_criterion(campaign_criterion)
        .with_ad_group_ad(ad_group_ad)
        .build();

    assert_eq!(row.get("ad_group_criterion.keyword.text"), "keyword1");
    assert_eq!(row.get("campaign_criterion.location.geo_target_constant"), "geoTargetConstants/2840");
    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.path1"), "offers");
}

#[test]
fn test_oneof_with_scalar_fields() {
    let criterion = AdGroupCriterionBuilder::new()
        .criterion_id(789)
        .cpc_bid_micros(5000000)
        .with_keyword("test keyword", 2)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    // Test both oneof and scalar fields
    assert_eq!(row.get("ad_group_criterion.criterion_id"), "789");
    assert_eq!(row.get("ad_group_criterion.cpc_bid_micros"), "5000000");
    assert_eq!(row.get("ad_group_criterion.keyword.text"), "test keyword");
}

#[test]
fn test_campaign_criterion_with_display_name() {
    let criterion = CampaignCriterionBuilder::new()
        .criterion_id(111)
        .display_name("New York")
        .with_location("geoTargetConstants/1023191")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(criterion)
        .build();

    assert_eq!(row.get("campaign_criterion.criterion_id"), "111");
    assert_eq!(row.get("campaign_criterion.display_name"), "New York");
    assert_eq!(row.get("campaign_criterion.location.geo_target_constant"), "geoTargetConstants/1023191");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_ad_group_criterion_no_oneof_set() {
    // Create criterion without setting the oneof field
    let criterion = AdGroupCriterionBuilder::new()
        .criterion_id(999)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    // Should return empty string when oneof is not set
    assert_eq!(row.get("ad_group_criterion.keyword.text"), "");
    assert_eq!(row.get("ad_group_criterion.keyword.match_type"), "");
}

#[test]
fn test_very_long_keyword() {
    let long_keyword = "a".repeat(1000);
    let criterion = AdGroupCriterionBuilder::new()
        .with_keyword(&long_keyword, 2)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    assert_eq!(row.get("ad_group_criterion.keyword.text"), long_keyword);
}

#[test]
fn test_keyword_with_newlines() {
    let criterion = AdGroupCriterionBuilder::new()
        .with_keyword("line1\nline2\nline3", 2)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_criterion(criterion)
        .build();

    assert_eq!(row.get("ad_group_criterion.keyword.text"), "line1\nline2\nline3");
}

#[test]
fn test_geo_target_with_various_formats() {
    let test_cases = vec![
        "geoTargetConstants/2840",
        "geoTargetConstants/1023191",
        "geoTargetConstants/9999999",
    ];

    for geo_target in test_cases {
        let criterion = CampaignCriterionBuilder::new()
            .with_location(geo_target)
            .build();

        let row = GoogleAdsRowBuilder::new()
            .with_campaign_criterion(criterion)
            .build();

        assert_eq!(row.get("campaign_criterion.location.geo_target_constant"), geo_target);
    }
}
