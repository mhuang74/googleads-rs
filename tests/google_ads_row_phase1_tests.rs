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

mod test_helpers;

use googleads_rs::google::ads::googleads::v19::resources::{
    ConversionAction, ConversionCustomVariable, Asset, CampaignAsset,
    AdGroupAsset, CustomerAsset, UserList, GeoTargetConstant,
};
use googleads_rs::google::ads::googleads::v19::resources::conversion_action::ValueSettings;
use test_helpers::GoogleAdsRowBuilder;

// ============================================================================
// ConversionAction Fields (attr_str! and method_str!)
// ============================================================================

#[test]
fn test_conversion_action_id() {
    let mut conversion_action = ConversionAction::default();
    conversion_action.id = 123456789;

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.id"), "123456789");
}

#[test]
fn test_conversion_action_name() {
    let mut conversion_action = ConversionAction::default();
    conversion_action.name = "Purchase".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.name"), "Purchase");
}

#[test]
fn test_conversion_action_resource_name() {
    let mut conversion_action = ConversionAction::default();
    conversion_action.resource_name = "customers/123/conversionActions/456".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.resource_name"), "customers/123/conversionActions/456");
}

#[test]
fn test_conversion_action_status() {
    use googleads_rs::google::ads::googleads::v19::enums::conversion_action_status_enum::ConversionActionStatus;

    let mut conversion_action = ConversionAction::default();
    conversion_action.status = ConversionActionStatus::Enabled as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.status"), "Enabled");
}

#[test]
fn test_conversion_action_include_in_conversions_metric() {
    let mut conversion_action = ConversionAction::default();
    conversion_action.include_in_conversions_metric = true;

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.include_in_conversions_metric"), "true");
}

#[test]
fn test_conversion_action_value_settings_default_value() {
    let mut conversion_action = ConversionAction::default();
    let mut value_settings = ValueSettings::default();
    value_settings.default_value = 49.99;
    conversion_action.value_settings = Some(value_settings);

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.value_settings.default_value"), "49.99");
}

#[test]
fn test_conversion_action_value_settings_default_currency_code() {
    let mut conversion_action = ConversionAction::default();
    let mut value_settings = ValueSettings::default();
    value_settings.default_currency_code = "USD".to_string();
    conversion_action.value_settings = Some(value_settings);

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.value_settings.default_currency_code"), "USD");
}

#[test]
fn test_conversion_action_type() {
    use googleads_rs::google::ads::googleads::v19::enums::conversion_action_type_enum::ConversionActionType;

    let mut conversion_action = ConversionAction::default();
    conversion_action.r#type = ConversionActionType::Webpage as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.type"), "Webpage");
}

#[test]
fn test_conversion_action_category() {
    use googleads_rs::google::ads::googleads::v19::enums::conversion_action_category_enum::ConversionActionCategory;

    let mut conversion_action = ConversionAction::default();
    conversion_action.category = ConversionActionCategory::Purchase as i32;

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
    let mut variable = ConversionCustomVariable::default();
    variable.id = 987654321;

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_custom_variable(variable)
        .build();

    assert_eq!(row.get("conversion_custom_variable.id"), "987654321");
}

#[test]
fn test_conversion_custom_variable_name() {
    let mut variable = ConversionCustomVariable::default();
    variable.name = "transaction_id".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_custom_variable(variable)
        .build();

    assert_eq!(row.get("conversion_custom_variable.name"), "transaction_id");
}

#[test]
fn test_conversion_custom_variable_tag() {
    let mut variable = ConversionCustomVariable::default();
    variable.tag = "txn_id".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_custom_variable(variable)
        .build();

    assert_eq!(row.get("conversion_custom_variable.tag"), "txn_id");
}

#[test]
fn test_conversion_custom_variable_resource_name() {
    let mut variable = ConversionCustomVariable::default();
    variable.resource_name = "customers/123/conversionCustomVariables/456".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_custom_variable(variable)
        .build();

    assert_eq!(row.get("conversion_custom_variable.resource_name"), "customers/123/conversionCustomVariables/456");
}

// ============================================================================
// Asset Fields
// ============================================================================

