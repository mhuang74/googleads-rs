// Unit tests for GoogleAdsRow::get() method - Enum Fields
//
// This module tests the method_str! and optional_method_str! macros
// which are used for extracting enum field values via accessor methods

#![allow(unused_imports)]
#![allow(clippy::field_reassign_with_default)]

mod test_helpers;

use googleads_rs::google::ads::googleads::v23::enums::{
    ad_group_criterion_status_enum::AdGroupCriterionStatus,
    ad_group_status_enum::AdGroupStatus, ad_group_type_enum::AdGroupType,
    ad_network_type_enum::AdNetworkType,
    advertising_channel_sub_type_enum::AdvertisingChannelSubType,
    advertising_channel_type_enum::AdvertisingChannelType,
    bidding_strategy_type_enum::BiddingStrategyType,
    campaign_serving_status_enum::CampaignServingStatus, campaign_status_enum::CampaignStatus,
    click_type_enum::ClickType, customer_status_enum::CustomerStatus, day_of_week_enum::DayOfWeek,
    device_enum::Device, slot_enum::Slot,
};
use test_helpers::{
    AdGroupBuilder, CampaignBuilder, CustomerBuilder, GoogleAdsRowBuilder, SegmentsBuilder,
};

// ============================================================================
// Campaign Enum Fields (method_str!)
// ============================================================================

#[test]
fn test_campaign_status_enabled() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Enabled)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.status"), "ENABLED");
}

#[test]
fn test_campaign_status_paused() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Paused)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.status"), "PAUSED");
}

#[test]
fn test_campaign_status_removed() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Removed)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.status"), "REMOVED");
}

#[test]
fn test_campaign_advertising_channel_type_search() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::Search)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "SEARCH");
}

#[test]
fn test_campaign_advertising_channel_type_display() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::Display)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "DISPLAY");
}

#[test]
fn test_campaign_advertising_channel_type_shopping() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::Shopping)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "SHOPPING");
}

#[test]
fn test_campaign_advertising_channel_type_video() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::Video)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.advertising_channel_type"), "VIDEO");
}

#[test]
fn test_campaign_advertising_channel_type_performance_max() {
    let campaign = CampaignBuilder::new()
        .advertising_channel_type(AdvertisingChannelType::PerformanceMax)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.advertising_channel_type"),
        "PERFORMANCE_MAX"
    );
}

// ============================================================================
// Campaign Bidding Strategy Type
// ============================================================================

#[test]
fn test_campaign_bidding_strategy_type_manual_cpc() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::ManualCpc)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.bidding_strategy_type"), "MANUAL_CPC");
}

#[test]
fn test_campaign_bidding_strategy_type_maximize_conversions() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::MaximizeConversions)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.bidding_strategy_type"),
        "MAXIMIZE_CONVERSIONS"
    );
}

#[test]
fn test_campaign_bidding_strategy_type_maximize_conversion_value() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::MaximizeConversionValue)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.bidding_strategy_type"),
        "MAXIMIZE_CONVERSION_VALUE"
    );
}

#[test]
fn test_campaign_bidding_strategy_type_target_cpa() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::TargetCpa)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.bidding_strategy_type"), "TARGET_CPA");
}

#[test]
fn test_campaign_bidding_strategy_type_target_impression_share() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::TargetImpressionShare)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.bidding_strategy_type"),
        "TARGET_IMPRESSION_SHARE"
    );
}

#[test]
fn test_campaign_bidding_strategy_type_target_roas() {
    let campaign = CampaignBuilder::new()
        .bidding_strategy_type(BiddingStrategyType::TargetRoas)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.bidding_strategy_type"), "TARGET_ROAS");
}

// Removed - Unsupported variant doesn't exist in BiddingStrategyType enum
// #[test]
// fn test_campaign_bidding_strategy_type_unsupported() {
//     let campaign = CampaignBuilder::new()
//         .bidding_strategy_type(BiddingStrategyType::Unsupported)
//         .build();
//
//     let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
//
//     assert_eq!(row.get("campaign.bidding_strategy_type"), "UNSUPPORTED");
// }

// ============================================================================
// Campaign Criterion Enum Fields
// ============================================================================

#[test]
fn test_campaign_criterion_status_present() {
    let mut campaign_criterion = test_helpers::CampaignCriterionBuilder::new().build();
    campaign_criterion.status = AdGroupCriterionStatus::Enabled as i32;

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(campaign_criterion)
        .build();

    assert_eq!(row.get("campaign_criterion.status"), "ENABLED");
}

#[test]
fn test_campaign_criterion_status_absent() {
    let campaign_criterion = test_helpers::CampaignCriterionBuilder::new().build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(campaign_criterion)
        .build();

    // Default/unspecified status
    assert_eq!(row.get("campaign_criterion.status"), "UNSPECIFIED");
}

// ============================================================================
// AdGroup Enum Fields
// ============================================================================

#[test]
fn test_ad_group_status_enabled() {
    let ad_group = AdGroupBuilder::new().status(AdGroupStatus::Enabled).build();

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();

    assert_eq!(row.get("ad_group.status"), "ENABLED");
}

#[test]
fn test_ad_group_status_paused() {
    let ad_group = AdGroupBuilder::new().status(AdGroupStatus::Paused).build();

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();

    assert_eq!(row.get("ad_group.status"), "PAUSED");
}

#[test]
fn test_ad_group_status_removed() {
    let ad_group = AdGroupBuilder::new().status(AdGroupStatus::Removed).build();

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();

    assert_eq!(row.get("ad_group.status"), "REMOVED");
}

