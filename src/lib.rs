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
//!

#![doc(html_root_url = "https://docs.rs/googleads-rs/0.10.0")]

include!(concat!(env!("OUT_DIR"), "/protos.rs"));

use crate::google::ads::googleads::v19::enums::bidding_strategy_type_enum::{
    BiddingStrategyType::ManualCpc, BiddingStrategyType::MaximizeConversionValue,
    BiddingStrategyType::MaximizeConversions, BiddingStrategyType::TargetCpa,
    BiddingStrategyType::TargetImpressionShare, BiddingStrategyType::TargetRoas,
};

use crate::google::ads::googleads::v19::enums::{
    ad_group_ad_primary_status_reason_enum::AdGroupAdPrimaryStatusReason,
    ad_group_criterion_primary_status_reason_enum::AdGroupCriterionPrimaryStatusReason,
    ad_group_primary_status_reason_enum::AdGroupPrimaryStatusReason,
    asset_group_primary_status_reason_enum::AssetGroupPrimaryStatusReason,
    asset_link_primary_status_reason_enum::AssetLinkPrimaryStatusReason,
    campaign_primary_status_reason_enum::CampaignPrimaryStatusReason,
};

use crate::google::ads::googleads::v19::resources::{
    ad::AdData::ResponsiveSearchAd, ad_group_criterion::Criterion::Keyword,
    campaign_criterion::Criterion::Keyword as CampaignKeyword,
    campaign_criterion::Criterion::Location, shared_criterion::Criterion::Keyword as SharedKeyword,
};

impl google::ads::googleads::v19::services::GoogleAdsRow {
    /// Returns GoogleAdsRow field value by field name
    ///
    /// # Arguments
    /// * `field_name` - A string slice that holds name of a field in GoogleAdsRow struct
    ///
    /// Support fields:
    /// * all non-list Metrics fields are supported
    /// * all non-list Segment fields are supported
    /// * basic Attribute fields are supported (eg. id, name, status)
    /// * other Attribute fields are supported when requested
    /// * for Attribute with Enum values, readable values returned for fields that ends with:
    ///   * status
    ///   * type
    ///   * device
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
        /// Macro to get value of an attribute as string
        /// Before
        ///     "ad_group.name" => format!("{}", self.ad_group.as_ref().unwrap().name),
        /// With Macro
        ///     "ad_group.name" => attr_str!([ad_group], name),
        macro_rules! attr_str {
            ([$( $parent:ident ),+], $attr:ident) => {
                format!("{}", self.$($parent.as_ref().unwrap().)+$attr)
            };
        }

        /// Macro to get value of an attribute as string on OPTIONAL parent
        /// HACK: limited to single level; consider TT Muncher pattern in future
        /// Before
        ///     "campaign_budget.amount_micros" => {
        ///         if let Some(budget) = self.campaign_budget.as_ref() {
        ///             format!("{}", budget.amount_micros)
        ///         } else {
        ///             "".to_string()
        ///         }
        ///     },
        /// With Macro
        ///     "campaign_budget.amount_micros" => optional_attr_str!(campaign_budget, amount_micros),
        macro_rules! optional_attr_str {
            ($parent:ident, $attr:ident) => {
                if let Some(o) = self.$parent.as_ref() {
                    format!("{}", o.$attr)
                } else {
                    "".to_string()
                }
            };
        }

        /// Macro to get result of a method call as debug string
        /// Before
        ///     "ad_group.status" => format!("{:#?}", self.ad_group.as_ref().unwrap().status()),
        /// With Macro
        ///     "ad_group.status" => method_str!([ad_group], status),
        macro_rules! method_str {
            ([$( $parent:ident ),+], $func:ident) => {
                format!("{:#?}", self.$($parent.as_ref().unwrap().)+$func())
            };
        }

        /// Macro to get result of a method call as debug string on OPTIONAL parent
        /// HACK: limited to single level; consider TT Muncher pattern in future
        /// Before
        ///     "campaign_criterion.status" => {
        ///         if let Some(campaign_criterion) = self.campaign_criterion.as_ref() {
        ///             format!("{}", campaign_criterion.status())
        ///         } else {
        ///             "".to_string()
        ///         }
        ///     },
        /// With Macro
        ///     "campaign_criterion.status" => optional_method_str!(campaign_criterion, status),
        macro_rules! optional_method_str {
            ($parent:ident, $func:ident) => {
                if let Some(o) = self.$parent.as_ref() {
                    format!("{:#?}", o.$func())
                } else {
                    "".to_string()
                }
            };
        }

        /// Macro to get result of an Enum match as debug String
        /// Before:
        ///     "ad_group_criterion.keyword.text" => {
        ///         if let Some(criterion) = self.ad_group_criterion.as_ref().unwrap().criterion.as_ref() {
        ///             match criterion {
        ///                 Keyword(ki) => format!("{}", ki.text),
        ///                 _ => "".to_string()
        ///             }
        ///        } else {
        ///             "".to_string()
        ///         }
        ///     },
        /// With Macro:
        ///     "ad_group_criterion.keyword.text" => enum_match_str!([ad_group_criterion], criterion, Keyword, text),
        macro_rules! enum_match_str {
            ([$( $parent:ident ),+], $match_attr:ident, $enum_class:ident, $enum_attr:ident) => {
                if let Some(x) = self.$($parent.as_ref().unwrap().)+$match_attr.as_ref() {
                    match x {
                        $enum_class(o) => format!("{}", o.$enum_attr),
                        _ => "".to_string()
                    }
                } else {
                    "".to_string()
                }
            };
        }

        /// useful for accessing AdData
        macro_rules! enum_match_iterator_str {
            ([$( $parent:ident ),+], $match_attr:ident, $enum_class:ident, $enum_iterator:ident, $enum_attr:ident) => {
                if let Some(x) = self.$($parent.as_ref().unwrap().)+$match_attr.as_ref() {
                    match x {
                        $enum_class(o) => {
                            o.$enum_iterator
                                .iter()
                                .map(|item| format!("{}", item.$enum_attr))
                                .collect::<Vec<String>>()
                                .join(", ")
                        },
                        _ => "".to_string()
                    }
                } else {
                    "".to_string()
                }
            };
        }

