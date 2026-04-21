//! Google Ads gRPC library.
//!
//! A gRPC client library for Google Ads API, generated automatically from the API definition files.
//!
//! Provides `GoogleAdsRow.get(path: &str)` accessor method to easily retrieve fields selected in GAQL.
//! Also provides `DynamicMutationBuilder` for constructing mutation requests dynamically via reflection.
//!
//! # Example — Reading
//!
//! ```ignore
//! let field_mask = response.field_mask.unwrap();
//! for row in response.results {
//!     for path in &field_mask.paths {
//!         print!("{}: {}\t", path, row.get(&path));
//!     }
//!     print!("\n");
//! }
//! ```
//!
//! # Example — Mutating
//!
//! ```ignore
//! let mut builder = DynamicMutationBuilder::new("Campaign", "1234567890");
//! builder.set_field("target_roas.target_roas", "3.5");
//! let request = builder.build()?;
//! client.mutate(request).await?;
//! ```

#![doc(html_root_url = "https://docs.rs/googleads-rs/23.2.0")]

#[allow(clippy::all)]
#[allow(clippy::doc_lazy_continuation)]
#[allow(unused_must_use)]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos.rs"));
}
pub use protos::*;

use once_cell::sync::Lazy;
use prost::Message;
use prost_reflect::{DescriptorPool, DynamicMessage, FieldDescriptor, Kind, ReflectMessage, Value};
use std::io::Cursor;

static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
    DescriptorPool::decode(bytes.as_ref()).expect("Failed to decode file descriptor set")
});

pub fn descriptor_pool() -> &'static DescriptorPool {
    &DESCRIPTOR_POOL
}

// ---------------------------------------------------------------------------
// Mutation types
// ---------------------------------------------------------------------------

const RESOURCES_FQN_PREFIX: &str = "google.ads.googleads.v23.resources";
const SERVICES_FQN_PREFIX: &str = "google.ads.googleads.v23.services";
const MUTATE_OP_FQN: &str = "google.ads.googleads.v23.services.MutateOperation";
const MUTATE_REQUEST_FQN: &str = "google.ads.googleads.v23.services.MutateGoogleAdsRequest";
const FIELD_MASK_FQN: &str = "google.protobuf.FieldMask";

#[derive(Debug, Clone)]
pub struct FieldUpdate {
    pub field_path: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationOp {
    Update,
    Create,
    Remove,
}

#[derive(Debug, Clone)]
pub struct DynamicMutationBuilder {
    resource_type: String,
    customer_id: String,
    operation_type: MutationOp,
    field_updates: Vec<FieldUpdate>,
    validate_only: bool,
    partial_failure: bool,
}

impl DynamicMutationBuilder {
    pub fn new(resource_type: &str, customer_id: &str) -> Self {
        Self {
            resource_type: resource_type.to_string(),
            customer_id: customer_id.to_string(),
            operation_type: MutationOp::Update,
            field_updates: Vec::new(),
            validate_only: false,
            partial_failure: true,
        }
    }

    pub fn set_field(&mut self, path: &str, value: &str) -> &mut Self {
        self.field_updates.push(FieldUpdate {
            field_path: path.to_string(),
            value: value.to_string(),
        });
        self
    }

    pub fn operation_type(&mut self, op: MutationOp) -> &mut Self {
        self.operation_type = op;
        self
    }

    pub fn validate_only(&mut self, v: bool) -> &mut Self {
        self.validate_only = v;
        self
    }

    pub fn partial_failure(&mut self, v: bool) -> &mut Self {
        self.partial_failure = v;
        self
    }

    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }

    pub fn field_updates(&self) -> &[FieldUpdate] {
        &self.field_updates
    }

