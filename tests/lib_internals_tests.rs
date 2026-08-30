// Tests for `src/lib.rs` internals: `DynamicMutationBuilder`, `coerce_value`,
// `set_field_path_value`, and reflection-driven row formatting paths that the
// resource-specific test files do not exercise.
//
// Coverage goals ( llvm-cov baseline misses this file addresses ):
// - Builder defaults, getters, flag chaining, Create/Remove operation arms
// - Pool-miss error path and `to_snake_case` underscore branch
// - `coerce_value` Float/Int32/Bool/Enum-error/unsupported-kind arms
// - `coerce_value` U32/U64 arms via a runtime-built descriptor pool (no
//   `uint*` field exists anywhere in the compiled Google Ads protos)
// - Nested-path overwrite reuse branch (`existing.clone()`)
// - Row scalar arms (F32, Bytes), unknown-enum-number fallback, list-item
//   guard, compact empty-value filter, presence-validation paths, asset
//   automation settings guards, and FieldMask formatting

mod test_helpers;

use googleads_rs::current_gads_version::common::CustomParameter;
use googleads_rs::current_gads_version::resources::Asset;
use googleads_rs::current_gads_version::services::campaign_operation;
use googleads_rs::{
    coerce_value, descriptor_pool, set_field_path_value, DynamicMutationBuilder, MutationOp,
};
use prost_reflect::{DynamicMessage, ReflectMessage, Value};
use test_helpers::{ChangeEventBuilder, GoogleAdsRowBuilder};

/// Descriptor FQN prefix for the current Google Ads API version, derived at
/// compile time from the crate's package version (major mirrors API major).
macro_rules! gads_fqn {
    ($rest:expr) => {
        concat!(
            "google.ads.googleads.v",
            env!("CARGO_PKG_VERSION_MAJOR"),
            $rest
        )
    };
}

// ============================================================================
// DynamicMutationBuilder — defaults, getters, flag chaining
// ============================================================================

#[test]
fn test_builder_defaults_and_getters() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "999888777");
    assert_eq!(builder.resource_type(), "Campaign");
    assert!(builder.field_updates().is_empty());

    builder.set_field("target_roas.target_roas", "3.5");
    builder.set_field("name", "Fall Sale");

    let updates = builder.field_updates();
    assert_eq!(updates.len(), 2);
    assert_eq!(updates[0].field_path, "target_roas.target_roas");
    assert_eq!(updates[0].value, "3.5");
    assert_eq!(updates[1].field_path, "name");
    assert_eq!(updates[1].value, "Fall Sale");

    // Defaults: partial_failure = true, validate_only = false.
    let request = builder
        .build("customers/999888777/campaigns/1")
        .expect("build should succeed");
    assert!(request.partial_failure);
    assert!(!request.validate_only);
    assert_eq!(request.customer_id, "999888777");
}

#[test]
fn test_builder_flag_chaining_mutates_request() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.set_field("name", "Chained");

    // Every setter returns &mut Self so the chain compiles and mutates state.
    builder
        .operation_type(MutationOp::Update)
        .validate_only(true)
        .partial_failure(false);

    let request = builder
        .build("customers/1234567890/campaigns/2")
        .expect("build should succeed");
    assert!(request.validate_only);
    assert!(!request.partial_failure);
}

// ============================================================================
// DynamicMutationBuilder — Create / Remove operation arms
// ============================================================================

#[test]
fn test_builder_create_operation_sets_create_field_only() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder
        .operation_type(MutationOp::Create)
        .set_field("name", "New Campaign");

    let mutate_op = builder
        .build_operation("customers/1234567890/campaigns/456")
        .expect("build_operation should succeed");

    let op_field = mutate_op
        .descriptor()
        .get_field_by_name("campaign_operation")
        .expect("campaign_operation field should exist");
    let op_value = mutate_op.get_field(&op_field);
    let op_msg = op_value
        .as_message()
        .expect("campaign_operation should be set");

    let create_field = op_msg
        .descriptor()
        .get_field_by_name("create")
        .expect("create field should exist");
    let update_field = op_msg
        .descriptor()
        .get_field_by_name("update")
        .expect("update field should exist");
    let mask_field = op_msg
        .descriptor()
        .get_field_by_name("update_mask")
        .expect("update_mask field should exist");

    assert!(op_msg.has_field(&create_field));
    assert!(!op_msg.has_field(&update_field));
    assert!(!op_msg.has_field(&mask_field));

    // Round-trip through the static type to pin the Create variant payload.
    let static_op: googleads_rs::current_gads_version::services::CampaignOperation =
        op_msg.transcode_to().expect("transcode_to should succeed");
    match static_op.operation.expect("operation set") {
        campaign_operation::Operation::Create(camp) => {
            assert_eq!(camp.name.as_deref(), Some("New Campaign"));
        }
        other => panic!("Expected Create, got {:?}", other),
    }
}

