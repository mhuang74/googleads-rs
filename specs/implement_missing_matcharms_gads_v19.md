# Implementation Plan: Missing GoogleAdsRow Match Arms for Google Ads API V19

## Executive Summary

This document outlines the implementation plan for adding missing match arms to `GoogleAdsRow::get()` in `src/lib.rs`. Based on analysis of the Google Ads API V19 field reference, we need to implement **406 additional fields** (71.5% of total fields).

**Current Status:**
- **Total Available Fields**: 568 (189 resources, 283 metrics, 96 segments)
- **Currently Implemented**: 162 fields (28.5%)
- **Missing**: 406 fields (71.5%)

**Breakdown by Category:**
- **Resources**: 18/189 implemented (9.5%) → **171 missing**
- **Metrics**: 124/283 implemented (43.8%) → **159 missing**
- **Segments**: 20/96 implemented (20.8%) → **76 missing**

---

## Phase 1: Core Resources (Priority: Critical)

**Target**: 50-60 fields | **Effort**: High | **Impact**: Critical

### 1.1 Conversion Tracking Resources

#### conversion_action
**Fields to implement**: ~8-10 fields
- `conversion_action.id` → attr_str
- `conversion_action.name` → attr_str
- `conversion_action.status` → method_str
- `conversion_action.type` → method_str
- `conversion_action.category` → method_str
- `conversion_action.counting_type` → method_str
- `conversion_action.include_in_conversions_metric` → attr_str
- `conversion_action.value_settings.default_value` → attr_str

**Macro Pattern**: Non-optional (conversion_action present when selected)

#### conversion_custom_variable
**Fields to implement**: ~4-5 fields
- `conversion_custom_variable.id` → attr_str
- `conversion_custom_variable.name` → attr_str
- `conversion_custom_variable.tag` → attr_str
- `conversion_custom_variable.status` → method_str

### 1.2 Asset Management

#### asset
**Fields to implement**: ~8-12 fields
- `asset.id` → attr_str
- `asset.name` → attr_str
- `asset.type` → method_str
- `asset.resource_name` → attr_str
- `asset.text_asset.text` → optional_enum_match_str (oneof)
- `asset.image_asset.full_size.url` → optional_enum_match_str (oneof)
- `asset.youtube_video_asset.youtube_video_id` → optional_enum_match_str (oneof)

**Macro Pattern**: Non-optional parent, oneof for asset type data

#### campaign_asset
**Fields to implement**: ~6-8 fields
- `campaign_asset.resource_name` → attr_str
- `campaign_asset.campaign` → attr_str
- `campaign_asset.asset` → attr_str
- `campaign_asset.field_type` → method_str
- `campaign_asset.status` → method_str

#### ad_group_asset
**Fields to implement**: ~6-8 fields
- `ad_group_asset.resource_name` → attr_str
- `ad_group_asset.ad_group` → attr_str
- `ad_group_asset.asset` → attr_str
- `ad_group_asset.field_type` → method_str
- `ad_group_asset.status` → method_str

#### customer_asset
**Fields to implement**: ~6-8 fields
- `customer_asset.resource_name` → attr_str
- `customer_asset.asset` → attr_str
- `customer_asset.field_type` → method_str
- `customer_asset.status` → method_str

### 1.3 Audience & Targeting

#### user_list
**Fields to implement**: ~8-10 fields
- `user_list.id` → attr_str
- `user_list.name` → attr_str
- `user_list.description` → attr_str
- `user_list.membership_status` → method_str
- `user_list.size_for_display` → attr_str
- `user_list.size_for_search` → attr_str
- `user_list.type` → method_str
- `user_list.match_rate_percentage` → attr_str

#### geo_target_constant
**Fields to implement**: ~6-8 fields
- `geo_target_constant.id` → attr_str
- `geo_target_constant.name` → attr_str
- `geo_target_constant.canonical_name` → attr_str
- `geo_target_constant.country_code` → attr_str
- `geo_target_constant.target_type` → attr_str
- `geo_target_constant.status` → method_str

---

## Phase 2: E-commerce & Location Metrics (Priority: High)

**Target**: 38 fields | **Effort**: Medium | **Impact**: High

### 2.1 E-commerce Metrics (12 fields)

All use `attr_str([metrics], field_name)` pattern:

```rust
"metrics.average_cart_size" => attr_str!([metrics], average_cart_size),
"metrics.average_order_value_micros" => attr_str!([metrics], average_order_value_micros),
"metrics.cost_of_goods_sold_micros" => attr_str!([metrics], cost_of_goods_sold_micros),
"metrics.cross_sell_cost_of_goods_sold_micros" => attr_str!([metrics], cross_sell_cost_of_goods_sold_micros),
"metrics.cross_sell_gross_profit_micros" => attr_str!([metrics], cross_sell_gross_profit_micros),
"metrics.cross_sell_revenue_micros" => attr_str!([metrics], cross_sell_revenue_micros),
"metrics.cross_sell_units_sold" => attr_str!([metrics], cross_sell_units_sold),
"metrics.gross_profit_margin" => attr_str!([metrics], gross_profit_margin),
"metrics.gross_profit_micros" => attr_str!([metrics], gross_profit_micros),
"metrics.lead_cost_of_goods_sold_micros" => attr_str!([metrics], lead_cost_of_goods_sold_micros),
"metrics.lead_gross_profit_micros" => attr_str!([metrics], lead_gross_profit_micros),
"metrics.lead_revenue_micros" => attr_str!([metrics], lead_revenue_micros),
"metrics.lead_units_sold" => attr_str!([metrics], lead_units_sold),
"metrics.orders" => attr_str!([metrics], orders),
"metrics.revenue_micros" => attr_str!([metrics], revenue_micros),
"metrics.units_sold" => attr_str!([metrics], units_sold),
```

### 2.2 Location Asset Metrics (14 fields)

All use `attr_str([metrics], field_name)` pattern:

```rust
"metrics.all_conversions_from_location_asset_click_to_call" => attr_str!([metrics], all_conversions_from_location_asset_click_to_call),
"metrics.all_conversions_from_location_asset_directions" => attr_str!([metrics], all_conversions_from_location_asset_directions),
"metrics.all_conversions_from_location_asset_menu" => attr_str!([metrics], all_conversions_from_location_asset_menu),
"metrics.all_conversions_from_location_asset_order" => attr_str!([metrics], all_conversions_from_location_asset_order),
"metrics.all_conversions_from_location_asset_other_engagement" => attr_str!([metrics], all_conversions_from_location_asset_other_engagement),
"metrics.all_conversions_from_location_asset_store_visits" => attr_str!([metrics], all_conversions_from_location_asset_store_visits),
"metrics.all_conversions_from_location_asset_website" => attr_str!([metrics], all_conversions_from_location_asset_website),
"metrics.eligible_impressions_from_location_asset_store_reach" => attr_str!([metrics], eligible_impressions_from_location_asset_store_reach),
"metrics.view_through_conversions_from_location_asset_click_to_call" => attr_str!([metrics], view_through_conversions_from_location_asset_click_to_call),
"metrics.view_through_conversions_from_location_asset_directions" => attr_str!([metrics], view_through_conversions_from_location_asset_directions),
"metrics.view_through_conversions_from_location_asset_menu" => attr_str!([metrics], view_through_conversions_from_location_asset_menu),
"metrics.view_through_conversions_from_location_asset_order" => attr_str!([metrics], view_through_conversions_from_location_asset_order),
"metrics.view_through_conversions_from_location_asset_other_engagement" => attr_str!([metrics], view_through_conversions_from_location_asset_other_engagement),
"metrics.view_through_conversions_from_location_asset_store_visits" => attr_str!([metrics], view_through_conversions_from_location_asset_store_visits),
"metrics.view_through_conversions_from_location_asset_website" => attr_str!([metrics], view_through_conversions_from_location_asset_website),
```

### 2.3 Customer Acquisition Metrics (12 fields)

```rust
"metrics.all_new_customer_lifetime_value" => attr_str!([metrics], all_new_customer_lifetime_value),
"metrics.new_customer_lifetime_value" => attr_str!([metrics], new_customer_lifetime_value),
"metrics.average_impression_frequency_per_user" => attr_str!([metrics], average_impression_frequency_per_user),
"metrics.unique_users" => attr_str!([metrics], unique_users),
```

---

## Phase 3: Product & Geo Segments (Priority: High)

**Target**: 31 fields | **Effort**: Low-Medium | **Impact**: High

### 3.1 Product Segments (18 fields)

All use `attr_str([segments], field_name)` pattern:

```rust
// Product categories
"segments.product_category_level1" => attr_str!([segments], product_category_level1),
"segments.product_category_level2" => attr_str!([segments], product_category_level2),
"segments.product_category_level3" => attr_str!([segments], product_category_level3),
"segments.product_category_level4" => attr_str!([segments], product_category_level4),
"segments.product_category_level5" => attr_str!([segments], product_category_level5),

// Product types
"segments.product_type_l1" => attr_str!([segments], product_type_l1),
"segments.product_type_l2" => attr_str!([segments], product_type_l2),
"segments.product_type_l3" => attr_str!([segments], product_type_l3),
"segments.product_type_l4" => attr_str!([segments], product_type_l4),
"segments.product_type_l5" => attr_str!([segments], product_type_l5),

// Product attributes
"segments.product_aggregator_id" => attr_str!([segments], product_aggregator_id),
"segments.product_brand" => attr_str!([segments], product_brand),
"segments.product_condition" => method_str!([segments], product_condition),
"segments.product_country" => attr_str!([segments], product_country),
"segments.product_custom_attribute0" => attr_str!([segments], product_custom_attribute0),
"segments.product_custom_attribute1" => attr_str!([segments], product_custom_attribute1),
"segments.product_custom_attribute2" => attr_str!([segments], product_custom_attribute2),
"segments.product_custom_attribute3" => attr_str!([segments], product_custom_attribute3),
"segments.product_custom_attribute4" => attr_str!([segments], product_custom_attribute4),
"segments.product_feed_label" => attr_str!([segments], product_feed_label),
"segments.product_language" => attr_str!([segments], product_language),
"segments.product_merchant_id" => attr_str!([segments], product_merchant_id),
"segments.product_store_id" => attr_str!([segments], product_store_id),
"segments.product_title" => attr_str!([segments], product_title),
```

### 3.2 Geo Target Segments (11 fields)

```rust
"segments.geo_target_airport" => attr_str!([segments], geo_target_airport),
"segments.geo_target_canton" => attr_str!([segments], geo_target_canton),
"segments.geo_target_city" => attr_str!([segments], geo_target_city),
"segments.geo_target_country" => attr_str!([segments], geo_target_country),
"segments.geo_target_county" => attr_str!([segments], geo_target_county),
"segments.geo_target_district" => attr_str!([segments], geo_target_district),
"segments.geo_target_metro" => attr_str!([segments], geo_target_metro),
"segments.geo_target_most_specific_location" => attr_str!([segments], geo_target_most_specific_location),
"segments.geo_target_postal_code" => attr_str!([segments], geo_target_postal_code),
"segments.geo_target_province" => attr_str!([segments], geo_target_province),
"segments.geo_target_region" => attr_str!([segments], geo_target_region),
"segments.geo_target_state" => attr_str!([segments], geo_target_state),
```

### 3.3 Resource Name Segments (2 fields)

```rust
"segments.campaign" => attr_str!([segments], campaign),
"segments.ad_group" => attr_str!([segments], ad_group),
"segments.asset_group" => attr_str!([segments], asset_group),
```

---

## Phase 4: Reporting Views (Priority: Medium-High)

**Target**: 15-20 fields | **Effort**: Medium | **Impact**: Medium-High

### 4.1 keyword_view
```rust
"keyword_view.resource_name" => attr_str!([keyword_view], resource_name),
```

### 4.2 landing_page_view
```rust
"landing_page_view.resource_name" => attr_str!([landing_page_view], resource_name),
"landing_page_view.unexpanded_final_url" => attr_str!([landing_page_view], unexpanded_final_url),
```

### 4.3 geographic_view
```rust
"geographic_view.resource_name" => attr_str!([geographic_view], resource_name),
"geographic_view.location_type" => method_str!([geographic_view], location_type),
"geographic_view.country_criterion_id" => attr_str!([geographic_view], country_criterion_id),
```

### 4.4 click_view
```rust
"click_view.resource_name" => attr_str!([click_view], resource_name),
"click_view.gclid" => attr_str!([click_view], gclid),
"click_view.area_of_interest.city" => attr_str!([click_view, area_of_interest], city),
"click_view.area_of_interest.country" => attr_str!([click_view, area_of_interest], country),
"click_view.location_of_presence.city" => attr_str!([click_view, location_of_presence], city),
"click_view.location_of_presence.country" => attr_str!([click_view, location_of_presence], country),
```