#[test]
fn test_asset_id() {
    let mut asset = Asset::default();
    asset.id = 111222333;

    let row = GoogleAdsRowBuilder::new()
        .with_asset(asset)
        .build();

    assert_eq!(row.get("asset.id"), "111222333");
}

#[test]
fn test_asset_name() {
    let mut asset = Asset::default();
    asset.name = "Summer Sale Image".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_asset(asset)
        .build();

    assert_eq!(row.get("asset.name"), "Summer Sale Image");
}

#[test]
fn test_asset_resource_name() {
    let mut asset = Asset::default();
    asset.resource_name = "customers/123/assets/456".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_asset(asset)
        .build();

    assert_eq!(row.get("asset.resource_name"), "customers/123/assets/456");
}

#[test]
fn test_asset_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_type_enum::AssetType;

    let mut asset = Asset::default();
    asset.r#type = AssetType::Image as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_asset(asset)
        .build();

    assert_eq!(row.get("asset.type"), "Image");
}

#[test]
fn test_asset_tracking_url_template() {
    let mut asset = Asset::default();
    asset.tracking_url_template = "https://example.com/track?id={lpurl}".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_asset(asset)
        .build();

    assert_eq!(row.get("asset.tracking_url_template"), "https://example.com/track?id={lpurl}");
}

#[test]
fn test_asset_policy_summary_approval_status() {
    use googleads_rs::google::ads::googleads::v19::enums::policy_approval_status_enum::PolicyApprovalStatus;
    use googleads_rs::google::ads::googleads::v19::resources::AssetPolicySummary;

    let mut asset = Asset::default();
    let mut policy_summary = AssetPolicySummary::default();
    policy_summary.approval_status = PolicyApprovalStatus::Approved as i32;
    asset.policy_summary = Some(policy_summary);

    let row = GoogleAdsRowBuilder::new()
        .with_asset(asset)
        .build();

    assert_eq!(row.get("asset.policy_summary.approval_status"), "Approved");
}

#[test]
fn test_asset_policy_summary_review_status() {
    use googleads_rs::google::ads::googleads::v19::enums::policy_review_status_enum::PolicyReviewStatus;
    use googleads_rs::google::ads::googleads::v19::resources::AssetPolicySummary;

    let mut asset = Asset::default();
    let mut policy_summary = AssetPolicySummary::default();
    policy_summary.review_status = PolicyReviewStatus::Reviewed as i32;
    asset.policy_summary = Some(policy_summary);

    let row = GoogleAdsRowBuilder::new()
        .with_asset(asset)
        .build();

    assert_eq!(row.get("asset.policy_summary.review_status"), "Reviewed");
}

// ============================================================================
// CampaignAsset Fields
// ============================================================================

#[test]
fn test_campaign_asset_resource_name() {
    let mut campaign_asset = CampaignAsset::default();
    campaign_asset.resource_name = "customers/123/campaignAssets/456~789~HEADLINE".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(row.get("campaign_asset.resource_name"), "customers/123/campaignAssets/456~789~HEADLINE");
}

#[test]
fn test_campaign_asset_campaign() {
    let mut campaign_asset = CampaignAsset::default();
    campaign_asset.campaign = "customers/123/campaigns/456".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(row.get("campaign_asset.campaign"), "customers/123/campaigns/456");
}

#[test]
fn test_campaign_asset_asset() {
    let mut campaign_asset = CampaignAsset::default();
    campaign_asset.asset = "customers/123/assets/789".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(row.get("campaign_asset.asset"), "customers/123/assets/789");
}

#[test]
fn test_campaign_asset_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let mut campaign_asset = CampaignAsset::default();
    campaign_asset.field_type = AssetFieldType::Headline as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_asset(campaign_asset)
        .build();

    assert_eq!(row.get("campaign_asset.field_type"), "Headline");
}

#[test]
fn test_campaign_asset_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_status_enum::AssetLinkStatus;

    let mut campaign_asset = CampaignAsset::default();
    campaign_asset.status = AssetLinkStatus::Enabled as i32;

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
    let mut ad_group_asset = AdGroupAsset::default();
    ad_group_asset.resource_name = "customers/123/adGroupAssets/456~789~HEADLINE".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.resource_name"), "customers/123/adGroupAssets/456~789~HEADLINE");
}

