include!(concat!(env!("OUT_DIR"), "/protos.rs"));

use crate::google::ads::googleads::v11::resources::campaign::CampaignBiddingStrategy::{
    BiddingStrategy,
    ManualCpc,
    MaximizeConversions,
    MaximizeConversionValue,
    TargetCpa,
    TargetImpressionShare,
    TargetRoas,
};

use crate::google::ads::googleads::v11::resources::{
    ad_group_criterion::Criterion::Keyword,
    campaign_criterion::Criterion::Keyword as CampaignKeyword,
    campaign_criterion::Criterion::Location
};

impl google::ads::googleads::v11::services::GoogleAdsRow {
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
    ///  * status
    ///  * type
    ///  * device
    ///
    /// # Examples
    ///
    /// ```
    /// let field_mask = response.field_mask.unwrap();
    /// for row in response.results {
    ///     for path in &field_mask.paths {
    ///         print!("{}\t", row.get(&path));
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
                            _ => "".to_string()
                        }
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                }
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
            "ad_group_ad.ad.id" => attr_str!([ad_group_ad, ad], id),
            "ad_group_ad.ad.name" => attr_str!([ad_group_ad, ad], name),
            "ad_group_ad.ad.type" => method_str!([ad_group_ad, ad], r#type),
            "ad_group_ad.status" => method_str!([ad_group_ad], status),
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
            "asset_field_type_view.field_type" => method_str!([asset_field_type_view], field_type),
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
            "campaign.bidding_strategy" => {
                if let Some(strategy) = self.campaign.as_ref().unwrap().campaign_bidding_strategy.as_ref() {
                    match strategy {
                        BiddingStrategy(str) => format!("BiddingStrategy: {}", str),
                        ManualCpc(mcpc) => format!("ManualCPC: enhanced={}", mcpc.enhanced_cpc_enabled),
                        MaximizeConversions(mc) => format!("MaximizeConverions: cpa={:.2}", mc.target_cpa_micros/1000000),
                        MaximizeConversionValue(mcv) => format!("MaximizeConversionValue: roas={:.2}", mcv.target_roas),
                        TargetCpa(tcpa) => format!("TargetCPA: cpa={:.2}", tcpa.target_cpa_micros/1000000),
                        TargetRoas(troas) => format!("TargetROAS: roas={:.2}", troas.target_roas),
                        TargetImpressionShare(timp) => format!("TargetImpShare: loc={}, share={:.2}%", timp.location, timp.location_fraction_micros/10000),
                        _ => "Unsupported".to_string()
                    }
                } else {
                    "".to_string()
                }
            },
            "campaign.bidding_strategy_type" => method_str!([campaign], bidding_strategy_type),
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
            "campaign_budget.amount_micros" => optional_attr_str!(campaign_budget, amount_micros),
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
            "extension_feed_item.extension_type" => method_str!([extension_feed_item], extension_type),
            "feed_placeholder_view.placeholder_type" => method_str!([feed_placeholder_view], placeholder_type),
            "search_term_view.ad_group" => attr_str!([search_term_view], ad_group),
            "search_term_view.search_term" => attr_str!([search_term_view], search_term),
            "search_term_view.status" => method_str!([search_term_view], status),
            "smart_campaign_search_term_view.campaign" => attr_str!([smart_campaign_search_term_view], campaign),
            "smart_campaign_search_term_view.search_term" => attr_str!([smart_campaign_search_term_view], search_term),
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
            "metrics.sk_ad_network_conversions" => attr_str!([metrics], sk_ad_network_conversions),
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
            "segments.ad_network_type" => method_str!([segments], ad_network_type),
            "segments.click_type" => method_str!([segments], click_type),
            "segments.date" => attr_str!([segments], date),
            "segments.day_of_week" => attr_str!([segments], day_of_week),
            "segments.device" => method_str!([segments], device),
            "segments.hour" => attr_str!([segments], hour),
            "segments.month" => attr_str!([segments], month),
            "segments.month_of_year" => attr_str!([segments], month_of_year),
            "segments.product_channel" => attr_str!([segments], product_channel),
            "segments.product_item_id" => attr_str!([segments], product_item_id),
            "segments.search_term_match_type" => method_str!([segments], search_term_match_type),
            "segments.week" => attr_str!([segments], week),
            "segments.year" => attr_str!([segments], year),
            _ => "unsupported".to_string()
        }
    }
}