    pub fn build_operation(&self, resource_name: &str) -> anyhow::Result<DynamicMessage> {
        let resource_fqn = format!("{}.{}", RESOURCES_FQN_PREFIX, self.resource_type);
        let resource_desc = DESCRIPTOR_POOL
            .get_message_by_name(&resource_fqn)
            .ok_or_else(|| {
                anyhow::anyhow!("Resource {} not found in descriptor pool", resource_fqn)
            })?;

        let mut resource = DynamicMessage::new(resource_desc);
        set_field_path_value(&mut resource, "resource_name", resource_name)?;

        for update in &self.field_updates {
            set_field_path_value(&mut resource, &update.field_path, &update.value)?;
        }

        let op_fqn = format!("{}.{}Operation", SERVICES_FQN_PREFIX, self.resource_type);
        let op_desc = DESCRIPTOR_POOL
            .get_message_by_name(&op_fqn)
            .ok_or_else(|| anyhow::anyhow!("Operation {} not found in descriptor pool", op_fqn))?;

        let mut operation = DynamicMessage::new(op_desc);

        match self.operation_type {
            MutationOp::Update => {
                operation.set_field_by_name("update", Value::Message(resource));
                let mask = self.build_field_mask_message()?;
                operation.set_field_by_name("update_mask", Value::Message(mask));
            }
            MutationOp::Create => {
                operation.set_field_by_name("create", Value::Message(resource));
            }
            MutationOp::Remove => {
                operation.set_field_by_name("remove", Value::String(resource_name.to_string()));
            }
        }

        let op_field_name = to_snake_case(&self.resource_type) + "_operation";

        let mutate_op_desc = DESCRIPTOR_POOL
            .get_message_by_name(MUTATE_OP_FQN)
            .ok_or_else(|| anyhow::anyhow!("MutateOperation not found in descriptor pool"))?;

        let mut mutate_op = DynamicMessage::new(mutate_op_desc);
        mutate_op.set_field_by_name(&op_field_name, Value::Message(operation));

        Ok(mutate_op)
    }

    pub fn build(
        &self,
        resource_name: &str,
    ) -> anyhow::Result<google::ads::googleads::v23::services::MutateGoogleAdsRequest> {
        let mutate_op = self.build_operation(resource_name)?;

        let request_desc = DESCRIPTOR_POOL
            .get_message_by_name(MUTATE_REQUEST_FQN)
            .ok_or_else(|| {
                anyhow::anyhow!("MutateGoogleAdsRequest not found in descriptor pool")
            })?;

        let mut request = DynamicMessage::new(request_desc);
        request.set_field_by_name("customer_id", Value::String(self.customer_id.clone()));
        request.set_field_by_name(
            "mutate_operations",
            Value::List(vec![Value::Message(mutate_op)]),
        );
        request.set_field_by_name("partial_failure", Value::Bool(self.partial_failure));
        request.set_field_by_name("validate_only", Value::Bool(self.validate_only));

        let static_request = request
            .transcode_to::<google::ads::googleads::v23::services::MutateGoogleAdsRequest>()
            .map_err(|e| anyhow::anyhow!("Failed to transcode MutateGoogleAdsRequest: {}", e))?;

        Ok(static_request)
    }

