#![allow(dead_code)]
#![allow(unused_variables)]

use googleads_rs::google::ads::googleads::v23::{
    resources::Campaign,
    services::{campaign_operation, CampaignOperation, MutateGoogleAdsRequest},
};
use googleads_rs::{
    coerce_value, descriptor_pool, generate_field_mask, set_field_path_value,
    DynamicMutationBuilder, FieldUpdate, MutationOp,
};
use once_cell::sync::Lazy;
use prost::Message;
use prost_reflect::{DescriptorPool, DynamicMessage, Kind, ReflectMessage, Value};

static POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
    DescriptorPool::decode(bytes.as_ref()).expect("Failed to decode file descriptor set")
});

// ============================================================================
// Phase 2.1: DescriptorPool Contains Mutation Types
// ============================================================================

#[test]
fn test_pool_contains_campaign_resource() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign message not found in pool");
    assert_eq!(
        desc.full_name(),
        "google.ads.googleads.v23.resources.Campaign"
    );
}

#[test]
fn test_pool_contains_campaign_operation() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.CampaignOperation")
        .expect("CampaignOperation not found in pool");
    assert_eq!(
        desc.full_name(),
        "google.ads.googleads.v23.services.CampaignOperation"
    );

    let update_field = desc
        .get_field_by_name("update")
        .expect("update field not found");
    assert!(matches!(update_field.kind(), Kind::Message(_)));
    assert!(!update_field.is_list());
}

#[test]
fn test_pool_contains_mutate_campaigns_request() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.MutateCampaignsRequest")
        .expect("MutateCampaignsRequest not found in pool");
    assert_eq!(
        desc.full_name(),
        "google.ads.googleads.v23.services.MutateCampaignsRequest"
    );

    let ops_field = desc
        .get_field_by_name("operations")
        .expect("operations field not found");
    assert!(ops_field.is_list());
}

#[test]
fn test_pool_contains_mutate_operation_unified() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.MutateOperation")
        .expect("MutateOperation not found in pool");
    assert_eq!(
        desc.full_name(),
        "google.ads.googleads.v23.services.MutateOperation"
    );

    let campaign_op_field = desc
        .get_field_by_name("campaign_operation")
        .expect("campaign_operation field not found in MutateOperation");
    assert!(matches!(campaign_op_field.kind(), Kind::Message(_)));
}

#[test]
fn test_pool_contains_target_roas_message() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas message not found in pool");
    assert_eq!(
        desc.full_name(),
        "google.ads.googleads.v23.common.TargetRoas"
    );

    let target_roas_field = desc
        .get_field_by_name("target_roas")
        .expect("target_roas field not found on TargetRoas");
    assert!(matches!(target_roas_field.kind(), Kind::Double));
}

#[test]
fn test_pool_contains_field_mask() {
    let desc = POOL
        .get_message_by_name("google.protobuf.FieldMask")
        .expect("FieldMask not found in pool");
    assert_eq!(desc.full_name(), "google.protobuf.FieldMask");

    let paths_field = desc
        .get_field_by_name("paths")
        .expect("paths field not found on FieldMask");
    assert!(paths_field.is_list());
}

#[test]
fn test_campaign_has_bidding_strategy_oneof() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let target_roas_field = campaign_desc
        .get_field_by_name("target_roas")
        .expect("target_roas field not found on Campaign");

    let oneof = target_roas_field
        .containing_oneof()
        .expect("target_roas should be part of a oneof");
    assert_eq!(oneof.name(), "campaign_bidding_strategy");

    let oneof_fields: Vec<String> = oneof.fields().map(|f| f.name().to_string()).collect();
    assert!(oneof_fields.contains(&"target_roas".to_string()));
    assert!(oneof_fields.contains(&"maximize_conversions".to_string()));
    assert!(
        oneof_fields.len() > 5,
        "oneof should have many variants, got {}",
        oneof_fields.len()
    );
}

