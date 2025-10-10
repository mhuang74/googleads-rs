// Unit tests for GoogleAdsRow::get() method - Enum Fields
//
// This module tests the method_str! and optional_method_str! macros
// which are used for extracting enum field values via accessor methods

mod test_helpers;

use test_helpers::{
    GoogleAdsRowBuilder, CampaignBuilder, AdGroupBuilder, CustomerBuilder, SegmentsBuilder,
};
use googleads_rs::google::ads::googleads::v19::enums::{
    campaign_status_enum::CampaignStatus,
    ad_group_status_enum::AdGroupStatus,
    ad_group_type_enum::AdGroupType,
    advertising_channel_type_enum::AdvertisingChannelType,
    advertising_channel_sub_type_enum::AdvertisingChannelSubType,
    bidding_strategy_type_enum::BiddingStrategyType,
    campaign_serving_status_enum::CampaignServingStatus,
    customer_status_enum::CustomerStatus,
    device_enum::Device,
    day_of_week_enum::DayOfWeek,
    ad_network_type_enum::AdNetworkType,
    click_type_enum::ClickType,
    slot_enum::Slot,
};

// ============================================================================
// Campaign Enum Fields (method_str!)
// ============================================================================

#[test]
fn test_campaign_status_enabled() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Enabled)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.status"), "Enabled");
}

#[test]
fn test_campaign_status_paused() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Paused)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.status"), "Paused");
}

#[test]
fn test_campaign_status_removed() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Removed)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.status"), "Removed");
}

#[test]
fn test_campaign_advertising_channel_type_search() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::Search)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "Search");
}

#[test]
fn test_campaign_advertising_channel_type_display() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::Display)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "Display");
}

#[test]
fn test_campaign_advertising_channel_type_shopping() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::Shopping)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "Shopping");
}

#[test]
fn test_campaign_advertising_channel_type_video() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::Video)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "Video");
}

#[test]
fn test_campaign_advertising_channel_type_performance_max() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::PerformanceMax)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "PerformanceMax");
}

// ============================================================================
// Campaign Bidding Strategy Type - Custom Mapping
// ============================================================================

#[test]
fn test_campaign_bidding_strategy_type_manual_cpc() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::ManualCpc)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    // Note: Custom mapping in lib.rs returns "ManualCPC" not "ManualCpc"
    assert_eq!(row.get("campaign.bidding_strategy_type"), "ManualCPC");
}

#[test]
fn test_campaign_bidding_strategy_type_maximize_conversions() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::MaximizeConversions)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    // Note: Custom mapping has typo "MaximizeConverions" (missing 's')
    assert_eq!(row.get("campaign.bidding_strategy_type"), "MaximizeConverions");
}

#[test]
fn test_campaign_bidding_strategy_type_maximize_conversion_value() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::MaximizeConversionValue)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.bidding_strategy_type"), "MaximizeConversionValue");
}

#[test]
fn test_campaign_bidding_strategy_type_target_cpa() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::TargetCpa)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.bidding_strategy_type"), "TargetCPA");
}

#[test]
fn test_campaign_bidding_strategy_type_target_roas() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::TargetRoas)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.bidding_strategy_type"), "TargetROAS");
}

#[test]
fn test_campaign_bidding_strategy_type_target_impression_share() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::TargetImpressionShare)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.bidding_strategy_type"), "TargetImpShare");
}

#[test]
fn test_campaign_bidding_strategy_type_unsupported() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::EnhancedCpc)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    // Custom mapping returns "Unsupported" for unmapped types
    assert_eq!(row.get("campaign.bidding_strategy_type"), "Unsupported");
}

// ============================================================================
// AdGroup Enum Fields (method_str!)
// ============================================================================

#[test]
fn test_ad_group_status_enabled() {
    let ad_group = AdGroupBuilder::new()
        .status(AdGroupStatus::Enabled)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.status"), "Enabled");
}

#[test]
fn test_ad_group_status_paused() {
    let ad_group = AdGroupBuilder::new()
        .status(AdGroupStatus::Paused)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.status"), "Paused");
}

#[test]
fn test_ad_group_status_removed() {
    let ad_group = AdGroupBuilder::new()
        .status(AdGroupStatus::Removed)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.status"), "Removed");
}

#[test]
fn test_ad_group_type_search_standard() {
    let ad_group = AdGroupBuilder::new()
        .ad_group_type(AdGroupType::SearchStandard)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.type"), "SearchStandard");
}

#[test]
fn test_ad_group_type_display_standard() {
    let ad_group = AdGroupBuilder::new()
        .ad_group_type(AdGroupType::DisplayStandard)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.type"), "DisplayStandard");
}

#[test]
fn test_ad_group_type_shopping_product_ads() {
    let ad_group = AdGroupBuilder::new()
        .ad_group_type(AdGroupType::ShoppingProductAds)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.type"), "ShoppingProductAds");
}

#[test]
fn test_ad_group_type_video_true_view_in_stream() {
    let ad_group = AdGroupBuilder::new()
        .ad_group_type(AdGroupType::VideoTrueViewInStream)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group(ad_group)
        .build();

    assert_eq!(row.get("ad_group.type"), "VideoTrueViewInStream");
}

// ============================================================================
// Customer Enum Fields (method_str!)
// ============================================================================

