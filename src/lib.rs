//! Google Ads gRPC library.
//!
//! A gRPC client library for Google Ads API, generated automatically from the API definition files.
//!
//! Provides `GoogleAdsRow.get(path: &str)` accessor method to easily retrieve fields selected in GAQL.
//!
//! Uses prost-reflect for dynamic field access, enabling support for any valid GAQL field path.
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
//!

#![doc(html_root_url = "https://docs.rs/googleads-rs/23.2.0")]

#[allow(clippy::all)]
#[allow(clippy::doc_lazy_continuation)]
#[allow(unused_must_use)]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos.rs"));
}
pub use protos::*;

use prost::Message;
use prost_reflect::{DynamicMessage, ReflectMessage, Value};
use std::sync::LazyLock;

/// File descriptor set bytes generated at build time
static FILE_DESCRIPTOR_SET_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));

/// Lazy-initialized descriptor pool for prost-reflect
static DESCRIPTOR_POOL: LazyLock<prost_reflect::DescriptorPool> = LazyLock::new(|| {
    prost_reflect::DescriptorPool::decode(FILE_DESCRIPTOR_SET_BYTES).expect(
        "Failed to decode file descriptor set. This should not happen if the build succeeded.",
    )
});

impl google::ads::googleads::v23::services::GoogleAdsRow {
    /// Returns GoogleAdsRow field value by field name
    ///
    /// # Arguments
    /// * `field_name` - A string slice that holds name of a field in GoogleAdsRow struct
    ///
    /// # Returns
    /// The field value as a string. Returns an empty string if the field is unset,
    /// or "not implemented by googleads-rs" if the field path is invalid.
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
        // Encode the row to bytes and decode as DynamicMessage
        let mut buf = Vec::new();
        if self.encode(&mut buf).is_err() {
            return "not implemented by googleads-rs".to_string();
        }

        // Get the GoogleAdsRow message descriptor
        let descriptor = DESCRIPTOR_POOL.get_message_by_name(
            "google.ads.googleads.v23.services.GoogleAdsRow"
        );

        let Some(descriptor) = descriptor else {
            return "not implemented by googleads-rs".to_string();
        };

        let Ok(dynamic) = DynamicMessage::decode(descriptor, &*buf) else {
            return "not implemented by googleads-rs".to_string();
        };

