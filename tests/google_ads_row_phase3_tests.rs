// Unit tests for GoogleAdsRow::get() method - Phase 3 Extended Resources
//
// This module tests extended resource coverage including account_budget,
// asset_group, audience, bidding_strategy, label, customer_client,
// search_term_view, smart_campaign_search_term_view, change_event,
// ad_group_ad_asset_view, and asset_field_type_view resources.

mod test_helpers;

use test_helpers::{
    AccountBudgetBuilder, AdGroupAdAssetViewBuilder, AssetFieldTypeViewBuilder, AssetGroupBuilder,
    AudienceBuilder, BiddingStrategyBuilder, ChangeEventBuilder, CustomerClientBuilder,
    GoogleAdsRowBuilder, LabelBuilder, SearchTermViewBuilder, SmartCampaignSearchTermViewBuilder,
};

// ============================================================================
// AccountBudget Resource Tests
// ============================================================================

#[test]
fn test_account_budget_id() {
    let account_budget = AccountBudgetBuilder::new().id(123456).build();

    let row = GoogleAdsRowBuilder::new()
        .with_account_budget(account_budget)
        .build();

    assert_eq!(row.get("account_budget.id"), "123456");
}

#[test]
fn test_account_budget_name() {
    let account_budget = AccountBudgetBuilder::new()
        .name("My Account Budget")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_account_budget(account_budget)
        .build();

    assert_eq!(row.get("account_budget.name"), "My Account Budget");
}