#[test]
fn test_builder_remove_operation_sets_resource_name_string() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.operation_type(MutationOp::Remove);

    let mutate_op = builder
        .build_operation("customers/1234567890/campaigns/456")
        .expect("build_operation should succeed");

    let op_field = mutate_op
        .descriptor()
        .get_field_by_name("campaign_operation")
        .expect("campaign_operation field should exist");
    let op_value = mutate_op.get_field(&op_field);
    let op_msg = op_value
        .as_message()
        .expect("campaign_operation should be set");

    let remove_field = op_msg
        .descriptor()
        .get_field_by_name("remove")
        .expect("remove field should exist");
    assert!(op_msg.has_field(&remove_field));
    assert_eq!(
        op_msg.get_field(&remove_field).as_str(),
        Some("customers/1234567890/campaigns/456")
    );
}

// ============================================================================
// DynamicMutationBuilder — snake_case op field name + pool-miss error
// ============================================================================

#[test]
fn test_builder_build_operation_ad_group_field_name() {
    // "AdGroup" exercises the underscore branch in to_snake_case.
    let mut builder = DynamicMutationBuilder::new("AdGroup", "1234567890");
    builder.set_field("name", "AG");

    let mutate_op = builder
        .build_operation("customers/1234567890/adGroups/1")
        .expect("build_operation should succeed");

    let ag_field = mutate_op
        .descriptor()
        .get_field_by_name("ad_group_operation")
        .expect("ad_group_operation field should exist on MutateOperation");
    assert!(mutate_op.has_field(&ag_field));
}

#[test]
fn test_builder_pool_miss_error_message() {
    let builder = DynamicMutationBuilder::new("NoSuchResource", "1234567890");
    let err = builder
        .build_operation("customers/1234567890/noSuchResources/1")
        .expect_err("unknown resource should fail");
    let msg = err.to_string();
    assert!(
        msg.contains("not found in descriptor pool"),
        "unexpected error: {}",
        msg
    );
}

// ============================================================================
// set_field_path_value — empty path and nested overwrite
// ============================================================================

#[test]
fn test_set_field_path_value_empty_path_errors() {
    let campaign_desc = descriptor_pool()
        .get_message_by_name(gads_fqn!(".resources.Campaign"))
        .expect("Campaign not found");
    let mut campaign = DynamicMessage::new(campaign_desc);

    // "": split('.') on "" yields [""] — the empty segment fails the leaf
    // lookup below, so no separate empty-path guard exists.
    let err = set_field_path_value(&mut campaign, "", "3.5").expect_err("empty path should fail");
    let msg = err.to_string();
    assert!(msg.contains("not found"), "unexpected error: {}", msg);
}

#[test]
fn test_set_field_path_value_nested_overwrite_reuses_existing_message() {
    let campaign_desc = descriptor_pool()
        .get_message_by_name(gads_fqn!(".resources.Campaign"))
        .expect("Campaign not found");
    let mut campaign = DynamicMessage::new(campaign_desc);

    set_field_path_value(&mut campaign, "target_roas.target_roas", "3.5").unwrap();
    // Second set hits the has_field-existing branch and must preserve the
    // nested message identity while replacing the leaf value.
    set_field_path_value(&mut campaign, "target_roas.target_roas", "4.5").unwrap();

    let tr_value = campaign.get_field_by_name("target_roas").unwrap();
    match &*tr_value {
        Value::Message(msg) => assert_eq!(
            msg.get_field_by_name("target_roas").unwrap().as_f64(),
            Some(4.5)
        ),
        _ => panic!("Expected Message"),
    }
}

// ============================================================================
// coerce_value — Float / Int32 / Bool / Enum-error / unsupported kind
// ============================================================================

#[test]
fn test_coerce_float_and_int32_arms() {
    let criterion_desc = descriptor_pool()
        .get_message_by_name(gads_fqn!(".resources.CampaignCriterion"))
        .expect("CampaignCriterion not found");
    let bid_modifier = criterion_desc
        .get_field_by_name("bid_modifier")
        .expect("bid_modifier not found");
    assert_eq!(coerce_value("1.5", &bid_modifier).unwrap(), Value::F32(1.5));
    let err = coerce_value("abc", &bid_modifier).expect_err("bad float should fail");
    assert!(err.to_string().contains("as float"), "unexpected: {}", err);

    let segments_desc = descriptor_pool()
        .get_message_by_name(gads_fqn!(".common.Segments"))
        .expect("Segments not found");
    let hour = segments_desc
        .get_field_by_name("hour")
        .expect("hour not found");
    assert_eq!(coerce_value("42", &hour).unwrap(), Value::I32(42));
    let err = coerce_value("abc", &hour).expect_err("bad int32 should fail");
    assert!(err.to_string().contains("as int32"), "unexpected: {}", err);
}