        Self::walk_path(&dynamic, field_name)
    }

    /// Returns multiple field values from the same row efficiently.
    /// The row is encoded once, then each field path is walked separately.
    pub fn get_many(&self, field_names: &[&str]) -> Vec<String> {
        let mut buf = Vec::new();
        if self.encode(&mut buf).is_err() {
            return field_names.iter().map(|_| "not implemented by googleads-rs".to_string()).collect();
        }

        let descriptor = DESCRIPTOR_POOL.get_message_by_name(
            "google.ads.googleads.v23.services.GoogleAdsRow"
        );

        let Some(descriptor) = descriptor else {
            return field_names.iter().map(|_| "not implemented by googleads-rs".to_string()).collect();
        };

        let Ok(dynamic) = DynamicMessage::decode(descriptor, &*buf) else {
            return field_names.iter().map(|_| "not implemented by googleads-rs".to_string()).collect();
        };

        field_names.iter().map(|path| Self::walk_path(&dynamic, path)).collect()
    }

    /// Walks a dotted GAQL field path and returns the value as a string
    fn walk_path(msg: &DynamicMessage, path: &str) -> String {
        // Handle empty path
        if path.is_empty() {
            return "not implemented by googleads-rs".to_string();
        }

        let segments: Vec<&str> = path.split('.').collect();
        Self::walk_path_recursive(msg, &segments, 0)
    }

    /// Recursive helper for walking path segments
    fn walk_path_recursive(msg: &DynamicMessage, segments: &[&str], index: usize) -> String {
        if index >= segments.len() {
            return "".to_string();
        }

        let segment = segments[index];

        // Get field from current message
        let Some(field) = msg.descriptor().get_field_by_name(segment) else {
            return "not implemented by googleads-rs".to_string();
        };

        // Check if this is an optional field that supports presence and it's not set
        if field.supports_presence() && !msg.has_field(&field) {
            // Return default value based on field type for scalar fields
            return Self::default_value_for_field(&field);
        }

        let value = msg.get_field(&field);

        // If this is the last segment, format and return the value
        if index == segments.len() - 1 {
            // If the final value is a message, return "not implemented" since
            // we can't format a message as a scalar string
            if matches!(value.as_ref(), Value::Message(_)) {
                return "not implemented by googleads-rs".to_string();
            }
            return Self::format_value(&value, Some(&field));
        }

        // Not the last segment - need to traverse deeper
        match value.as_ref() {
            Value::Message(nested_msg) => {
                Self::walk_path_recursive(nested_msg, segments, index + 1)
            }
            _ => {
                // Cannot traverse further - return "not implemented"
                "not implemented by googleads-rs".to_string()
            }
        }
    }

    /// Returns the default value for a field as a string
    fn default_value_for_field(field: &prost_reflect::FieldDescriptor) -> String {
        match field.kind() {
            prost_reflect::Kind::Int32 | prost_reflect::Kind::Int64 |
            prost_reflect::Kind::Sint32 | prost_reflect::Kind::Sint64 |
            prost_reflect::Kind::Sfixed32 | prost_reflect::Kind::Sfixed64 => "0".to_string(),
            prost_reflect::Kind::Uint32 | prost_reflect::Kind::Uint64 |
            prost_reflect::Kind::Fixed32 | prost_reflect::Kind::Fixed64 => "0".to_string(),
            prost_reflect::Kind::Float | prost_reflect::Kind::Double => "0".to_string(),
            prost_reflect::Kind::Bool => "false".to_string(),
            prost_reflect::Kind::String => "".to_string(),
            prost_reflect::Kind::Bytes => "".to_string(),
            prost_reflect::Kind::Enum(enum_desc) => {
                // Find the default value (usually 0 which is UNSPECIFIED)
                if let Some(default_value) = enum_desc.values().find(|v| v.number() == 0) {
                    default_value.name().to_string()
                } else {
                    "".to_string()
                }
            }
            prost_reflect::Kind::Message(_) => "".to_string(),
        }
    }

    /// Formats any prost-reflect Value as a string
    fn format_value(value: &Value, field_desc: Option<&prost_reflect::FieldDescriptor>) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::I64(v) => v.to_string(),
            Value::I32(v) => v.to_string(),
            Value::U64(v) => v.to_string(),
            Value::U32(v) => v.to_string(),
            Value::F64(v) => v.to_string(),
            Value::F32(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Bytes(b) => format!("{:?}", b),
            Value::EnumNumber(num) => {
                // Resolve to enum variant name using the field descriptor
                if let Some(field) = field_desc {
                    // Get the enum descriptor from the field's kind
                    if let prost_reflect::Kind::Enum(enum_desc) = field.kind() {
                        // Try to find the variant with this number by iterating through values
                        for value in enum_desc.values() {
                            if value.number() == *num {
                                return value.name().to_string();
                            }
                        }
                    }
                }
                // Fallback: return the number as string
                num.to_string()
            }
            Value::Message(msg) => {
                // For terminal messages, format with debug
                format!("{:#?}", msg)
            }
            Value::List(list) => {
                if list.is_empty() {
                    return "".to_string();
                }

                // Format based on the element type
                let first = &list[0];
                match first {
                    Value::String(_) | Value::I64(_) | Value::I32(_) | Value::U64(_) | Value::U32(_) |
                    Value::F64(_) | Value::F32(_) | Value::Bool(_) | Value::EnumNumber(_) => {
                        // Scalars and enums - join with ", "
                        list.iter()
                            .map(|v| Self::format_value(v, field_desc))
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                    Value::Message(_) => {
                        // Messages - format each with debug, join with "; "
                        list.iter()
                            .map(|v| Self::format_value(v, field_desc))
                            .collect::<Vec<_>>()
                            .join("; ")
                    }
                    _ => {
                        list.iter()
                            .map(|v| Self::format_value(v, field_desc))
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                }
            }
            Value::Map(map) => {
                if map.is_empty() {
                    return "".to_string();
                }
                format!("{:#?}", map)
            }
        }
    }
}