---

## Phase 5: Asset & Label Resources (Priority: Medium)

**Target**: 15-20 fields | **Effort**: Low-Medium | **Impact**: Medium

### 5.1 asset_group_asset
```rust
"asset_group_asset.resource_name" => attr_str!([asset_group_asset], resource_name),
"asset_group_asset.asset_group" => attr_str!([asset_group_asset], asset_group),
"asset_group_asset.asset" => attr_str!([asset_group_asset], asset),
"asset_group_asset.field_type" => method_str!([asset_group_asset], field_type),
"asset_group_asset.status" => method_str!([asset_group_asset], status),
"asset_group_asset.performance_label" => method_str!([asset_group_asset], performance_label),
```

### 5.2 asset_group_signal
```rust
"asset_group_signal.resource_name" => attr_str!([asset_group_signal], resource_name),
"asset_group_signal.asset_group" => attr_str!([asset_group_signal], asset_group),
"asset_group_signal.audience.audience" => attr_str!([asset_group_signal, audience], audience),
```

### 5.3 Label Relations
```rust
"campaign_label.resource_name" => attr_str!([campaign_label], resource_name),
"campaign_label.campaign" => attr_str!([campaign_label], campaign),
"campaign_label.label" => attr_str!([campaign_label], label),

"ad_group_label.resource_name" => attr_str!([ad_group_label], resource_name),
"ad_group_label.ad_group" => attr_str!([ad_group_label], ad_group),
"ad_group_label.label" => attr_str!([ad_group_label], label),

"ad_group_ad_label.resource_name" => attr_str!([ad_group_ad_label], resource_name),
"ad_group_ad_label.ad_group_ad" => attr_str!([ad_group_ad_label], ad_group_ad),
"ad_group_ad_label.label" => attr_str!([ad_group_ad_label], label),
```

---

## Phase 6: Remaining High-Value Resources (Priority: Medium)

**Target**: 40-50 fields | **Effort**: High | **Impact**: Medium

### 6.1 recommendation
```rust
"recommendation.resource_name" => attr_str!([recommendation], resource_name),
"recommendation.type" => method_str!([recommendation], r#type),
"recommendation.impact.base_metrics.clicks" => attr_str!([recommendation, impact, base_metrics], clicks),
"recommendation.impact.base_metrics.impressions" => attr_str!([recommendation, impact, base_metrics], impressions),
"recommendation.impact.base_metrics.cost_micros" => attr_str!([recommendation, impact, base_metrics], cost_micros),
```

### 6.2 campaign_shared_set
```rust
"campaign_shared_set.resource_name" => attr_str!([campaign_shared_set], resource_name),
"campaign_shared_set.campaign" => attr_str!([campaign_shared_set], campaign),
"campaign_shared_set.shared_set" => attr_str!([campaign_shared_set], shared_set),
"campaign_shared_set.status" => method_str!([campaign_shared_set], status),
```

### 6.3 shared_set & shared_criterion
```rust
"shared_set.id" => attr_str!([shared_set], id),
"shared_set.name" => attr_str!([shared_set], name),
"shared_set.type" => method_str!([shared_set], r#type),
"shared_set.status" => method_str!([shared_set], status),
"shared_set.member_count" => attr_str!([shared_set], member_count),

"shared_criterion.resource_name" => attr_str!([shared_criterion], resource_name),
"shared_criterion.shared_set" => attr_str!([shared_criterion], shared_set),
"shared_criterion.criterion_id" => attr_str!([shared_criterion], criterion_id),
"shared_criterion.type" => method_str!([shared_criterion], r#type),
"shared_criterion.keyword.text" => enum_match_str!([shared_criterion], criterion, Keyword, text),
```

### 6.4 feed & feed_item
```rust
"feed.id" => attr_str!([feed], id),
"feed.name" => attr_str!([feed], name),
"feed.status" => method_str!([feed], status),

"feed_item.id" => attr_str!([feed_item], id),
"feed_item.feed" => attr_str!([feed_item], feed),
"feed_item.status" => method_str!([feed_item], status),
```

---

## Phase 7: Specialized Metrics & Segments (Priority: Low-Medium)

**Target**: 60-70 fields | **Effort**: Low | **Impact**: Low-Medium