#[test]
fn test_ad_group_asset_ad_group() {
    let mut ad_group_asset = AdGroupAsset::default();
    ad_group_asset.ad_group = "customers/123/adGroups/456".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.ad_group"), "customers/123/adGroups/456");
}

#[test]
fn test_ad_group_asset_asset() {
    let mut ad_group_asset = AdGroupAsset::default();
    ad_group_asset.asset = "customers/123/assets/789".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.asset"), "customers/123/assets/789");
}

#[test]
fn test_ad_group_asset_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let mut ad_group_asset = AdGroupAsset::default();
    ad_group_asset.field_type = AssetFieldType::Description as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.field_type"), "Description");
}

#[test]
fn test_ad_group_asset_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_status_enum::AssetLinkStatus;

    let mut ad_group_asset = AdGroupAsset::default();
    ad_group_asset.status = AssetLinkStatus::Paused as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_asset(ad_group_asset)
        .build();

    assert_eq!(row.get("ad_group_asset.status"), "Paused");
}

#[test]
fn test_ad_group_asset_primary_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_primary_status_enum::AssetLinkPrimaryStatus;

    let mut ad_group_asset = AdGroupAsset::default();
    ad_group_asset.primary_status = AssetLinkPrimaryStatus::Eligible as i32;

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
    let mut customer_asset = CustomerAsset::default();
    customer_asset.resource_name = "customers/123/customerAssets/789~SITELINK".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    assert_eq!(row.get("customer_asset.resource_name"), "customers/123/customerAssets/789~SITELINK");
}

#[test]
fn test_customer_asset_asset() {
    let mut customer_asset = CustomerAsset::default();
    customer_asset.asset = "customers/123/assets/789".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    assert_eq!(row.get("customer_asset.asset"), "customers/123/assets/789");
}

#[test]
fn test_customer_asset_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let mut customer_asset = CustomerAsset::default();
    customer_asset.field_type = AssetFieldType::Sitelink as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_customer_asset(customer_asset)
        .build();

    assert_eq!(row.get("customer_asset.field_type"), "Sitelink");
}

#[test]
fn test_customer_asset_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_status_enum::AssetLinkStatus;

    let mut customer_asset = CustomerAsset::default();
    customer_asset.status = AssetLinkStatus::Enabled as i32;

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
    let mut user_list = UserList::default();
    user_list.id = 555666777;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.id"), "555666777");
}

#[test]
fn test_user_list_name() {
    let mut user_list = UserList::default();
    user_list.name = "Website Visitors - Last 30 Days".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.name"), "Website Visitors - Last 30 Days");
}

#[test]
fn test_user_list_description() {
    let mut user_list = UserList::default();
    user_list.description = "Users who visited the website in the last 30 days".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.description"), "Users who visited the website in the last 30 days");
}

#[test]
fn test_user_list_resource_name() {
    let mut user_list = UserList::default();
    user_list.resource_name = "customers/123/userLists/456".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.resource_name"), "customers/123/userLists/456");
}

#[test]
fn test_user_list_membership_life_span() {
    let mut user_list = UserList::default();
    user_list.membership_life_span = 30;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.membership_life_span"), "30");
}

#[test]
fn test_user_list_size_for_display() {
    let mut user_list = UserList::default();
    user_list.size_for_display = 15000;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.size_for_display"), "15000");
}

#[test]
fn test_user_list_size_for_search() {
    let mut user_list = UserList::default();
    user_list.size_for_search = 12000;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.size_for_search"), "12000");
}

#[test]
fn test_user_list_match_rate_percentage() {
    let mut user_list = UserList::default();
    user_list.match_rate_percentage = 85;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.match_rate_percentage"), "85");
}

#[test]
fn test_user_list_read_only() {
    let mut user_list = UserList::default();
    user_list.read_only = true;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.read_only"), "true");
}

#[test]
fn test_user_list_eligible_for_search() {
    let mut user_list = UserList::default();
    user_list.eligible_for_search = true;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.eligible_for_search"), "true");
}