    fn build_field_mask_message(&self) -> anyhow::Result<DynamicMessage> {
        let desc = DESCRIPTOR_POOL
            .get_message_by_name(FIELD_MASK_FQN)
            .ok_or_else(|| anyhow::anyhow!("FieldMask not found in descriptor pool"))?;

        let paths = generate_field_mask(&self.field_updates);
        let mut mask = DynamicMessage::new(desc);
        mask.set_field_by_name(
            "paths",
            Value::List(paths.into_iter().map(Value::String).collect()),
        );
        Ok(mask)
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

pub fn coerce_value(value_str: &str, field_desc: &FieldDescriptor) -> anyhow::Result<Value> {
    match field_desc.kind() {
        Kind::Double => value_str
            .parse::<f64>()
            .map(Value::F64)
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}' as double: {}", value_str, e)),
        Kind::Float => value_str
            .parse::<f32>()
            .map(Value::F32)
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}' as float: {}", value_str, e)),
        Kind::Int32 | Kind::Sint32 | Kind::Sfixed32 => value_str
            .parse::<i32>()
            .map(Value::I32)
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}' as int32: {}", value_str, e)),
        Kind::Int64 | Kind::Sint64 | Kind::Sfixed64 => value_str
            .parse::<i64>()
            .map(Value::I64)
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}' as int64: {}", value_str, e)),
        Kind::Uint32 | Kind::Fixed32 => value_str
            .parse::<u32>()
            .map(Value::U32)
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}' as uint32: {}", value_str, e)),
        Kind::Uint64 | Kind::Fixed64 => value_str
            .parse::<u64>()
            .map(Value::U64)
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}' as uint64: {}", value_str, e)),
        Kind::Bool => value_str
            .parse::<bool>()
            .map(Value::Bool)
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}' as bool: {}", value_str, e)),
        Kind::String => Ok(Value::String(value_str.to_string())),
        Kind::Enum(enum_desc) => {
            if let Some(ev) = enum_desc.get_value_by_name(value_str) {
                Ok(Value::EnumNumber(ev.number()))
            } else {
                value_str
                    .parse::<i32>()
                    .map(Value::EnumNumber)
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to parse '{}' as enum name or number: {}",
                            value_str,
                            e
                        )
                    })
            }
        }
        _ => Err(anyhow::anyhow!(
            "Cannot coerce value to type {:?}",
            field_desc.kind()
        )),
    }
}

pub fn set_field_path_value(
    msg: &mut DynamicMessage,
    field_path: &str,
    value_str: &str,
) -> anyhow::Result<()> {
    let segments: Vec<&str> = field_path.split('.').collect();
    if segments.is_empty() {
        return Err(anyhow::anyhow!("Empty field path"));
    }
    set_field_path_recursive(msg, &segments, value_str)
}

fn set_field_path_recursive(
    msg: &mut DynamicMessage,
    segments: &[&str],
    value_str: &str,
) -> anyhow::Result<()> {
    let segment = segments[0];
    let remaining = &segments[1..];

    let field_desc = msg.descriptor().get_field_by_name(segment).ok_or_else(|| {
        anyhow::anyhow!(
            "Field '{}' not found on {}",
            segment,
            msg.descriptor().full_name()
        )
    })?;

    if remaining.is_empty() {
        let value = coerce_value(value_str, &field_desc)?;
        msg.set_field(&field_desc, value);
        Ok(())
    } else {
        match field_desc.kind() {
            Kind::Message(nested_desc) => {
                let mut nested = if msg.has_field(&field_desc) {
                    match &*msg.get_field(&field_desc) {
                        Value::Message(existing) => existing.clone(),
                        _ => DynamicMessage::new(nested_desc.clone()),
                    }
                } else {
                    DynamicMessage::new(nested_desc.clone())
                };
                set_field_path_recursive(&mut nested, remaining, value_str)?;
                msg.set_field(&field_desc, Value::Message(nested));
                Ok(())
            }
            _ => Err(anyhow::anyhow!(
                "Cannot traverse into non-message field '{}' of type {:?}",
                segment,
                field_desc.kind()
            )),
        }
    }
}

pub fn generate_field_mask(field_updates: &[FieldUpdate]) -> Vec<String> {
    field_updates.iter().map(|u| u.field_path.clone()).collect()
}

const GOOGLE_ADS_ROW_FQN: &str = "google.ads.googleads.v23.services.GoogleAdsRow";