#[test]
fn test_pool_contains_service_descriptors() {
    let service = POOL
        .get_service_by_name("google.ads.googleads.v23.services.CampaignService")
        .expect("CampaignService not found in pool");
    assert_eq!(
        service.full_name(),
        "google.ads.googleads.v23.services.CampaignService"
    );

    let methods: Vec<String> = service.methods().map(|m| m.name().to_string()).collect();
    assert!(
        methods.contains(&"MutateCampaigns".to_string()),
        "CampaignService should have MutateCampaigns method, got: {:?}",
        methods
    );
}

#[test]
fn test_pool_contains_google_ads_service() {
    let service = POOL
        .get_service_by_name("google.ads.googleads.v23.services.GoogleAdsService")
        .expect("GoogleAdsService not found in pool");

    let methods: Vec<String> = service.methods().map(|m| m.name().to_string()).collect();
    assert!(
        methods.contains(&"Mutate".to_string()),
        "GoogleAdsService should have Mutate method, got: {:?}",
        methods
    );
}

// ============================================================================
// Phase 2.2: DynamicMessage Construction for Mutations
// ============================================================================

#[test]
fn test_dynamic_campaign_construction() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);

    campaign.set_field_by_name(
        "resource_name",
        Value::String("customers/123/campaigns/456".to_string()),
    );

    assert_eq!(
        campaign
            .get_field_by_name("resource_name")
            .unwrap()
            .as_str(),
        Some("customers/123/campaigns/456")
    );
}

#[test]
fn test_dynamic_nested_message_construction() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let target_roas_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let mut target_roas = DynamicMessage::new(target_roas_desc);
    target_roas.set_field_by_name("target_roas", Value::F64(3.5));

    let mut campaign = DynamicMessage::new(campaign_desc);
    campaign.set_field_by_name(
        "resource_name",
        Value::String("customers/123/campaigns/456".to_string()),
    );
    campaign.set_field_by_name("target_roas", Value::Message(target_roas));

    let retrieved = campaign.get_field_by_name("target_roas").unwrap();
    match &*retrieved {
        Value::Message(msg) => {
            let inner_val = msg.get_field_by_name("target_roas").unwrap();
            assert_eq!(inner_val.as_f64(), Some(3.5));
        }
        _ => panic!("Expected Message value for target_roas"),
    }
}

#[test]
fn test_dynamic_oneof_setting() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let target_roas_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let mut target_roas = DynamicMessage::new(target_roas_desc);
    target_roas.set_field_by_name("target_roas", Value::F64(3.5));

    let mut campaign = DynamicMessage::new(campaign_desc.clone());
    campaign.set_field_by_name("target_roas", Value::Message(target_roas));

    let target_roas_field = campaign_desc.get_field_by_name("target_roas").unwrap();
    assert!(
        campaign.has_field(&target_roas_field),
        "target_roas oneof variant should be set"
    );

    let max_conv_val_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.MaximizeConversionValue")
        .expect("MaximizeConversionValue not found");
    let max_conv_val = DynamicMessage::new(max_conv_val_desc);

    campaign.set_field_by_name("maximize_conversion_value", Value::Message(max_conv_val));

    assert!(
        !campaign.has_field(&target_roas_field),
        "target_roas should be cleared after setting a different oneof variant"
    );
    let max_field = campaign_desc
        .get_field_by_name("maximize_conversion_value")
        .unwrap();
    assert!(
        campaign.has_field(&max_field),
        "maximize_conversion_value should now be set"
    );
}

#[test]
fn test_dynamic_campaign_operation_construction() {
    let operation_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.CampaignOperation")
        .expect("CampaignOperation not found");

    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let target_roas_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let field_mask_desc = POOL
        .get_message_by_name("google.protobuf.FieldMask")
        .expect("FieldMask not found");

    let mut target_roas = DynamicMessage::new(target_roas_desc);
    target_roas.set_field_by_name("target_roas", Value::F64(3.5));

    let mut campaign = DynamicMessage::new(campaign_desc);
    campaign.set_field_by_name(
        "resource_name",
        Value::String("customers/123/campaigns/456".to_string()),
    );
    campaign.set_field_by_name("target_roas", Value::Message(target_roas));

    let mut field_mask = DynamicMessage::new(field_mask_desc);
    field_mask.set_field_by_name(
        "paths",
        Value::List(vec![Value::String("target_roas.target_roas".to_string())]),
    );

    let mut operation = DynamicMessage::new(operation_desc.clone());
    operation.set_field_by_name("update", Value::Message(campaign));
    operation.set_field_by_name("update_mask", Value::Message(field_mask));

    let update_field = operation_desc.get_field_by_name("update").unwrap();
    assert!(
        operation.has_field(&update_field),
        "update field should be set"
    );
}

