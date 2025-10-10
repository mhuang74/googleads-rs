// Unit tests for GoogleAdsRow::get() method - Phases 4-7
//
// This module tests fields implemented in Phases 4, 5, 6, and 7:
// - Phase 4: Reporting Views (keyword_view, landing_page_view, geographic_view, click_view)
// - Phase 5: Asset & Label Resources (asset_group_asset, asset_group_signal, labels)
// - Phase 6: High-Value Resources (recommendation, shared_set, shared_criterion, feed)
// - Phase 7: Specialized Metrics & Segments (asset performance, hotel, SKAdNetwork)

mod test_helpers;

use test_helpers::{GoogleAdsRowBuilder, MetricsBuilder, SegmentsBuilder};
use googleads_rs::google::ads::googleads::v19::resources::{
    KeywordView, LandingPageView, GeographicView, ClickView, AssetGroupAsset, AssetGroupSignal,
    CampaignLabel, AdGroupLabel, AdGroupAdLabel, Recommendation, CampaignSharedSet, SharedSet,
    SharedCriterion,
};
use googleads_rs::google::ads::googleads::v19::common::{ClickLocation, Metrics, Segments};

// ============================================================================
// Phase 4: Reporting Views - KeywordView
// ============================================================================

#[test]
fn test_keyword_view_resource_name() {
    let keyword_view = KeywordView {
        resource_name: "customers/123/keywordViews/456~789".to_string(),
    };

    let row = GoogleAdsRowBuilder::new()
        .with_keyword_view(keyword_view)
        .build();

    assert_eq!(row.get("keyword_view.resource_name"), "customers/123/keywordViews/456~789");
}

// ============================================================================
// Phase 4: Reporting Views - LandingPageView
// ============================================================================

