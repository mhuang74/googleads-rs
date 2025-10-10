// Unit tests for GoogleAdsRow::get() method - Repeated Field Iterators
//
// This module tests the enum_match_iterator_str! macro
// which is used for extracting and joining repeated nested fields

mod test_helpers;

use test_helpers::{
    GoogleAdsRowBuilder, AdBuilder, AdGroupAdBuilder,
};

// ============================================================================
// ResponsiveSearchAd Headlines (enum_match_iterator_str!)
// ============================================================================

#[test]
fn test_responsive_search_ad_single_headline() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Best Shoes Online"],
            vec!["Shop Now"],
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

    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.headlines"), "Best Shoes Online");
}

#[test]
fn test_responsive_search_ad_multiple_headlines() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline 1", "Headline 2", "Headline 3"],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        "Headline 1, Headline 2, Headline 3"
    );
}

#[test]
fn test_responsive_search_ad_max_headlines() {
    // Responsive Search Ads can have up to 15 headlines
    let headlines = vec![
        "Headline 1", "Headline 2", "Headline 3", "Headline 4", "Headline 5",
        "Headline 6", "Headline 7", "Headline 8", "Headline 9", "Headline 10",
        "Headline 11", "Headline 12", "Headline 13", "Headline 14", "Headline 15",
    ];

    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            headlines.clone(),
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

    let expected = headlines.join(", ");
    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.headlines"), expected);
}

#[test]
fn test_responsive_search_ad_headlines_with_special_characters() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["50% Off!", "Buy Now - Save Big", "Limited Time \"Offer\""],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        "50% Off!, Buy Now - Save Big, Limited Time \"Offer\""
    );
}

#[test]
fn test_responsive_search_ad_headlines_with_unicode() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Café ☕", "Naïve résumé", "日本語"],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        "Café ☕, Naïve résumé, 日本語"
    );
}

// ============================================================================
// ResponsiveSearchAd Descriptions (enum_match_iterator_str!)
// ============================================================================

#[test]
fn test_responsive_search_ad_single_description() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline"],
            vec!["Free shipping on all orders"],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.descriptions"),
        "Free shipping on all orders"
    );
}

#[test]
fn test_responsive_search_ad_multiple_descriptions() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline"],
            vec!["Description 1", "Description 2", "Description 3", "Description 4"],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.descriptions"),
        "Description 1, Description 2, Description 3, Description 4"
    );
}

#[test]
fn test_responsive_search_ad_long_descriptions() {
    let long_desc1 = "This is a very long description that contains lots of information about the product";
    let long_desc2 = "Another lengthy description with many words to test the handling of longer text";

    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline"],
            vec![long_desc1, long_desc2],
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

    let expected = format!("{}, {}", long_desc1, long_desc2);
    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.descriptions"), expected);
}

// ============================================================================
// Combined Tests - Headlines and Descriptions
// ============================================================================

#[test]
fn test_responsive_search_ad_headlines_and_descriptions() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline 1", "Headline 2"],
            vec!["Description 1", "Description 2"],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        "Headline 1, Headline 2"
    );
    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.descriptions"),
        "Description 1, Description 2"
    );
    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.path1"), "sale");
    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.path2"), "2024");
}

#[test]
fn test_responsive_search_ad_realistic_example() {
    let ad = AdBuilder::new()
        .id(123456)
        .name("My RSA")
        .with_responsive_search_ad(
            vec![
                "Buy Running Shoes",
                "Premium Running Shoes",
                "Best Running Shoes Online",
                "Free Shipping Available",
                "Shop Now - Save 20%",
            ],
            vec![
                "Shop our wide selection of running shoes. Free shipping on orders over $50.",
                "Premium quality running shoes from top brands. Order today!",
            ],
            Some("sale"),
            Some("2024"),
        )
        .build();

    let ad_group_ad = AdGroupAdBuilder::new()
        .status(2) // Enabled
        .with_ad(ad)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();

    assert_eq!(row.get("ad_group_ad.ad.id"), "123456");
    assert_eq!(row.get("ad_group_ad.ad.name"), "My RSA");
    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        "Buy Running Shoes, Premium Running Shoes, Best Running Shoes Online, Free Shipping Available, Shop Now - Save 20%"
    );
    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.descriptions"),
        "Shop our wide selection of running shoes. Free shipping on orders over $50., Premium quality running shoes from top brands. Order today!"
    );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_responsive_search_ad_empty_headlines() {
    // Should not happen in practice, but test empty vector handling
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec![],
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

    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.headlines"), "");
}

#[test]
fn test_responsive_search_ad_empty_descriptions() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Headline"],
            vec![],
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

    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.descriptions"), "");
}

#[test]
fn test_responsive_search_ad_headlines_with_empty_strings() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["", "Valid Headline", ""],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        ", Valid Headline, "
    );
}

#[test]
fn test_responsive_search_ad_headlines_with_commas() {
    // Test that commas in headlines are preserved
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Buy Now, Save Later", "Quality, Price, Service"],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        "Buy Now, Save Later, Quality, Price, Service"
    );
}

#[test]
fn test_responsive_search_ad_without_setting_oneof() {
    // Create an ad without setting the ad_data oneof
    let ad = AdBuilder::new()
        .id(999)
        .build();

    let ad_group_ad = AdGroupAdBuilder::new()
        .with_ad(ad)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad(ad_group_ad)
        .build();

    // Should return empty string when oneof variant is not set
    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.headlines"), "");
    assert_eq!(row.get("ad_group_ad.ad.responsive_search_ad.descriptions"), "");
}

// ============================================================================
// Whitespace and Formatting Tests
// ============================================================================

#[test]
fn test_responsive_search_ad_headlines_with_leading_trailing_spaces() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec![" Leading space", "Trailing space ", " Both "],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        " Leading space, Trailing space ,  Both "
    );
}

#[test]
fn test_responsive_search_ad_headlines_with_newlines() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Line 1\nLine 2", "Normal"],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        "Line 1\nLine 2, Normal"
    );
}

#[test]
fn test_responsive_search_ad_headlines_with_tabs() {
    let ad = AdBuilder::new()
        .with_responsive_search_ad(
            vec!["Tab\there", "Normal"],
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

    assert_eq!(
        row.get("ad_group_ad.ad.responsive_search_ad.headlines"),
        "Tab\there, Normal"
    );
}