#[test]
fn test_dynamic_mutate_campaigns_request_construction() {
    let request_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.MutateCampaignsRequest")
        .expect("MutateCampaignsRequest not found");

    let operation_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.CampaignOperation")
        .expect("CampaignOperation not found");

    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let target_roas_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let field_mask_desc = POOL
        .get_message_by_name("google.protobuf.FieldMask")
        .expect("FieldMask not found");

    let mut target_roas = DynamicMessage::new(target_roas_desc);
    target_roas.set_field_by_name("target_roas", Value::F64(3.5));

    let mut campaign = DynamicMessage::new(campaign_desc);
    campaign.set_field_by_name(
        "resource_name",
        Value::String("customers/123/campaigns/456".to_string()),
    );
    campaign.set_field_by_name("target_roas", Value::Message(target_roas));

    let mut field_mask = DynamicMessage::new(field_mask_desc);
    field_mask.set_field_by_name(
        "paths",
        Value::List(vec![Value::String("target_roas.target_roas".to_string())]),
    );

    let mut operation = DynamicMessage::new(operation_desc);
    operation.set_field_by_name("update", Value::Message(campaign));
    operation.set_field_by_name("update_mask", Value::Message(field_mask));

    let mut request = DynamicMessage::new(request_desc.clone());
    request.set_field_by_name("customer_id", Value::String("1234567890".to_string()));
    request.set_field_by_name("operations", Value::List(vec![Value::Message(operation)]));
    request.set_field_by_name("partial_failure", Value::Bool(true));
    request.set_field_by_name("validate_only", Value::Bool(false));

    let ops_field = request_desc.get_field_by_name("operations").unwrap();
    assert!(request.has_field(&ops_field));
}

// ============================================================================
// Phase 2.3: DynamicMessage → Static Type Round-Trip
// ============================================================================

#[test]
fn test_dynamic_campaign_roundtrip_encode_decode() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let target_roas_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let mut target_roas = DynamicMessage::new(target_roas_desc);
    target_roas.set_field_by_name("target_roas", Value::F64(3.5));

    let mut campaign = DynamicMessage::new(campaign_desc);
    campaign.set_field_by_name(
        "resource_name",
        Value::String("customers/123/campaigns/456".to_string()),
    );
    campaign.set_field_by_name("target_roas", Value::Message(target_roas));

    let encoded = campaign.encode_to_vec();

    let decoded: Campaign =
        Campaign::decode(encoded.as_slice()).expect("Failed to decode as static Campaign");
    assert_eq!(decoded.resource_name, "customers/123/campaigns/456");
}

#[test]
fn test_dynamic_campaign_operation_roundtrip() {
    let operation_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.CampaignOperation")
        .expect("CampaignOperation not found");

    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let target_roas_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let field_mask_desc = POOL
        .get_message_by_name("google.protobuf.FieldMask")
        .expect("FieldMask not found");

    let mut target_roas = DynamicMessage::new(target_roas_desc);
    target_roas.set_field_by_name("target_roas", Value::F64(3.5));

    let mut campaign = DynamicMessage::new(campaign_desc);
    campaign.set_field_by_name(
        "resource_name",
        Value::String("customers/123/campaigns/456".to_string()),
    );
    campaign.set_field_by_name("target_roas", Value::Message(target_roas));

    let mut field_mask = DynamicMessage::new(field_mask_desc);
    field_mask.set_field_by_name(
        "paths",
        Value::List(vec![Value::String("target_roas.target_roas".to_string())]),
    );

    let mut operation = DynamicMessage::new(operation_desc);
    operation.set_field_by_name("update", Value::Message(campaign));
    operation.set_field_by_name("update_mask", Value::Message(field_mask));

    let encoded = operation.encode_to_vec();

    let decoded: CampaignOperation = CampaignOperation::decode(encoded.as_slice())
        .expect("Failed to decode as static CampaignOperation");

    assert!(decoded.operation.is_some(), "operation should be set");
    match decoded.operation.unwrap() {
        campaign_operation::Operation::Update(camp) => {
            assert_eq!(camp.resource_name, "customers/123/campaigns/456");
        }
        other => panic!("Expected Update variant, got {:?}", other),
    }

    assert!(decoded.update_mask.is_some(), "update_mask should be set");
    let decoded_mask = decoded.update_mask.unwrap();
    assert_eq!(decoded_mask.paths, vec!["target_roas.target_roas"]);
}

