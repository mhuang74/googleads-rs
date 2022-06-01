
pub mod google {
    pub mod ads {
        pub mod googleads {
            pub mod v10 {
                pub mod common {
                    tonic::include_proto!("google.ads.googleads.v10.common");
                }
                pub mod enums {
                    tonic::include_proto!("google.ads.googleads.v10.enums");
                }
                pub mod errors {
                    tonic::include_proto!("google.ads.googleads.v10.errors");
                }
                pub mod resources {
                    tonic::include_proto!("google.ads.googleads.v10.resources");
                }
                pub mod services {
                    tonic::include_proto!("google.ads.googleads.v10.services");
                }
            }
        }
    }
    pub mod longrunning {
        tonic::include_proto!("google.longrunning");
    }
    pub mod rpc {
        pub mod context {
            tonic::include_proto!("google.rpc.context");
        }
        tonic::include_proto!("google.rpc");
    }
}

impl google::ads::googleads::v10::services::GoogleAdsRow {
    /// Returns GoogleAdsRow field value by field name
    ///
    /// # Arguments
    /// * `field_name` - A string slice that holds name of a field in GoogleAdsRow struct
    ///
    /// Support fields:
    /// * all non-list Metrics fields are supported
    /// * all non-list Segment fields are supported
    /// * only basic Attribute fields are supported (eg. id, name, status)
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
        match field_name {
            "account_budget.id" => format!("{:?}", self.account_budget.as_ref().unwrap().id),
            "account_budget.name" => format!("{:?}", self.account_budget.as_ref().unwrap().name),
            "account_budget.status" => format!("{:?}", self.account_budget.as_ref().unwrap().status()),
            "ad_group.cpc_bid_micros" => format!("{:?}", self.ad_group.as_ref().unwrap().cpc_bid_micros),
            "ad_group.cpm_bid_micros" => format!("{:?}", self.ad_group.as_ref().unwrap().cpm_bid_micros),
            "ad_group.cpv_bid_micros" => format!("{:?}", self.ad_group.as_ref().unwrap().cpv_bid_micros),
            "ad_group.effective_cpc_bid_micros" => format!("{:?}", self.ad_group.as_ref().unwrap().effective_cpc_bid_micros),
            "ad_group.effective_target_cpa_micros" => format!("{:?}", self.ad_group.as_ref().unwrap().effective_target_cpa_micros),
            "ad_group.effective_target_cpa_source" => format!("{:?}", self.ad_group.as_ref().unwrap().effective_target_cpa_source),
            "ad_group.effective_target_roas" => format!("{:?}", self.ad_group.as_ref().unwrap().effective_target_roas),
            "ad_group.effective_target_roas_source" => format!("{:?}", self.ad_group.as_ref().unwrap().effective_target_roas_source),
            "ad_group.id" => format!("{:?}", self.ad_group.as_ref().unwrap().id),
            "ad_group.name" => format!("{:?}", self.ad_group.as_ref().unwrap().name),
            "ad_group.status" => format!("{:?}", self.ad_group.as_ref().unwrap().status()),
            "ad_group.target_cpa_micros" => format!("{:?}", self.ad_group.as_ref().unwrap().target_cpa_micros),
            "ad_group.target_cpm_micros" => format!("{:?}", self.ad_group.as_ref().unwrap().target_cpm_micros),
            "ad_group.target_roas" => format!("{:?}", self.ad_group.as_ref().unwrap().target_roas),
            "ad_group.type" => format!("{:?}", self.ad_group.as_ref().unwrap().r#type()),
            "ad_group_ad.ad.id" => format!("{:?}", self.ad_group_ad.as_ref().unwrap().ad.as_ref().unwrap().id),
            "ad_group_ad.ad.name" => format!("{:?}", self.ad_group_ad.as_ref().unwrap().ad.as_ref().unwrap().name),
            "ad_group_ad.ad.type" => format!("{:?}", self.ad_group_ad.as_ref().unwrap().ad.as_ref().unwrap().r#type()),
            "ad_group_ad.status" => format!("{:?}", self.ad_group_ad.as_ref().unwrap().status()),
            "ad_group_criterion.bid_modifier" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().bid_modifier),
            "ad_group_criterion.cpc_bid_micros" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().cpc_bid_micros),
            "ad_group_criterion.cpm_bid_micros" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().cpm_bid_micros),
            "ad_group_criterion.cpv_bid_micros" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().cpv_bid_micros),
            "ad_group_criterion.criterion_id" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().criterion_id),
            "ad_group_criterion.effective_cpc_bid_micros" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().effective_cpc_bid_micros),
            "ad_group_criterion.effective_cpc_bid_source" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().effective_cpc_bid_source),
            "ad_group_criterion.effective_cpm_bid_micros" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().effective_cpm_bid_micros),
            "ad_group_criterion.effective_cpm_bid_source" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().effective_cpm_bid_source),
            "ad_group_criterion.effective_cpv_bid_micros" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().effective_cpv_bid_micros),
            "ad_group_criterion.effective_cpv_bid_source" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().effective_cpv_bid_source),
            "ad_group_criterion.effective_percent_cpc_bid_micros" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().effective_percent_cpc_bid_micros),
            "ad_group_criterion.effective_percent_cpc_bid_source" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().effective_percent_cpc_bid_source),
            "ad_group_criterion.keyword.text" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().criterion.as_ref().unwrap()),
            "ad_group_criterion.keyword.match_type" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().criterion.as_ref().unwrap()),
            "ad_group_criterion.status" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().status()),
            "ad_group_criterion.type" => format!("{:?}", self.ad_group_criterion.as_ref().unwrap().r#type()),
            "asset_field_type_view.field_type" => format!("{:?}", self.asset_field_type_view.as_ref().unwrap().field_type()),
            "audience.description" => format!("{:?}", self.audience.as_ref().unwrap().description),
            "audience.id" => format!("{:?}", self.audience.as_ref().unwrap().id),
            "audience.name" => format!("{:?}", self.audience.as_ref().unwrap().name),
            "audience.status" => format!("{:?}", self.audience.as_ref().unwrap().status()),
            "bidding_strategy.id" => format!("{:?}", self.bidding_strategy.as_ref().unwrap().id),
            "bidding_strategy.name" => format!("{:?}", self.bidding_strategy.as_ref().unwrap().name),
            "bidding_strategy.status" => format!("{:?}", self.bidding_strategy.as_ref().unwrap().status()),
            "campaign.advertising_channel_type" => format!("{:?}", self.campaign.as_ref().unwrap().advertising_channel_type()),
            "campaign.advertising_channel_sub_type" => format!("{:?}", self.campaign.as_ref().unwrap().advertising_channel_sub_type()),
            "campaign.base_campaign" => format!("{:?}", self.campaign.as_ref().unwrap().base_campaign),
            "campaign.bidding_strategy" => format!("{:?}", self.campaign.as_ref().unwrap().campaign_bidding_strategy),
            "campaign.bidding_strategy_type" => format!("{:?}", self.campaign.as_ref().unwrap().bidding_strategy_type()),
            "campaign.campaign_budget" => format!("{:?}", self.campaign.as_ref().unwrap().campaign_budget),
            "campaign.dynamic_search_ads_setting.domain_name" => format!("{:?}", self.campaign.as_ref().unwrap().dynamic_search_ads_setting.as_ref().unwrap().domain_name),
            "campaign.dynamic_search_ads_setting.language_code" => format!("{:?}", self.campaign.as_ref().unwrap().dynamic_search_ads_setting.as_ref().unwrap().language_code),
            "campaign.dynamic_search_ads_setting.use_supplied_urls_only" => format!("{:?}", self.campaign.as_ref().unwrap().dynamic_search_ads_setting.as_ref().unwrap().use_supplied_urls_only),
            "campaign.end_date" => format!("{:?}", self.campaign.as_ref().unwrap().end_date),
            "campaign.experiment_type" => format!("{:?}", self.campaign.as_ref().unwrap().experiment_type()),
            "campaign.id" => format!("{:?}", self.campaign.as_ref().unwrap().id),
            "campaign.name" => format!("{:?}", self.campaign.as_ref().unwrap().name),
            "campaign.serving_status" => format!("{:?}", self.campaign.as_ref().unwrap().serving_status()),
            "campaign.status" => format!("{:?}", self.campaign.as_ref().unwrap().status()),
            "campaign_budget.amount_micros" => format!("{:?}", self.campaign_budget.as_ref().unwrap().amount_micros),
            "customer.id" => format!("{:?}", self.customer.as_ref().unwrap().id),
            "customer.descriptive_name" => format!("{:?}", self.customer.as_ref().unwrap().descriptive_name),
            "customer.status" => format!("{:?}", self.customer.as_ref().unwrap().status()),
            "customer.currency_code" => format!("{:?}", self.customer.as_ref().unwrap().currency_code),
            "customer.time_zone" => format!("{:?}", self.customer.as_ref().unwrap().time_zone),
            "customer_client.client_customer" => format!("{:?}", self.customer_client.as_ref().unwrap().client_customer),
            "customer_client.currency_code" => format!("{:?}", self.customer_client.as_ref().unwrap().currency_code),
            "customer_client.descriptive_name" => format!("{:?}", self.customer_client.as_ref().unwrap().descriptive_name),
            "customer_client.id" => format!("{:?}", self.customer_client.as_ref().unwrap().id),
            "customer_client.level" => format!("{:?}", self.customer_client.as_ref().unwrap().level),
            "customer_client.manager" => format!("{:?}", self.customer_client.as_ref().unwrap().manager),
            "customer_client.status" => format!("{:?}", self.customer_client.as_ref().unwrap().status()),
            "customer_client.time_zone" => format!("{:?}", self.customer_client.as_ref().unwrap().time_zone),
            "extension_feed_item.extension_type" => format!("{:?}", self.extension_feed_item.as_ref().unwrap().extension_type()),
            "feed_placeholder_view.placeholder_type" => format!("{:?}", self.feed_placeholder_view.as_ref().unwrap().placeholder_type()),
            "search_term_view.ad_group" => format!("{:?}", self.search_term_view.as_ref().unwrap().ad_group),
            "search_term_view.search_term" => format!("{:?}", self.search_term_view.as_ref().unwrap().search_term),
            "search_term_view.status" => format!("{:?}", self.search_term_view.as_ref().unwrap().status()),
            "metrics.absolute_top_impression_percentage" => format!("{:?}", self.metrics.as_ref().unwrap().absolute_top_impression_percentage),
            "metrics.active_view_cpm" => format!("{:?}", self.metrics.as_ref().unwrap().active_view_cpm),
            "metrics.active_view_ctr" => format!("{:?}", self.metrics.as_ref().unwrap().active_view_ctr),
            "metrics.active_view_impressions" => format!("{:?}", self.metrics.as_ref().unwrap().active_view_impressions),
            "metrics.active_view_measurability" => format!("{:?}", self.metrics.as_ref().unwrap().active_view_measurability),
            "metrics.active_view_measurable_cost_micros" => format!("{:?}", self.metrics.as_ref().unwrap().active_view_measurable_cost_micros),
            "metrics.active_view_measurable_impressions" => format!("{:?}", self.metrics.as_ref().unwrap().active_view_measurable_impressions),
            "metrics.active_view_viewability" => format!("{:?}", self.metrics.as_ref().unwrap().active_view_viewability),
            "metrics.all_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions),
            "metrics.all_conversions_by_conversion_date" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_by_conversion_date),
            "metrics.all_conversions_from_click_to_call" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_click_to_call),
            "metrics.all_conversions_from_directions" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_directions),
            "metrics.all_conversions_from_interactions_rate" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_interactions_rate),
            "metrics.all_conversions_from_interactions_value_per_interaction" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_interactions_value_per_interaction),
            "metrics.all_conversions_from_menu" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_menu),
            "metrics.all_conversions_from_order" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_order),
            "metrics.all_conversions_from_other_engagement" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_other_engagement),
            "metrics.all_conversions_from_store_visit" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_store_visit),
            "metrics.all_conversions_from_store_website" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_from_store_website),
            "metrics.all_conversions_value" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_value),
            "metrics.all_conversions_value_by_conversion_date" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_value_by_conversion_date),
            "metrics.all_conversions_value_per_cost" => format!("{:?}", self.metrics.as_ref().unwrap().all_conversions_value_per_cost),
            "metrics.average_cost" => format!("{:?}", self.metrics.as_ref().unwrap().average_cost),
            "metrics.average_cpc" => format!("{:?}", self.metrics.as_ref().unwrap().average_cpc),
            "metrics.average_cpe" => format!("{:?}", self.metrics.as_ref().unwrap().average_cpe),
            "metrics.average_cpm" => format!("{:?}", self.metrics.as_ref().unwrap().average_cpm),
            "metrics.average_cpv" => format!("{:?}", self.metrics.as_ref().unwrap().average_cpv),
            "metrics.average_page_views" => format!("{:?}", self.metrics.as_ref().unwrap().average_page_views),
            "metrics.average_time_on_site" => format!("{:?}", self.metrics.as_ref().unwrap().average_time_on_site),
            "metrics.benchmark_average_max_cpc" => format!("{:?}", self.metrics.as_ref().unwrap().benchmark_average_max_cpc),
            "metrics.benchmark_ctr" => format!("{:?}", self.metrics.as_ref().unwrap().benchmark_ctr),
            "metrics.biddable_app_install_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().biddable_app_install_conversions),
            "metrics.biddable_app_post_install_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().biddable_app_post_install_conversions),
            "metrics.bounce_rate" => format!("{:?}", self.metrics.as_ref().unwrap().bounce_rate),
            "metrics.clicks" => format!("{:?}", self.metrics.as_ref().unwrap().clicks),
            "metrics.combined_clicks" => format!("{:?}", self.metrics.as_ref().unwrap().combined_clicks),
            "metrics.combined_clicks_per_query" => format!("{:?}", self.metrics.as_ref().unwrap().combined_clicks_per_query),
            "metrics.combined_queries" => format!("{:?}", self.metrics.as_ref().unwrap().combined_queries),
            "metrics.content_budget_lost_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().content_budget_lost_impression_share),
            "metrics.content_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().content_impression_share),
            "metrics.content_rank_lost_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().content_rank_lost_impression_share),
            "metrics.conversion_last_conversion_date" => format!("{:?}", self.metrics.as_ref().unwrap().conversion_last_conversion_date),
            "metrics.conversion_last_received_request_date_time" => format!("{:?}", self.metrics.as_ref().unwrap().conversion_last_received_request_date_time),
            "metrics.conversions" => format!("{:?}", self.metrics.as_ref().unwrap().conversions),
            "metrics.conversions_by_conversion_date" => format!("{:?}", self.metrics.as_ref().unwrap().conversions_by_conversion_date),
            "metrics.conversions_from_interactions_rate" => format!("{:?}", self.metrics.as_ref().unwrap().conversions_from_interactions_rate),
            "metrics.conversions_from_interactions_value_per_interaction" => format!("{:?}", self.metrics.as_ref().unwrap().conversions_from_interactions_value_per_interaction),
            "metrics.conversions_value" => format!("{:?}", self.metrics.as_ref().unwrap().conversions_value),
            "metrics.conversions_value_by_conversion_date" => format!("{:?}", self.metrics.as_ref().unwrap().conversions_value_by_conversion_date),
            "metrics.conversions_value_per_cost" => format!("{:?}", self.metrics.as_ref().unwrap().conversions_value_per_cost),
            "metrics.cost_micros" => format!("{:?}", self.metrics.as_ref().unwrap().cost_micros),
            "metrics.cost_per_all_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().cost_per_all_conversions),
            "metrics.cost_per_conversion" => format!("{:?}", self.metrics.as_ref().unwrap().cost_per_conversion),
            "metrics.cost_per_current_model_attributed_conversion" => format!("{:?}", self.metrics.as_ref().unwrap().cost_per_current_model_attributed_conversion),
            "metrics.cross_device_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().cross_device_conversions),
            "metrics.ctr" => format!("{:?}", self.metrics.as_ref().unwrap().ctr),
            "metrics.current_model_attributed_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().current_model_attributed_conversions),
            "metrics.current_model_attributed_conversions_from_interactions_rate" => format!("{:?}", self.metrics.as_ref().unwrap().current_model_attributed_conversions_from_interactions_rate),
            "metrics.current_model_attributed_conversions_from_interactions_value_per_interaction" => format!("{:?}", self.metrics.as_ref().unwrap().current_model_attributed_conversions_from_interactions_value_per_interaction),
            "metrics.current_model_attributed_conversions_value" => format!("{:?}", self.metrics.as_ref().unwrap().current_model_attributed_conversions_value),
            "metrics.current_model_attributed_conversions_value_per_cost" => format!("{:?}", self.metrics.as_ref().unwrap().current_model_attributed_conversions_value_per_cost),
            "metrics.engagement_rate" => format!("{:?}", self.metrics.as_ref().unwrap().engagement_rate),
            "metrics.engagements" => format!("{:?}", self.metrics.as_ref().unwrap().engagements),
            "metrics.gmail_forwards" => format!("{:?}", self.metrics.as_ref().unwrap().gmail_forwards),
            "metrics.gmail_saves" => format!("{:?}", self.metrics.as_ref().unwrap().gmail_saves),
            "metrics.gmail_secondary_clicks" => format!("{:?}", self.metrics.as_ref().unwrap().gmail_secondary_clicks),
            "metrics.historical_creative_quality_score" => format!("{:?}", self.metrics.as_ref().unwrap().historical_creative_quality_score),
            "metrics.historical_landing_page_quality_score" => format!("{:?}", self.metrics.as_ref().unwrap().historical_landing_page_quality_score),
            "metrics.historical_quality_score" => format!("{:?}", self.metrics.as_ref().unwrap().historical_quality_score),
            "metrics.historical_search_predicted_ctr" => format!("{:?}", self.metrics.as_ref().unwrap().historical_search_predicted_ctr),
            "metrics.hotel_average_lead_value_micros" => format!("{:?}", self.metrics.as_ref().unwrap().hotel_average_lead_value_micros),
            "metrics.hotel_commission_rate_micros" => format!("{:?}", self.metrics.as_ref().unwrap().hotel_commission_rate_micros),
            "metrics.hotel_eligible_impressions" => format!("{:?}", self.metrics.as_ref().unwrap().hotel_eligible_impressions),
            "metrics.hotel_expected_commission_cost" => format!("{:?}", self.metrics.as_ref().unwrap().hotel_expected_commission_cost),
            "metrics.hotel_price_difference_percentage" => format!("{:?}", self.metrics.as_ref().unwrap().hotel_price_difference_percentage),
            "metrics.impressions" => format!("{:?}", self.metrics.as_ref().unwrap().impressions),
            "metrics.impressions_from_store_reach" => format!("{:?}", self.metrics.as_ref().unwrap().impressions_from_store_reach),
            "metrics.interaction_rate" => format!("{:?}", self.metrics.as_ref().unwrap().interaction_rate),
            "metrics.interactions" => format!("{:?}", self.metrics.as_ref().unwrap().interactions),
            "metrics.invalid_click_rate" => format!("{:?}", self.metrics.as_ref().unwrap().invalid_click_rate),
            "metrics.invalid_clicks" => format!("{:?}", self.metrics.as_ref().unwrap().invalid_clicks),
            "metrics.message_chat_rate" => format!("{:?}", self.metrics.as_ref().unwrap().message_chat_rate),
            "metrics.message_chats" => format!("{:?}", self.metrics.as_ref().unwrap().message_chats),
            "metrics.message_impressions" => format!("{:?}", self.metrics.as_ref().unwrap().message_impressions),
            "metrics.mobile_friendly_clicks_percentage" => format!("{:?}", self.metrics.as_ref().unwrap().mobile_friendly_clicks_percentage),
            "metrics.optimization_score_uplift" => format!("{:?}", self.metrics.as_ref().unwrap().optimization_score_uplift),
            "metrics.optimization_score_url" => format!("{:?}", self.metrics.as_ref().unwrap().optimization_score_url),
            "metrics.organic_clicks" => format!("{:?}", self.metrics.as_ref().unwrap().organic_clicks),
            "metrics.organic_clicks_per_query" => format!("{:?}", self.metrics.as_ref().unwrap().organic_clicks_per_query),
            "metrics.organic_impressions" => format!("{:?}", self.metrics.as_ref().unwrap().organic_impressions),
            "metrics.organic_impressions_per_query" => format!("{:?}", self.metrics.as_ref().unwrap().organic_impressions_per_query),
            "metrics.organic_queries" => format!("{:?}", self.metrics.as_ref().unwrap().organic_queries),
            "metrics.percent_new_visitors" => format!("{:?}", self.metrics.as_ref().unwrap().percent_new_visitors),
            "metrics.phone_calls" => format!("{:?}", self.metrics.as_ref().unwrap().phone_calls),
            "metrics.phone_impressions" => format!("{:?}", self.metrics.as_ref().unwrap().phone_impressions),
            "metrics.phone_through_rate" => format!("{:?}", self.metrics.as_ref().unwrap().phone_through_rate),
            "metrics.relative_ctr" => format!("{:?}", self.metrics.as_ref().unwrap().relative_ctr),
            "metrics.search_absolute_top_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_absolute_top_impression_share),
            "metrics.search_budget_lost_absolute_top_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_budget_lost_absolute_top_impression_share),
            "metrics.search_budget_lost_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_budget_lost_impression_share),
            "metrics.search_budget_lost_top_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_budget_lost_top_impression_share),
            "metrics.search_click_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_click_share),
            "metrics.search_exact_match_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_exact_match_impression_share),
            "metrics.search_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_impression_share),
            "metrics.search_rank_lost_absolute_top_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_rank_lost_absolute_top_impression_share),
            "metrics.search_rank_lost_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_rank_lost_impression_share),
            "metrics.search_rank_lost_top_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_rank_lost_top_impression_share),
            "metrics.search_top_impression_share" => format!("{:?}", self.metrics.as_ref().unwrap().search_top_impression_share),
            "metrics.sk_ad_network_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().sk_ad_network_conversions),
            "metrics.speed_score" => format!("{:?}", self.metrics.as_ref().unwrap().speed_score),
            "metrics.top_impression_percentage" => format!("{:?}", self.metrics.as_ref().unwrap().top_impression_percentage),
            "metrics.valid_accelerated_mobile_pages_clicks_percentage" => format!("{:?}", self.metrics.as_ref().unwrap().valid_accelerated_mobile_pages_clicks_percentage),
            "metrics.value_per_all_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().value_per_all_conversions),
            "metrics.value_per_all_conversions_by_conversion_date" => format!("{:?}", self.metrics.as_ref().unwrap().value_per_all_conversions_by_conversion_date),
            "metrics.value_per_conversion" => format!("{:?}", self.metrics.as_ref().unwrap().value_per_conversion),
            "metrics.value_per_conversions_by_conversion_date" => format!("{:?}", self.metrics.as_ref().unwrap().value_per_conversions_by_conversion_date),
            "metrics.value_per_current_model_attributed_conversion" => format!("{:?}", self.metrics.as_ref().unwrap().value_per_current_model_attributed_conversion),
            "metrics.video_quartile_p100_rate" => format!("{:?}", self.metrics.as_ref().unwrap().video_quartile_p100_rate),
            "metrics.video_quartile_p25_rate" => format!("{:?}", self.metrics.as_ref().unwrap().video_quartile_p25_rate),
            "metrics.video_quartile_p50_rate" => format!("{:?}", self.metrics.as_ref().unwrap().video_quartile_p50_rate),
            "metrics.video_quartile_p75_rate" => format!("{:?}", self.metrics.as_ref().unwrap().video_quartile_p75_rate),
            "metrics.video_view_rate" => format!("{:?}", self.metrics.as_ref().unwrap().video_view_rate),
            "metrics.video_views" => format!("{:?}", self.metrics.as_ref().unwrap().video_views),
            "metrics.view_through_conversions" => format!("{:?}", self.metrics.as_ref().unwrap().view_through_conversions),
            "segments.ad_network_type" => format!("{:?}", self.segments.as_ref().unwrap().ad_network_type()),
            "segments.click_type" => format!("{:?}", self.segments.as_ref().unwrap().click_type()),
            "segments.date" => format!("{:?}", self.segments.as_ref().unwrap().date),
            "segments.day_of_week" => format!("{:?}", self.segments.as_ref().unwrap().day_of_week),
            "segments.device" => format!("{:?}", self.segments.as_ref().unwrap().device()),
            "segments.hour" => format!("{:?}", self.segments.as_ref().unwrap().hour),
            "segments.month" => format!("{:?}", self.segments.as_ref().unwrap().month),
            "segments.month_of_year" => format!("{:?}", self.segments.as_ref().unwrap().month_of_year),
            "segments.product_channel" => format!("{:?}", self.segments.as_ref().unwrap().product_channel),
            "segments.product_item_id" => format!("{:?}", self.segments.as_ref().unwrap().product_item_id),
            "segments.search_term_match_type" => format!("{:?}", self.segments.as_ref().unwrap().search_term_match_type()),
            "segments.week" => format!("{:?}", self.segments.as_ref().unwrap().week),
            "segments.year" => format!("{:?}", self.segments.as_ref().unwrap().year),
            _ => "unsupported".to_string()
        }
    }
}