### 7.1 Asset Performance Metrics (10 fields)

```rust
"metrics.asset_best_performance_cost_percentage" => attr_str!([metrics], asset_best_performance_cost_percentage),
"metrics.asset_best_performance_impression_percentage" => attr_str!([metrics], asset_best_performance_impression_percentage),
"metrics.asset_good_performance_cost_percentage" => attr_str!([metrics], asset_good_performance_cost_percentage),
"metrics.asset_good_performance_impression_percentage" => attr_str!([metrics], asset_good_performance_impression_percentage),
"metrics.asset_learning_performance_cost_percentage" => attr_str!([metrics], asset_learning_performance_cost_percentage),
"metrics.asset_learning_performance_impression_percentage" => attr_str!([metrics], asset_learning_performance_impression_percentage),
"metrics.asset_low_performance_cost_percentage" => attr_str!([metrics], asset_low_performance_cost_percentage),
"metrics.asset_low_performance_impression_percentage" => attr_str!([metrics], asset_low_performance_impression_percentage),
"metrics.asset_unrated_performance_cost_percentage" => attr_str!([metrics], asset_unrated_performance_cost_percentage),
"metrics.asset_unrated_performance_impression_percentage" => attr_str!([metrics], asset_unrated_performance_impression_percentage),
```

### 7.2 Asset Pinning Metrics (7 fields)

```rust
"metrics.asset_pinned_as_description_position_one_count" => attr_str!([metrics], asset_pinned_as_description_position_one_count),
"metrics.asset_pinned_as_description_position_two_count" => attr_str!([metrics], asset_pinned_as_description_position_two_count),
"metrics.asset_pinned_as_headline_position_one_count" => attr_str!([metrics], asset_pinned_as_headline_position_one_count),
"metrics.asset_pinned_as_headline_position_two_count" => attr_str!([metrics], asset_pinned_as_headline_position_two_count),
"metrics.asset_pinned_as_headline_position_three_count" => attr_str!([metrics], asset_pinned_as_headline_position_three_count),
"metrics.asset_pinned_total_count" => attr_str!([metrics], asset_pinned_total_count),
```

### 7.3 Auction Insights Metrics (6 fields)

```rust
"metrics.auction_insight_search_absolute_top_impression_percentage" => attr_str!([metrics], auction_insight_search_absolute_top_impression_percentage),
"metrics.auction_insight_search_impression_share" => attr_str!([metrics], auction_insight_search_impression_share),
"metrics.auction_insight_search_outranking_share" => attr_str!([metrics], auction_insight_search_outranking_share),
"metrics.auction_insight_search_overlap_rate" => attr_str!([metrics], auction_insight_search_overlap_rate),
"metrics.auction_insight_search_position_above_rate" => attr_str!([metrics], auction_insight_search_position_above_rate),
"metrics.auction_insight_search_top_impression_percentage" => attr_str!([metrics], auction_insight_search_top_impression_percentage),
```

### 7.4 Hotel Segments (10 fields)

```rust
"segments.hotel_booking_window_days" => attr_str!([segments], hotel_booking_window_days),
"segments.hotel_center_id" => attr_str!([segments], hotel_center_id),
"segments.hotel_check_in_date" => attr_str!([segments], hotel_check_in_date),
"segments.hotel_check_in_day_of_week" => method_str!([segments], hotel_check_in_day_of_week),
"segments.hotel_city" => attr_str!([segments], hotel_city),
"segments.hotel_class" => attr_str!([segments], hotel_class),
"segments.hotel_country" => attr_str!([segments], hotel_country),
"segments.hotel_date_selection_type" => method_str!([segments], hotel_date_selection_type),
"segments.hotel_length_of_stay" => attr_str!([segments], hotel_length_of_stay),
"segments.hotel_price_bucket" => method_str!([segments], hotel_price_bucket),
"segments.hotel_rate_rule_id" => attr_str!([segments], hotel_rate_rule_id),
"segments.hotel_rate_type" => method_str!([segments], hotel_rate_type),
"segments.hotel_state" => attr_str!([segments], hotel_state),
"segments.partner_hotel_id" => attr_str!([segments], partner_hotel_id),
```

### 7.5 SKAdNetwork Segments (9 fields)