#[test]
fn test_dynamic_transcode_to_static() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    campaign.set_field_by_name(
        "resource_name",
        Value::String("customers/999/campaigns/111".to_string()),
    );

    let static_campaign: Campaign = campaign.transcode_to().expect("transcode_to failed");
    assert_eq!(static_campaign.resource_name, "customers/999/campaigns/111");
}

#[test]
fn test_dynamic_campaign_operation_transcode_to_static() {
    let operation_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.CampaignOperation")
        .expect("CampaignOperation not found");

    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let target_roas_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let field_mask_desc = POOL
        .get_message_by_name("google.protobuf.FieldMask")
        .expect("FieldMask not found");

    let mut target_roas = DynamicMessage::new(target_roas_desc);
    target_roas.set_field_by_name("target_roas", Value::F64(4.2));

    let mut campaign = DynamicMessage::new(campaign_desc);
    campaign.set_field_by_name(
        "resource_name",
        Value::String("customers/123/campaigns/456".to_string()),
    );
    campaign.set_field_by_name("target_roas", Value::Message(target_roas));

    let mut field_mask = DynamicMessage::new(field_mask_desc);
    field_mask.set_field_by_name(
        "paths",
        Value::List(vec![Value::String("target_roas.target_roas".to_string())]),
    );

    let mut operation = DynamicMessage::new(operation_desc);
    operation.set_field_by_name("update", Value::Message(campaign));
    operation.set_field_by_name("update_mask", Value::Message(field_mask));

    let static_op: CampaignOperation = operation.transcode_to().expect("transcode_to failed");
    assert!(static_op.operation.is_some());
    assert!(static_op.update_mask.is_some());
    match static_op.operation.unwrap() {
        campaign_operation::Operation::Update(camp) => {
            assert_eq!(camp.resource_name, "customers/123/campaigns/456");
        }
        other => panic!("Expected Update, got {:?}", other),
    }
}

// ============================================================================
// Phase 2.5: Value Coercion / Type Safety
// ============================================================================

#[test]
fn test_value_type_mismatch_rejected() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);

    let result = campaign.try_set_field_by_name("resource_name", Value::I32(42));
    assert!(result.is_err(), "Setting string field to i32 should fail");
}

#[test]
fn test_enum_field_set_by_number() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let status_field = campaign_desc
        .get_field_by_name("status")
        .expect("status field not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    campaign.set_field(&status_field, Value::EnumNumber(3));

    let val = campaign.get_field(&status_field);
    match &*val {
        Value::EnumNumber(n) => assert_eq!(*n, 3),
        _ => panic!("Expected EnumNumber"),
    }
}

#[test]
fn test_field_descriptor_kind_for_target_roas() {
    let target_roas_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let target_roas_field = target_roas_desc.get_field_by_name("target_roas").unwrap();
    assert!(
        matches!(target_roas_field.kind(), Kind::Double),
        "target_roas should be Double"
    );
    assert!(
        target_roas_field.supports_presence(),
        "target_roas is proto3 optional, should support presence"
    );

    let cpc_ceiling_field = target_roas_desc
        .get_field_by_name("cpc_bid_ceiling_micros")
        .unwrap();
    assert!(
        matches!(cpc_ceiling_field.kind(), Kind::Int64),
        "cpc_bid_ceiling_micros should be Int64"
    );
}

// ============================================================================
// Phase 3: Architecture Validation — Field Path Traversal + Value Coercion
// (Uses library functions: coerce_value, set_field_path_value)
// ============================================================================

