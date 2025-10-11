// Unit tests for GoogleAdsRow::get() method - Phase 1 Fields
//
// This module tests the newly implemented Phase 1 match arms for:
// - conversion_action
// - conversion_custom_variable
// - asset
// - campaign_asset
// - ad_group_asset
// - customer_asset
// - user_list
// - geo_target_constant

#![allow(clippy::field_reassign_with_default)]

mod test_helpers;

use googleads_rs::google::ads::googleads::v19::resources::conversion_action::ValueSettings;
use googleads_rs::google::ads::googleads::v19::resources::{
    AdGroupAsset, Asset, CampaignAsset, ConversionAction, ConversionCustomVariable, CustomerAsset,
    GeoTargetConstant, UserList,
};
use test_helpers::GoogleAdsRowBuilder;

// ============================================================================
// ConversionAction Fields (attr_str! and method_str!)
// ============================================================================

#[test]
fn test_conversion_action_id() {
    let conversion_action = ConversionAction {
        id: 123456789,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.id"), "123456789");
}

#[test]
fn test_conversion_action_name() {
    let conversion_action = ConversionAction {
        name: "Purchase".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.name"), "Purchase");
}

#[test]
fn test_conversion_action_resource_name() {
    let conversion_action = ConversionAction {
        resource_name: "customers/123/conversionActions/456".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(
        row.get("conversion_action.resource_name"),
        "customers/123/conversionActions/456"
    );
}

#[test]
fn test_conversion_action_status() {
    use googleads_rs::google::ads::googleads::v19::enums::conversion_action_status_enum::ConversionActionStatus;

    let conversion_action = ConversionAction {
        status: ConversionActionStatus::Enabled as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.status"), "Enabled");
}

#[test]
fn test_conversion_action_include_in_conversions_metric() {
    let conversion_action = ConversionAction {
        include_in_conversions_metric: true,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(
        row.get("conversion_action.include_in_conversions_metric"),
        "true"
    );
}

#[test]
fn test_conversion_action_value_settings_default_value() {
    let conversion_action = ConversionAction {
        value_settings: Some(ValueSettings {
            default_value: 49.99,
            ..Default::default()
        }),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(
        row.get("conversion_action.value_settings.default_value"),
        "49.99"
    );
}

#[test]
fn test_conversion_action_value_settings_default_currency_code() {
    let conversion_action = ConversionAction {
        value_settings: Some(ValueSettings {
            default_currency_code: "USD".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(
        row.get("conversion_action.value_settings.default_currency_code"),
        "USD"
    );
}

#[test]
fn test_conversion_action_type() {
    use googleads_rs::google::ads::googleads::v19::enums::conversion_action_type_enum::ConversionActionType;

    let conversion_action = ConversionAction {
        r#type: ConversionActionType::Webpage as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.type"), "Webpage");
}

#[test]
fn test_conversion_action_category() {
    use googleads_rs::google::ads::googleads::v19::enums::conversion_action_category_enum::ConversionActionCategory;

    let conversion_action = ConversionAction {
        category: ConversionActionCategory::Purchase as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.category"), "Purchase");
}

// ============================================================================
// ConversionCustomVariable Fields
// ============================================================================

#[test]
fn test_conversion_custom_variable_id() {
    let variable = ConversionCustomVariable {
        id: 987654321,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_custom_variable(variable)
        .build();

    assert_eq!(row.get("conversion_custom_variable.id"), "987654321");
}

#[test]
fn test_conversion_custom_variable_name() {
    let variable = ConversionCustomVariable {
        name: "transaction_id".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_custom_variable(variable)
        .build();

    assert_eq!(row.get("conversion_custom_variable.name"), "transaction_id");
}

#[test]
fn test_conversion_custom_variable_tag() {
    let variable = ConversionCustomVariable {
        tag: "txn_id".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_custom_variable(variable)
        .build();

    assert_eq!(row.get("conversion_custom_variable.tag"), "txn_id");
}

#[test]
fn test_conversion_custom_variable_resource_name() {
    let variable = ConversionCustomVariable {
        resource_name: "customers/123/conversionCustomVariables/456".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_custom_variable(variable)
        .build();

    assert_eq!(
        row.get("conversion_custom_variable.resource_name"),
        "customers/123/conversionCustomVariables/456"
    );
}

// ============================================================================
// Asset Fields
// ============================================================================

#[test]
fn test_asset_id() {
    let asset = Asset {
        id: 111222333,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();

    assert_eq!(row.get("asset.id"), "111222333");
}

#[test]
fn test_asset_name() {
    let asset = Asset {
        name: "Summer Sale Image".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();

    assert_eq!(row.get("asset.name"), "Summer Sale Image");
}

#[test]
fn test_asset_resource_name() {
    let asset = Asset {
        resource_name: "customers/123/assets/456".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();

    assert_eq!(row.get("asset.resource_name"), "customers/123/assets/456");
}

#[test]
fn test_asset_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_type_enum::AssetType;

    let asset = Asset {
        r#type: AssetType::Image as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();

    assert_eq!(row.get("asset.type"), "Image");
}

#[test]
fn test_asset_tracking_url_template() {
    let asset = Asset {
        tracking_url_template: "https://example.com/track?id={lpurl}".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();

    assert_eq!(
        row.get("asset.tracking_url_template"),
        "https://example.com/track?id={lpurl}"
    );
}

#[test]
fn test_asset_policy_summary_approval_status() {
    use googleads_rs::google::ads::googleads::v19::enums::policy_approval_status_enum::PolicyApprovalStatus;
    use googleads_rs::google::ads::googleads::v19::resources::AssetPolicySummary;

    let asset = Asset {
        policy_summary: Some(AssetPolicySummary {
            approval_status: PolicyApprovalStatus::Approved as i32,
            ..Default::default()
        }),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();

    assert_eq!(row.get("asset.policy_summary.approval_status"), "Approved");
}

#[test]
fn test_asset_policy_summary_review_status() {
    use googleads_rs::google::ads::googleads::v19::enums::policy_review_status_enum::PolicyReviewStatus;
    use googleads_rs::google::ads::googleads::v19::resources::AssetPolicySummary;

    let asset = Asset {
        policy_summary: Some(AssetPolicySummary {
            review_status: PolicyReviewStatus::Reviewed as i32,
            ..Default::default()
        }),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();

    assert_eq!(row.get("asset.policy_summary.review_status"), "Reviewed");
}

// ============================================================================
// CampaignAsset Fields
// ============================================================================

#[test]
fn test_campaign_asset_resource_name() {
    let campaign_asset = CampaignAsset {
        resource_name: "customers/123/campaignAssets/456~789~HEADLINE".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(
        row.get("campaign_asset.resource_name"),
        "customers/123/campaignAssets/456~789~HEADLINE"
    );
}

#[test]
fn test_campaign_asset_campaign() {
    let campaign_asset = CampaignAsset {
        campaign: "customers/123/campaigns/456".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(
        row.get("campaign_asset.campaign"),
        "customers/123/campaigns/456"
    );
}

#[test]
fn test_campaign_asset_asset() {
    let campaign_asset = CampaignAsset {
        asset: "customers/123/assets/789".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(row.get("campaign_asset.asset"), "customers/123/assets/789");
}

#[test]
fn test_campaign_asset_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let campaign_asset = CampaignAsset {
        field_type: AssetFieldType::Headline as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(row.get("campaign_asset.field_type"), "Headline");
}

#[test]
fn test_campaign_asset_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_status_enum::AssetLinkStatus;

    let campaign_asset = CampaignAsset {
        status: AssetLinkStatus::Enabled as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(row.get("campaign_asset.status"), "Enabled");
}

// ============================================================================
// AdGroupAsset Fields
// ============================================================================

#[test]
fn test_ad_group_asset_resource_name() {
    let ad_group_asset = AdGroupAsset {
        resource_name: "customers/123/adGroupAssets/456~789~HEADLINE".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(
        row.get("ad_group_asset.resource_name"),
        "customers/123/adGroupAssets/456~789~HEADLINE"
    );
}

#[test]
fn test_ad_group_asset_ad_group() {
    let ad_group_asset = AdGroupAsset {
        ad_group: "customers/123/adGroups/456".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(
        row.get("ad_group_asset.ad_group"),
        "customers/123/adGroups/456"
    );
}

#[test]
fn test_ad_group_asset_asset() {
    let ad_group_asset = AdGroupAsset {
        asset: "customers/123/assets/789".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.asset"), "customers/123/assets/789");
}

#[test]
fn test_ad_group_asset_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let ad_group_asset = AdGroupAsset {
        field_type: AssetFieldType::Description as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.field_type"), "Description");
}

#[test]
fn test_ad_group_asset_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_status_enum::AssetLinkStatus;

    let ad_group_asset = AdGroupAsset {
        status: AssetLinkStatus::Paused as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.status"), "Paused");
}

#[test]
fn test_ad_group_asset_primary_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_primary_status_enum::AssetLinkPrimaryStatus;

    let ad_group_asset = AdGroupAsset {
        primary_status: AssetLinkPrimaryStatus::Eligible as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.primary_status"), "Eligible");
}

// ============================================================================
// CustomerAsset Fields
// ============================================================================

#[test]
fn test_customer_asset_resource_name() {
    let customer_asset = CustomerAsset {
        resource_name: "customers/123/customerAssets/789~SITELINK".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    assert_eq!(
        row.get("customer_asset.resource_name"),
        "customers/123/customerAssets/789~SITELINK"
    );
}

#[test]
fn test_customer_asset_asset() {
    let customer_asset = CustomerAsset {
        asset: "customers/123/assets/789".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    assert_eq!(row.get("customer_asset.asset"), "customers/123/assets/789");
}

#[test]
fn test_customer_asset_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let customer_asset = CustomerAsset {
        field_type: AssetFieldType::Sitelink as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    assert_eq!(row.get("customer_asset.field_type"), "Sitelink");
}

#[test]
fn test_customer_asset_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_status_enum::AssetLinkStatus;

    let customer_asset = CustomerAsset {
        status: AssetLinkStatus::Enabled as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    assert_eq!(row.get("customer_asset.status"), "Enabled");
}

// ============================================================================
// UserList Fields
// ============================================================================

#[test]
fn test_user_list_id() {
    let user_list = UserList {
        id: 555666777,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.id"), "555666777");
}

#[test]
fn test_user_list_name() {
    let user_list = UserList {
        name: "Website Visitors - Last 30 Days".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.name"), "Website Visitors - Last 30 Days");
}

#[test]
fn test_user_list_description() {
    let user_list = UserList {
        description: "Users who visited the website in the last 30 days".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(
        row.get("user_list.description"),
        "Users who visited the website in the last 30 days"
    );
}

#[test]
fn test_user_list_resource_name() {
    let user_list = UserList {
        resource_name: "customers/123/userLists/456".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(
        row.get("user_list.resource_name"),
        "customers/123/userLists/456"
    );
}

#[test]
fn test_user_list_membership_life_span() {
    let user_list = UserList {
        membership_life_span: 30,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.membership_life_span"), "30");
}

#[test]
fn test_user_list_size_for_display() {
    let user_list = UserList {
        size_for_display: 15000,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.size_for_display"), "15000");
}

#[test]
fn test_user_list_size_for_search() {
    let user_list = UserList {
        size_for_search: 12000,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.size_for_search"), "12000");
}

#[test]
fn test_user_list_match_rate_percentage() {
    let user_list = UserList {
        match_rate_percentage: 85,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.match_rate_percentage"), "85");
}

#[test]
fn test_user_list_read_only() {
    let user_list = UserList {
        read_only: true,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.read_only"), "true");
}

#[test]
fn test_user_list_eligible_for_search() {
    let user_list = UserList {
        eligible_for_search: true,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.eligible_for_search"), "true");
}

#[test]
fn test_user_list_eligible_for_display() {
    let user_list = UserList {
        eligible_for_display: true,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.eligible_for_display"), "true");
}

#[test]
fn test_user_list_membership_status() {
    use googleads_rs::google::ads::googleads::v19::enums::user_list_membership_status_enum::UserListMembershipStatus;

    let user_list = UserList {
        membership_status: UserListMembershipStatus::Open as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.membership_status"), "Open");
}

#[test]
fn test_user_list_size_range_for_display() {
    use googleads_rs::google::ads::googleads::v19::enums::user_list_size_range_enum::UserListSizeRange;

    let user_list = UserList {
        size_range_for_display: UserListSizeRange::TenThousandToFiftyThousand as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(
        row.get("user_list.size_range_for_display"),
        "TenThousandToFiftyThousand"
    );
}

#[test]
fn test_user_list_type() {
    use googleads_rs::google::ads::googleads::v19::enums::user_list_type_enum::UserListType;

    let user_list = UserList {
        r#type: UserListType::Remarketing as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.type"), "Remarketing");
}

// ============================================================================
// GeoTargetConstant Fields
// ============================================================================

#[test]
fn test_geo_target_constant_id() {
    let geo_target = GeoTargetConstant {
        id: 1023191,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.id"), "1023191");
}

#[test]
fn test_geo_target_constant_name() {
    let geo_target = GeoTargetConstant {
        name: "New York,NY,United States".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(
        row.get("geo_target_constant.name"),
        "New York,NY,United States"
    );
}

#[test]
fn test_geo_target_constant_canonical_name() {
    let geo_target = GeoTargetConstant {
        canonical_name: "New York,New York,United States".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(
        row.get("geo_target_constant.canonical_name"),
        "New York,New York,United States"
    );
}

#[test]
fn test_geo_target_constant_country_code() {
    let geo_target = GeoTargetConstant {
        country_code: "US".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.country_code"), "US");
}

#[test]
fn test_geo_target_constant_target_type() {
    let geo_target = GeoTargetConstant {
        target_type: "City".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.target_type"), "City");
}

#[test]
fn test_geo_target_constant_resource_name() {
    let geo_target = GeoTargetConstant {
        resource_name: "geoTargetConstants/1023191".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(
        row.get("geo_target_constant.resource_name"),
        "geoTargetConstants/1023191"
    );
}

// ============================================================================
// Edge Cases and Multiple Resources
// ============================================================================

#[test]
fn test_multiple_phase1_resources_in_same_row() {
    let conversion_action = ConversionAction {
        id: 111,
        name: "Purchase".to_string(),
        ..Default::default()
    };

    let user_list = UserList {
        id: 222,
        name: "Buyers".to_string(),
        ..Default::default()
    };

    let geo_target = GeoTargetConstant {
        id: 333,
        name: "California".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .with_user_list(user_list)
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("conversion_action.id"), "111");
    assert_eq!(row.get("conversion_action.name"), "Purchase");
    assert_eq!(row.get("user_list.id"), "222");
    assert_eq!(row.get("user_list.name"), "Buyers");
    assert_eq!(row.get("geo_target_constant.id"), "333");
    assert_eq!(row.get("geo_target_constant.name"), "California");
}

#[test]
fn test_empty_string_values_phase1() {
    let asset = Asset {
        name: "".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();

    assert_eq!(row.get("asset.name"), "");
}

#[test]
fn test_special_characters_in_phase1_names() {
    let conversion_action = ConversionAction {
        name: "Purchase: Q4 2024 - \"Special\" (Test)".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(
        row.get("conversion_action.name"),
        "Purchase: Q4 2024 - \"Special\" (Test)"
    );
}

#[test]
fn test_zero_values_phase1() {
    let user_list = UserList {
        id: 0,
        size_for_display: 0,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new().with_user_list(user_list).build();

    assert_eq!(row.get("user_list.id"), "0");
    assert_eq!(row.get("user_list.size_for_display"), "0");
}

#[test]
fn test_very_large_numbers_phase1() {
    let conversion_action = ConversionAction {
        id: 9_223_372_036_854_775_807, // i64::MAX
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.id"), "9223372036854775807");
}
