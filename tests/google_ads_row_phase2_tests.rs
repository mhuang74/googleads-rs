// Unit tests for GoogleAdsRow::get() method - Phase 2 Fields
//
// This module tests Phase 2 implementation:
// - E-commerce metrics (16 fields)
// - Location Asset metrics (15 fields)
// - Customer Acquisition metrics (4 fields)
//
// All fields use the attr_str! macro for scalar field extraction

#![allow(unused_imports)]

mod test_helpers;

use googleads_rs::google::ads::googleads::v22::common::Metrics;
use test_helpers::{GoogleAdsRowBuilder, MetricsBuilder};

// ============================================================================
// E-COMMERCE METRICS (Phase 2.1)
// ============================================================================

#[test]
fn test_metrics_average_cart_size() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.average_cart_size = 3.5;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.average_cart_size"), "3.5");
}

#[test]
fn test_metrics_average_order_value_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.average_order_value_micros = 75500000; // $75.50

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.average_order_value_micros"), "75500000");
}

#[test]
fn test_metrics_cost_of_goods_sold_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.cost_of_goods_sold_micros = 12300000; // $12.30

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.cost_of_goods_sold_micros"), "12300000");
}

#[test]
fn test_metrics_cross_sell_cost_of_goods_sold_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.cross_sell_cost_of_goods_sold_micros = 5400000; // $5.40

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.cross_sell_cost_of_goods_sold_micros"),
        "5400000"
    );
}

#[test]
fn test_metrics_cross_sell_gross_profit_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.cross_sell_gross_profit_micros = 8600000; // $8.60

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.cross_sell_gross_profit_micros"), "8600000");
}

#[test]
fn test_metrics_cross_sell_revenue_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.cross_sell_revenue_micros = 14000000; // $14.00

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.cross_sell_revenue_micros"), "14000000");
}

#[test]
fn test_metrics_cross_sell_units_sold() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.cross_sell_units_sold = 127.0;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.cross_sell_units_sold"), "127");
}

#[test]
fn test_metrics_gross_profit_margin() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.gross_profit_margin = 0.42; // 42%

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.gross_profit_margin"), "0.42");
}

#[test]
fn test_metrics_gross_profit_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.gross_profit_micros = 45000000; // $45.00

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.gross_profit_micros"), "45000000");
}

#[test]
fn test_metrics_lead_cost_of_goods_sold_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.lead_cost_of_goods_sold_micros = 3200000; // $3.20

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.lead_cost_of_goods_sold_micros"), "3200000");
}

#[test]
fn test_metrics_lead_gross_profit_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.lead_gross_profit_micros = 7800000; // $7.80

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.lead_gross_profit_micros"), "7800000");
}

#[test]
fn test_metrics_lead_revenue_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.lead_revenue_micros = 11000000; // $11.00

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.lead_revenue_micros"), "11000000");
}

#[test]
fn test_metrics_lead_units_sold() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.lead_units_sold = 89.0;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.lead_units_sold"), "89");
}

#[test]
fn test_metrics_orders() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.orders = 523.0;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.orders"), "523");
}

#[test]
fn test_metrics_revenue_micros() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.revenue_micros = 125000000; // $125.00

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.revenue_micros"), "125000000");
}

#[test]
fn test_metrics_units_sold() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.units_sold = 1247.0;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.units_sold"), "1247");
}

// ============================================================================
// LOCATION ASSET METRICS (Phase 2.2)
// ============================================================================

#[test]
fn test_metrics_all_conversions_from_location_asset_click_to_call() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.all_conversions_from_location_asset_click_to_call = 45.5;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.all_conversions_from_location_asset_click_to_call"),
        "45.5"
    );
}

#[test]
fn test_metrics_all_conversions_from_location_asset_directions() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.all_conversions_from_location_asset_directions = 125.3;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.all_conversions_from_location_asset_directions"),
        "125.3"
    );
}

#[test]
fn test_metrics_all_conversions_from_location_asset_menu() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.all_conversions_from_location_asset_menu = 32.7;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.all_conversions_from_location_asset_menu"),
        "32.7"
    );
}

#[test]
fn test_metrics_all_conversions_from_location_asset_order() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.all_conversions_from_location_asset_order = 78.2;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.all_conversions_from_location_asset_order"),
        "78.2"
    );
}

#[test]
fn test_metrics_all_conversions_from_location_asset_other_engagement() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.all_conversions_from_location_asset_other_engagement = 15.9;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.all_conversions_from_location_asset_other_engagement"),
        "15.9"
    );
}

#[test]
fn test_metrics_all_conversions_from_location_asset_store_visits() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.all_conversions_from_location_asset_store_visits = 234.6;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.all_conversions_from_location_asset_store_visits"),
        "234.6"
    );
}

#[test]
fn test_metrics_all_conversions_from_location_asset_website() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.all_conversions_from_location_asset_website = 189.4;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.all_conversions_from_location_asset_website"),
        "189.4"
    );
}

#[test]
fn test_metrics_eligible_impressions_from_location_asset_store_reach() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.eligible_impressions_from_location_asset_store_reach = 50000;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.eligible_impressions_from_location_asset_store_reach"),
        "50000"
    );
}

#[test]
fn test_metrics_view_through_conversions_from_location_asset_click_to_call() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.view_through_conversions_from_location_asset_click_to_call = 12.3;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.view_through_conversions_from_location_asset_click_to_call"),
        "12.3"
    );
}

