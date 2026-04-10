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

    /// Format value at a dotted path
    fn format_value_at_path(&self, msg: &DynamicMessage, path: &str) -> String {
        let path_segments: Vec<&str> = path.split('.').collect();
        self.format_value_recursive(msg, &path_segments, None)
    }

    /// Recursively format value at a path
    fn format_value_recursive(&self, msg: &DynamicMessage, path: &[&str], field_desc: Option<&prost_reflect::FieldDescriptor>) -> String {
        if path.is_empty() {
            if let Some(desc) = field_desc {
                let value = msg.get_field(desc);
                return match &*value {
                    Value::Message(sub) => format!("{:?}", sub),
                    Value::List(list) => self.format_list(list, desc),
                    _ => self.format_scalar(&value, desc),
                };
            } else {
                return format!("{:?}", msg);
            }
        }

        let segment = path[0];
        let remaining = &path[1..];

        // Look up the field by name
        let desc = match msg.descriptor().get_field_by_name(segment) {
            Some(desc) => desc,
            None => return "not implemented by googleads-rs".to_string()
        };

        // Check if field has presence and is unset
        if desc.supports_presence() && !msg.has_field(&desc) {
            return String::new();
        }

        let value = msg.get_field(&desc);

        match &*value {
            Value::Message(sub_msg) => {
                self.format_value_recursive(sub_msg, remaining, Some(&desc))
            }
            Value::List(list) => {
                if remaining.is_empty() {
                    self.format_list(list, &desc)
                } else {
                    // Walk into each message item
                    list.iter()
                        .map(|item| match item {
                            Value::Message(sub) => self.format_value_recursive(sub, remaining, Some(&desc)),
                            _ => String::new()
                        })
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            }
            _ => {
                if remaining.is_empty() {
                    self.format_scalar(&value, &desc)
                } else {
                    // Can't recurse into scalar types
                    String::new()
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
                Value::Message(msg) => format!("{:?}", msg),
                _ => self.format_scalar(item, field_desc)
            })
            .collect::<Vec<_>>()
            .join(sep)
    }
}