impl google::ads::googleads::v23::services::GoogleAdsRow {
    /// Returns a field value from the GoogleAdsRow by its GAQL field path.
    ///
    /// This method uses `prost-reflect` to dynamically access any field in the row
    /// using the same dot-separated paths used in GAQL queries (e.g., `"campaign.id"`,
    /// `"ad_group.name"`). This eliminates the need to hardcode field accessors for
    /// each field in the API.
    ///
    /// # Arguments
    /// * `field_name` - A dot-separated field path (e.g., `"campaign.id"`, `"segments.device"`)
    ///
    /// # Returns
    /// The field value formatted as a string. Returns an empty string if the field
    /// is not set or doesn't exist in the response.
    ///
    /// # Performance
    /// This method encodes the row to protobuf bytes and decodes it as a `DynamicMessage`
    /// on each call. For retrieving multiple fields from the same row, prefer [`Self::get_many`].
    ///
    /// # Example
    ///
    /// ```ignore
    /// // After executing a GAQL query like:
    /// // SELECT campaign.id, campaign.name, campaign.status FROM campaign
    ///
    /// let field_mask = response.field_mask.unwrap();
    /// for row in response.results {
    ///     for path in &field_mask.paths {
    ///         // path might be "campaign.id", "campaign.name", etc.
    ///         print!("{}: {}\t", path, row.get(path));
    ///     }
    ///     print!("\n");
    /// }
    /// ```
    pub fn get(&self, field_name: &str) -> String {
        // Encode the GoogleAdsRow to bytes, then decode as DynamicMessage
        let encoded = self.encode_to_vec();

        let descriptor = DESCRIPTOR_POOL
            .get_message_by_name(GOOGLE_ADS_ROW_FQN)
            .expect("GoogleAdsRow descriptor not found");

        let dynamic_msg = DynamicMessage::decode(descriptor, Cursor::new(&encoded))
            .expect("Failed to decode GoogleAdsRow as DynamicMessage");

        self.get_field_from_dynamic(&dynamic_msg, field_name)
    }

    /// Returns multiple field values from the GoogleAdsRow efficiently.
    ///
    /// Like [`Self::get`], this uses `prost-reflect` for dynamic field access, but encodes
    /// the row only once and retrieves all requested fields. This is significantly more
    /// efficient than calling `get()` multiple times when you need several fields.
    ///
    /// # Arguments
    /// * `field_names` - A slice of dot-separated field paths (e.g., `["campaign.id", "campaign.name"]`)
    ///
    /// # Returns
    /// A `Vec<String>` containing the field values in the same order as `field_names`.
    /// Returns empty strings for fields that are not set or don't exist in the response.
    ///
    /// # Performance
    /// Encodes the row to protobuf bytes once and walks all paths in a single pass.
    /// Preferred over multiple `get()` calls when retrieving 2+ fields.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // After executing a GAQL query with multiple selected fields
    /// let fields = vec!["campaign.id", "campaign.name", "campaign.status", "segments.device"];
    ///
    /// for row in response.results {
    ///     let values = row.get_many(&fields);
    ///     // values[0] = campaign.id, values[1] = campaign.name, etc.
    ///     println!("Campaign {} (ID: {}) is {:?} on {:?}",
    ///         &values[1], &values[0], &values[2], &values[3]);
    /// }
    /// ```
    pub fn get_many(&self, field_names: &[&str]) -> Vec<String> {
        // Encode the GoogleAdsRow to bytes, then decode as DynamicMessage
        let encoded = self.encode_to_vec();

        let descriptor = DESCRIPTOR_POOL
            .get_message_by_name(GOOGLE_ADS_ROW_FQN)
            .expect("GoogleAdsRow descriptor not found");

        let dynamic_msg = DynamicMessage::decode(descriptor, Cursor::new(&encoded))
            .expect("Failed to decode GoogleAdsRow as DynamicMessage");

        field_names
            .iter()
            .map(|field_name| self.get_field_from_dynamic(&dynamic_msg, field_name))
            .collect()
    }