#[test]
fn test_coerce_bool_arm_and_error() {
    let request_desc = descriptor_pool()
        .get_message_by_name(gads_fqn!(".services.MutateGoogleAdsRequest"))
        .expect("MutateGoogleAdsRequest not found");
    let partial_failure = request_desc
        .get_field_by_name("partial_failure")
        .expect("partial_failure not found");

    assert_eq!(
        coerce_value("true", &partial_failure).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        coerce_value("false", &partial_failure).unwrap(),
        Value::Bool(false)
    );
    let err = coerce_value("not-a-bool", &partial_failure).expect_err("bad bool should fail");
    assert!(err.to_string().contains("as bool"), "unexpected: {}", err);
}

#[test]
fn test_coerce_enum_invalid_name_errors() {
    let campaign_desc = descriptor_pool()
        .get_message_by_name(gads_fqn!(".resources.Campaign"))
        .expect("Campaign not found");
    let status = campaign_desc
        .get_field_by_name("status")
        .expect("status not found");

    let err =
        coerce_value("NOT_REAL_ENUM_VALUE", &status).expect_err("invalid enum name should fail");
    let msg = err.to_string();
    assert!(
        msg.contains("as enum name or number"),
        "unexpected: {}",
        msg
    );
}

#[test]
fn test_coerce_unsupported_kind_bytes() {
    let media_bundle_desc = descriptor_pool()
        .get_message_by_name(gads_fqn!(".common.MediaBundleAsset"))
        .expect("MediaBundleAsset not found");
    let data = media_bundle_desc
        .get_field_by_name("data")
        .expect("data bytes field not found");
    let err = coerce_value("abc", &data).expect_err("bytes kind is unsupported");
    assert!(
        err.to_string().contains("Cannot coerce value to type"),
        "unexpected: {}",
        err
    );
}

// ============================================================================
// coerce_value — U32 / U64 arms via runtime-built descriptor pool
// ============================================================================

/// Builds a tiny descriptor pool containing `message W { uint32 a; uint64 b; }`
/// because no `uint*` field exists in the compiled Google Ads protos.
#[test]
fn test_coerce_uint32_and_uint64_arms() {
    use prost_reflect::DescriptorPool;
    use prost_types::field_descriptor_proto::{Label, Type};
    use prost_types::{
        DescriptorProto, FieldDescriptorProto, FileDescriptorProto, FileDescriptorSet,
    };

    let field = |name: &str, number: i32, ty: Type| {
        let mut f = FieldDescriptorProto::default();
        f.name = Some(name.to_string());
        f.number = Some(number);
        f.label = Some(Label::Optional as i32);
        f.r#type = Some(ty as i32);
        f
    };

    let file = FileDescriptorProto {
        name: Some("uint_pool.proto".to_string()),
        package: Some("uint.test".to_string()),
        syntax: Some("proto3".to_string()),
        message_type: vec![DescriptorProto {
            name: Some("W".to_string()),
            field: vec![field("a", 1, Type::Uint32), field("b", 2, Type::Uint64)],
            ..Default::default()
        }],
        ..Default::default()
    };

    let pool = DescriptorPool::from_file_descriptor_set(FileDescriptorSet { file: vec![file] })
        .expect("hand-built uint pool should be valid");

    let w_desc = pool
        .get_message_by_name("uint.test.W")
        .expect("W message not found in hand-built pool");
    let a = w_desc.get_field_by_name("a").expect("field a not found");
    let b = w_desc.get_field_by_name("b").expect("field b not found");

    assert_eq!(coerce_value("7", &a).unwrap(), Value::U32(7));
    assert_eq!(coerce_value("9", &b).unwrap(), Value::U64(9));

    let err = coerce_value("xyz", &a).expect_err("bad uint32 should fail");
    assert!(err.to_string().contains("as uint32"), "unexpected: {}", err);
    let err = coerce_value("xyz", &b).expect_err("bad uint64 should fail");
    assert!(err.to_string().contains("as uint64"), "unexpected: {}", err);
}

// ============================================================================
// Row formatting — scalar arms, enum fallback, list guard, compact filter
// ============================================================================

#[test]
fn test_row_get_float_scalar_bid_modifier() {
    let criterion = googleads_rs::current_gads_version::resources::CampaignCriterion {
        bid_modifier: Some(1.25),
        ..Default::default()
    };
    let row = GoogleAdsRowBuilder::new()
        .with_campaign_criterion(criterion)
        .build();
    assert_eq!(row.get("campaign_criterion.bid_modifier"), "1.25");
}