#[test]
fn test_landing_page_view_resource_name() {
    let landing_page_view = LandingPageView {
        resource_name: "customers/123/landingPageViews/456".to_string(),
        unexpanded_final_url: "https://example.com/product".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_landing_page_view(landing_page_view)
        .build();

    assert_eq!(row.get("landing_page_view.resource_name"), "customers/123/landingPageViews/456");
}

#[test]
fn test_landing_page_view_unexpanded_final_url() {
    let landing_page_view = LandingPageView {
        unexpanded_final_url: "https://example.com/landing?param=value".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_landing_page_view(landing_page_view)
        .build();

    assert_eq!(row.get("landing_page_view.unexpanded_final_url"), "https://example.com/landing?param=value");
}

// ============================================================================
// Phase 4: Reporting Views - GeographicView
// ============================================================================

#[test]
fn test_geographic_view_resource_name() {
    let geographic_view = GeographicView {
        resource_name: "customers/123/geographicViews/456~789".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geographic_view(geographic_view)
        .build();

    assert_eq!(row.get("geographic_view.resource_name"), "customers/123/geographicViews/456~789");
}

#[test]
fn test_geographic_view_location_type() {
    use googleads_rs::google::ads::googleads::v19::enums::geo_targeting_type_enum::GeoTargetingType;

    let geographic_view = GeographicView {
        location_type: GeoTargetingType::LocationOfPresence as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geographic_view(geographic_view)
        .build();

    assert_eq!(row.get("geographic_view.location_type"), "LocationOfPresence");
}

#[test]
fn test_geographic_view_country_criterion_id() {
    let geographic_view = GeographicView {
        country_criterion_id: 2840, // United States
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_geographic_view(geographic_view)
        .build();

    assert_eq!(row.get("geographic_view.country_criterion_id"), "2840");
}

// ============================================================================
// Phase 4: Reporting Views - ClickView
// ============================================================================

#[test]
fn test_click_view_resource_name() {
    let click_view = ClickView {
        resource_name: "customers/123/clickViews/2024-01-01~abcd1234".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_click_view(click_view)
        .build();

    assert_eq!(row.get("click_view.resource_name"), "customers/123/clickViews/2024-01-01~abcd1234");
}

#[test]
fn test_click_view_gclid() {
    let click_view = ClickView {
        gclid: "TeSter-123".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_click_view(click_view)
        .build();

    assert_eq!(row.get("click_view.gclid"), "TeSter-123");
}

#[test]
fn test_click_view_area_of_interest() {
    let click_view = ClickView {
        area_of_interest: Some(ClickLocation {
            city: "San Francisco".to_string(),
            country: "United States".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_click_view(click_view)
        .build();

    assert_eq!(row.get("click_view.area_of_interest.city"), "San Francisco");
    assert_eq!(row.get("click_view.area_of_interest.country"), "United States");
}

#[test]
fn test_click_view_location_of_presence() {
    let click_view = ClickView {
        location_of_presence: Some(ClickLocation {
            city: "New York".to_string(),
            country: "United States".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_click_view(click_view)
        .build();

    assert_eq!(row.get("click_view.location_of_presence.city"), "New York");
    assert_eq!(row.get("click_view.location_of_presence.country"), "United States");
}

// ============================================================================
// Phase 5: AssetGroupAsset
// ============================================================================

#[test]
fn test_asset_group_asset_resource_name() {
    let asset_group_asset = AssetGroupAsset {
        resource_name: "customers/123/assetGroupAssets/456~789".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    assert_eq!(row.get("asset_group_asset.resource_name"), "customers/123/assetGroupAssets/456~789");
}

#[test]
fn test_asset_group_asset_field_type() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_field_type_enum::AssetFieldType;

    let asset_group_asset = AssetGroupAsset {
        field_type: AssetFieldType::Headline as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    assert_eq!(row.get("asset_group_asset.field_type"), "Headline");
}

#[test]
fn test_asset_group_asset_status() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_link_status_enum::AssetLinkStatus;

    let asset_group_asset = AssetGroupAsset {
        status: AssetLinkStatus::Enabled as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    assert_eq!(row.get("asset_group_asset.status"), "Enabled");
}

#[test]
fn test_asset_group_asset_performance_label() {
    use googleads_rs::google::ads::googleads::v19::enums::asset_performance_label_enum::AssetPerformanceLabel;

    let asset_group_asset = AssetGroupAsset {
        performance_label: AssetPerformanceLabel::Best as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_asset(asset_group_asset)
        .build();

    assert_eq!(row.get("asset_group_asset.performance_label"), "Best");
}

// ============================================================================
// Phase 5: AssetGroupSignal
// ============================================================================

#[test]
fn test_asset_group_signal_resource_name() {
    let asset_group_signal = AssetGroupSignal {
        resource_name: "customers/123/assetGroupSignals/456~789".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_asset_group_signal(asset_group_signal)
        .build();

    assert_eq!(row.get("asset_group_signal.resource_name"), "customers/123/assetGroupSignals/456~789");
}

// ============================================================================
// Phase 5: Label Relations
// ============================================================================

#[test]
fn test_campaign_label_resource_name() {
    let campaign_label = CampaignLabel {
        resource_name: "customers/123/campaignLabels/456~789".to_string(),
        campaign: "customers/123/campaigns/456".to_string(),
        label: "customers/123/labels/789".to_string(),
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_label(campaign_label)
        .build();

    assert_eq!(row.get("campaign_label.resource_name"), "customers/123/campaignLabels/456~789");
    assert_eq!(row.get("campaign_label.campaign"), "customers/123/campaigns/456");
    assert_eq!(row.get("campaign_label.label"), "customers/123/labels/789");
}

#[test]
fn test_ad_group_label_all_fields() {
    let ad_group_label = AdGroupLabel {
        resource_name: "customers/123/adGroupLabels/456~789".to_string(),
        ad_group: "customers/123/adGroups/456".to_string(),
        label: "customers/123/labels/789".to_string(),
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_label(ad_group_label)
        .build();

    assert_eq!(row.get("ad_group_label.resource_name"), "customers/123/adGroupLabels/456~789");
    assert_eq!(row.get("ad_group_label.ad_group"), "customers/123/adGroups/456");
    assert_eq!(row.get("ad_group_label.label"), "customers/123/labels/789");
}

#[test]
fn test_ad_group_ad_label_all_fields() {
    let ad_group_ad_label = AdGroupAdLabel {
        resource_name: "customers/123/adGroupAdLabels/456~789~012".to_string(),
        ad_group_ad: "customers/123/adGroupAds/456~012".to_string(),
        label: "customers/123/labels/789".to_string(),
    };

    let row = GoogleAdsRowBuilder::new()
        .with_ad_group_ad_label(ad_group_ad_label)
        .build();

    assert_eq!(row.get("ad_group_ad_label.resource_name"), "customers/123/adGroupAdLabels/456~789~012");
    assert_eq!(row.get("ad_group_ad_label.ad_group_ad"), "customers/123/adGroupAds/456~012");
    assert_eq!(row.get("ad_group_ad_label.label"), "customers/123/labels/789");
}

// ============================================================================
// Phase 6: Recommendation
// ============================================================================

#[test]
fn test_recommendation_resource_name() {
    let recommendation = Recommendation {
        resource_name: "customers/123/recommendations/KEYWORD_123".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_recommendation(recommendation)
        .build();

    assert_eq!(row.get("recommendation.resource_name"), "customers/123/recommendations/KEYWORD_123");
}

#[test]
fn test_recommendation_type() {
    use googleads_rs::google::ads::googleads::v19::enums::recommendation_type_enum::RecommendationType;

    let recommendation = Recommendation {
        r#type: RecommendationType::Keyword as i32,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_recommendation(recommendation)
        .build();

    assert_eq!(row.get("recommendation.type"), "Keyword");
}

#[test]
fn test_recommendation_impact_metrics() {
    use googleads_rs::google::ads::googleads::v19::resources::recommendation::{
        RecommendationImpact, RecommendationMetrics,
    };

    let recommendation = Recommendation {
        impact: Some(RecommendationImpact {
            base_metrics: Some(RecommendationMetrics {
                clicks: 1000.0,
                impressions: 50000.0,
                cost_micros: 25000000,
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_recommendation(recommendation)
        .build();

    assert_eq!(row.get("recommendation.impact.base_metrics.clicks"), "1000");
    assert_eq!(row.get("recommendation.impact.base_metrics.impressions"), "50000");
    assert_eq!(row.get("recommendation.impact.base_metrics.cost_micros"), "25000000");
}

// ============================================================================
// Phase 6: SharedSet and SharedCriterion
// ============================================================================

#[test]
fn test_shared_set_all_fields() {
    use googleads_rs::google::ads::googleads::v19::enums::shared_set_type_enum::SharedSetType;
    use googleads_rs::google::ads::googleads::v19::enums::shared_set_status_enum::SharedSetStatus;

    let shared_set = SharedSet {
        id: 123456,
        name: "Negative Keywords List".to_string(),
        r#type: SharedSetType::NegativeKeywords as i32,
        status: SharedSetStatus::Enabled as i32,
        member_count: 42,
        resource_name: "customers/123/sharedSets/456".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_shared_set(shared_set)
        .build();

    assert_eq!(row.get("shared_set.id"), "123456");
    assert_eq!(row.get("shared_set.name"), "Negative Keywords List");
    assert_eq!(row.get("shared_set.type"), "NegativeKeywords");
    assert_eq!(row.get("shared_set.status"), "Enabled");
    assert_eq!(row.get("shared_set.member_count"), "42");
    assert_eq!(row.get("shared_set.resource_name"), "customers/123/sharedSets/456");
}

#[test]
fn test_shared_criterion_keyword() {
    use googleads_rs::google::ads::googleads::v19::resources::shared_criterion::Criterion;
    use googleads_rs::google::ads::googleads::v19::common::KeywordInfo;
    use googleads_rs::google::ads::googleads::v19::enums::criterion_type_enum::CriterionType;

    let shared_criterion = SharedCriterion {
        resource_name: "customers/123/sharedCriteria/456~789".to_string(),
        shared_set: "customers/123/sharedSets/456".to_string(),
        criterion_id: 789,
        r#type: CriterionType::Keyword as i32,
        criterion: Some(Criterion::Keyword(KeywordInfo {
            text: "cheap flights".to_string(),
            match_type: 2, // PHRASE
        })),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_shared_criterion(shared_criterion)
        .build();

    assert_eq!(row.get("shared_criterion.resource_name"), "customers/123/sharedCriteria/456~789");
    assert_eq!(row.get("shared_criterion.shared_set"), "customers/123/sharedSets/456");
    assert_eq!(row.get("shared_criterion.criterion_id"), "789");
    assert_eq!(row.get("shared_criterion.type"), "Keyword");
    assert_eq!(row.get("shared_criterion.keyword.text"), "cheap flights");
}

// ============================================================================
// Phase 6: CampaignSharedSet
// ============================================================================

#[test]
fn test_campaign_shared_set_all_fields() {
    use googleads_rs::google::ads::googleads::v19::enums::campaign_shared_set_status_enum::CampaignSharedSetStatus;

    let campaign_shared_set = CampaignSharedSet {
        resource_name: "customers/123/campaignSharedSets/456~789".to_string(),
        campaign: "customers/123/campaigns/456".to_string(),
        shared_set: "customers/123/sharedSets/789".to_string(),
        status: CampaignSharedSetStatus::Enabled as i32,
    };

    let row = GoogleAdsRowBuilder::new()
        .with_campaign_shared_set(campaign_shared_set)
        .build();

    assert_eq!(row.get("campaign_shared_set.resource_name"), "customers/123/campaignSharedSets/456~789");
    assert_eq!(row.get("campaign_shared_set.campaign"), "customers/123/campaigns/456");
    assert_eq!(row.get("campaign_shared_set.shared_set"), "customers/123/sharedSets/789");
    assert_eq!(row.get("campaign_shared_set.status"), "Enabled");
}

// ============================================================================
// Phase 7: Asset Performance Metrics
// ============================================================================

#[test]
fn test_asset_performance_metrics() {
    let metrics = Metrics {
        asset_best_performance_cost_percentage: 15.5,
        asset_best_performance_impression_percentage: 20.3,
        asset_good_performance_cost_percentage: 35.2,
        asset_good_performance_impression_percentage: 40.1,
        asset_learning_performance_cost_percentage: 25.0,
        asset_learning_performance_impression_percentage: 22.5,
        asset_low_performance_cost_percentage: 15.3,
        asset_low_performance_impression_percentage: 12.1,
        asset_unrated_performance_cost_percentage: 9.0,
        asset_unrated_performance_impression_percentage: 5.0,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.asset_best_performance_cost_percentage"), "15.5");
    assert_eq!(row.get("metrics.asset_best_performance_impression_percentage"), "20.3");
    assert_eq!(row.get("metrics.asset_good_performance_cost_percentage"), "35.2");
    assert_eq!(row.get("metrics.asset_good_performance_impression_percentage"), "40.1");
    assert_eq!(row.get("metrics.asset_learning_performance_cost_percentage"), "25");
    assert_eq!(row.get("metrics.asset_learning_performance_impression_percentage"), "22.5");
    assert_eq!(row.get("metrics.asset_low_performance_cost_percentage"), "15.3");
    assert_eq!(row.get("metrics.asset_low_performance_impression_percentage"), "12.1");
    assert_eq!(row.get("metrics.asset_unrated_performance_cost_percentage"), "9");
    assert_eq!(row.get("metrics.asset_unrated_performance_impression_percentage"), "5");
}

// ============================================================================
// Phase 7: Asset Pinning Metrics
// ============================================================================

#[test]
fn test_asset_pinning_metrics() {
    let metrics = Metrics {
        asset_pinned_as_description_position_one_count: 10,
        asset_pinned_as_description_position_two_count: 8,
        asset_pinned_as_headline_position_one_count: 15,
        asset_pinned_as_headline_position_two_count: 12,
        asset_pinned_as_headline_position_three_count: 9,
        asset_pinned_total_count: 54,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.asset_pinned_as_description_position_one_count"), "10");
    assert_eq!(row.get("metrics.asset_pinned_as_description_position_two_count"), "8");
    assert_eq!(row.get("metrics.asset_pinned_as_headline_position_one_count"), "15");
    assert_eq!(row.get("metrics.asset_pinned_as_headline_position_two_count"), "12");
    assert_eq!(row.get("metrics.asset_pinned_as_headline_position_three_count"), "9");
    assert_eq!(row.get("metrics.asset_pinned_total_count"), "54");
}

// ============================================================================
// Phase 7: Auction Insights Metrics
// ============================================================================

#[test]
fn test_auction_insights_metrics() {
    let metrics = Metrics {
        auction_insight_search_absolute_top_impression_percentage: 25.5,
        auction_insight_search_impression_share: 45.2,
        auction_insight_search_outranking_share: 60.1,
        auction_insight_search_overlap_rate: 15.3,
        auction_insight_search_position_above_rate: 35.8,
        auction_insight_search_top_impression_percentage: 50.0,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.auction_insight_search_absolute_top_impression_percentage"), "25.5");
    assert_eq!(row.get("metrics.auction_insight_search_impression_share"), "45.2");
    assert_eq!(row.get("metrics.auction_insight_search_outranking_share"), "60.1");
    assert_eq!(row.get("metrics.auction_insight_search_overlap_rate"), "15.3");
    assert_eq!(row.get("metrics.auction_insight_search_position_above_rate"), "35.8");
    assert_eq!(row.get("metrics.auction_insight_search_top_impression_percentage"), "50");
}

// ============================================================================
// Phase 7: Remaining Specialized Metrics
// ============================================================================

#[test]
fn test_specialized_metrics() {
    let metrics = Metrics {
        average_target_cpa_micros: 5000000,
        average_target_roas: 3.5,
        cross_device_conversions_value_micros: 125000000,
        general_invalid_click_rate: 0.02,
        general_invalid_clicks: 15,
        linked_entities_count: 42,
        publisher_organic_clicks: 100,
        publisher_purchased_clicks: 500,
        publisher_unknown_clicks: 25,
        sk_ad_network_total_conversions: 75,
        video_view_rate_in_feed: 0.15,
        video_view_rate_in_stream: 0.25,
        video_view_rate_shorts: 0.35,
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics)
        .build();

    assert_eq!(row.get("metrics.average_target_cpa_micros"), "5000000");
    assert_eq!(row.get("metrics.average_target_roas"), "3.5");
    assert_eq!(row.get("metrics.cross_device_conversions_value_micros"), "125000000");
    assert_eq!(row.get("metrics.general_invalid_click_rate"), "0.02");
    assert_eq!(row.get("metrics.general_invalid_clicks"), "15");
    assert_eq!(row.get("metrics.linked_entities_count"), "42");
    assert_eq!(row.get("metrics.publisher_organic_clicks"), "100");
    assert_eq!(row.get("metrics.publisher_purchased_clicks"), "500");
    assert_eq!(row.get("metrics.publisher_unknown_clicks"), "25");
    assert_eq!(row.get("metrics.sk_ad_network_total_conversions"), "75");
    assert_eq!(row.get("metrics.video_view_rate_in_feed"), "0.15");
    assert_eq!(row.get("metrics.video_view_rate_in_stream"), "0.25");
    assert_eq!(row.get("metrics.video_view_rate_shorts"), "0.35");
}

// ============================================================================
// Phase 3: Product Segments
// ============================================================================

#[test]
fn test_product_segments_categories() {
    let segments = Segments {
        product_category_level1: "Electronics".to_string(),
        product_category_level2: "Computers & Accessories".to_string(),
        product_category_level3: "Laptops".to_string(),
        product_category_level4: "Gaming Laptops".to_string(),
        product_category_level5: "High Performance".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.product_category_level1"), "Electronics");
    assert_eq!(row.get("segments.product_category_level2"), "Computers & Accessories");
    assert_eq!(row.get("segments.product_category_level3"), "Laptops");
    assert_eq!(row.get("segments.product_category_level4"), "Gaming Laptops");
    assert_eq!(row.get("segments.product_category_level5"), "High Performance");
}

#[test]
fn test_product_segments_types() {
    let segments = Segments {
        product_type_l1: "Apparel".to_string(),
        product_type_l2: "Men's Clothing".to_string(),
        product_type_l3: "Jackets".to_string(),
        product_type_l4: "Winter Jackets".to_string(),
        product_type_l5: "Down Jackets".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.product_type_l1"), "Apparel");
    assert_eq!(row.get("segments.product_type_l2"), "Men's Clothing");
    assert_eq!(row.get("segments.product_type_l3"), "Jackets");
    assert_eq!(row.get("segments.product_type_l4"), "Winter Jackets");
    assert_eq!(row.get("segments.product_type_l5"), "Down Jackets");
}

#[test]
fn test_product_segments_attributes() {
    use googleads_rs::google::ads::googleads::v19::enums::product_condition_enum::ProductCondition;

    let segments = Segments {
        product_brand: "Nike".to_string(),
        product_condition: ProductCondition::New as i32,
        product_country: "US".to_string(),
        product_language: "en".to_string(),
        product_merchant_id: 123456,
        product_title: "Nike Air Max Running Shoes".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.product_brand"), "Nike");
    assert_eq!(row.get("segments.product_condition"), "New");
    assert_eq!(row.get("segments.product_country"), "US");
    assert_eq!(row.get("segments.product_language"), "en");
    assert_eq!(row.get("segments.product_merchant_id"), "123456");
    assert_eq!(row.get("segments.product_title"), "Nike Air Max Running Shoes");
}

// ============================================================================
// Phase 3: Geo Target Segments
// ============================================================================

#[test]
fn test_geo_target_segments() {
    let segments = Segments {
        geo_target_city: "San Francisco".to_string(),
        geo_target_country: "United States".to_string(),
        geo_target_state: "California".to_string(),
        geo_target_postal_code: "94102".to_string(),
        geo_target_metro: "San Francisco-Oakland-San Jose CA".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.geo_target_city"), "San Francisco");
    assert_eq!(row.get("segments.geo_target_country"), "United States");
    assert_eq!(row.get("segments.geo_target_state"), "California");
    assert_eq!(row.get("segments.geo_target_postal_code"), "94102");
    assert_eq!(row.get("segments.geo_target_metro"), "San Francisco-Oakland-San Jose CA");
}

#[test]
fn test_geo_target_segments_additional() {
    let segments = Segments {
        geo_target_airport: "SFO".to_string(),
        geo_target_canton: "Zurich".to_string(),
        geo_target_county: "San Francisco County".to_string(),
        geo_target_district: "Financial District".to_string(),
        geo_target_province: "Ontario".to_string(),
        geo_target_region: "West Coast".to_string(),
        geo_target_most_specific_location: "94102, San Francisco, CA".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.geo_target_airport"), "SFO");
    assert_eq!(row.get("segments.geo_target_canton"), "Zurich");
    assert_eq!(row.get("segments.geo_target_county"), "San Francisco County");
    assert_eq!(row.get("segments.geo_target_district"), "Financial District");
    assert_eq!(row.get("segments.geo_target_province"), "Ontario");
    assert_eq!(row.get("segments.geo_target_region"), "West Coast");
    assert_eq!(row.get("segments.geo_target_most_specific_location"), "94102, San Francisco, CA");
}

// ============================================================================
// Phase 3: Resource Name Segments
// ============================================================================

#[test]
fn test_resource_name_segments() {
    let segments = Segments {
        campaign: "customers/123/campaigns/456".to_string(),
        ad_group: "customers/123/adGroups/789".to_string(),
        asset_group: "customers/123/assetGroups/012".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.campaign"), "customers/123/campaigns/456");
    assert_eq!(row.get("segments.ad_group"), "customers/123/adGroups/789");
    assert_eq!(row.get("segments.asset_group"), "customers/123/assetGroups/012");
}

// ============================================================================
// Phase 7: Hotel Segments
// ============================================================================

#[test]
fn test_hotel_segments() {
    use googleads_rs::google::ads::googleads::v19::enums::day_of_week_enum::DayOfWeek;
    use googleads_rs::google::ads::googleads::v19::enums::hotel_date_selection_type_enum::HotelDateSelectionType;
    use googleads_rs::google::ads::googleads::v19::enums::hotel_price_bucket_enum::HotelPriceBucket;
    use googleads_rs::google::ads::googleads::v19::enums::hotel_rate_type_enum::HotelRateType;

    let segments = Segments {
        hotel_booking_window_days: 14,
        hotel_center_id: 123456,
        hotel_check_in_date: "2024-12-25".to_string(),
        hotel_check_in_day_of_week: DayOfWeek::Wednesday as i32,
        hotel_city: "San Francisco".to_string(),
        hotel_class: 4,
        hotel_country: "US".to_string(),
        hotel_date_selection_type: HotelDateSelectionType::DefaultSelection as i32,
        hotel_length_of_stay: 3,
        hotel_price_bucket: HotelPriceBucket::LowestUnique as i32,
        hotel_rate_rule_id: "WEEKEND_SPECIAL".to_string(),
        hotel_rate_type: HotelRateType::QualifiedRate as i32,
        hotel_state: "CA".to_string(),
        partner_hotel_id: "HOTEL_987654".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.hotel_booking_window_days"), "14");
    assert_eq!(row.get("segments.hotel_center_id"), "123456");
    assert_eq!(row.get("segments.hotel_check_in_date"), "2024-12-25");
    assert_eq!(row.get("segments.hotel_check_in_day_of_week"), "Wednesday");
    assert_eq!(row.get("segments.hotel_city"), "San Francisco");
    assert_eq!(row.get("segments.hotel_class"), "4");
    assert_eq!(row.get("segments.hotel_country"), "US");
    assert_eq!(row.get("segments.hotel_date_selection_type"), "DefaultSelection");
    assert_eq!(row.get("segments.hotel_length_of_stay"), "3");
    assert_eq!(row.get("segments.hotel_price_bucket"), "LowestUnique");
    assert_eq!(row.get("segments.hotel_rate_rule_id"), "WEEKEND_SPECIAL");
    assert_eq!(row.get("segments.hotel_rate_type"), "QualifiedRate");
    assert_eq!(row.get("segments.hotel_state"), "CA");
    assert_eq!(row.get("segments.partner_hotel_id"), "HOTEL_987654");
}

// ============================================================================
// Phase 7: SKAdNetwork Segments
// ============================================================================

#[test]
fn test_sk_ad_network_segments() {
    use googleads_rs::google::ads::googleads::v19::enums::sk_ad_network_ad_event_type_enum::SkAdNetworkAdEventType;
    use googleads_rs::google::ads::googleads::v19::enums::sk_ad_network_attribution_credit_enum::SkAdNetworkAttributionCredit;
    use googleads_rs::google::ads::googleads::v19::enums::sk_ad_network_coarse_conversion_value_enum::SkAdNetworkCoarseConversionValue;
    use googleads_rs::google::ads::googleads::v19::enums::sk_ad_network_source_type_enum::SkAdNetworkSourceType;
    use googleads_rs::google::ads::googleads::v19::enums::sk_ad_network_user_type_enum::SkAdNetworkUserType;

    let segments = Segments {
        sk_ad_network_ad_event_type: SkAdNetworkAdEventType::Interaction as i32,
        sk_ad_network_attribution_credit: SkAdNetworkAttributionCredit::Won as i32,
        sk_ad_network_coarse_conversion_value: SkAdNetworkCoarseConversionValue::High as i32,
        sk_ad_network_fine_conversion_value: 42,
        sk_ad_network_postback_sequence_index: 1,
        sk_ad_network_redistributed_fine_conversion_value: 40,
        sk_ad_network_source_domain: "example.com".to_string(),
        sk_ad_network_source_type: SkAdNetworkSourceType::Website as i32,
        sk_ad_network_user_type: SkAdNetworkUserType::NewInstaller as i32,
        sk_ad_network_version: "4.0".to_string(),
        ..Default::default()
    };

    let row = GoogleAdsRowBuilder::new()
        .with_segments(segments)
        .build();

    assert_eq!(row.get("segments.sk_ad_network_ad_event_type"), "Interaction");
    assert_eq!(row.get("segments.sk_ad_network_attribution_credit"), "Won");
    assert_eq!(row.get("segments.sk_ad_network_coarse_conversion_value"), "High");
    assert_eq!(row.get("segments.sk_ad_network_fine_conversion_value"), "42");
    assert_eq!(row.get("segments.sk_ad_network_postback_sequence_index"), "1");
    assert_eq!(row.get("segments.sk_ad_network_redistributed_fine_conversion_value"), "40");
    assert_eq!(row.get("segments.sk_ad_network_source_domain"), "example.com");
    assert_eq!(row.get("segments.sk_ad_network_source_type"), "Website");
    assert_eq!(row.get("segments.sk_ad_network_user_type"), "NewInstaller");
    assert_eq!(row.get("segments.sk_ad_network_version"), "4.0");
}