#[test]
fn test_field_path_traversal_scalar() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    set_field_path_value(
        &mut campaign,
        "resource_name",
        "customers/123/campaigns/456",
    )
    .unwrap();

    assert_eq!(
        campaign
            .get_field_by_name("resource_name")
            .unwrap()
            .as_str(),
        Some("customers/123/campaigns/456")
    );
}

#[test]
fn test_field_path_traversal_nested() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    set_field_path_value(&mut campaign, "target_roas.target_roas", "3.5").unwrap();

    let target_roas_val = campaign.get_field_by_name("target_roas").unwrap();
    match &*target_roas_val {
        Value::Message(msg) => {
            let inner = msg.get_field_by_name("target_roas").unwrap();
            assert_eq!(inner.as_f64(), Some(3.5));
        }
        _ => panic!("Expected Message"),
    }
}

#[test]
fn test_field_path_traversal_oneof() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc.clone());
    set_field_path_value(&mut campaign, "target_roas.target_roas", "3.5").unwrap();

    let target_roas_field = campaign_desc.get_field_by_name("target_roas").unwrap();
    assert!(campaign.has_field(&target_roas_field));

    set_field_path_value(
        &mut campaign,
        "maximize_conversion_value.target_roas",
        "2.0",
    )
    .unwrap();

    assert!(!campaign.has_field(&target_roas_field));
    let max_field = campaign_desc
        .get_field_by_name("maximize_conversion_value")
        .unwrap();
    assert!(campaign.has_field(&max_field));
}

#[test]
fn test_value_coercion_double() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");
    let field = desc.get_field_by_name("target_roas").unwrap();

    let val = coerce_value("3.5", &field).unwrap();
    assert_eq!(val, Value::F64(3.5));
}

#[test]
fn test_value_coercion_int64() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");
    let field = desc.get_field_by_name("cpc_bid_ceiling_micros").unwrap();

    let val = coerce_value("5000000", &field).unwrap();
    assert_eq!(val, Value::I64(5_000_000));
}

#[test]
fn test_value_coercion_enum_by_name() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");
    let field = desc.get_field_by_name("status").unwrap();

    if let Kind::Enum(enum_desc) = field.kind() {
        let val = coerce_value("ENABLED", &field).unwrap();
        match val {
            Value::EnumNumber(n) => {
                let ev = enum_desc.get_value(n).unwrap();
                assert_eq!(ev.name(), "ENABLED");
            }
            _ => panic!("Expected EnumNumber"),
        }
    }
}

#[test]
fn test_value_coercion_enum_by_number() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");
    let field = desc.get_field_by_name("status").unwrap();

    let val = coerce_value("3", &field).unwrap();
    match val {
        Value::EnumNumber(n) => assert_eq!(n, 3),
        _ => panic!("Expected EnumNumber"),
    }
}

#[test]
fn test_full_mutation_pipeline_dynamic() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    set_field_path_value(
        &mut campaign,
        "resource_name",
        "customers/123/campaigns/456",
    )
    .unwrap();
    set_field_path_value(&mut campaign, "target_roas.target_roas", "3.5").unwrap();

    let operation_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.CampaignOperation")
        .expect("CampaignOperation not found");

    let field_mask_desc = POOL
        .get_message_by_name("google.protobuf.FieldMask")
        .expect("FieldMask not found");

    let mut field_mask = DynamicMessage::new(field_mask_desc);
    field_mask.set_field_by_name(
        "paths",
        Value::List(vec![Value::String("target_roas.target_roas".to_string())]),
    );

    let mut operation = DynamicMessage::new(operation_desc);
    operation.set_field_by_name("update", Value::Message(campaign));
    operation.set_field_by_name("update_mask", Value::Message(field_mask));

    let static_op: CampaignOperation = operation.transcode_to().expect("transcode_to failed");

    match static_op.operation.unwrap() {
        campaign_operation::Operation::Update(camp) => {
            assert_eq!(camp.resource_name, "customers/123/campaigns/456");
        }
        other => panic!("Expected Update, got {:?}", other),
    }
    assert_eq!(
        static_op.update_mask.unwrap().paths,
        vec!["target_roas.target_roas"]
    );
}