        /// Macro to get result of an Enum match on OPTIONAL parent
        /// HACK: limited to single level; consider TT Muncher pattern in future
        /// Before:
        ///     "campaign_criterion.location.geo_target_constant" => {
        ///         if let Some(campaign_criterion) = self.campaign_criterion.as_ref() {
        ///             if let Some(criterion) = campaign_criterion.criterion.as_ref() {
        ///                 match criterion {
        ///                     Location(li) => format!("{}", li.geo_target_constant),
        ///                     _ => "".to_string()
        ///                 }
        ///             } else {
        ///                 "".to_string()
        ///             }
        ///         } else {
        ///             "".to_string()
        ///         }
        ///     },
        /// With Macro:
        ///     "campaign_criterion.location.geo_target_constant" => optional_enum_match_str!([campaign_criterion], criterion, Location, geo_target_constant),
        macro_rules! optional_enum_match_str {
            ($parent:ident, $match_attr:ident, $enum_class:ident, $enum_attr:ident) => {
                if let Some(p) = self.$parent.as_ref() {
                    if let Some(a) = p.$match_attr.as_ref() {
                        match a {
                            $enum_class(o) => format!("{}", o.$enum_attr),
                            _ => "".to_string(),
                        }
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                }
            };
        }

        /// Macro to get repeated enum field as comma-separated debug string
        /// Before:
        ///     "campaign.primary_status_reasons" => {
        ///         self.campaign.as_ref().unwrap()
        ///             .primary_status_reasons
        ///             .iter()
        ///             .map(|&v| format!("{:#?}",
        ///                 CampaignPrimaryStatusReason::from_i32(v).unwrap_or_default()))
        ///             .collect::<Vec<String>>()
        ///             .join(", ")
        ///     }
        /// With Macro:
        ///     "campaign.primary_status_reasons" => repeated_enum_str!([campaign], primary_status_reasons, CampaignPrimaryStatusReason),
        macro_rules! repeated_enum_str {
            ([$( $parent:ident ),+], $attr:ident, $enum_type:ty) => {
                self.$($parent.as_ref().unwrap().)+$attr
                    .iter()
                    .map(|&v| format!("{:#?}",
                        <$enum_type>::from_i32(v).unwrap_or_default()))
                    .collect::<Vec<String>>()
                    .join(", ")
            };
        }

        /// Macro to format repeated nested message fields as comma-separated strings
        /// Before:
        ///     "ad_group_asset.primary_status_details" => {
        ///         self.ad_group_asset.as_ref().unwrap()
        ///             .primary_status_details
        ///             .iter()
        ///             .map(|item| format!("{:#?}", item))
        ///             .collect::<Vec<String>>()
        ///             .join("; ")
        ///     }
        /// With Macro:
        ///     "ad_group_asset.primary_status_details" => repeated_message_str!([ad_group_asset], primary_status_details),
        macro_rules! repeated_message_str {
            ([$( $parent:ident ),+], $attr:ident) => {
                self.$($parent.as_ref().unwrap().)+$attr
                    .iter()
                    .map(|item| format!("{:#?}", item))
                    .collect::<Vec<String>>()
                    .join("; ")
            };
        }

        match field_name {
            "account_budget.id" => attr_str!([account_budget], id),
            "account_budget.name" => attr_str!([account_budget], name),
            "account_budget.status" => method_str!([account_budget], status),
            "ad_group.cpc_bid_micros" => attr_str!([ad_group], cpc_bid_micros),
            "ad_group.cpm_bid_micros" => attr_str!([ad_group], cpm_bid_micros),
            "ad_group.cpv_bid_micros" => attr_str!([ad_group], cpv_bid_micros),
            "ad_group.effective_cpc_bid_micros" => attr_str!([ad_group], effective_cpc_bid_micros),
            "ad_group.effective_target_cpa_micros" => attr_str!([ad_group], effective_target_cpa_micros),
            "ad_group.effective_target_cpa_source" => attr_str!([ad_group], effective_target_cpa_source),
            "ad_group.effective_target_roas" => attr_str!([ad_group], effective_target_roas),
            "ad_group.effective_target_roas_source" => attr_str!([ad_group], effective_target_roas_source),
            "ad_group.id" => attr_str!([ad_group], id),
            "ad_group.name" => attr_str!([ad_group], name),
            "ad_group.status" => method_str!([ad_group], status),
            "ad_group.target_cpa_micros" => attr_str!([ad_group], target_cpa_micros),
            "ad_group.target_cpm_micros" => attr_str!([ad_group], target_cpm_micros),
            "ad_group.target_roas" => attr_str!([ad_group], target_roas),
            "ad_group.type" => method_str!([ad_group], r#type),
            "ad_group.primary_status" => method_str!([ad_group], primary_status),
            "ad_group.primary_status_reasons" => repeated_enum_str!([ad_group], primary_status_reasons, AdGroupPrimaryStatusReason),
            "ad_group.labels" => self.ad_group.as_ref().unwrap().labels.join(", "),
            "ad_group_ad.ad.id" => attr_str!([ad_group_ad, ad], id),
            "ad_group_ad.ad.name" => attr_str!([ad_group_ad, ad], name),
            "ad_group_ad.ad.responsive_search_ad.headlines" => enum_match_iterator_str!([ad_group_ad, ad], ad_data, ResponsiveSearchAd, headlines, text),
            "ad_group_ad.ad.responsive_search_ad.descriptions" => enum_match_iterator_str!([ad_group_ad, ad], ad_data, ResponsiveSearchAd, descriptions, text),
            "ad_group_ad.ad.responsive_search_ad.path1" => enum_match_str!([ad_group_ad, ad], ad_data, ResponsiveSearchAd, path1),
            "ad_group_ad.ad.responsive_search_ad.path2" => enum_match_str!([ad_group_ad, ad], ad_data, ResponsiveSearchAd, path2),
            "ad_group_ad.ad.type" => method_str!([ad_group_ad, ad], r#type),
            "ad_group_ad.status" => method_str!([ad_group_ad], status),
            "ad_group_ad.primary_status" => method_str!([ad_group_ad], primary_status),
            "ad_group_ad.primary_status_reasons" => repeated_enum_str!([ad_group_ad], primary_status_reasons, AdGroupAdPrimaryStatusReason),
            "ad_group_ad.labels" => self.ad_group_ad.as_ref().unwrap().labels.join(", "),
            "ad_group_ad_asset_view.resource_name" => attr_str!([ad_group_ad_asset_view], resource_name),
            "ad_group_ad_asset_view.asset" => attr_str!([ad_group_ad_asset_view], asset),
            "ad_group_ad_asset_view.field_type" => method_str!([ad_group_ad_asset_view], field_type),
            "ad_group_ad_asset_view.pinned_field" => method_str!([ad_group_ad_asset_view], pinned_field),
            "ad_group_ad_asset_view.performance_label" => method_str!([ad_group_ad_asset_view], performance_label),
            "ad_group_criterion.bid_modifier" => attr_str!([ad_group_criterion], bid_modifier),
            "ad_group_criterion.cpc_bid_micros" => attr_str!([ad_group_criterion], cpc_bid_micros),
            "ad_group_criterion.cpm_bid_micros" => attr_str!([ad_group_criterion], cpm_bid_micros),
            "ad_group_criterion.cpv_bid_micros" => attr_str!([ad_group_criterion], cpv_bid_micros),
            "ad_group_criterion.criterion_id" => attr_str!([ad_group_criterion], criterion_id),
            "ad_group_criterion.effective_cpc_bid_micros" => attr_str!([ad_group_criterion], effective_cpc_bid_micros),
            "ad_group_criterion.effective_cpc_bid_source" => attr_str!([ad_group_criterion], effective_cpc_bid_source),
            "ad_group_criterion.effective_cpm_bid_micros" => attr_str!([ad_group_criterion], effective_cpm_bid_micros),
            "ad_group_criterion.effective_cpm_bid_source" => attr_str!([ad_group_criterion], effective_cpm_bid_source),
            "ad_group_criterion.effective_cpv_bid_micros" => attr_str!([ad_group_criterion], effective_cpv_bid_micros),
            "ad_group_criterion.effective_cpv_bid_source" => attr_str!([ad_group_criterion], effective_cpv_bid_source),
            "ad_group_criterion.effective_percent_cpc_bid_micros" => attr_str!([ad_group_criterion], effective_percent_cpc_bid_micros),
            "ad_group_criterion.effective_percent_cpc_bid_source" => attr_str!([ad_group_criterion], effective_percent_cpc_bid_source),
            "ad_group_criterion.keyword.text" => enum_match_str!([ad_group_criterion], criterion, Keyword, text),
            "ad_group_criterion.keyword.match_type" => enum_match_str!([ad_group_criterion], criterion, Keyword, match_type),
            "ad_group_criterion.status" => method_str!([ad_group_criterion], status),
            "ad_group_criterion.type" => method_str!([ad_group_criterion], r#type),
            "ad_group_criterion.primary_status" => method_str!([ad_group_criterion], primary_status),
            "ad_group_criterion.primary_status_reasons" => repeated_enum_str!([ad_group_criterion], primary_status_reasons, AdGroupCriterionPrimaryStatusReason),
            "ad_group_criterion.labels" => self.ad_group_criterion.as_ref().unwrap().labels.join(", "),
            // ===== AD_GROUP_ASSET =====
            "ad_group_asset.resource_name" => attr_str!([ad_group_asset], resource_name),
            "ad_group_asset.ad_group" => attr_str!([ad_group_asset], ad_group),
            "ad_group_asset.asset" => attr_str!([ad_group_asset], asset),
            "ad_group_asset.field_type" => method_str!([ad_group_asset], field_type),
            "ad_group_asset.status" => method_str!([ad_group_asset], status),
            "ad_group_asset.source" => method_str!([ad_group_asset], source),
            "ad_group_asset.primary_status" => method_str!([ad_group_asset], primary_status),
            "ad_group_asset.primary_status_reasons" => repeated_enum_str!([ad_group_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
            "ad_group_asset.primary_status_details" => repeated_message_str!([ad_group_asset], primary_status_details),
            // ===== ASSET =====
            "asset.id" => attr_str!([asset], id),
            "asset.name" => attr_str!([asset], name),
            "asset.type" => method_str!([asset], r#type),
            "asset.resource_name" => attr_str!([asset], resource_name),
            "asset.final_url_suffix" => attr_str!([asset], final_url_suffix),
            "asset.tracking_url_template" => attr_str!([asset], tracking_url_template),
            "asset.source" => method_str!([asset], source),
            "asset.policy_summary.approval_status" => method_str!([asset, policy_summary], approval_status),
            "asset.policy_summary.review_status" => method_str!([asset, policy_summary], review_status),
            "asset_field_type_view.field_type" => method_str!([asset_field_type_view], field_type),
            "asset_group.id" => attr_str!([asset_group], id),
            "asset_group.name" => attr_str!([asset_group], name),
            "asset_group.status" => method_str!([asset_group], status),
            "asset_group.resource_name" => attr_str!([asset_group], resource_name),
            "asset_group.campaign" => attr_str!([asset_group], campaign),
            "asset_group.ad_strength" => attr_str!([asset_group], ad_strength),
            "asset_group.primary_status" => method_str!([asset_group], primary_status),
            "asset_group.primary_status_reasons" => repeated_enum_str!([asset_group], primary_status_reasons, AssetGroupPrimaryStatusReason),
            "audience.description" => attr_str!([audience], description),
            "audience.id" => attr_str!([audience], id),
            "audience.name" => attr_str!([audience], name),
            "audience.status" => method_str!([audience], status),
            "bidding_strategy.id" => attr_str!([bidding_strategy], id),
            "bidding_strategy.name" => attr_str!([bidding_strategy], name),
            "bidding_strategy.status" => method_str!([bidding_strategy], status),
            "campaign.advertising_channel_type" => method_str!([campaign], advertising_channel_type),
            "campaign.advertising_channel_sub_type" => method_str!([campaign], advertising_channel_sub_type),
            "campaign.base_campaign" => attr_str!([campaign], base_campaign),
            "campaign.bidding_strategy_type" => {
                let strategy = self.campaign.as_ref().unwrap().bidding_strategy_type();
                match strategy {
                    ManualCpc => "ManualCPC".to_string(),
                    MaximizeConversions => "MaximizeConverions".to_string(),
                    MaximizeConversionValue => "MaximizeConversionValue".to_string(),
                    TargetCpa => "TargetCPA".to_string(),
                    TargetRoas => "TargetROAS".to_string(),
                    TargetImpressionShare => "TargetImpShare".to_string(),
                    _ => "Unsupported".to_string()
                }

            },
            "campaign.primary_status" => method_str!([campaign], primary_status),
            "campaign.primary_status_reasons" => repeated_enum_str!([campaign], primary_status_reasons, CampaignPrimaryStatusReason),
            "campaign_criterion.campaign" => optional_attr_str!(campaign_criterion, campaign),
            "campaign_criterion.criterion_id" => optional_attr_str!(campaign_criterion, criterion_id),
            "campaign_criterion.display_name" => optional_attr_str!(campaign_criterion, display_name),
            "campaign_criterion.keyword.text" => optional_enum_match_str!(campaign_criterion, criterion, CampaignKeyword, text), 
            "campaign_criterion.status" => optional_method_str!(campaign_criterion, status),
            "campaign_criterion.type" => optional_method_str!(campaign_criterion, r#type),
            "campaign_criterion.location.geo_target_constant" => optional_enum_match_str!(campaign_criterion, criterion, Location, geo_target_constant),
            "campaign.campaign_budget" => attr_str!([campaign], campaign_budget),
            "campaign.dynamic_search_ads_setting.domain_name" => attr_str!([campaign, dynamic_search_ads_setting], domain_name),
            "campaign.dynamic_search_ads_setting.language_code" => attr_str!([campaign, dynamic_search_ads_setting], language_code),
            "campaign.dynamic_search_ads_setting.use_supplied_urls_only" => attr_str!([campaign, dynamic_search_ads_setting], use_supplied_urls_only),
            "campaign.end_date" => attr_str!([campaign], end_date),
            "campaign.experiment_type" => method_str!([campaign], experiment_type),
            "campaign.id" => attr_str!([campaign], id),
            "campaign.name" => attr_str!([campaign], name),
            "campaign.network_settings.target_content_network" => attr_str!([campaign, network_settings], target_content_network),
            "campaign.network_settings.target_google_search" => attr_str!([campaign, network_settings], target_google_search),
            "campaign.network_settings.target_partner_search_network" => attr_str!([campaign, network_settings], target_partner_search_network),
            "campaign.network_settings.target_search_network" => attr_str!([campaign, network_settings], target_search_network),
            "campaign.optimization_score" => attr_str!([campaign], optimization_score),
            "campaign.performance_max_upgrade.performance_max_campaign" => attr_str!([campaign, performance_max_upgrade], performance_max_campaign),
            "campaign.performance_max_upgrade.pre_upgrade_campaign" => attr_str!([campaign, performance_max_upgrade], pre_upgrade_campaign),
            "campaign.performance_max_upgrade.status" => method_str!([campaign, performance_max_upgrade], status),
            "campaign.serving_status" => method_str!([campaign], serving_status),
            "campaign.status" => method_str!([campaign], status),
            "campaign.labels" => self.campaign.as_ref().unwrap().labels.join(", "),
            "campaign_budget.amount_micros" => optional_attr_str!(campaign_budget, amount_micros),
            // ===== CAMPAIGN_ASSET =====
            "campaign_asset.resource_name" => attr_str!([campaign_asset], resource_name),
            "campaign_asset.campaign" => attr_str!([campaign_asset], campaign),
            "campaign_asset.asset" => attr_str!([campaign_asset], asset),
            "campaign_asset.field_type" => method_str!([campaign_asset], field_type),
            "campaign_asset.status" => method_str!([campaign_asset], status),
            "campaign_asset.source" => method_str!([campaign_asset], source),
            "campaign_asset.primary_status" => method_str!([campaign_asset], primary_status),
            "campaign_asset.primary_status_reasons" => repeated_enum_str!([campaign_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
            "campaign_asset.primary_status_details" => repeated_message_str!([campaign_asset], primary_status_details),
            // ===== CONVERSION_ACTION =====
            "conversion_action.id" => attr_str!([conversion_action], id),
            "conversion_action.name" => attr_str!([conversion_action], name),
            "conversion_action.resource_name" => attr_str!([conversion_action], resource_name),
            "conversion_action.status" => method_str!([conversion_action], status),
            "conversion_action.type" => method_str!([conversion_action], r#type),
            "conversion_action.origin" => method_str!([conversion_action], origin),
            "conversion_action.category" => method_str!([conversion_action], category),
            "conversion_action.counting_type" => method_str!([conversion_action], counting_type),
            "conversion_action.include_in_conversions_metric" => attr_str!([conversion_action], include_in_conversions_metric),
            "conversion_action.value_settings.default_value" => attr_str!([conversion_action, value_settings], default_value),
            "conversion_action.value_settings.default_currency_code" => attr_str!([conversion_action, value_settings], default_currency_code),
            "conversion_action.value_settings.always_use_default_value" => attr_str!([conversion_action, value_settings], always_use_default_value),
            // ===== CONVERSION_CUSTOM_VARIABLE =====
            "conversion_custom_variable.id" => attr_str!([conversion_custom_variable], id),
            "conversion_custom_variable.name" => attr_str!([conversion_custom_variable], name),
            "conversion_custom_variable.resource_name" => attr_str!([conversion_custom_variable], resource_name),
            "conversion_custom_variable.tag" => attr_str!([conversion_custom_variable], tag),
            "conversion_custom_variable.status" => method_str!([conversion_custom_variable], status),
            "conversion_custom_variable.owner_customer" => attr_str!([conversion_custom_variable], owner_customer),
            "customer.id" => attr_str!([customer], id),
            "customer.descriptive_name" => attr_str!([customer], descriptive_name),
            "customer.status" => method_str!([customer], status),
            "customer.currency_code" => attr_str!([customer], currency_code),
            "customer.time_zone" => attr_str!([customer], time_zone),
            "customer_client.client_customer" => attr_str!([customer_client], client_customer),
            "customer_client.currency_code" => attr_str!([customer_client], currency_code),
            "customer_client.descriptive_name" => attr_str!([customer_client], descriptive_name),
            "customer_client.id" => attr_str!([customer_client], id),
            "customer_client.level" => attr_str!([customer_client], level),
            "customer_client.manager" => attr_str!([customer_client], manager),
            "customer_client.status" => method_str!([customer_client], status),
            "customer_client.time_zone" => attr_str!([customer_client], time_zone),
            // ===== CUSTOMER_ASSET =====
            "customer_asset.resource_name" => attr_str!([customer_asset], resource_name),
            "customer_asset.asset" => attr_str!([customer_asset], asset),
            "customer_asset.field_type" => method_str!([customer_asset], field_type),
            "customer_asset.status" => method_str!([customer_asset], status),
            "customer_asset.source" => method_str!([customer_asset], source),
            "customer_asset.primary_status" => method_str!([customer_asset], primary_status),
            "customer_asset.primary_status_reasons" => repeated_enum_str!([customer_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
            "customer_asset.primary_status_details" => repeated_message_str!([customer_asset], primary_status_details),
            // ===== GEO_TARGET_CONSTANT =====
            "geo_target_constant.id" => attr_str!([geo_target_constant], id),
            "geo_target_constant.name" => attr_str!([geo_target_constant], name),
            "geo_target_constant.canonical_name" => attr_str!([geo_target_constant], canonical_name),
            "geo_target_constant.country_code" => attr_str!([geo_target_constant], country_code),
            "geo_target_constant.target_type" => attr_str!([geo_target_constant], target_type),
            "geo_target_constant.status" => method_str!([geo_target_constant], status),
            "geo_target_constant.resource_name" => attr_str!([geo_target_constant], resource_name),
            "label.id" => attr_str!([label], id),
            "label.name" => attr_str!([label], name),
            "label.status" => method_str!([label], status),
            "search_term_view.ad_group" => attr_str!([search_term_view], ad_group),
            "search_term_view.search_term" => attr_str!([search_term_view], search_term),
            "search_term_view.status" => method_str!([search_term_view], status),
            "smart_campaign_search_term_view.campaign" => attr_str!([smart_campaign_search_term_view], campaign),
            "smart_campaign_search_term_view.search_term" => attr_str!([smart_campaign_search_term_view], search_term),
            // ===== USER_LIST =====
            "user_list.id" => attr_str!([user_list], id),
            "user_list.name" => attr_str!([user_list], name),
            "user_list.description" => attr_str!([user_list], description),
            "user_list.resource_name" => attr_str!([user_list], resource_name),
            "user_list.membership_status" => method_str!([user_list], membership_status),
            "user_list.membership_life_span" => attr_str!([user_list], membership_life_span),
            "user_list.size_for_display" => attr_str!([user_list], size_for_display),
            "user_list.size_range_for_display" => method_str!([user_list], size_range_for_display),
            "user_list.size_for_search" => attr_str!([user_list], size_for_search),
            "user_list.size_range_for_search" => method_str!([user_list], size_range_for_search),
            "user_list.type" => method_str!([user_list], r#type),
            "user_list.match_rate_percentage" => attr_str!([user_list], match_rate_percentage),
            "user_list.read_only" => attr_str!([user_list], read_only),
            "user_list.eligible_for_search" => attr_str!([user_list], eligible_for_search),
            "user_list.eligible_for_display" => attr_str!([user_list], eligible_for_display),
            // change event
            "change_event.change_date_time" => attr_str!([change_event], change_date_time), 
            "change_event.change_resource_type" => method_str!([change_event], change_resource_type), 
            "change_event.change_resource_name" => attr_str!([change_event], change_resource_name), 
            "change_event.client_type" => method_str!([change_event], client_type), 
            "change_event.user_email" => attr_str!([change_event], user_email), 
            "change_event.resource_change_operation" => method_str!([change_event], resource_change_operation), 
            "change_event.changed_fields" => {
                // comma delmited list of paths
                format!("'{}'", self.change_event.as_ref().unwrap().changed_fields.as_ref().unwrap().paths.join(", "))
            },
            "change_event.campaign" => attr_str!([change_event], campaign), 
            // metrics
            "metrics.absolute_top_impression_percentage" => attr_str!([metrics], absolute_top_impression_percentage),
            "metrics.active_view_cpm" => attr_str!([metrics], active_view_cpm),
            "metrics.active_view_ctr" => attr_str!([metrics], active_view_ctr),
            "metrics.active_view_impressions" => attr_str!([metrics], active_view_impressions),
            "metrics.active_view_measurability" => attr_str!([metrics], active_view_measurability),
            "metrics.active_view_measurable_cost_micros" => attr_str!([metrics], active_view_measurable_cost_micros),
            "metrics.active_view_measurable_impressions" => attr_str!([metrics], active_view_measurable_impressions),
            "metrics.active_view_viewability" => attr_str!([metrics], active_view_viewability),
            "metrics.all_conversions" => attr_str!([metrics], all_conversions),
            "metrics.all_conversions_by_conversion_date" => attr_str!([metrics], all_conversions_by_conversion_date),
            "metrics.all_conversions_from_click_to_call" => attr_str!([metrics], all_conversions_from_click_to_call),
            "metrics.all_conversions_from_directions" => attr_str!([metrics], all_conversions_from_directions),
            "metrics.all_conversions_from_interactions_rate" => attr_str!([metrics], all_conversions_from_interactions_rate),
            "metrics.all_conversions_from_interactions_value_per_interaction" => attr_str!([metrics], all_conversions_from_interactions_value_per_interaction),
            "metrics.all_conversions_from_menu" => attr_str!([metrics], all_conversions_from_menu),
            "metrics.all_conversions_from_order" => attr_str!([metrics], all_conversions_from_order),
            "metrics.all_conversions_from_other_engagement" => attr_str!([metrics], all_conversions_from_other_engagement),
            "metrics.all_conversions_from_store_visit" => attr_str!([metrics], all_conversions_from_store_visit),
            "metrics.all_conversions_from_store_website" => attr_str!([metrics], all_conversions_from_store_website),
            "metrics.all_conversions_value" => attr_str!([metrics], all_conversions_value),
            "metrics.all_conversions_value_by_conversion_date" => attr_str!([metrics], all_conversions_value_by_conversion_date),
            "metrics.all_conversions_value_per_cost" => attr_str!([metrics], all_conversions_value_per_cost),
            "metrics.average_cost" => attr_str!([metrics], average_cost),
            "metrics.average_cpc" => attr_str!([metrics], average_cpc),
            "metrics.average_cpe" => attr_str!([metrics], average_cpe),
            "metrics.average_cpm" => attr_str!([metrics], average_cpm),
            "metrics.average_cpv" => attr_str!([metrics], average_cpv),
            "metrics.average_page_views" => attr_str!([metrics], average_page_views),
            "metrics.average_time_on_site" => attr_str!([metrics], average_time_on_site),
            "metrics.benchmark_average_max_cpc" => attr_str!([metrics], benchmark_average_max_cpc),
            "metrics.benchmark_ctr" => attr_str!([metrics], benchmark_ctr),
            "metrics.biddable_app_install_conversions" => attr_str!([metrics], biddable_app_install_conversions),
            "metrics.biddable_app_post_install_conversions" => attr_str!([metrics], biddable_app_post_install_conversions),
            "metrics.bounce_rate" => attr_str!([metrics], bounce_rate),
            "metrics.clicks" => attr_str!([metrics], clicks),
            "metrics.combined_clicks" => attr_str!([metrics], combined_clicks),
            "metrics.combined_clicks_per_query" => attr_str!([metrics], combined_clicks_per_query),
            "metrics.combined_queries" => attr_str!([metrics], combined_queries),
            "metrics.content_budget_lost_impression_share" => attr_str!([metrics], content_budget_lost_impression_share),
            "metrics.content_impression_share" => attr_str!([metrics], content_impression_share),
            "metrics.content_rank_lost_impression_share" => attr_str!([metrics], content_rank_lost_impression_share),
            "metrics.conversion_last_conversion_date" => attr_str!([metrics], conversion_last_conversion_date),
            "metrics.conversion_last_received_request_date_time" => attr_str!([metrics], conversion_last_received_request_date_time),
            "metrics.conversions" => attr_str!([metrics], conversions),
            "metrics.conversions_by_conversion_date" => attr_str!([metrics], conversions_by_conversion_date),
            "metrics.conversions_from_interactions_rate" => attr_str!([metrics], conversions_from_interactions_rate),
            "metrics.conversions_from_interactions_value_per_interaction" => attr_str!([metrics], conversions_from_interactions_value_per_interaction),
            "metrics.conversions_value" => attr_str!([metrics], conversions_value),
            "metrics.conversions_value_by_conversion_date" => attr_str!([metrics], conversions_value_by_conversion_date),
            "metrics.conversions_value_per_cost" => attr_str!([metrics], conversions_value_per_cost),
            "metrics.cost_micros" => attr_str!([metrics], cost_micros),
            "metrics.cost_per_all_conversions" => attr_str!([metrics], cost_per_all_conversions),
            "metrics.cost_per_conversion" => attr_str!([metrics], cost_per_conversion),
            "metrics.cost_per_current_model_attributed_conversion" => attr_str!([metrics], cost_per_current_model_attributed_conversion),
            "metrics.cross_device_conversions" => attr_str!([metrics], cross_device_conversions),
            "metrics.ctr" => attr_str!([metrics], ctr),
            "metrics.current_model_attributed_conversions" => attr_str!([metrics], current_model_attributed_conversions),
            "metrics.current_model_attributed_conversions_from_interactions_rate" => attr_str!([metrics], current_model_attributed_conversions_from_interactions_rate),
            "metrics.current_model_attributed_conversions_from_interactions_value_per_interaction" => attr_str!([metrics], current_model_attributed_conversions_from_interactions_value_per_interaction),
            "metrics.current_model_attributed_conversions_value" => attr_str!([metrics], current_model_attributed_conversions_value),
            "metrics.current_model_attributed_conversions_value_per_cost" => attr_str!([metrics], current_model_attributed_conversions_value_per_cost),
            "metrics.engagement_rate" => attr_str!([metrics], engagement_rate),
            "metrics.engagements" => attr_str!([metrics], engagements),
            "metrics.gmail_forwards" => attr_str!([metrics], gmail_forwards),
            "metrics.gmail_saves" => attr_str!([metrics], gmail_saves),
            "metrics.gmail_secondary_clicks" => attr_str!([metrics], gmail_secondary_clicks),
            "metrics.historical_creative_quality_score" => attr_str!([metrics], historical_creative_quality_score),
            "metrics.historical_landing_page_quality_score" => attr_str!([metrics], historical_landing_page_quality_score),
            "metrics.historical_quality_score" => attr_str!([metrics], historical_quality_score),
            "metrics.historical_search_predicted_ctr" => attr_str!([metrics], historical_search_predicted_ctr),
            "metrics.hotel_average_lead_value_micros" => attr_str!([metrics], hotel_average_lead_value_micros),
            "metrics.hotel_commission_rate_micros" => attr_str!([metrics], hotel_commission_rate_micros),
            "metrics.hotel_eligible_impressions" => attr_str!([metrics], hotel_eligible_impressions),
            "metrics.hotel_expected_commission_cost" => attr_str!([metrics], hotel_expected_commission_cost),
            "metrics.hotel_price_difference_percentage" => attr_str!([metrics], hotel_price_difference_percentage),
            "metrics.impressions" => attr_str!([metrics], impressions),
            "metrics.impressions_from_store_reach" => attr_str!([metrics], impressions_from_store_reach),
            "metrics.interaction_rate" => attr_str!([metrics], interaction_rate),
            "metrics.interactions" => attr_str!([metrics], interactions),
            "metrics.invalid_click_rate" => attr_str!([metrics], invalid_click_rate),
            "metrics.invalid_clicks" => attr_str!([metrics], invalid_clicks),
            "metrics.message_chat_rate" => attr_str!([metrics], message_chat_rate),
            "metrics.message_chats" => attr_str!([metrics], message_chats),
            "metrics.message_impressions" => attr_str!([metrics], message_impressions),
            "metrics.mobile_friendly_clicks_percentage" => attr_str!([metrics], mobile_friendly_clicks_percentage),
            "metrics.optimization_score_uplift" => attr_str!([metrics], optimization_score_uplift),
            "metrics.optimization_score_url" => attr_str!([metrics], optimization_score_url),
            "metrics.organic_clicks" => attr_str!([metrics], organic_clicks),
            "metrics.organic_clicks_per_query" => attr_str!([metrics], organic_clicks_per_query),
            "metrics.organic_impressions" => attr_str!([metrics], organic_impressions),
            "metrics.organic_impressions_per_query" => attr_str!([metrics], organic_impressions_per_query),
            "metrics.organic_queries" => attr_str!([metrics], organic_queries),
            "metrics.percent_new_visitors" => attr_str!([metrics], percent_new_visitors),
            "metrics.phone_calls" => attr_str!([metrics], phone_calls),
            "metrics.phone_impressions" => attr_str!([metrics], phone_impressions),
            "metrics.phone_through_rate" => attr_str!([metrics], phone_through_rate),
            "metrics.relative_ctr" => attr_str!([metrics], relative_ctr),
            "metrics.search_absolute_top_impression_share" => attr_str!([metrics], search_absolute_top_impression_share),
            "metrics.search_budget_lost_absolute_top_impression_share" => attr_str!([metrics], search_budget_lost_absolute_top_impression_share),
            "metrics.search_budget_lost_impression_share" => attr_str!([metrics], search_budget_lost_impression_share),
            "metrics.search_budget_lost_top_impression_share" => attr_str!([metrics], search_budget_lost_top_impression_share),
            "metrics.search_click_share" => attr_str!([metrics], search_click_share),
            "metrics.search_exact_match_impression_share" => attr_str!([metrics], search_exact_match_impression_share),
            "metrics.search_impression_share" => attr_str!([metrics], search_impression_share),
            "metrics.search_rank_lost_absolute_top_impression_share" => attr_str!([metrics], search_rank_lost_absolute_top_impression_share),
            "metrics.search_rank_lost_impression_share" => attr_str!([metrics], search_rank_lost_impression_share),
            "metrics.search_rank_lost_top_impression_share" => attr_str!([metrics], search_rank_lost_top_impression_share),
            "metrics.search_top_impression_share" => attr_str!([metrics], search_top_impression_share),
            "metrics.sk_ad_network_installs" => attr_str!([metrics], sk_ad_network_installs),
            "metrics.speed_score" => attr_str!([metrics], speed_score),
            "metrics.top_impression_percentage" => attr_str!([metrics], top_impression_percentage),
            "metrics.valid_accelerated_mobile_pages_clicks_percentage" => attr_str!([metrics], valid_accelerated_mobile_pages_clicks_percentage),
            "metrics.value_per_all_conversions" => attr_str!([metrics], value_per_all_conversions),
            "metrics.value_per_all_conversions_by_conversion_date" => attr_str!([metrics], value_per_all_conversions_by_conversion_date),
            "metrics.value_per_conversion" => attr_str!([metrics], value_per_conversion),
            "metrics.value_per_conversions_by_conversion_date" => attr_str!([metrics], value_per_conversions_by_conversion_date),
            "metrics.value_per_current_model_attributed_conversion" => attr_str!([metrics], value_per_current_model_attributed_conversion),
            "metrics.video_quartile_p100_rate" => attr_str!([metrics], video_quartile_p100_rate),
            "metrics.video_quartile_p25_rate" => attr_str!([metrics], video_quartile_p25_rate),
            "metrics.video_quartile_p50_rate" => attr_str!([metrics], video_quartile_p50_rate),
            "metrics.video_quartile_p75_rate" => attr_str!([metrics], video_quartile_p75_rate),
            "metrics.video_view_rate" => attr_str!([metrics], video_view_rate),
            "metrics.video_views" => attr_str!([metrics], video_views),
            "metrics.view_through_conversions" => attr_str!([metrics], view_through_conversions),
            // ===== E-COMMERCE METRICS (Phase 2) =====
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
            // ===== LOCATION ASSET METRICS (Phase 2) =====
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
            // ===== CUSTOMER ACQUISITION METRICS (Phase 2) =====
            "metrics.all_new_customer_lifetime_value" => attr_str!([metrics], all_new_customer_lifetime_value),
            "metrics.new_customer_lifetime_value" => attr_str!([metrics], new_customer_lifetime_value),
            "metrics.average_impression_frequency_per_user" => attr_str!([metrics], average_impression_frequency_per_user),
            "metrics.unique_users" => attr_str!([metrics], unique_users),
            // ===== PRODUCT SEGMENTS (Phase 3) =====
            "segments.product_category_level1" => attr_str!([segments], product_category_level1),
            "segments.product_category_level2" => attr_str!([segments], product_category_level2),
            "segments.product_category_level3" => attr_str!([segments], product_category_level3),
            "segments.product_category_level4" => attr_str!([segments], product_category_level4),
            "segments.product_category_level5" => attr_str!([segments], product_category_level5),
            "segments.product_type_l1" => attr_str!([segments], product_type_l1),
            "segments.product_type_l2" => attr_str!([segments], product_type_l2),
            "segments.product_type_l3" => attr_str!([segments], product_type_l3),
            "segments.product_type_l4" => attr_str!([segments], product_type_l4),
            "segments.product_type_l5" => attr_str!([segments], product_type_l5),
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
            // ===== GEO TARGET SEGMENTS (Phase 3) =====
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
            // ===== RESOURCE NAME SEGMENTS (Phase 3) =====
            "segments.campaign" => attr_str!([segments], campaign),
            "segments.ad_group" => attr_str!([segments], ad_group),
            "segments.asset_group" => attr_str!([segments], asset_group),
            "segments.ad_destination_type" => method_str!([segments], ad_destination_type),
            "segments.ad_network_type" => method_str!([segments], ad_network_type),
            "segments.click_type" => method_str!([segments], click_type),
            "segments.conversion_action" => attr_str!([segments], conversion_action),
            "segments.conversion_action_category" => method_str!([segments], conversion_action_category),
            "segments.conversion_action_name" => attr_str!([segments], conversion_action_name),
            "segments.date" => attr_str!([segments], date),
            "segments.day_of_week" => method_str!([segments], day_of_week),
            "segments.device" => method_str!([segments], device),
            "segments.hour" => attr_str!([segments], hour),
            "segments.month" => attr_str!([segments], month),
            "segments.month_of_year" => method_str!([segments], month_of_year),
            "segments.product_channel" => method_str!([segments], product_channel),
            "segments.product_channel_exclusivity" => method_str!([segments], product_channel_exclusivity),
            "segments.product_item_id" => attr_str!([segments], product_item_id),
            "segments.quarter" => attr_str!([segments], quarter),
            "segments.search_term_match_type" => method_str!([segments], search_term_match_type),
            "segments.slot" => method_str!([segments], slot),
            "segments.week" => attr_str!([segments], week),
            "segments.year" => attr_str!([segments], year),
            // ===== KEYWORD_VIEW (Phase 4) =====
            "keyword_view.resource_name" => attr_str!([keyword_view], resource_name),
            // ===== LANDING_PAGE_VIEW (Phase 4) =====
            "landing_page_view.resource_name" => attr_str!([landing_page_view], resource_name),
            "landing_page_view.unexpanded_final_url" => attr_str!([landing_page_view], unexpanded_final_url),
            // ===== GEOGRAPHIC_VIEW (Phase 4) =====
            "geographic_view.resource_name" => attr_str!([geographic_view], resource_name),
            "geographic_view.location_type" => method_str!([geographic_view], location_type),
            "geographic_view.country_criterion_id" => attr_str!([geographic_view], country_criterion_id),
            // ===== CLICK_VIEW (Phase 4) =====
            "click_view.resource_name" => attr_str!([click_view], resource_name),
            "click_view.gclid" => attr_str!([click_view], gclid),
            "click_view.area_of_interest.city" => attr_str!([click_view, area_of_interest], city),
            "click_view.area_of_interest.country" => attr_str!([click_view, area_of_interest], country),
            "click_view.location_of_presence.city" => attr_str!([click_view, location_of_presence], city),
            "click_view.location_of_presence.country" => attr_str!([click_view, location_of_presence], country),
            // ===== ASSET_GROUP_ASSET (Phase 5) =====
            "asset_group_asset.resource_name" => attr_str!([asset_group_asset], resource_name),
            "asset_group_asset.asset_group" => attr_str!([asset_group_asset], asset_group),
            "asset_group_asset.asset" => attr_str!([asset_group_asset], asset),
            "asset_group_asset.field_type" => method_str!([asset_group_asset], field_type),
            "asset_group_asset.status" => method_str!([asset_group_asset], status),
            "asset_group_asset.performance_label" => method_str!([asset_group_asset], performance_label),
            "asset_group_asset.primary_status" => method_str!([asset_group_asset], primary_status),
            "asset_group_asset.primary_status_reasons" => repeated_enum_str!([asset_group_asset], primary_status_reasons, AssetLinkPrimaryStatusReason),
            "asset_group_asset.primary_status_details" => repeated_message_str!([asset_group_asset], primary_status_details),
            // ===== ASSET_GROUP_SIGNAL (Phase 5) =====
            "asset_group_signal.resource_name" => attr_str!([asset_group_signal], resource_name),
            "asset_group_signal.asset_group" => attr_str!([asset_group_signal], asset_group),
            // ===== CAMPAIGN_LABEL (Phase 5) =====
            "campaign_label.resource_name" => attr_str!([campaign_label], resource_name),
            "campaign_label.campaign" => attr_str!([campaign_label], campaign),
            "campaign_label.label" => attr_str!([campaign_label], label),
            // ===== AD_GROUP_LABEL (Phase 5) =====
            "ad_group_label.resource_name" => attr_str!([ad_group_label], resource_name),
            "ad_group_label.ad_group" => attr_str!([ad_group_label], ad_group),
            "ad_group_label.label" => attr_str!([ad_group_label], label),
            // ===== AD_GROUP_AD_LABEL (Phase 5) =====
            "ad_group_ad_label.resource_name" => attr_str!([ad_group_ad_label], resource_name),
            "ad_group_ad_label.ad_group_ad" => attr_str!([ad_group_ad_label], ad_group_ad),
            "ad_group_ad_label.label" => attr_str!([ad_group_ad_label], label),
            // ===== RECOMMENDATION (Phase 6) =====
            "recommendation.resource_name" => attr_str!([recommendation], resource_name),
            "recommendation.type" => method_str!([recommendation], r#type),
            "recommendation.impact.base_metrics.clicks" => attr_str!([recommendation, impact, base_metrics], clicks),
            "recommendation.impact.base_metrics.impressions" => attr_str!([recommendation, impact, base_metrics], impressions),
            "recommendation.impact.base_metrics.cost_micros" => attr_str!([recommendation, impact, base_metrics], cost_micros),
            // ===== CAMPAIGN_SHARED_SET (Phase 6) =====
            "campaign_shared_set.resource_name" => attr_str!([campaign_shared_set], resource_name),
            "campaign_shared_set.campaign" => attr_str!([campaign_shared_set], campaign),
            "campaign_shared_set.shared_set" => attr_str!([campaign_shared_set], shared_set),
            "campaign_shared_set.status" => method_str!([campaign_shared_set], status),
            // ===== SHARED_SET (Phase 6) =====
            "shared_set.id" => attr_str!([shared_set], id),
            "shared_set.name" => attr_str!([shared_set], name),
            "shared_set.type" => method_str!([shared_set], r#type),
            "shared_set.status" => method_str!([shared_set], status),
            "shared_set.member_count" => attr_str!([shared_set], member_count),
            "shared_set.resource_name" => attr_str!([shared_set], resource_name),
            // ===== SHARED_CRITERION (Phase 6) =====
            "shared_criterion.resource_name" => attr_str!([shared_criterion], resource_name),
            "shared_criterion.shared_set" => attr_str!([shared_criterion], shared_set),
            "shared_criterion.criterion_id" => attr_str!([shared_criterion], criterion_id),
            "shared_criterion.type" => method_str!([shared_criterion], r#type),
            "shared_criterion.keyword.text" => enum_match_str!([shared_criterion], criterion, SharedKeyword, text),
            // ===== ASSET PERFORMANCE METRICS (Phase 7) =====
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
            // ===== ASSET PINNING METRICS (Phase 7) =====
            "metrics.asset_pinned_as_description_position_one_count" => attr_str!([metrics], asset_pinned_as_description_position_one_count),
            "metrics.asset_pinned_as_description_position_two_count" => attr_str!([metrics], asset_pinned_as_description_position_two_count),
            "metrics.asset_pinned_as_headline_position_one_count" => attr_str!([metrics], asset_pinned_as_headline_position_one_count),
            "metrics.asset_pinned_as_headline_position_two_count" => attr_str!([metrics], asset_pinned_as_headline_position_two_count),
            "metrics.asset_pinned_as_headline_position_three_count" => attr_str!([metrics], asset_pinned_as_headline_position_three_count),
            "metrics.asset_pinned_total_count" => attr_str!([metrics], asset_pinned_total_count),
            // ===== AUCTION INSIGHTS METRICS (Phase 7) =====
            "metrics.auction_insight_search_absolute_top_impression_percentage" => attr_str!([metrics], auction_insight_search_absolute_top_impression_percentage),
            "metrics.auction_insight_search_impression_share" => attr_str!([metrics], auction_insight_search_impression_share),
            "metrics.auction_insight_search_outranking_share" => attr_str!([metrics], auction_insight_search_outranking_share),
            "metrics.auction_insight_search_overlap_rate" => attr_str!([metrics], auction_insight_search_overlap_rate),
            "metrics.auction_insight_search_position_above_rate" => attr_str!([metrics], auction_insight_search_position_above_rate),
            "metrics.auction_insight_search_top_impression_percentage" => attr_str!([metrics], auction_insight_search_top_impression_percentage),
            // ===== REMAINING METRICS (Phase 7) =====
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
            // ===== HOTEL SEGMENTS (Phase 7) =====
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
            // ===== SKADNETWORK SEGMENTS (Phase 7) =====
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
            _ => "not implemented by googleads-rs".to_string()
        }
    }
}