#[test]
fn test_ad_group_type_search_standard() {
    let ad_group = AdGroupBuilder::new()
        .ad_group_type(AdGroupType::SearchStandard)
        .build();

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();

    assert_eq!(row.get("ad_group.type"), "SEARCH_STANDARD");
}

#[test]
fn test_ad_group_type_display_standard() {
    let ad_group = AdGroupBuilder::new()
        .ad_group_type(AdGroupType::DisplayStandard)
        .build();

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();

    assert_eq!(row.get("ad_group.type"), "DISPLAY_STANDARD");
}

#[test]
fn test_ad_group_type_shopping_product_ads() {
    let ad_group = AdGroupBuilder::new()
        .ad_group_type(AdGroupType::ShoppingProductAds)
        .build();

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();

    assert_eq!(row.get("ad_group.type"), "SHOPPING_PRODUCT_ADS");
}

#[test]
fn test_ad_group_type_video_true_view_in_stream() {
    let ad_group = AdGroupBuilder::new()
        .ad_group_type(AdGroupType::VideoTrueViewInStream)
        .build();

    let row = GoogleAdsRowBuilder::new().with_ad_group(ad_group).build();

    assert_eq!(row.get("ad_group.type"), "VIDEO_TRUE_VIEW_IN_STREAM");
}

// ============================================================================
// Customer Enum Fields
// ============================================================================

#[test]
fn test_customer_status_enabled() {
    use googleads_rs::google::ads::googleads::v23::resources::Customer;

    let mut customer = Customer::default();
    customer.id = Some(123);
    customer.status = CustomerStatus::Enabled as i32;

    let row = GoogleAdsRowBuilder::new().with_customer(customer).build();

    assert_eq!(row.get("customer.status"), "ENABLED");
}

#[test]
fn test_customer_status_canceled() {
    use googleads_rs::google::ads::googleads::v23::resources::Customer;

    let mut customer = Customer::default();
    customer.id = Some(123);
    customer.status = CustomerStatus::Canceled as i32;

    let row = GoogleAdsRowBuilder::new().with_customer(customer).build();

    assert_eq!(row.get("customer.status"), "CANCELED");
}

#[test]
fn test_customer_status_suspended() {
    use googleads_rs::google::ads::googleads::v23::resources::Customer;

    let mut customer = Customer::default();
    customer.id = Some(123);
    customer.status = CustomerStatus::Suspended as i32;

    let row = GoogleAdsRowBuilder::new().with_customer(customer).build();

    assert_eq!(row.get("customer.status"), "SUSPENDED");
}

// ============================================================================
// Segments Enum Fields (method_str!)
// ============================================================================

#[test]
fn test_segments_device_mobile() {
    let segments = SegmentsBuilder::new().device(Device::Mobile).build();

    let row = GoogleAdsRowBuilder::new().with_segments(segments).build();

    assert_eq!(row.get("segments.device"), "MOBILE");
}

#[test]
fn test_segments_device_desktop() {
    let segments = SegmentsBuilder::new().device(Device::Desktop).build();

    let row = GoogleAdsRowBuilder::new().with_segments(segments).build();

    assert_eq!(row.get("segments.device"), "DESKTOP");
}

#[test]
fn test_segments_device_tablet() {
    let segments = SegmentsBuilder::new().device(Device::Tablet).build();

    let row = GoogleAdsRowBuilder::new().with_segments(segments).build();

    assert_eq!(row.get("segments.device"), "TABLET");
}

#[test]
fn test_segments_day_of_week_monday() {
    let segments = SegmentsBuilder::new()
        .day_of_week(DayOfWeek::Monday)
        .build();

    let row = GoogleAdsRowBuilder::new().with_segments(segments).build();

    assert_eq!(row.get("segments.day_of_week"), "MONDAY");
}

#[test]
fn test_segments_day_of_week_friday() {
    let segments = SegmentsBuilder::new()
        .day_of_week(DayOfWeek::Friday)
        .build();

    let row = GoogleAdsRowBuilder::new().with_segments(segments).build();

    assert_eq!(row.get("segments.day_of_week"), "FRIDAY");
}

#[test]
fn test_segments_day_of_week_sunday() {
    let segments = SegmentsBuilder::new()
        .day_of_week(DayOfWeek::Sunday)
        .build();

    let row = GoogleAdsRowBuilder::new().with_segments(segments).build();

    assert_eq!(row.get("segments.day_of_week"), "SUNDAY");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_default_enum_values() {
    let campaign = CampaignBuilder::new().build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Default/unspecified enum values should return "UNSPECIFIED"
    assert_eq!(row.get("campaign.status"), "UNSPECIFIED");
}

#[test]
fn test_device_unspecified() {
    let segments = SegmentsBuilder::new()
        .device(Device::Unspecified)
        .build();

    let row = GoogleAdsRowBuilder::new().with_segments(segments).build();

    assert_eq!(row.get("segments.device"), "UNSPECIFIED");
}

#[test]
fn test_multiple_enum_fields() {
    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Enabled)
        .advertising_channel_type(AdvertisingChannelType::Search)
        .bidding_strategy_type(BiddingStrategyType::ManualCpc)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.status"), "ENABLED");
    assert_eq!(row.get("campaign.advertising_channel_type"), "SEARCH");
    assert_eq!(row.get("campaign.bidding_strategy_type"), "MANUAL_CPC");
}