#[test]
fn test_metrics_view_through_conversions_from_location_asset_directions() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.view_through_conversions_from_location_asset_directions = 28.7;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.view_through_conversions_from_location_asset_directions"),
        "28.7"
    );
}

#[test]
fn test_metrics_view_through_conversions_from_location_asset_menu() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.view_through_conversions_from_location_asset_menu = 7.1;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.view_through_conversions_from_location_asset_menu"),
        "7.1"
    );
}

#[test]
fn test_metrics_view_through_conversions_from_location_asset_order() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.view_through_conversions_from_location_asset_order = 19.5;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.view_through_conversions_from_location_asset_order"),
        "19.5"
    );
}

#[test]
fn test_metrics_view_through_conversions_from_location_asset_other_engagement() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.view_through_conversions_from_location_asset_other_engagement = 4.2;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.view_through_conversions_from_location_asset_other_engagement"),
        "4.2"
    );
}

#[test]
fn test_metrics_view_through_conversions_from_location_asset_store_visits() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.view_through_conversions_from_location_asset_store_visits = 56.8;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.view_through_conversions_from_location_asset_store_visits"),
        "56.8"
    );
}

#[test]
fn test_metrics_view_through_conversions_from_location_asset_website() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.view_through_conversions_from_location_asset_website = 43.9;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.view_through_conversions_from_location_asset_website"),
        "43.9"
    );
}

// ============================================================================
// CUSTOMER ACQUISITION METRICS (Phase 2.3)
// ============================================================================

#[test]
fn test_metrics_all_new_customer_lifetime_value() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.all_new_customer_lifetime_value = 15678.90;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.all_new_customer_lifetime_value"),
        "15678.9"
    );
}

#[test]
fn test_metrics_new_customer_lifetime_value() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.new_customer_lifetime_value = 12345.67;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.new_customer_lifetime_value"), "12345.67");
}

#[test]
fn test_metrics_average_impression_frequency_per_user() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.average_impression_frequency_per_user = 3.7;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(
        row.get("metrics.average_impression_frequency_per_user"),
        "3.7"
    );
}

#[test]
fn test_metrics_unique_users() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.unique_users = 125643;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.unique_users"), "125643");
}

// ============================================================================
// EDGE CASES AND COMBINED TESTS
// ============================================================================

#[test]
fn test_phase2_zero_values() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.orders = 0.0;
    metrics_with_field.revenue_micros = 0;
    metrics_with_field.unique_users = 0;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.orders"), "0");
    assert_eq!(row.get("metrics.revenue_micros"), "0");
    assert_eq!(row.get("metrics.unique_users"), "0");
}

#[test]
fn test_phase2_large_values() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.revenue_micros = 999999999999; // $999,999.99
    metrics_with_field.unique_users = 9999999;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    assert_eq!(row.get("metrics.revenue_micros"), "999999999999");
    assert_eq!(row.get("metrics.unique_users"), "9999999");
}

#[test]
fn test_phase2_decimal_precision() {
    let metrics = MetricsBuilder::new().build();
    let mut metrics_with_field = metrics;
    metrics_with_field.gross_profit_margin = 0.123456789;
    metrics_with_field.average_cart_size = 2.987654321;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_field)
        .build();

    let result_margin = row.get("metrics.gross_profit_margin");
    let result_cart = row.get("metrics.average_cart_size");

    // Verify precision is preserved (at least 6 decimal places)
    assert!(result_margin.starts_with("0.12345"));
    assert!(result_cart.starts_with("2.98765"));
}

#[test]
fn test_phase2_combined_with_existing_metrics() {
    let metrics = MetricsBuilder::new()
        .impressions(10000)
        .clicks(500)
        .conversions(25.5)
        .build();

    let mut metrics_with_phase2 = metrics;
    metrics_with_phase2.orders = 20.0;
    metrics_with_phase2.revenue_micros = 5000000; // $5.00
    metrics_with_phase2.unique_users = 1500;
    metrics_with_phase2.all_conversions_from_location_asset_store_visits = 10.2;

    let row = GoogleAdsRowBuilder::new()
        .with_metrics(metrics_with_phase2)
        .build();

    // Existing metrics still work
    assert_eq!(row.get("metrics.impressions"), "10000");
    assert_eq!(row.get("metrics.clicks"), "500");
    assert_eq!(row.get("metrics.conversions"), "25.5");

    // Phase 2 metrics work
    assert_eq!(row.get("metrics.orders"), "20");
    assert_eq!(row.get("metrics.revenue_micros"), "5000000");
    assert_eq!(row.get("metrics.unique_users"), "1500");
    assert_eq!(
        row.get("metrics.all_conversions_from_location_asset_store_visits"),
        "10.2"
    );
}

#[test]
fn test_phase2_not_implemented_field() {
    let metrics = MetricsBuilder::new().build();
    let row = GoogleAdsRowBuilder::new().with_metrics(metrics).build();

    // Non-existent field should return "not implemented"
    assert_eq!(
        row.get("metrics.nonexistent_phase2_field"),
        "not implemented by googleads-rs"
    );
}

#[test]
fn test_phase2_metrics_absent_parent() {
    // Create row without metrics
    let row = GoogleAdsRowBuilder::new().build();

    // All Phase 2 metric fields should panic (as they use attr_str! which calls unwrap())
    // This is expected behavior for non-optional resources
    let result = std::panic::catch_unwind(|| row.get("metrics.orders"));
    assert!(
        result.is_err(),
        "Expected panic when metrics parent is absent"
    );
}
