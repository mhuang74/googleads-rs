// Unit tests for GoogleAdsRow::get() method - Nested Message Fields
//
// This module tests nested message field extraction using attr_str! macro
// with chained parent references

mod test_helpers;

use test_helpers::{CampaignBuilder, GoogleAdsRowBuilder};

// ============================================================================
// Campaign.NetworkSettings Nested Fields
// ============================================================================

#[test]
fn test_campaign_network_settings_target_search_network() {
    let campaign = CampaignBuilder::new()
        .with_network_settings(true, false, false, true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_search_network"),
        "true"
    );
}

#[test]
fn test_campaign_network_settings_target_content_network() {
    let campaign = CampaignBuilder::new()
        .with_network_settings(true, true, false, true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_content_network"),
        "true"
    );
}

#[test]
fn test_campaign_network_settings_target_partner_search_network() {
    let campaign = CampaignBuilder::new()
        .with_network_settings(true, false, true, true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_partner_search_network"),
        "true"
    );
}

#[test]
fn test_campaign_network_settings_target_google_search() {
    let campaign = CampaignBuilder::new()
        .with_network_settings(true, false, false, true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_google_search"),
        "true"
    );
}

#[test]
fn test_campaign_network_settings_all_false() {
    let campaign = CampaignBuilder::new()
        .with_network_settings(false, false, false, false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_search_network"),
        "false"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_content_network"),
        "false"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_partner_search_network"),
        "false"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_google_search"),
        "false"
    );
}

#[test]
fn test_campaign_network_settings_all_true() {
    let campaign = CampaignBuilder::new()
        .with_network_settings(true, true, true, true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_search_network"),
        "true"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_content_network"),
        "true"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_partner_search_network"),
        "true"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_google_search"),
        "true"
    );
}

// ============================================================================
// Campaign.DynamicSearchAdsSetting Nested Fields
// ============================================================================

#[test]
fn test_campaign_dynamic_search_ads_domain_name() {
    let campaign = CampaignBuilder::new()
        .with_dynamic_search_ads_setting("www.example.com", "en", false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.domain_name"),
        "www.example.com"
    );
}

#[test]
fn test_campaign_dynamic_search_ads_language_code() {
    let campaign = CampaignBuilder::new()
        .with_dynamic_search_ads_setting("www.example.com", "es", false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.language_code"),
        "es"
    );
}

#[test]
fn test_campaign_dynamic_search_ads_use_supplied_urls_only_true() {
    let campaign = CampaignBuilder::new()
        .with_dynamic_search_ads_setting("www.example.com", "en", true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.use_supplied_urls_only"),
        "true"
    );
}

#[test]
fn test_campaign_dynamic_search_ads_use_supplied_urls_only_false() {
    let campaign = CampaignBuilder::new()
        .with_dynamic_search_ads_setting("www.example.com", "en", false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.use_supplied_urls_only"),
        "false"
    );
}

#[test]
fn test_campaign_dynamic_search_ads_multiple_languages() {
    let test_cases = vec![
        ("en", "English"),
        ("es", "Spanish"),
        ("fr", "French"),
        ("de", "German"),
        ("ja", "Japanese"),
        ("zh", "Chinese"),
    ];

    for (lang_code, _lang_name) in test_cases {
        let campaign = CampaignBuilder::new()
            .with_dynamic_search_ads_setting("www.example.com", lang_code, false)
            .build();

        let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

        assert_eq!(
            row.get("campaign.dynamic_search_ads_setting.language_code"),
            lang_code
        );
    }
}

#[test]
fn test_campaign_dynamic_search_ads_various_domains() {
    let test_domains = vec![
        "www.example.com",
        "shop.example.com",
        "example.co.uk",
        "example.com.au",
        "subdomain.example.org",
    ];

    for domain in test_domains {
        let campaign = CampaignBuilder::new()
            .with_dynamic_search_ads_setting(domain, "en", false)
            .build();

        let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

        assert_eq!(
            row.get("campaign.dynamic_search_ads_setting.domain_name"),
            domain
        );
    }
}

// ============================================================================
// Combined Tests - Multiple Nested Messages
// ============================================================================

#[test]
fn test_campaign_with_both_nested_messages() {
    let campaign = CampaignBuilder::new()
        .id(12345)
        .name("Multi-Network DSA Campaign")
        .with_network_settings(true, true, false, true)
        .with_dynamic_search_ads_setting("www.shop.com", "en", true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Test scalar fields
    assert_eq!(row.get("campaign.id"), "12345");
    assert_eq!(row.get("campaign.name"), "Multi-Network DSA Campaign");

    // Test network settings
    assert_eq!(
        row.get("campaign.network_settings.target_search_network"),
        "true"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_content_network"),
        "true"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_partner_search_network"),
        "false"
    );

    // Test DSA settings
    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.domain_name"),
        "www.shop.com"
    );
    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.language_code"),
        "en"
    );
    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.use_supplied_urls_only"),
        "true"
    );
}

