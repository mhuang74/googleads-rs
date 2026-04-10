//! Google Ads gRPC library.
//!
//! A gRPC client library for Google Ads API, generated automatically from the API definition files.
//!
//! Provides `GoogleAdsRow.get(path: &str)` accessor method to easily retrieve fields selected in GAQL.
//!
//! # Example
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
use prost_reflect::{DescriptorPool, DynamicMessage, ReflectMessage, Value};
use std::io::Cursor;

static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));
    DescriptorPool::decode(bytes.as_ref()).expect("Failed to decode file descriptor set")
});

const GOOGLE_ADS_ROW_FQN: &str = "google.ads.googleads.v23.services.GoogleAdsRow";

impl google::ads::googleads::v23::services::GoogleAdsRow {
    /// Returns GoogleAdsRow field value by field name
    ///
    /// # Arguments
    /// * `field_name` - A string slice that holds name of a field in GoogleAdsRow struct
    ///
    /// # Example
    ///
    /// ```ignore
    /// let field_mask = response.field_mask.unwrap();
    /// for row in response.results {
    ///     for path in &field_mask.paths {
    ///         print!("{}: {}\t", path, row.get(&path));
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

    /// Returns multiple field values efficiently
    ///
    /// # Arguments
    /// * `field_names` - A slice of field names to retrieve
    ///
    /// This encodes the GoogleAdsRow once and then walks multiple paths,
    /// which is more efficient than calling `get()` multiple times.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let fields = vec!["campaign.id", "campaign.name", "campaign.status"];
    /// let values = row.get_many(&fields);
    /// ```
    pub fn get_many(&self, field_names: &[&str]) -> Vec<String> {
        // Encode the GoogleAdsRow to bytes, then decode as DynamicMessage
        let encoded = self.encode_to_vec();

        let descriptor = DESCRIPTOR_POOL
            .get_message_by_name(GOOGLE_ADS_ROW_FQN)
            .expect("GoogleAdsRow descriptor not found");

        let dynamic_msg = DynamicMessage::decode(descriptor, Cursor::new(&encoded))
            .expect("Failed to decode GoogleAdsRow as DynamicMessage");

        field_names.iter()
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
            field if field.starts_with("ad_group_ad.ad.responsive_search_ad.headlines") ||
                   field.starts_with("ad_group_ad.ad.responsive_search_ad.descriptions") => {
                // The GAQL path stops at the repeated message, but users expect .text extracted
                self.format_value_at_path(dyn_msg, &format!("{}.text", field_name))
            }
            // General case: use reflection to walk the path
            _ => self.format_value_at_path(dyn_msg, field_name)
        }
    }

    /// Format asset_automation_settings as "TYPE:STATUS" pairs
    fn format_asset_automation_settings(&self, dyn_msg: &DynamicMessage) -> String {
        // Navigate to campaign message
        let campaign_field = dyn_msg.descriptor().get_field_by_name("campaign")
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
        let settings_field = campaign_msg.descriptor().get_field_by_name("asset_automation_settings")
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
        settings_list.iter()
            .filter_map(|item| match item {
                Value::Message(setting_msg) => {
                    let type_field = match setting_msg.descriptor().get_field_by_name("asset_automation_type") {
                        Some(f) => f,
                        None => return None,
                    };
                    let type_value = setting_msg.get_field(&type_field);

                    let status_field = match setting_msg.descriptor().get_field_by_name("asset_automation_status") {
                        Some(f) => f,
                        None => return None,
                    };
                    let status_value = setting_msg.get_field(&status_field);

                    let type_name = self.format_scalar(&type_value, &type_field);
                    let status_name = self.format_scalar(&status_value, &status_field);

                    Some(format!("{}:{}", type_name, status_name))
                }
                _ => None
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
            Value::List(list) => {
                list.iter()
                    .filter_map(|item| match item {
                        Value::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            }
            _ => String::new(),
        }
    }

    /// Format value at a dotted path
    fn format_value_at_path(&self, msg: &DynamicMessage, path: &str) -> String {
        let path_segments: Vec<&str> = path.split('.').collect();
        self.format_value_recursive(msg, &path_segments, None)
    }

    /// Recursively format value at a path
    fn format_value_recursive(&self, msg: &DynamicMessage, path: &[&str], _field_desc: Option<&prost_reflect::FieldDescriptor>) -> String {
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
            None => return "not implemented by googleads-rs".to_string()
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
                            Value::Message(sub) => self.format_value_recursive(sub, remaining, None),
                            _ => String::new()
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
                    enum_desc.get_value(*n)
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
            _ => format!("{:?}", value)
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

        items.iter()
            .map(|item| match item {
                Value::Message(msg) => self.format_message_compact(msg),
                _ => self.format_scalar(item, field_desc)
            })
            .collect::<Vec<_>>()
            .join(sep)
    }

    /// Format a message in a compact "field:value" format
    fn format_message_compact(&self, msg: &DynamicMessage) -> String {
        let fields: Vec<String> = msg.descriptor()
            .fields()
            .filter_map(|field_desc| {
                // Only show fields that are set
                if field_desc.supports_presence() && !msg.has_field(&field_desc) {
                    return None;
                }

                let value = msg.get_field(&field_desc);
                let formatted_value = match &*value {
                    Value::Message(sub_msg) => self.format_message_compact(sub_msg),
                    _ => self.format_scalar(&value, &field_desc)
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