#[test]
fn test_unified_mutate_pipeline_dynamic() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    set_field_path_value(
        &mut campaign,
        "resource_name",
        "customers/123/campaigns/456",
    )
    .unwrap();
    set_field_path_value(&mut campaign, "target_roas.target_roas", "3.5").unwrap();

    let campaign_op_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.CampaignOperation")
        .expect("CampaignOperation not found");

    let field_mask_desc = POOL
        .get_message_by_name("google.protobuf.FieldMask")
        .expect("FieldMask not found");

    let mut field_mask = DynamicMessage::new(field_mask_desc);
    field_mask.set_field_by_name(
        "paths",
        Value::List(vec![Value::String("target_roas.target_roas".to_string())]),
    );

    let mut campaign_op = DynamicMessage::new(campaign_op_desc);
    campaign_op.set_field_by_name("update", Value::Message(campaign));
    campaign_op.set_field_by_name("update_mask", Value::Message(field_mask));

    let mutate_op_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.MutateOperation")
        .expect("MutateOperation not found");

    let mut mutate_op = DynamicMessage::new(mutate_op_desc);
    mutate_op.set_field_by_name("campaign_operation", Value::Message(campaign_op));

    let request_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.services.MutateGoogleAdsRequest")
        .expect("MutateGoogleAdsRequest not found");

    let mut request = DynamicMessage::new(request_desc);
    request.set_field_by_name("customer_id", Value::String("1234567890".to_string()));
    request.set_field_by_name(
        "mutate_operations",
        Value::List(vec![Value::Message(mutate_op)]),
    );
    request.set_field_by_name("partial_failure", Value::Bool(true));
    request.set_field_by_name("validate_only", Value::Bool(false));

    let static_request: MutateGoogleAdsRequest = request
        .transcode_to()
        .expect("transcode_to MutateGoogleAdsRequest failed");

    assert_eq!(static_request.customer_id, "1234567890");
    assert_eq!(static_request.mutate_operations.len(), 1);
    assert!(static_request.partial_failure);
    assert!(!static_request.validate_only);
}

#[test]
fn test_field_path_error_invalid_field() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    let result = set_field_path_value(&mut campaign, "nonexistent_field", "3.5");
    assert!(result.is_err());
}

#[test]
fn test_field_path_error_traverse_scalar() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    let result = set_field_path_value(&mut campaign, "resource_name.invalid_sub_field", "3.5");
    assert!(result.is_err());
}

#[test]
fn test_field_path_error_type_mismatch() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let field = desc.get_field_by_name("target_roas").unwrap();
    let result = coerce_value("not_a_number", &field);
    assert!(result.is_err());
}

// ============================================================================
// Phase 4: DynamicMutationBuilder — POC Implementation
// ============================================================================

#[test]
fn test_builder_update_campaign_target_roas() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.set_field("target_roas.target_roas", "3.5");

    let request = builder
        .build("customers/1234567890/campaigns/456")
        .expect("build should succeed");

    assert_eq!(request.customer_id, "1234567890");
    assert_eq!(request.mutate_operations.len(), 1);
    assert!(request.partial_failure);
    assert!(!request.validate_only);
}

#[test]
fn test_builder_validate_only() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.set_field("target_roas.target_roas", "3.5");
    builder.validate_only(true);

    let request = builder
        .build("customers/1234567890/campaigns/456")
        .expect("build should succeed");

    assert!(request.validate_only);
}

#[test]
fn test_builder_build_operation_dynamic() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.set_field("target_roas.target_roas", "3.5");

    let mutate_op = builder
        .build_operation("customers/1234567890/campaigns/456")
        .expect("build_operation should succeed");

    let campaign_op_field = mutate_op
        .descriptor()
        .get_field_by_name("campaign_operation")
        .expect("campaign_operation field should exist");
    assert!(mutate_op.has_field(&campaign_op_field));
}

#[test]
fn test_builder_roundtrip_campaign_target_roas() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.set_field("target_roas.target_roas", "3.5");

    let request = builder
        .build("customers/1234567890/campaigns/456")
        .expect("build should succeed");

    let encoded = request.encode_to_vec();
    let decoded = MutateGoogleAdsRequest::decode(encoded.as_slice())
        .expect("round-trip decode should succeed");

    assert_eq!(decoded.customer_id, "1234567890");
    assert_eq!(decoded.mutate_operations.len(), 1);
}