#[test]
fn test_customer_status_enabled() {
    use googleads_rs::google::ads::googleads::v19::resources::Customer;

    let mut customer = Customer::default();
    customer.id = 123;
    customer.status = CustomerStatus::Enabled as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_customer(customer)
        .build();

    assert_eq!(row.get("customer.status"), "Enabled");
}

#[test]
fn test_customer_status_canceled() {
    use googleads_rs::google::ads::googleads::v19::resources::Customer;

    let mut customer = Customer::default();
    customer.id = 123;
    customer.status = CustomerStatus::Canceled as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_customer(customer)
        .build();

    assert_eq!(row.get("customer.status"), "Canceled");
}

#[test]
fn test_customer_status_suspended() {
    use googleads_rs::google::ads::googleads::v19::resources::Customer;

    let mut customer = Customer::default();
    customer.id = 123;
    customer.status = CustomerStatus::Suspended as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_customer(customer)
        .build();

    assert_eq!(row.get("customer.status"), "Suspended");
}

// ============================================================================
// Segments Enum Fields (method_str!)
// ============================================================================

#[test]
fn test_segments_device_mobile() {
    let segments = SegmentsBuilder::new()
        .device(Device::Mobile)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.device"), "Mobile");
}

#[test]
fn test_segments_device_desktop() {
    let segments = SegmentsBuilder::new()
        .device(Device::Desktop)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.device"), "Desktop");
}

#[test]
fn test_segments_device_tablet() {
    let segments = SegmentsBuilder::new()
        .device(Device::Tablet)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.device"), "Tablet");
}

#[test]
fn test_segments_day_of_week_monday() {
    let segments = SegmentsBuilder::new()
        .day_of_week(DayOfWeek::Monday)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.day_of_week"), "Monday");
}

#[test]
fn test_segments_day_of_week_friday() {
    let segments = SegmentsBuilder::new()
        .day_of_week(DayOfWeek::Friday)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.day_of_week"), "Friday");
}

#[test]
fn test_segments_day_of_week_sunday() {
    let segments = SegmentsBuilder::new()
        .day_of_week(DayOfWeek::Sunday)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.day_of_week"), "Sunday");
}

// ============================================================================
// Optional Enum Fields (optional_method_str!)
// ============================================================================

#[test]
fn test_campaign_criterion_status_present() {
    use googleads_rs::google::ads::googleads::v19::resources::CampaignCriterion;
    use googleads_rs::google::ads::googleads::v19::enums::campaign_criterion_status_enum::CampaignCriterionStatus;

    let mut criterion = CampaignCriterion::default();
    criterion.status = CampaignCriterionStatus::Enabled as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(criterion)
        .build();

    assert_eq!(row.get("campaign_criterion.status"), "Enabled");
}

#[test]
fn test_campaign_criterion_status_absent() {
    // Create row without campaign_criterion
    let row = GoogleAdsRowBuilder::new().build();

    assert_eq!(row.get("campaign_criterion.status"), "");
}

#[test]
fn test_campaign_criterion_type_present() {
    use googleads_rs::google::ads::googleads::v19::resources::CampaignCriterion;
    use googleads_rs::google::ads::googleads::v19::enums::criterion_type_enum::CriterionType;

    let mut criterion = CampaignCriterion::default();
    criterion.r#type = CriterionType::Keyword as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(criterion)
        .build();

    assert_eq!(row.get("campaign_criterion.type"), "Keyword");
}

#[test]
fn test_campaign_criterion_type_absent() {
    let row = GoogleAdsRowBuilder::new().build();

    assert_eq!(row.get("campaign_criterion.type"), "");
}

// ============================================================================
// Unspecified/Unknown Enum Values
// ============================================================================

#[test]
fn test_campaign_status_unspecified() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Unspecified)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.status"), "Unspecified");
}

#[test]
fn test_campaign_status_unknown() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Unknown)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    assert_eq!(row.get("campaign.status"), "Unknown");
}

#[test]
fn test_device_unspecified() {
    let segments = SegmentsBuilder::new()
        .device(Device::Unspecified)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.device"), "Unspecified");
}

// ============================================================================
// Multiple Enum Fields in Same Row
// ============================================================================

#[test]
fn test_multiple_enum_fields() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Enabled)
        .advertising_channel_type(AdvertisingChannelType::Search)
        .bidding_strategy_type(BiddingStrategyType::TargetCpa)
        .build();

    let ad_group = AdGroupBuilder::new()
        .status(AdGroupStatus::Paused)
        .ad_group_type(AdGroupType::SearchStandard)
        .build();

    let segments = SegmentsBuilder::new()
        .device(Device::Mobile)
        .day_of_week(DayOfWeek::Monday)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .with_ad_group(ad_group)
        .with_segments(segments)
        .build();

    assert_eq!(row.get("campaign.status"), "Enabled");
    assert_eq!(row.get("campaign.advertising_channel_type"), "Search");
    assert_eq!(row.get("campaign.bidding_strategy_type"), "TargetCPA");
    assert_eq!(row.get("ad_group.status"), "Paused");
    assert_eq!(row.get("ad_group.type"), "SearchStandard");
    assert_eq!(row.get("segments.device"), "Mobile");
    assert_eq!(row.get("segments.day_of_week"), "Monday");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_default_enum_values() {
    // Default enum values should be Unspecified (0)
    let campaign = CampaignBuilder::new().build();
    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .build();

    // Default i32 value is 0, which maps to Unspecified
    assert_eq!(row.get("campaign.status"), "Unspecified");
}