#[test]
fn test_user_list_eligible_for_display() {
    let mut user_list = UserList::default();
    user_list.eligible_for_display = true;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.eligible_for_display"), "true");
}

#[test]
fn test_user_list_membership_status() {
    use googleads_rs::google::ads::googleads::v19::enums::user_list_membership_status_enum::UserListMembershipStatus;

    let mut user_list = UserList::default();
    user_list.membership_status = UserListMembershipStatus::Open as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.membership_status"), "Open");
}

#[test]
fn test_user_list_size_range_for_display() {
    use googleads_rs::google::ads::googleads::v19::enums::user_list_size_range_enum::UserListSizeRange;

    let mut user_list = UserList::default();
    user_list.size_range_for_display = UserListSizeRange::TenThousandToFiftyThousand as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.size_range_for_display"), "TenThousandToFiftyThousand");
}

#[test]
fn test_user_list_type() {
    use googleads_rs::google::ads::googleads::v19::enums::user_list_type_enum::UserListType;

    let mut user_list = UserList::default();
    user_list.r#type = UserListType::Remarketing as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.type"), "Remarketing");
}

// ============================================================================
// GeoTargetConstant Fields
// ============================================================================

#[test]
fn test_geo_target_constant_id() {
    let mut geo_target = GeoTargetConstant::default();
    geo_target.id = 1023191;

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.id"), "1023191");
}

#[test]
fn test_geo_target_constant_name() {
    let mut geo_target = GeoTargetConstant::default();
    geo_target.name = "New York,NY,United States".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.name"), "New York,NY,United States");
}

#[test]
fn test_geo_target_constant_canonical_name() {
    let mut geo_target = GeoTargetConstant::default();
    geo_target.canonical_name = "New York,New York,United States".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.canonical_name"), "New York,New York,United States");
}

#[test]
fn test_geo_target_constant_country_code() {
    let mut geo_target = GeoTargetConstant::default();
    geo_target.country_code = "US".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.country_code"), "US");
}

#[test]
fn test_geo_target_constant_target_type() {
    let mut geo_target = GeoTargetConstant::default();
    geo_target.target_type = "City".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.target_type"), "City");
}

#[test]
fn test_geo_target_constant_resource_name() {
    let mut geo_target = GeoTargetConstant::default();
    geo_target.resource_name = "geoTargetConstants/1023191".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_geo_target_constant(geo_target)
        .build();

    assert_eq!(row.get("geo_target_constant.resource_name"), "geoTargetConstants/1023191");
}

// ============================================================================
// Edge Cases and Multiple Resources
// ============================================================================

#[test]
fn test_multiple_phase1_resources_in_same_row() {
    let mut conversion_action = ConversionAction::default();
    conversion_action.id = 111;
    conversion_action.name = "Purchase".to_string();

    let mut user_list = UserList::default();
    user_list.id = 222;
    user_list.name = "Buyers".to_string();

    let mut geo_target = GeoTargetConstant::default();
    geo_target.id = 333;
    geo_target.name = "California".to_string();

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
    let mut asset = Asset::default();
    asset.name = "".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_asset(asset)
        .build();

    assert_eq!(row.get("asset.name"), "");
}

#[test]
fn test_special_characters_in_phase1_names() {
    let mut conversion_action = ConversionAction::default();
    conversion_action.name = "Purchase: Q4 2024 - \"Special\" (Test)".to_string();

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.name"), "Purchase: Q4 2024 - \"Special\" (Test)");
}

#[test]
fn test_zero_values_phase1() {
    let mut user_list = UserList::default();
    user_list.id = 0;
    user_list.size_for_display = 0;

    let row = GoogleAdsRowBuilder::new()
        .with_user_list(user_list)
        .build();

    assert_eq!(row.get("user_list.id"), "0");
    assert_eq!(row.get("user_list.size_for_display"), "0");
}

#[test]
fn test_very_large_numbers_phase1() {
    let mut conversion_action = ConversionAction::default();
    conversion_action.id = 9_223_372_036_854_775_807; // i64::MAX

    let row = GoogleAdsRowBuilder::new()
        .with_conversion_action(conversion_action)
        .build();

    assert_eq!(row.get("conversion_action.id"), "9223372036854775807");
}
