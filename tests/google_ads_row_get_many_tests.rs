// Unit tests for GoogleAdsRow::get_many() method

mod test_helpers;

use test_helpers::{CampaignBuilder, GoogleAdsRowBuilder, MetricsBuilder, SegmentsBuilder};

#[test]
fn test_get_many_single_field() {
    let campaign = CampaignBuilder::new()
        .id(12345)
        .name("Test Campaign")
        .build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let fields = vec!["campaign.id"];
    let values = row.get_many(&fields);
    assert_eq!(values.len(), 1);
    assert_eq!(values[0], "12345");
}

#[test]
fn test_get_many_multiple_fields() {
    let campaign = CampaignBuilder::new()
        .id(12345)
        .name("Test Campaign")
        .build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let fields = vec!["campaign.id", "campaign.name"];
    let values = row.get_many(&fields);
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], "12345");
    assert_eq!(values[1], "Test Campaign");
}

#[test]
fn test_get_many_empty_fields() {
    let campaign = CampaignBuilder::new().id(12345).build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let fields: Vec<&str> = vec![];
    let values = row.get_many(&fields);
    assert_eq!(values.len(), 0);
}

#[test]
fn test_get_many_cross_resources() {
    let campaign = CampaignBuilder::new()
        .id(12345)
        .name("Test Campaign")
        .build();
    let metrics = MetricsBuilder::new().clicks(100).impressions(1000).build();
    let segments = SegmentsBuilder::new().date("2024-01-15").build();
    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .with_metrics(metrics)
        .with_segments(segments)
        .build();
    let fields = vec![
        "campaign.id",
        "campaign.name",
        "metrics.clicks",
        "metrics.impressions",
        "segments.date",
    ];
    let values = row.get_many(&fields);
    assert_eq!(values.len(), 5);
    assert_eq!(values[0], "12345");
    assert_eq!(values[1], "Test Campaign");
    assert_eq!(values[2], "100");
    assert_eq!(values[3], "1000");
    assert_eq!(values[4], "2024-01-15");
}

#[test]
fn test_get_many_missing_parent_resource() {
    let row = GoogleAdsRowBuilder::new().build();
    let fields = vec!["campaign.id", "campaign.name"];
    let values = row.get_many(&fields);
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], "");
    assert_eq!(values[1], "");
}

#[test]
fn test_get_many_equivalent_to_multiple_get() {
    let campaign = CampaignBuilder::new()
        .id(12345)
        .name("Test Campaign")
        .build();
    let metrics = MetricsBuilder::new().clicks(100).build();
    let row = GoogleAdsRowBuilder::new()
        .with_campaign(campaign)
        .with_metrics(metrics)
        .build();
    let fields = vec!["campaign.id", "campaign.name", "metrics.clicks"];
    let many_values = row.get_many(&fields);
    let individual_values: Vec<String> = fields.iter().map(|f| row.get(f)).collect();
    assert_eq!(many_values, individual_values);
}

#[test]
fn test_get_many_with_invalid_field() {
    let campaign = CampaignBuilder::new().id(12345).build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let fields = vec!["campaign.id", "campaign.invalid_field", "campaign.name"];
    let values = row.get_many(&fields);
    assert_eq!(values.len(), 3);
    assert_eq!(values[0], "12345");
    assert_eq!(values[1], "not implemented by googleads-rs");
    assert_eq!(values[2], "");
}

#[test]
fn test_get_many_duplicate_fields() {
    let campaign = CampaignBuilder::new().id(12345).build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let fields = vec!["campaign.id", "campaign.id", "campaign.id"];
    let values = row.get_many(&fields);
    assert_eq!(values.len(), 3);
    assert_eq!(values[0], "12345");
    assert_eq!(values[1], "12345");
    assert_eq!(values[2], "12345");
}

#[test]
fn test_get_many_preserves_field_order() {
    let campaign = CampaignBuilder::new().id(1).name("A").build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let fields1 = vec!["campaign.id", "campaign.name"];
    let values1 = row.get_many(&fields1);
    let fields2 = vec!["campaign.name", "campaign.id"];
    let values2 = row.get_many(&fields2);
    assert_eq!(values1[0], "1");
    assert_eq!(values1[1], "A");
    assert_eq!(values2[0], "A");
    assert_eq!(values2[1], "1");
}

#[test]
fn test_get_many_large_number_of_fields() {
    let campaign = CampaignBuilder::new().id(12345).name("Test").build();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    let mut fields: Vec<&str> = Vec::new();
    for _ in 0..50 {
        fields.push("campaign.id");
        fields.push("campaign.name");
    }
    let values = row.get_many(&fields);
    assert_eq!(values.len(), 100);
}