```rust
"segments.sk_ad_network_ad_event_type" => method_str!([segments], sk_ad_network_ad_event_type),
"segments.sk_ad_network_attribution_credit" => method_str!([segments], sk_ad_network_attribution_credit),
"segments.sk_ad_network_coarse_conversion_value" => method_str!([segments], sk_ad_network_coarse_conversion_value),
"segments.sk_ad_network_fine_conversion_value" => attr_str!([segments], sk_ad_network_fine_conversion_value),
"segments.sk_ad_network_postback_sequence_index" => attr_str!([segments], sk_ad_network_postback_sequence_index),
"segments.sk_ad_network_redistributed_fine_conversion_value" => attr_str!([segments], sk_ad_network_redistributed_fine_conversion_value),
"segments.sk_ad_network_source_domain" => attr_str!([segments], sk_ad_network_source_domain),
"segments.sk_ad_network_source_type" => method_str!([segments], sk_ad_network_source_type),
"segments.sk_ad_network_user_type" => method_str!([segments], sk_ad_network_user_type),
"segments.sk_ad_network_version" => attr_str!([segments], sk_ad_network_version),
```

### 7.6 Remaining Metrics (20+ fields)

```rust
"metrics.average_target_cpa_micros" => attr_str!([metrics], average_target_cpa_micros),
"metrics.average_target_roas" => attr_str!([metrics], average_target_roas),
"metrics.cross_device_conversions_value_micros" => attr_str!([metrics], cross_device_conversions_value_micros),
"metrics.general_invalid_click_rate" => attr_str!([metrics], general_invalid_click_rate),
"metrics.general_invalid_clicks" => attr_str!([metrics], general_invalid_clicks),
"metrics.linked_entities_count" => attr_str!([metrics], linked_entities_count),
"metrics.publisher_organic_clicks" => attr_str!([metrics], publisher_organic_clicks),
"metrics.publisher_purchased_clicks" => attr_str!([metrics], publisher_purchased_clicks),
"metrics.publisher_unknown_clicks" => attr_str!([metrics], publisher_unknown_clicks),
"metrics.results_conversions_purchase" => attr_str!([metrics], results_conversions_purchase),
"metrics.sk_ad_network_total_conversions" => attr_str!([metrics], sk_ad_network_total_conversions),
"metrics.store_visits_last_click_model_attributed_conversions" => attr_str!([metrics], store_visits_last_click_model_attributed_conversions),
"metrics.video_view_rate_in_feed" => attr_str!([metrics], video_view_rate_in_feed),
"metrics.video_view_rate_in_stream" => attr_str!([metrics], video_view_rate_in_stream),
"metrics.video_view_rate_shorts" => attr_str!([metrics], video_view_rate_shorts),
```

---

## Phase 8: Specialized Resources (Priority: As Needed)

**Target**: 100+ fields | **Effort**: Very High | **Impact**: Variable

This phase includes specialized resources that are used less frequently but may be critical for specific use cases:

### 8.1 Performance Max Resources
- asset_group_listing_group_filter
- asset_group_product_group_view
- asset_group_top_combination_view

### 8.2 Shopping Resources
- shopping_performance_view
- shopping_product
- product_group_view

### 8.3 Hotel Resources
- hotel_group_view
- hotel_performance_view
- hotel_reconciliation

### 8.4 Local Services Resources
- local_services_lead
- local_services_employee
- local_services_lead_conversation
- local_services_verification_artifact

### 8.5 Experiment Resources
- experiment
- experiment_arm
- campaign_draft

### 8.6 Budget & Billing Resources
- account_budget_proposal
- billing_setup
- campaign_group

### 8.7 Bidding Resources
- bidding_data_exclusion
- bidding_seasonality_adjustment
- bidding_strategy_simulation
- campaign_simulation
- ad_group_simulation

### 8.8 Other Specialized Resources
- batch_job
- offline_user_data_job
- offline_conversion_upload_client_summary
- offline_conversion_upload_conversion_action_summary
- carrier_constant
- currency_constant
- language_constant
- mobile_app_category_constant
- mobile_device_constant
- operating_system_version_constant
- product_category_constant
- topic_constant

---

## Implementation Guidelines

### Macro Selection Decision Tree