#[test]
fn test_campaign_with_nested_and_enum_fields() {
    use googleads_rs::google::ads::googleads::v19::enums::{
        advertising_channel_type_enum::AdvertisingChannelType, campaign_status_enum::CampaignStatus,
    };

    let campaign = CampaignBuilder::new()
        .id(99999)
        .name("Complex Campaign")
        .status(CampaignStatus::Enabled)
        .advertising_channel_type(AdvertisingChannelType::Search)
        .with_network_settings(true, false, false, true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Scalar fields
    assert_eq!(row.get("campaign.id"), "99999");
    assert_eq!(row.get("campaign.name"), "Complex Campaign");

    // Enum fields
    assert_eq!(row.get("campaign.status"), "Enabled");
    assert_eq!(row.get("campaign.advertising_channel_type"), "Search");

    // Nested fields
    assert_eq!(
        row.get("campaign.network_settings.target_search_network"),
        "true"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_google_search"),
        "true"
    );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_campaign_network_settings_mixed_true_false() {
    // Test various combinations
    let test_cases = vec![
        (true, false, false, false),
        (false, true, false, false),
        (false, false, true, false),
        (false, false, false, true),
        (true, true, false, false),
        (true, false, true, false),
        (true, false, false, true),
    ];

    for (search, content, partner, google) in test_cases {
        let campaign = CampaignBuilder::new()
            .with_network_settings(search, content, partner, google)
            .build();

        let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

        assert_eq!(
            row.get("campaign.network_settings.target_search_network"),
            search.to_string()
        );
        assert_eq!(
            row.get("campaign.network_settings.target_content_network"),
            content.to_string()
        );
        assert_eq!(
            row.get("campaign.network_settings.target_partner_search_network"),
            partner.to_string()
        );
        assert_eq!(
            row.get("campaign.network_settings.target_google_search"),
            google.to_string()
        );
    }
}

#[test]
fn test_campaign_dsa_with_special_domain_characters() {
    let campaign = CampaignBuilder::new()
        .with_dynamic_search_ads_setting("shop-online.example-store.com", "en", false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.domain_name"),
        "shop-online.example-store.com"
    );
}

#[test]
fn test_campaign_dsa_empty_domain() {
    let campaign = CampaignBuilder::new()
        .with_dynamic_search_ads_setting("", "en", false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.domain_name"),
        ""
    );
}

#[test]
fn test_campaign_dsa_empty_language_code() {
    let campaign = CampaignBuilder::new()
        .with_dynamic_search_ads_setting("www.example.com", "", false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.language_code"),
        ""
    );
}

// ============================================================================
// Realistic Scenarios
// ============================================================================

#[test]
fn test_search_campaign_network_settings() {
    // Typical Search campaign: search network + Google search, no content network
    let campaign = CampaignBuilder::new()
        .id(111111)
        .name("Search Campaign - Brand")
        .with_network_settings(true, false, false, true)
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
    assert_eq!(
        row.get("campaign.network_settings.target_partner_search_network"),
        "false"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_google_search"),
        "true"
    );
}

#[test]
fn test_display_campaign_network_settings() {
    // Typical Display campaign: content network only
    let campaign = CampaignBuilder::new()
        .id(222222)
        .name("Display Campaign - Remarketing")
        .with_network_settings(false, true, false, false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_search_network"),
        "false"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_content_network"),
        "true"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_partner_search_network"),
        "false"
    );
    assert_eq!(
        row.get("campaign.network_settings.target_google_search"),
        "false"
    );
}

#[test]
fn test_dsa_campaign_with_url_restrictions() {
    // DSA campaign with URL restrictions enabled
    let campaign = CampaignBuilder::new()
        .id(333333)
        .name("DSA - Products Only")
        .with_dynamic_search_ads_setting("www.store.com", "en", true)
        .with_network_settings(true, false, false, true)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.id"), "333333");
    assert_eq!(row.get("campaign.name"), "DSA - Products Only");
    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.domain_name"),
        "www.store.com"
    );
    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.language_code"),
        "en"
    );
    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.use_supplied_urls_only"),
        "true"
    );
}

#[test]
fn test_dsa_campaign_without_url_restrictions() {
    // DSA campaign crawling entire website
    let campaign = CampaignBuilder::new()
        .id(444444)
        .name("DSA - Full Site")
        .with_dynamic_search_ads_setting("www.company.com", "en", false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.dynamic_search_ads_setting.use_supplied_urls_only"),
        "false"
    );
}