#[test]
fn test_builder_multiple_fields() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.set_field("target_roas.target_roas", "3.5");
    builder.set_field("target_roas.cpc_bid_ceiling_micros", "5000000");

    let request = builder
        .build("customers/1234567890/campaigns/456")
        .expect("build should succeed");

    assert_eq!(request.mutate_operations.len(), 1);
}

#[test]
fn test_builder_create_operation() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.operation_type(MutationOp::Create);
    builder.set_field("name", "My Campaign");

    let request = builder
        .build("customers/1234567890/campaigns/456")
        .expect("build should succeed");

    assert_eq!(request.mutate_operations.len(), 1);
}

#[test]
fn test_builder_remove_operation() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.operation_type(MutationOp::Remove);

    let request = builder
        .build("customers/1234567890/campaigns/456")
        .expect("build should succeed");

    assert_eq!(request.mutate_operations.len(), 1);
}

#[test]
fn test_builder_unknown_resource() {
    let mut builder = DynamicMutationBuilder::new("NonexistentResource", "1234567890");
    builder.set_field("some_field", "value");

    let result = builder.build("customers/1234567890/nonexistent/456");
    assert!(result.is_err(), "Unknown resource type should fail");
}

#[test]
fn test_builder_field_mask_generation() {
    let updates = vec![
        FieldUpdate {
            field_path: "target_roas.target_roas".to_string(),
            value: "3.5".to_string(),
        },
        FieldUpdate {
            field_path: "name".to_string(),
            value: "Test".to_string(),
        },
    ];
    let mask = generate_field_mask(&updates);
    assert_eq!(mask, vec!["target_roas.target_roas", "name"]);
}

#[test]
fn test_public_descriptor_pool() {
    let pool = descriptor_pool();
    let desc = pool
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign should be accessible via public descriptor_pool()");
    assert_eq!(
        desc.full_name(),
        "google.ads.googleads.v23.resources.Campaign"
    );
}

#[test]
fn test_public_set_field_path_value() {
    let campaign_desc = POOL
        .get_message_by_name("google.ads.googleads.v23.resources.Campaign")
        .expect("Campaign not found");

    let mut campaign = DynamicMessage::new(campaign_desc);
    set_field_path_value(
        &mut campaign,
        "resource_name",
        "customers/123/campaigns/456",
    )
    .unwrap();
    set_field_path_value(&mut campaign, "target_roas.target_roas", "3.5").unwrap();

    let target_roas_val = campaign.get_field_by_name("target_roas").unwrap();
    match &*target_roas_val {
        Value::Message(msg) => {
            let inner = msg.get_field_by_name("target_roas").unwrap();
            assert_eq!(inner.as_f64(), Some(3.5));
        }
        _ => panic!("Expected Message"),
    }
}

#[test]
fn test_public_coerce_value() {
    let desc = POOL
        .get_message_by_name("google.ads.googleads.v23.common.TargetRoas")
        .expect("TargetRoas not found");

    let field = desc.get_field_by_name("target_roas").unwrap();
    let val = coerce_value("3.5", &field).unwrap();
    assert_eq!(val, Value::F64(3.5));

    let int_field = desc.get_field_by_name("cpc_bid_ceiling_micros").unwrap();
    let int_val = coerce_value("5000000", &int_field).unwrap();
    assert_eq!(int_val, Value::I64(5_000_000));
}

#[test]
fn test_builder_full_pipeline_target_roas() {
    let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
    builder.set_field("target_roas.target_roas", "3.5");
    builder.partial_failure(true);
    builder.validate_only(true);

    let request = builder
        .build("customers/1234567890/campaigns/456")
        .expect("full pipeline build should succeed");

    assert_eq!(request.customer_id, "1234567890");
    assert_eq!(request.mutate_operations.len(), 1);
    assert!(request.partial_failure);
    assert!(request.validate_only);

    let encoded = request.encode_to_vec();
    let decoded = MutateGoogleAdsRequest::decode(encoded.as_slice())
        .expect("round-trip decode should succeed");
    assert_eq!(decoded.customer_id, "1234567890");
}