```
Is the parent resource optional (might be absent)?
├─ YES → Use optional_* macros
│   ├─ Is it a scalar field? → optional_attr_str!(parent, field)
│   ├─ Is it an enum with accessor? → optional_method_str!(parent, method)
│   └─ Is it a oneof variant? → optional_enum_match_str!(parent, match_field, Variant, attr)
│
└─ NO → Use non-optional macros
    ├─ Is it a scalar field? → attr_str!([parent], field) or attr_str!([parent, nested], field)
    ├─ Is it an enum with accessor? → method_str!([parent], method)
    ├─ Is it a oneof variant? → enum_match_str!([parent], match_field, Variant, attr)
    └─ Is it repeated nested in oneof? → enum_match_iterator_str!([parent], match_field, Variant, iterator, attr)
```

### Testing Strategy

For each phase:
1. **Unit Tests**: Add tests in `tests/` directory with sample GoogleAdsRow instances
2. **Integration Tests**: Test with real GAQL queries (if available)
3. **Regression Tests**: Ensure existing match arms continue to work
4. **Coverage Tests**: Verify all new paths return values (not "not implemented")

### Code Organization

Add match arms in alphabetical order within each category:
1. Resources (alphabetically by resource name)
2. Metrics (alphabetically by metric name)
3. Segments (alphabetically by segment name)

### Documentation

For each new resource block:
```rust
// ===== RESOURCE_NAME =====
"resource_name.field1" => attr_str!([resource_name], field1),
"resource_name.field2" => method_str!([resource_name], field2),
```

---

## Implementation Effort Estimates

| Phase | Fields | Complexity | Estimated Hours | Priority |
|-------|--------|------------|-----------------|----------|
| Phase 1 | 50-60 | High | 16-20 hours | Critical |
| Phase 2 | 38 | Medium | 4-6 hours | High |
| Phase 3 | 31 | Low-Medium | 3-4 hours | High |
| Phase 4 | 15-20 | Medium | 4-6 hours | Medium-High |
| Phase 5 | 15-20 | Low-Medium | 3-4 hours | Medium |
| Phase 6 | 40-50 | High | 12-16 hours | Medium |
| Phase 7 | 60-70 | Low | 6-8 hours | Low-Medium |
| Phase 8 | 100+ | Very High | 30-40 hours | As Needed |
| **Total** | **406** | - | **78-104 hours** | - |

---

## Quick Reference: Top 20 Most Common Missing Fields

Based on typical Google Ads reporting queries:

1. `conversion_action.name` - Essential for conversion tracking
2. `conversion_action.category` - Grouping conversions
3. `asset.name` - Asset reporting
4. `campaign_asset.field_type` - Asset placement
5. `ad_group_asset.field_type` - Asset placement
6. `user_list.name` - Audience reporting
7. `geo_target_constant.name` - Location reporting
8. `segments.product_category_level1` - Shopping campaigns
9. `segments.product_brand` - Shopping campaigns
10. `segments.geo_target_country` - Location segmentation
11. `segments.campaign` - Cross-resource analysis
12. `segments.ad_group` - Cross-resource analysis
13. `metrics.orders` - E-commerce tracking
14. `metrics.revenue_micros` - E-commerce tracking
15. `metrics.average_order_value_micros` - E-commerce analysis
16. `keyword_view.resource_name` - Keyword reporting
17. `landing_page_view.unexpanded_final_url` - Landing page analysis
18. `geographic_view.location_type` - Geographic reporting
19. `click_view.gclid` - Click tracking
20. `asset_group_asset.performance_label` - Performance Max reporting

---

## Success Metrics

- **Coverage**: Increase from 28.5% to 95%+ field coverage
- **Resource Coverage**: From 18/189 (9.5%) to 80+ resources (42%+)
- **Metrics Coverage**: From 124/283 (43.8%) to 250+ metrics (88%+)
- **Segments Coverage**: From 20/96 (20.8%) to 85+ segments (88%+)

---

## References

- **Field Reference**: `specs/all_googleadsrow_fields_and_metrics_v19.md`
- **Implementation Guide**: `specs/how_to_implement_get_matcharms.md`
- **Current Implementation**: `src/lib.rs` (lines 65-505)
- **Analysis Document**: `specs/missing_fields_analysis.md`

---

## Maintenance Notes

- Keep match arms in alphabetical order within each category
- Document any custom enum mappings (like bidding_strategy_type)
- Update this plan as phases are completed
- Add phase completion dates and actual effort hours
- Track any discovered issues or edge cases