    /// Internal method to get a field value from a DynamicMessage
    fn get_field_from_dynamic(&self, dyn_msg: &DynamicMessage, field_name: &str) -> String {
        match field_name {
            // Special case for campaign.asset_automation_settings
            field if field.starts_with("campaign.asset_automation_settings") => {
                self.format_asset_automation_settings(dyn_msg)
            }
            // Special case for ad_group_ad.ad.responsive_search_ad.headlines/descriptions
            field
                if field.starts_with("ad_group_ad.ad.responsive_search_ad.headlines")
                    || field.starts_with("ad_group_ad.ad.responsive_search_ad.descriptions") =>
            {
                // The GAQL path stops at the repeated message, but users expect .text extracted
                self.format_value_at_path(dyn_msg, &format!("{}.text", field_name))
            }
            // General case: use reflection to walk the path
            _ => self.format_value_at_path(dyn_msg, field_name),
        }
    }

    /// Format asset_automation_settings as "TYPE:STATUS" pairs
    fn format_asset_automation_settings(&self, dyn_msg: &DynamicMessage) -> String {
        // Navigate to campaign message
        let campaign_field = dyn_msg
            .descriptor()
            .get_field_by_name("campaign")
            .expect("campaign field not found");

        if !dyn_msg.has_field(&campaign_field) {
            return String::new();
        }

        let campaign_value = dyn_msg.get_field(&campaign_field);
        let campaign_msg = match &*campaign_value {
            Value::Message(msg) => msg,
            _ => return String::new(),
        };

        // Get asset_automation_settings repeated field
        let settings_field = campaign_msg
            .descriptor()
            .get_field_by_name("asset_automation_settings")
            .expect("asset_automation_settings field not found");

        if !campaign_msg.has_field(&settings_field) {
            return String::new();
        }

        let settings_value = campaign_msg.get_field(&settings_field);
        let settings_list = match &*settings_value {
            Value::List(list) => list,
            _ => return String::new(),
        };

        // Format each item as "TYPE:STATUS"
        settings_list
            .iter()
            .filter_map(|item| match item {
                Value::Message(setting_msg) => {
                    let type_field = match setting_msg
                        .descriptor()
                        .get_field_by_name("asset_automation_type")
                    {
                        Some(f) => f,
                        None => return None,
                    };
                    let type_value = setting_msg.get_field(&type_field);

                    let status_field = match setting_msg
                        .descriptor()
                        .get_field_by_name("asset_automation_status")
                    {
                        Some(f) => f,
                        None => return None,
                    };
                    let status_value = setting_msg.get_field(&status_field);

                    let type_name = self.format_scalar(&type_value, &type_field);
                    let status_name = self.format_scalar(&status_value, &status_field);

                    Some(format!("{}:{}", type_name, status_name))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Format FieldMask as comma-separated list of paths
    fn format_field_mask(&self, field_mask: &DynamicMessage) -> String {
        let paths_field = match field_mask.descriptor().get_field_by_name("paths") {
            Some(f) => f,
            None => return String::new(),
        };

        // Don't check has_field for repeated fields - just get the value
        let paths_value = field_mask.get_field(&paths_field);
        match &*paths_value {
            Value::List(list) => list
                .iter()
                .filter_map(|item| match item {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(", "),
            _ => String::new(),
        }
    }

    /// Format value at a dotted path
    fn format_value_at_path(&self, msg: &DynamicMessage, path: &str) -> String {
        let path_segments: Vec<&str> = path.split('.').collect();
        self.format_value_recursive(msg, &path_segments, None)
    }

    /// Recursively format value at a path
    fn format_value_recursive(
        &self,
        msg: &DynamicMessage,
        path: &[&str],
        _field_desc: Option<&prost_reflect::FieldDescriptor>,
    ) -> String {
        if path.is_empty() {
            // This shouldn't happen in normal usage
            return format!("{:?}", msg);
        }

        let segment = path[0];

        // Check for empty segment (from trailing dots or double dots)
        if segment.is_empty() {
            return "not implemented by googleads-rs".to_string();
        }

        let remaining = &path[1..];

        // Look up the field by name
        let desc = match msg.descriptor().get_field_by_name(segment) {
            Some(desc) => desc,
            None => return "not implemented by googleads-rs".to_string(),
        };

        // Check if field has presence and is unset
        if desc.supports_presence() && !msg.has_field(&desc) {
            // Before returning empty, validate that remaining path would be valid
            // This ensures invalid paths like "campaign.invalid_field" return "not implemented"
            // even when campaign is not set
            if !remaining.is_empty() {
                // Try to validate the remaining path by checking field existence
                if let prost_reflect::Kind::Message(msg_desc) = desc.kind() {
                    // Validate the next segment exists
                    if msg_desc.get_field_by_name(remaining[0]).is_none() {
                        return "not implemented by googleads-rs".to_string();
                    }
                }
            }
            return String::new();
        }

        let value = msg.get_field(&desc);

        match &*value {
            Value::Message(sub_msg) => {
                if remaining.is_empty() {
                    // Format the message directly
                    if sub_msg.descriptor().full_name() == "google.protobuf.FieldMask" {
                        self.format_field_mask(sub_msg)
                    } else {
                        // Partial paths (e.g., "campaign" without a field) are not supported
                        "not implemented by googleads-rs".to_string()
                    }
                } else {
                    // Continue traversing the path
                    self.format_value_recursive(sub_msg, remaining, None)
                }
            }
            Value::List(list) => {
                if remaining.is_empty() {
                    self.format_list(list, &desc)
                } else {
                    // Walk into each message item
                    list.iter()
                        .map(|item| match item {
                            Value::Message(sub) => {
                                self.format_value_recursive(sub, remaining, None)
                            }
                            _ => String::new(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            }
            _ => {
                if remaining.is_empty() {
                    self.format_scalar(&value, &desc)
                } else {
                    // Can't recurse into scalar types - any remaining path is invalid
                    "not implemented by googleads-rs".to_string()
                }
            }
        }
    }

    /// Format a scalar value
    fn format_scalar(&self, value: &Value, field_desc: &prost_reflect::FieldDescriptor) -> String {
        match value {
            Value::EnumNumber(n) => {
                // Resolve enum number to name
                if let prost_reflect::Kind::Enum(enum_desc) = field_desc.kind() {
                    enum_desc
                        .get_value(*n)
                        .map(|v| v.name().to_string())
                        .unwrap_or_else(|| n.to_string())
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::I32(i) => i.to_string(),
            Value::I64(i) => i.to_string(),
            Value::U32(u) => u.to_string(),
            Value::U64(u) => u.to_string(),
            Value::F32(f) => f.to_string(),
            Value::F64(d) => d.to_string(),
            Value::Bytes(b) => format!("{:?}", b),
            _ => format!("{:?}", value),
        }
    }

    /// Format a list of values
    fn format_list(&self, items: &[Value], field_desc: &prost_reflect::FieldDescriptor) -> String {
        if items.is_empty() {
            return String::new();
        }

        let is_message_list = items.iter().any(|v| matches!(v, Value::Message(_)));

        // For messages, use ; as separator (matches existing behavior)
        let sep = if is_message_list { "; " } else { ", " };

        items
            .iter()
            .map(|item| match item {
                Value::Message(msg) => self.format_message_compact(msg),
                _ => self.format_scalar(item, field_desc),
            })
            .collect::<Vec<_>>()
            .join(sep)
    }

    /// Format a message in a compact "field:value" format
    fn format_message_compact(&self, msg: &DynamicMessage) -> String {
        let fields: Vec<String> = msg
            .descriptor()
            .fields()
            .filter_map(|field_desc| {
                // Only show fields that are set
                if field_desc.supports_presence() && !msg.has_field(&field_desc) {
                    return None;
                }

                let value = msg.get_field(&field_desc);
                let formatted_value = match &*value {
                    Value::Message(sub_msg) => self.format_message_compact(sub_msg),
                    _ => self.format_scalar(&value, &field_desc),
                };

                if formatted_value.is_empty() {
                    None
                } else {
                    Some(format!("{}:{}", field_desc.name(), formatted_value))
                }
            })
            .collect();

        fields.join(" ")
    }
}
