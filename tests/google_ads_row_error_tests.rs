// Unit tests for GoogleAdsRow::get() method - Error Conditions and Edge Cases
//
// This module tests error handling, unimplemented paths, and edge cases

#![allow(unused_imports)]

mod test_helpers;

use test_helpers::{
    AdGroupBuilder, CampaignBuilder, GoogleAdsRowBuilder, MetricsBuilder, SegmentsBuilder,
};

// ============================================================================
// Unimplemented Field Paths
// ============================================================================

#[test]
fn test_unimplemented_field_path_returns_not_implemented() {
    let row = GoogleAdsRowBuilder::new().build();

    assert_eq!(
        row.get("unknown.field.path"),
        "not implemented by googleads-rs"
    );
}

#[test]
fn test_unimplemented_campaign_field() {
    let campaign = CampaignBuilder::new().id(123).build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Test a field that exists in the proto but isn't implemented in get()
    assert_eq!(
        row.get("campaign.nonexistent_field"),
        "not implemented by googleads-rs"
    );
}

#[test]
fn test_unimplemented_resource() {
    let row = GoogleAdsRowBuilder::new().build();

    assert_eq!(
        row.get("unimplemented_resource.field"),
        "not implemented by googleads-rs"
    );
}

#[test]
fn test_partial_field_path() {
    let campaign = CampaignBuilder::new().id(123).build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Incomplete path
    assert_eq!(row.get("campaign"), "not implemented by googleads-rs");
}

#[test]
fn test_typo_in_field_name() {
    let campaign = CampaignBuilder::new().id(123).build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Common typo: "campaing" instead of "campaign"
    assert_eq!(row.get("campaing.id"), "not implemented by googleads-rs");
}

#[test]
fn test_case_sensitive_field_names() {
    let campaign = CampaignBuilder::new().id(123).build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Field names are case-sensitive
    assert_eq!(row.get("Campaign.ID"), "not implemented by googleads-rs");

    assert_eq!(row.get("CAMPAIGN.ID"), "not implemented by googleads-rs");
}

#[test]
fn test_extra_dots_in_path() {
    let campaign = CampaignBuilder::new().id(123).build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign..id"), "not implemented by googleads-rs");

    assert_eq!(row.get("campaign.id."), "not implemented by googleads-rs");
}

#[test]
fn test_empty_string_field_path() {
    let row = GoogleAdsRowBuilder::new().build();

    assert_eq!(row.get(""), "not implemented by googleads-rs");
}

#[test]
fn test_whitespace_in_field_path() {
    let campaign = CampaignBuilder::new().id(123).build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get(" campaign.id"), "not implemented by googleads-rs");

    assert_eq!(row.get("campaign.id "), "not implemented by googleads-rs");

    assert_eq!(row.get("campaign .id"), "not implemented by googleads-rs");
}

// ============================================================================
// Missing Optional Parents
// ============================================================================

#[test]
fn test_accessing_field_when_parent_absent() {
    // Create row without campaign
    let row = GoogleAdsRowBuilder::new().build();

    // This should not panic - optional_attr_str! handles missing parent
    assert_eq!(row.get("campaign_budget.amount_micros"), "");
}

#[test]
fn test_accessing_multiple_fields_with_missing_parent() {
    let row = GoogleAdsRowBuilder::new().build();

    // Test multiple optional fields with missing parent
    assert_eq!(row.get("campaign_criterion.status"), "");
    assert_eq!(row.get("campaign_criterion.type"), "");
    assert_eq!(row.get("campaign_criterion.criterion_id"), "");
    assert_eq!(row.get("campaign_criterion.display_name"), "");
}

#[test]
fn test_empty_row_optional_fields_safe() {
    let row = GoogleAdsRowBuilder::new().build();

    // Test that accessing OPTIONAL fields on empty row doesn't panic
    // Note: Required fields (like campaign.id) would panic with unwrap()
    // which is expected behavior - they should only be queried when present
    let test_paths = vec![
        "campaign_budget.amount_micros",
        "campaign_criterion.status",
        "campaign_criterion.type",
        "campaign_criterion.keyword.text",
    ];

    for path in test_paths {
        let result = row.get(path);
        // Optional fields should return empty string when parent is absent
        assert_eq!(
            result, "",
            "Path: {} should return empty string when parent absent",
            path
        );
    }
}

// ============================================================================
// Boundary Conditions
// ============================================================================

#[test]
fn test_very_long_field_path() {
    let row = GoogleAdsRowBuilder::new().build();

    let long_path = "a.b.c.d.e.f.g.h.i.j.k.l.m.n.o.p.q.r.s.t.u.v.w.x.y.z";
    assert_eq!(row.get(long_path), "not implemented by googleads-rs");
}

#[test]
fn test_field_path_with_numbers() {
    let row = GoogleAdsRowBuilder::new().build();

    assert_eq!(
        row.get("campaign123.field456"),
        "not implemented by googleads-rs"
    );
}

#[test]
fn test_field_path_with_underscores() {
    let campaign = CampaignBuilder::new().id(123).build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Valid path with underscores
    assert_eq!(row.get("campaign.campaign_budget"), "");

    // Invalid path with wrong underscores
    assert_eq!(
        row.get("campaign.campaign_budget_wrong"),
        "not implemented by googleads-rs"
    );
}