#[test]
fn test_account_budget_status() {
    use googleads_rs::google::ads::googleads::v19::enums::account_budget_status_enum::AccountBudgetStatus;

    let account_budget = AccountBudgetBuilder::new()
        .status(AccountBudgetStatus::Approved as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_account_budget(account_budget)
        .build();

    assert_eq!(row.get("account_budget.status"), "Approved");
}

#[test]
fn test_account_budget_all_fields() {
    use googleads_rs::google::ads::googleads::v19::enums::account_budget_status_enum::AccountBudgetStatus;

    let account_budget = AccountBudgetBuilder::new()
        .id(999888777)
        .name("Annual Budget 2024")
        .status(AccountBudgetStatus::Approved as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_account_budget(account_budget)
        .build();

    assert_eq!(row.get("account_budget.id"), "999888777");
    assert_eq!(row.get("account_budget.name"), "Annual Budget 2024");
    assert_eq!(row.get("account_budget.status"), "Approved");
}

// ============================================================================
// AssetGroup Resource Tests
// ============================================================================

#[test]
fn test_asset_group_id() {
    let asset_group = AssetGroupBuilder::new().id(111222).build();

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();

    assert_eq!(row.get("asset_group.id"), "111222");
}

#[test]
fn test_asset_group_name() {
    let asset_group = AssetGroupBuilder::new()
        .name("Performance Max Asset Group")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();

    assert_eq!(row.get("asset_group.name"), "Performance Max Asset Group");
}

#[test]
fn test_asset_group_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_group_status_enum::AssetGroupStatus;

    let asset_group = AssetGroupBuilder::new()
        .status(AssetGroupStatus::Enabled as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();

    assert_eq!(row.get("asset_group.status"), "Enabled");
}

#[test]
fn test_asset_group_resource_name() {
    let asset_group = AssetGroupBuilder::new()
        .resource_name("customers/123/assetGroups/456")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();

    assert_eq!(
        row.get("asset_group.resource_name"),
        "customers/123/assetGroups/456"
    );
}

#[test]
fn test_asset_group_campaign() {
    let asset_group = AssetGroupBuilder::new()
        .campaign("customers/123/campaigns/789")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();

    assert_eq!(
        row.get("asset_group.campaign"),
        "customers/123/campaigns/789"
    );
}

#[test]
fn test_asset_group_ad_strength() {
    use googleads_rs::google::ads::googleads::v19::enums::ad_strength_enum::AdStrength;

    let asset_group = AssetGroupBuilder::new()
        .ad_strength(AdStrength::Excellent as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group(asset_group)
        .build();

    // ad_strength is stored as int, so it returns the numeric value
    assert_eq!(row.get("asset_group.ad_strength"), "7");
}

// ============================================================================
// Audience Resource Tests
// ============================================================================

#[test]
fn test_audience_id() {
    let audience = AudienceBuilder::new().id(55566).build();

    let row = GoogleAdsRowBuilder::new().with_audience(audience).build();

    assert_eq!(row.get("audience.id"), "55566");
}

#[test]
fn test_audience_name() {
    let audience = AudienceBuilder::new().name("High Value Customers").build();

    let row = GoogleAdsRowBuilder::new().with_audience(audience).build();

    assert_eq!(row.get("audience.name"), "High Value Customers");
}

#[test]
fn test_audience_description() {
    let audience = AudienceBuilder::new()
        .description("Customers who have spent over $1000")
        .build();

    let row = GoogleAdsRowBuilder::new().with_audience(audience).build();

    assert_eq!(
        row.get("audience.description"),
        "Customers who have spent over $1000"
    );
}

#[test]
fn test_audience_status() {
    use googleads_rs::google::ads::googleads::v19::enums::audience_status_enum::AudienceStatus;

    let audience = AudienceBuilder::new()
        .status(AudienceStatus::Enabled as i32)
        .build();

    let row = GoogleAdsRowBuilder::new().with_audience(audience).build();

    assert_eq!(row.get("audience.status"), "Enabled");
}

// ============================================================================
// BiddingStrategy Resource Tests
// ============================================================================

#[test]
fn test_bidding_strategy_id() {
    let bidding_strategy = BiddingStrategyBuilder::new().id(987654).build();

    let row = GoogleAdsRowBuilder::new()
        .with_bidding_strategy(bidding_strategy)
        .build();

    assert_eq!(row.get("bidding_strategy.id"), "987654");
}

#[test]
fn test_bidding_strategy_name() {
    let bidding_strategy = BiddingStrategyBuilder::new()
        .name("Target CPA Portfolio")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_bidding_strategy(bidding_strategy)
        .build();

    assert_eq!(row.get("bidding_strategy.name"), "Target CPA Portfolio");
}

#[test]
fn test_bidding_strategy_status() {
    use googleads_rs::google::ads::googleads::v19::enums::bidding_strategy_status_enum::BiddingStrategyStatus;

    let bidding_strategy = BiddingStrategyBuilder::new()
        .status(BiddingStrategyStatus::Enabled as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_bidding_strategy(bidding_strategy)
        .build();

    assert_eq!(row.get("bidding_strategy.status"), "Enabled");
}

// ============================================================================
// Label Resource Tests
// ============================================================================

#[test]
fn test_label_id() {
    let label = LabelBuilder::new().id(11223344).build();

    let row = GoogleAdsRowBuilder::new().with_label(label).build();

    assert_eq!(row.get("label.id"), "11223344");
}

#[test]
fn test_label_name() {
    let label = LabelBuilder::new().name("High Priority").build();

    let row = GoogleAdsRowBuilder::new().with_label(label).build();

    assert_eq!(row.get("label.name"), "High Priority");
}

#[test]
fn test_label_status() {
    use googleads_rs::google::ads::googleads::v19::enums::label_status_enum::LabelStatus;

    let label = LabelBuilder::new()
        .status(LabelStatus::Enabled as i32)
        .build();

    let row = GoogleAdsRowBuilder::new().with_label(label).build();

    assert_eq!(row.get("label.status"), "Enabled");
}

// ============================================================================
// CustomerClient Resource Tests
// ============================================================================

#[test]
fn test_customer_client_id() {
    let customer_client = CustomerClientBuilder::new().id(555666777).build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_client(customer_client)
        .build();

    assert_eq!(row.get("customer_client.id"), "555666777");
}

#[test]
fn test_customer_client_client_customer() {
    let customer_client = CustomerClientBuilder::new()
        .client_customer("customers/123456")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_client(customer_client)
        .build();

    assert_eq!(
        row.get("customer_client.client_customer"),
        "customers/123456"
    );
}

#[test]
fn test_customer_client_currency_code() {
    let customer_client = CustomerClientBuilder::new().currency_code("USD").build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_client(customer_client)
        .build();

    assert_eq!(row.get("customer_client.currency_code"), "USD");
}

#[test]
fn test_customer_client_descriptive_name() {
    let customer_client = CustomerClientBuilder::new()
        .descriptive_name("Acme Inc")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_client(customer_client)
        .build();

    assert_eq!(row.get("customer_client.descriptive_name"), "Acme Inc");
}

#[test]
fn test_customer_client_level() {
    let customer_client = CustomerClientBuilder::new().level(2).build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_client(customer_client)
        .build();

    assert_eq!(row.get("customer_client.level"), "2");
}

#[test]
fn test_customer_client_manager() {
    let customer_client = CustomerClientBuilder::new().manager(true).build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_client(customer_client)
        .build();

    assert_eq!(row.get("customer_client.manager"), "true");
}

#[test]
fn test_customer_client_status() {
    use googleads_rs::google::ads::googleads::v19::enums::customer_status_enum::CustomerStatus;

    let customer_client = CustomerClientBuilder::new()
        .status(CustomerStatus::Enabled as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_client(customer_client)
        .build();

    assert_eq!(row.get("customer_client.status"), "Enabled");
}

#[test]
fn test_customer_client_time_zone() {
    let customer_client = CustomerClientBuilder::new()
        .time_zone("America/New_York")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_customer_client(customer_client)
        .build();

    assert_eq!(row.get("customer_client.time_zone"), "America/New_York");
}

// ============================================================================
// SearchTermView Resource Tests
// ============================================================================

#[test]
fn test_search_term_view_ad_group() {
    let search_term_view = SearchTermViewBuilder::new()
        .ad_group("customers/123/adGroups/456")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_search_term_view(search_term_view)
        .build();

    assert_eq!(
        row.get("search_term_view.ad_group"),
        "customers/123/adGroups/456"
    );
}

#[test]
fn test_search_term_view_search_term() {
    let search_term_view = SearchTermViewBuilder::new()
        .search_term("running shoes")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_search_term_view(search_term_view)
        .build();

    assert_eq!(row.get("search_term_view.search_term"), "running shoes");
}

#[test]
fn test_search_term_view_status() {
    use googleads_rs::google::ads::googleads::v19::enums::search_term_targeting_status_enum::SearchTermTargetingStatus;

    let search_term_view = SearchTermViewBuilder::new()
        .status(SearchTermTargetingStatus::Added as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_search_term_view(search_term_view)
        .build();

    assert_eq!(row.get("search_term_view.status"), "Added");
}

// ============================================================================
// SmartCampaignSearchTermView Resource Tests
// ============================================================================

#[test]
fn test_smart_campaign_search_term_view_campaign() {
    let smart_campaign_search_term_view = SmartCampaignSearchTermViewBuilder::new()
        .campaign("customers/123/campaigns/789")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_smart_campaign_search_term_view(smart_campaign_search_term_view)
        .build();

    assert_eq!(
        row.get("smart_campaign_search_term_view.campaign"),
        "customers/123/campaigns/789"
    );
}

#[test]
fn test_smart_campaign_search_term_view_search_term() {
    let smart_campaign_search_term_view = SmartCampaignSearchTermViewBuilder::new()
        .search_term("best coffee shop")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_smart_campaign_search_term_view(smart_campaign_search_term_view)
        .build();

    assert_eq!(
        row.get("smart_campaign_search_term_view.search_term"),
        "best coffee shop"
    );
}

// ============================================================================
// ChangeEvent Resource Tests
// ============================================================================

#[test]
fn test_change_event_change_date_time() {
    let change_event = ChangeEventBuilder::new()
        .change_date_time("2024-10-10 12:34:56+00:00")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(
        row.get("change_event.change_date_time"),
        "2024-10-10 12:34:56+00:00"
    );
}

#[test]
fn test_change_event_change_resource_type() {
    use googleads_rs::google::ads::googleads::v19::enums::change_event_resource_type_enum::ChangeEventResourceType;

    let change_event = ChangeEventBuilder::new()
        .change_resource_type(ChangeEventResourceType::Campaign as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(row.get("change_event.change_resource_type"), "Campaign");
}

#[test]
fn test_change_event_change_resource_name() {
    let change_event = ChangeEventBuilder::new()
        .change_resource_name("customers/123/campaigns/456")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(
        row.get("change_event.change_resource_name"),
        "customers/123/campaigns/456"
    );
}

#[test]
fn test_change_event_client_type() {
    use googleads_rs::google::ads::googleads::v19::enums::change_client_type_enum::ChangeClientType;

    let change_event = ChangeEventBuilder::new()
        .client_type(ChangeClientType::GoogleAdsWebClient as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(row.get("change_event.client_type"), "GoogleAdsWebClient");
}

#[test]
fn test_change_event_user_email() {
    let change_event = ChangeEventBuilder::new()
        .user_email("user@example.com")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(row.get("change_event.user_email"), "user@example.com");
}

#[test]
fn test_change_event_resource_change_operation() {
    use googleads_rs::google::ads::googleads::v19::enums::resource_change_operation_enum::ResourceChangeOperation;

    let change_event = ChangeEventBuilder::new()
        .resource_change_operation(ResourceChangeOperation::Update as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(row.get("change_event.resource_change_operation"), "Update");
}

#[test]
fn test_change_event_changed_fields() {
    let change_event = ChangeEventBuilder::new()
        .changed_fields(vec!["campaign.name", "campaign.status"])
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(
        row.get("change_event.changed_fields"),
        "'campaign.name, campaign.status'"
    );
}

#[test]
fn test_change_event_campaign() {
    let change_event = ChangeEventBuilder::new()
        .campaign("customers/123/campaigns/789")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(
        row.get("change_event.campaign"),
        "customers/123/campaigns/789"
    );
}

// ============================================================================
// AdGroupAdAssetView Resource Tests
// ============================================================================

#[test]
fn test_ad_group_ad_asset_view_resource_name() {
    let ad_group_ad_asset_view = AdGroupAdAssetViewBuilder::new()
        .resource_name("customers/123/adGroupAdAssetViews/456")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad_asset_view(ad_group_ad_asset_view)
        .build();

    assert_eq!(
        row.get("ad_group_ad_asset_view.resource_name"),
        "customers/123/adGroupAdAssetViews/456"
    );
}

#[test]
fn test_ad_group_ad_asset_view_asset() {
    let ad_group_ad_asset_view = AdGroupAdAssetViewBuilder::new()
        .asset("customers/123/assets/789")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad_asset_view(ad_group_ad_asset_view)
        .build();

    assert_eq!(
        row.get("ad_group_ad_asset_view.asset"),
        "customers/123/assets/789"
    );
}

#[test]
fn test_ad_group_ad_asset_view_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let ad_group_ad_asset_view = AdGroupAdAssetViewBuilder::new()
        .field_type(AssetFieldType::Headline as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad_asset_view(ad_group_ad_asset_view)
        .build();

    assert_eq!(row.get("ad_group_ad_asset_view.field_type"), "Headline");
}

#[test]
fn test_ad_group_ad_asset_view_pinned_field() {
    use googleads_rs::google::ads::googleads::v19::enums::served_asset_field_type_enum::ServedAssetFieldType;

    let ad_group_ad_asset_view = AdGroupAdAssetViewBuilder::new()
        .pinned_field(ServedAssetFieldType::Headline1 as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad_asset_view(ad_group_ad_asset_view)
        .build();

    assert_eq!(row.get("ad_group_ad_asset_view.pinned_field"), "Headline1");
}

#[test]
fn test_ad_group_ad_asset_view_performance_label() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_performance_label_enum::AssetPerformanceLabel;

    let ad_group_ad_asset_view = AdGroupAdAssetViewBuilder::new()
        .performance_label(AssetPerformanceLabel::Best as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad_asset_view(ad_group_ad_asset_view)
        .build();

    assert_eq!(row.get("ad_group_ad_asset_view.performance_label"), "Best");
}

// ============================================================================
// AssetFieldTypeView Resource Tests
// ============================================================================

#[test]
fn test_asset_field_type_view_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let asset_field_type_view = AssetFieldTypeViewBuilder::new()
        .field_type(AssetFieldType::Description as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_asset_field_type_view(asset_field_type_view)
        .build();

    assert_eq!(row.get("asset_field_type_view.field_type"), "Description");
}

// ============================================================================
// Edge Cases and Combined Tests
// ============================================================================

#[test]
fn test_multiple_phase3_resources_in_same_row() {
    use googleads_rs::google::ads::googleads::v19::enums::audience_status_enum::AudienceStatus;
    use googleads_rs::google::ads::googleads::v19::enums::label_status_enum::LabelStatus;

    let label = LabelBuilder::new()
        .id(111)
        .name("Test Label")
        .status(LabelStatus::Enabled as i32)
        .build();

    let audience = AudienceBuilder::new()
        .id(222)
        .name("Test Audience")
        .status(AudienceStatus::Enabled as i32)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_label(label)
        .with_audience(audience)
        .build();

    assert_eq!(row.get("label.id"), "111");
    assert_eq!(row.get("label.name"), "Test Label");
    assert_eq!(row.get("audience.id"), "222");
    assert_eq!(row.get("audience.name"), "Test Audience");
}

#[test]
fn test_search_term_with_special_characters() {
    let search_term_view = SearchTermViewBuilder::new()
        .search_term("\"running shoes\" -cheap")
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_search_term_view(search_term_view)
        .build();

    assert_eq!(
        row.get("search_term_view.search_term"),
        "\"running shoes\" -cheap"
    );
}

#[test]
fn test_change_event_multiple_changed_fields() {
    let change_event = ChangeEventBuilder::new()
        .changed_fields(vec![
            "campaign.name",
            "campaign.status",
            "campaign.budget",
            "campaign.bidding_strategy",
        ])
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();

    assert_eq!(
        row.get("change_event.changed_fields"),
        "'campaign.name, campaign.status, campaign.budget, campaign.bidding_strategy'"
    );
}