#[test]
fn test_row_get_bytes_scalar_asset_media_bundle_data() {
    // MediaBundleAsset.data is a bytes leaf reached through the Asset oneof;
    // prost-reflect's Value::Bytes Debug-prints as b"...".
    let asset = Asset {
        asset_data: Some(
            googleads_rs::current_gads_version::resources::asset::AssetData::MediaBundleAsset(
                googleads_rs::current_gads_version::common::MediaBundleAsset {
                    data: Some(b"ab".to_vec()),
                },
            ),
        ),
        ..Default::default()
    };
    let row = GoogleAdsRowBuilder::new().with_asset(asset).build();
    assert_eq!(row.get("asset.media_bundle_asset.data"), "b\"ab\"");
}

#[test]
fn test_row_get_unset_campaign_subfield_validation() {
    // Empty row: campaign unset. An invalid subfield returns the sentinel.
    let row = GoogleAdsRowBuilder::new().build();
    assert_eq!(
        row.get("campaign.invalid_field"),
        "not implemented by googleads-rs"
    );
    // A valid subfield on an unset parent returns empty.
    assert_eq!(row.get("campaign.id"), "");
}

#[test]
fn test_row_get_unset_scalar_parent_with_trailing_segment() {
    // `id` is an optional scalar (Int64): supports presence, kind is not
    // Message. With campaign set but id unset, a trailing segment beyond the
    // scalar leaf reaches depth 2, takes the non-Message wildcat arm of the
    // validation match, and reports the sentinel.
    let campaign = googleads_rs::current_gads_version::resources::Campaign {
        name: Some("Set but id-free".to_string()),
        ..Default::default()
    };
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    assert_eq!(
        row.get("campaign.id.subfield"),
        "not implemented by googleads-rs"
    );
}

#[test]
fn test_row_get_list_item_guard_scalar_with_trailing_segment() {
    // Repeated string field walked with a trailing segment: the single scalar
    // item hits the non-message guard and contributes an empty string.
    let campaign = googleads_rs::current_gads_version::resources::Campaign {
        labels: vec!["L1".to_string()],
        ..Default::default()
    };
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    assert_eq!(row.get("campaign.labels.bogus"), "");
}

#[test]
fn test_row_get_unknown_enum_number_returns_number_string() {
    // Status 999 has no enum value name: format_scalar falls back to the
    // number itself.
    let campaign = googleads_rs::current_gads_version::resources::Campaign {
        status: 999,
        ..Default::default()
    };
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    assert_eq!(row.get("campaign.status"), "999");
}

#[test]
fn test_row_get_compact_filters_empty_value_fields() {
    // A repeated message item whose only non-empty field is `key`: the empty
    // `value` pair is filtered out of the compact formatting.
    let campaign = googleads_rs::current_gads_version::resources::Campaign {
        url_custom_parameters: vec![CustomParameter {
            key: Some("k".to_string()),
            value: Some(String::new()),
        }],
        ..Default::default()
    };
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    assert_eq!(row.get("campaign.url_custom_parameters"), "key:k");
}

// ============================================================================
// Row formatting — asset_automation_settings guards (reshuffle-verified)
// ============================================================================

#[test]
fn test_row_asset_automation_settings_campaign_unset_returns_empty() {
    let row = GoogleAdsRowBuilder::new().build();
    assert_eq!(row.get("campaign.asset_automation_settings"), "");
}

#[test]
fn test_row_asset_automation_settings_unset_returns_empty() {
    let campaign = googleads_rs::current_gads_version::resources::Campaign::default();
    let row = GoogleAdsRowBuilder::new().with_campaign(campaign).build();
    assert_eq!(row.get("campaign.asset_automation_settings"), "");
}

// ============================================================================
// Row formatting — FieldMask (reshuffle-verified happy path)
// ============================================================================

#[test]
fn test_row_get_change_event_changed_fields_field_mask() {
    let change_event = ChangeEventBuilder::new()
        .change_date_time("2024-01-01 00:00:00")
        .changed_fields(vec!["campaign.id", "campaign.name"])
        .build();
    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();
    assert_eq!(
        row.get("change_event.changed_fields"),
        "campaign.id, campaign.name"
    );
}

#[test]
fn test_row_get_change_event_changed_fields_empty_paths() {
    let change_event = ChangeEventBuilder::new()
        .change_date_time("2024-01-01 00:00:00")
        .changed_fields(vec![])
        .build();
    let row = GoogleAdsRowBuilder::new()
        .with_change_event(change_event)
        .build();
    assert_eq!(row.get("change_event.changed_fields"), "");
}