#[test]
fn test_field_path_with_special_characters() {
    let row = GoogleAdsRowBuilder::new().build();

    let special_paths = vec![
        "campaign@field",
        "campaign#field",
        "campaign$field",
        "campaign%field",
        "campaign&field",
        "campaign*field",
    ];

    for path in special_paths {
        assert_eq!(
            row.get(path),
            "not implemented by googleads-rs",
            "Failed for path: {}",
            path
        );
    }
}

// ============================================================================
// Default Values
// ============================================================================

#[test]
fn test_default_numeric_values() {
    let campaign = CampaignBuilder::new().build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Default i64 values should be 0
    assert_eq!(row.get("campaign.id"), "0");
}

#[test]
fn test_default_string_values() {
    let campaign = CampaignBuilder::new().build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Default string values should be empty
    assert_eq!(row.get("campaign.name"), "");
    assert_eq!(row.get("campaign.end_date"), "");
}

#[test]
fn test_default_bool_values() {
    let campaign = CampaignBuilder::new()
        .with_network_settings(false, false, false, false)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(
        row.get("campaign.network_settings.target_search_network"),
        "false"
    );
}

#[test]
fn test_default_enum_values() {
    use googleads_rs::google::ads::googleads::v21::enums::campaign_status_enum::CampaignStatus;

    let campaign = CampaignBuilder::new()
        .status(CampaignStatus::Unspecified)
        .build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    assert_eq!(row.get("campaign.status"), "Unspecified");
}

// ============================================================================
// Multiple Resources in Row
// ============================================================================

#[test]
fn test_accessing_missing_resource_in_populated_row() {
    let campaign = CampaignBuilder::new().id(111).build();

    let metrics = MetricsBuilder::new().impressions(1000).build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .with_metrics(metrics)
        .build();

    // Campaign and metrics are present
    assert_eq!(row.get("campaign.id"), "111");
    assert_eq!(row.get("metrics.impressions"), "1000");

    // Optional fields like campaign_budget should return empty when absent
    assert_eq!(row.get("campaign_budget.amount_micros"), "");
}

#[test]
fn test_mixed_present_and_absent_resources() {
    let campaign = CampaignBuilder::new().id(222).name("Test Campaign").build();

    let segments = SegmentsBuilder::new().date("2024-10-10").build();

    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .with_segments(segments)
        .build();

    // Present resources
    assert_eq!(row.get("campaign.id"), "222");
    assert_eq!(row.get("segments.date"), "2024-10-10");

    // Optional absent resources return empty string
    assert_eq!(row.get("campaign_criterion.status"), "");
    assert_eq!(row.get("campaign_budget.amount_micros"), "");
}

// ============================================================================
// Real-world Error Scenarios
// ============================================================================

#[test]
fn test_common_field_name_mistakes() {
    let campaign = CampaignBuilder::new().id(123).name("Test").build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Common mistakes developers might make
    let mistakes = vec![
        ("campaign.campaignid", "campaign.id"),  // Missing underscore
        ("campaign.campaign_id", "campaign.id"), // Extra word
        ("campaign.name_", "campaign.name"),     // Trailing underscore
        ("campaign.status_", "campaign.status"), // Trailing underscore
    ];

    for (wrong, _correct) in mistakes {
        assert_eq!(
            row.get(wrong),
            "not implemented by googleads-rs",
            "Wrong path '{}' should return not implemented",
            wrong
        );
    }
}

#[test]
fn test_sql_injection_like_input() {
    let row = GoogleAdsRowBuilder::new().build();

    // Test that weird inputs don't cause issues
    let weird_inputs = vec![
        "'; DROP TABLE campaigns; --",
        "campaign.id; DELETE FROM ads;",
        "../../../etc/passwd",
        "campaign.id OR 1=1",
    ];

    for input in weird_inputs {
        let result = row.get(input);
        assert_eq!(
            result, "not implemented by googleads-rs",
            "Weird input '{}' should return not implemented",
            input
        );
    }
}

#[test]
fn test_unicode_in_field_paths() {
    let row = GoogleAdsRowBuilder::new().build();

    let unicode_paths = vec![
        "campaign.名前", // Japanese
        "campaign.名字", // Chinese
        "campaign.имя",  // Russian
        "campaign.🎯",   // Emoji
    ];

    for path in unicode_paths {
        assert_eq!(
            row.get(path),
            "not implemented by googleads-rs",
            "Unicode path '{}' should return not implemented",
            path
        );
    }
}

// ============================================================================
// Performance Edge Cases
// ============================================================================

#[test]
fn test_many_sequential_gets() {
    let campaign = CampaignBuilder::new().id(999).name("Test").build();

    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();

    // Call get() many times to ensure no memory issues
    for _ in 0..1000 {
        assert_eq!(row.get("campaign.id"), "999");
        assert_eq!(row.get("campaign.name"), "Test");
    }
}

#[test]
fn test_many_different_unimplemented_paths() {
    let row = GoogleAdsRowBuilder::new().build();

    // Test many different unimplemented paths
    for i in 0..100 {
        let path = format!("resource{}.field{}", i, i);
        assert_eq!(row.get(&path), "not implemented by googleads-rs");
    }
}
