
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
            "account_budget.id" => self.account_budget.as_ref().unwrap().id.to_string(),
            "account_budget.name" => self.account_budget.as_ref().unwrap().name.to_string(),
            "account_budget.status" => self.account_budget.as_ref().unwrap().status.to_string(),
            "ad_group.cpc_bid_micros" => self.ad_group.as_ref().unwrap().cpc_bid_micros.to_string(),
            "ad_group.cpm_bid_micros" => self.ad_group.as_ref().unwrap().cpm_bid_micros.to_string(),
            "ad_group.cpv_bid_micros" => self.ad_group.as_ref().unwrap().cpv_bid_micros.to_string(),
            "ad_group.effective_cpc_bid_micros" => self.ad_group.as_ref().unwrap().effective_cpc_bid_micros.to_string(),
            "ad_group.effective_target_cpa_micros" => self.ad_group.as_ref().unwrap().effective_target_cpa_micros.to_string(),
            "ad_group.effective_target_cpa_source" => self.ad_group.as_ref().unwrap().effective_target_cpa_source.to_string(),
            "ad_group.effective_target_roas" => self.ad_group.as_ref().unwrap().effective_target_roas.to_string(),
            "ad_group.effective_target_roas_source" => self.ad_group.as_ref().unwrap().effective_target_roas_source.to_string(),
            "ad_group.id" => self.ad_group.as_ref().unwrap().id.to_string(),
            "ad_group.name" => self.ad_group.as_ref().unwrap().name.to_string(),
            "ad_group.status" => self.ad_group.as_ref().unwrap().status.to_string(),
            "ad_group.target_cpa_micros" => self.ad_group.as_ref().unwrap().target_cpa_micros.to_string(),
            "ad_group.target_cpm_micros" => self.ad_group.as_ref().unwrap().target_cpm_micros.to_string(),
            "ad_group.target_roas" => self.ad_group.as_ref().unwrap().target_roas.to_string(),
            "ad_group.type" => self.ad_group.as_ref().unwrap().r#type.to_string(),
            "ad_group_ad.ad.id" => self.ad_group_ad.as_ref().unwrap().ad.as_ref().unwrap().id.to_string(),
            "ad_group_ad.ad.name" => self.ad_group_ad.as_ref().unwrap().ad.as_ref().unwrap().name.to_string(),
            "ad_group_ad.ad.type" => self.ad_group_ad.as_ref().unwrap().ad.as_ref().unwrap().r#type.to_string(),
            "ad_group_ad.status" => self.ad_group_ad.as_ref().unwrap().status.to_string(),
            "ad_group_criterion.bid_modifier" => self.ad_group_criterion.as_ref().unwrap().bid_modifier.to_string(),
            "ad_group_criterion.cpc_bid_micros" => self.ad_group_criterion.as_ref().unwrap().cpc_bid_micros.to_string(),
            "ad_group_criterion.cpm_bid_micros" => self.ad_group_criterion.as_ref().unwrap().cpm_bid_micros.to_string(),
            "ad_group_criterion.cpv_bid_micros" => self.ad_group_criterion.as_ref().unwrap().cpv_bid_micros.to_string(),
            "ad_group_criterion.criterion_id" => self.ad_group_criterion.as_ref().unwrap().criterion_id.to_string(),
            "ad_group_criterion.effective_cpc_bid_micros" => self.ad_group_criterion.as_ref().unwrap().effective_cpc_bid_micros.to_string(),
            "ad_group_criterion.effective_cpc_bid_source" => self.ad_group_criterion.as_ref().unwrap().effective_cpc_bid_source.to_string(),
            "ad_group_criterion.effective_cpm_bid_micros" => self.ad_group_criterion.as_ref().unwrap().effective_cpm_bid_micros.to_string(),
            "ad_group_criterion.effective_cpm_bid_source" => self.ad_group_criterion.as_ref().unwrap().effective_cpm_bid_source.to_string(),
            "ad_group_criterion.effective_cpv_bid_micros" => self.ad_group_criterion.as_ref().unwrap().effective_cpv_bid_micros.to_string(),
            "ad_group_criterion.effective_cpv_bid_source" => self.ad_group_criterion.as_ref().unwrap().effective_cpv_bid_source.to_string(),
            "ad_group_criterion.effective_percent_cpc_bid_micros" => self.ad_group_criterion.as_ref().unwrap().effective_percent_cpc_bid_micros.to_string(),
            "ad_group_criterion.effective_percent_cpc_bid_source" => self.ad_group_criterion.as_ref().unwrap().effective_percent_cpc_bid_source.to_string(),
            "ad_group_criterion.status" => self.ad_group_criterion.as_ref().unwrap().status.to_string(),
            "ad_group_criterion.type" => self.ad_group_criterion.as_ref().unwrap().r#type.to_string(),
            "audience.description" => self.audience.as_ref().unwrap().description.to_string(),
            "audience.id" => self.audience.as_ref().unwrap().id.to_string(),
            "audience.name" => self.audience.as_ref().unwrap().name.to_string(),
            "audience.status" => self.audience.as_ref().unwrap().status.to_string(),
            "bidding_strategy.id" => self.bidding_strategy.as_ref().unwrap().id.to_string(),
            "bidding_strategy.name" => self.bidding_strategy.as_ref().unwrap().name.to_string(),
            "bidding_strategy.status" => self.bidding_strategy.as_ref().unwrap().status.to_string(),
            "campaign.advertising_channel_type" => self.campaign.as_ref().unwrap().advertising_channel_type().to_string(),
            "campaign.advertising_channel_sub_type" => self.campaign.as_ref().unwrap().advertising_channel_sub_type().to_string(),
            "campaign.base_campaign" => self.campaign.as_ref().unwrap().base_campaign.to_string(),
            "campaign.bidding_strategy_type" => self.campaign.as_ref().unwrap().bidding_strategy_type.to_string(),
            "campaign.campaign_budget" => self.campaign.as_ref().unwrap().campaign_budget.to_string(),
            "campaign.dynamic_search_ads_setting.domain_name" => self.campaign.as_ref().unwrap().dynamic_search_ads_setting.as_ref().unwrap().domain_name.to_string(),
            "campaign.dynamic_search_ads_setting.language_code" => self.campaign.as_ref().unwrap().dynamic_search_ads_setting.as_ref().unwrap().language_code.to_string(),
            "campaign.dynamic_search_ads_setting.use_supplied_urls_only" => self.campaign.as_ref().unwrap().dynamic_search_ads_setting.as_ref().unwrap().use_supplied_urls_only.to_string(),
            "campaign.end_date" => self.campaign.as_ref().unwrap().end_date.to_string(),
            "campaign.experiment_type" => self.campaign.as_ref().unwrap().experiment_type.to_string(),
            "campaign.id" => self.campaign.as_ref().unwrap().id.to_string(),
            "campaign.name" => self.campaign.as_ref().unwrap().name.to_string(),
            "campaign.network_settings.target_content_network" => self.campaign.as_ref().unwrap().network_settings.as_ref().unwrap().target_content_network.to_string(),
            "campaign.network_settings.target_google_search" => self.campaign.as_ref().unwrap().network_settings.as_ref().unwrap().target_google_search.to_string(),
            "campaign.network_settings.target_partner_search_network" => self.campaign.as_ref().unwrap().network_settings.as_ref().unwrap().target_partner_search_network.to_string(),
            "campaign.network_settings.target_search_network" => self.campaign.as_ref().unwrap().network_settings.as_ref().unwrap().target_search_network.to_string(),
            "campaign.serving_status" => self.campaign.as_ref().unwrap().serving_status.to_string(),
            "campaign.status" => self.campaign.as_ref().unwrap().status.to_string(),
            "customer.id" => self.customer.as_ref().unwrap().id.to_string(),
            "customer.descriptive_name" => self.customer.as_ref().unwrap().descriptive_name.to_string(),
            "customer.status" => self.customer.as_ref().unwrap().status.to_string(),
            "customer.currency_code" => self.customer.as_ref().unwrap().currency_code.to_string(),
            "customer.time_zone" => self.customer.as_ref().unwrap().time_zone.to_string(),
            "customer_client.client_customer" => self.customer_client.as_ref().unwrap().client_customer.to_string(),
            "customer_client.currency_code" => self.customer_client.as_ref().unwrap().currency_code.to_string(),
            "customer_client.descriptive_name" => self.customer_client.as_ref().unwrap().descriptive_name.to_string(),
            "customer_client.id" => self.customer_client.as_ref().unwrap().id.to_string(),
            "customer_client.level" => self.customer_client.as_ref().unwrap().level.to_string(),
            "customer_client.manager" => self.customer_client.as_ref().unwrap().manager.to_string(),
            "customer_client.status" => self.customer_client.as_ref().unwrap().status.to_string(),
            "customer_client.time_zone" => self.customer_client.as_ref().unwrap().time_zone.to_string(),
            "metrics.absolute_top_impression_percentage" => self.metrics.as_ref().unwrap().absolute_top_impression_percentage.to_string(),
            "metrics.active_view_cpm" => self.metrics.as_ref().unwrap().active_view_cpm.to_string(),
            "metrics.active_view_ctr" => self.metrics.as_ref().unwrap().active_view_ctr.to_string(),
            "metrics.active_view_impressions" => self.metrics.as_ref().unwrap().active_view_impressions.to_string(),
            "metrics.active_view_measurability" => self.metrics.as_ref().unwrap().active_view_measurability.to_string(),
            "metrics.active_view_measurable_cost_micros" => self.metrics.as_ref().unwrap().active_view_measurable_cost_micros.to_string(),
            "metrics.active_view_measurable_impressions" => self.metrics.as_ref().unwrap().active_view_measurable_impressions.to_string(),
            "metrics.active_view_viewability" => self.metrics.as_ref().unwrap().active_view_viewability.to_string(),
            "metrics.all_conversions" => self.metrics.as_ref().unwrap().all_conversions.to_string(),
            "metrics.all_conversions_by_conversion_date" => self.metrics.as_ref().unwrap().all_conversions_by_conversion_date.to_string(),
            "metrics.all_conversions_from_click_to_call" => self.metrics.as_ref().unwrap().all_conversions_from_click_to_call.to_string(),
            "metrics.all_conversions_from_directions" => self.metrics.as_ref().unwrap().all_conversions_from_directions.to_string(),
            "metrics.all_conversions_from_interactions_rate" => self.metrics.as_ref().unwrap().all_conversions_from_interactions_rate.to_string(),
            "metrics.all_conversions_from_interactions_value_per_interaction" => self.metrics.as_ref().unwrap().all_conversions_from_interactions_value_per_interaction.to_string(),
            "metrics.all_conversions_from_menu" => self.metrics.as_ref().unwrap().all_conversions_from_menu.to_string(),
            "metrics.all_conversions_from_order" => self.metrics.as_ref().unwrap().all_conversions_from_order.to_string(),
            "metrics.all_conversions_from_other_engagement" => self.metrics.as_ref().unwrap().all_conversions_from_other_engagement.to_string(),
            "metrics.all_conversions_from_store_visit" => self.metrics.as_ref().unwrap().all_conversions_from_store_visit.to_string(),
            "metrics.all_conversions_from_store_website" => self.metrics.as_ref().unwrap().all_conversions_from_store_website.to_string(),
            "metrics.all_conversions_value" => self.metrics.as_ref().unwrap().all_conversions_value.to_string(),
            "metrics.all_conversions_value_by_conversion_date" => self.metrics.as_ref().unwrap().all_conversions_value_by_conversion_date.to_string(),
            "metrics.all_conversions_value_per_cost" => self.metrics.as_ref().unwrap().all_conversions_value_per_cost.to_string(),
            "metrics.average_cost" => self.metrics.as_ref().unwrap().average_cost.to_string(),
            "metrics.average_cpc" => self.metrics.as_ref().unwrap().average_cpc.to_string(),
            "metrics.average_cpe" => self.metrics.as_ref().unwrap().average_cpe.to_string(),
            "metrics.average_cpm" => self.metrics.as_ref().unwrap().average_cpm.to_string(),
            "metrics.average_cpv" => self.metrics.as_ref().unwrap().average_cpv.to_string(),
            "metrics.average_page_views" => self.metrics.as_ref().unwrap().average_page_views.to_string(),
            "metrics.average_time_on_site" => self.metrics.as_ref().unwrap().average_time_on_site.to_string(),
            "metrics.benchmark_average_max_cpc" => self.metrics.as_ref().unwrap().benchmark_average_max_cpc.to_string(),
            "metrics.benchmark_ctr" => self.metrics.as_ref().unwrap().benchmark_ctr.to_string(),
            "metrics.biddable_app_install_conversions" => self.metrics.as_ref().unwrap().biddable_app_install_conversions.to_string(),
            "metrics.biddable_app_post_install_conversions" => self.metrics.as_ref().unwrap().biddable_app_post_install_conversions.to_string(),
            "metrics.bounce_rate" => self.metrics.as_ref().unwrap().bounce_rate.to_string(),
            "metrics.clicks" => self.metrics.as_ref().unwrap().clicks.to_string(),
            "metrics.combined_clicks" => self.metrics.as_ref().unwrap().combined_clicks.to_string(),
            "metrics.combined_clicks_per_query" => self.metrics.as_ref().unwrap().combined_clicks_per_query.to_string(),
            "metrics.combined_queries" => self.metrics.as_ref().unwrap().combined_queries.to_string(),
            "metrics.content_budget_lost_impression_share" => self.metrics.as_ref().unwrap().content_budget_lost_impression_share.to_string(),
            "metrics.content_impression_share" => self.metrics.as_ref().unwrap().content_impression_share.to_string(),
            "metrics.content_rank_lost_impression_share" => self.metrics.as_ref().unwrap().content_rank_lost_impression_share.to_string(),
            "metrics.conversion_last_conversion_date" => self.metrics.as_ref().unwrap().conversion_last_conversion_date.to_string(),
            "metrics.conversion_last_received_request_date_time" => self.metrics.as_ref().unwrap().conversion_last_received_request_date_time.to_string(),
            "metrics.conversions" => self.metrics.as_ref().unwrap().conversions.to_string(),
            "metrics.conversions_by_conversion_date" => self.metrics.as_ref().unwrap().conversions_by_conversion_date.to_string(),
            "metrics.conversions_from_interactions_rate" => self.metrics.as_ref().unwrap().conversions_from_interactions_rate.to_string(),
            "metrics.conversions_from_interactions_value_per_interaction" => self.metrics.as_ref().unwrap().conversions_from_interactions_value_per_interaction.to_string(),
            "metrics.conversions_value" => self.metrics.as_ref().unwrap().conversions_value.to_string(),
            "metrics.conversions_value_by_conversion_date" => self.metrics.as_ref().unwrap().conversions_value_by_conversion_date.to_string(),
            "metrics.conversions_value_per_cost" => self.metrics.as_ref().unwrap().conversions_value_per_cost.to_string(),
            "metrics.cost_micros" => self.metrics.as_ref().unwrap().cost_micros.to_string(),
            "metrics.cost_per_all_conversions" => self.metrics.as_ref().unwrap().cost_per_all_conversions.to_string(),
            "metrics.cost_per_conversion" => self.metrics.as_ref().unwrap().cost_per_conversion.to_string(),
            "metrics.cost_per_current_model_attributed_conversion" => self.metrics.as_ref().unwrap().cost_per_current_model_attributed_conversion.to_string(),
            "metrics.cross_device_conversions" => self.metrics.as_ref().unwrap().cross_device_conversions.to_string(),
            "metrics.ctr" => self.metrics.as_ref().unwrap().ctr.to_string(),
            "metrics.current_model_attributed_conversions" => self.metrics.as_ref().unwrap().current_model_attributed_conversions.to_string(),
            "metrics.current_model_attributed_conversions_from_interactions_rate" => self.metrics.as_ref().unwrap().current_model_attributed_conversions_from_interactions_rate.to_string(),
            "metrics.current_model_attributed_conversions_from_interactions_value_per_interaction" => self.metrics.as_ref().unwrap().current_model_attributed_conversions_from_interactions_value_per_interaction.to_string(),
            "metrics.current_model_attributed_conversions_value" => self.metrics.as_ref().unwrap().current_model_attributed_conversions_value.to_string(),
            "metrics.current_model_attributed_conversions_value_per_cost" => self.metrics.as_ref().unwrap().current_model_attributed_conversions_value_per_cost.to_string(),
            "metrics.engagement_rate" => self.metrics.as_ref().unwrap().engagement_rate.to_string(),
            "metrics.engagements" => self.metrics.as_ref().unwrap().engagements.to_string(),
            "metrics.gmail_forwards" => self.metrics.as_ref().unwrap().gmail_forwards.to_string(),
            "metrics.gmail_saves" => self.metrics.as_ref().unwrap().gmail_saves.to_string(),
            "metrics.gmail_secondary_clicks" => self.metrics.as_ref().unwrap().gmail_secondary_clicks.to_string(),
            "metrics.historical_creative_quality_score" => self.metrics.as_ref().unwrap().historical_creative_quality_score.to_string(),
            "metrics.historical_landing_page_quality_score" => self.metrics.as_ref().unwrap().historical_landing_page_quality_score.to_string(),
            "metrics.historical_quality_score" => self.metrics.as_ref().unwrap().historical_quality_score.to_string(),
            "metrics.historical_search_predicted_ctr" => self.metrics.as_ref().unwrap().historical_search_predicted_ctr.to_string(),
            "metrics.hotel_average_lead_value_micros" => self.metrics.as_ref().unwrap().hotel_average_lead_value_micros.to_string(),
            "metrics.hotel_commission_rate_micros" => self.metrics.as_ref().unwrap().hotel_commission_rate_micros.to_string(),
            "metrics.hotel_eligible_impressions" => self.metrics.as_ref().unwrap().hotel_eligible_impressions.to_string(),
            "metrics.hotel_expected_commission_cost" => self.metrics.as_ref().unwrap().hotel_expected_commission_cost.to_string(),
            "metrics.hotel_price_difference_percentage" => self.metrics.as_ref().unwrap().hotel_price_difference_percentage.to_string(),
            "metrics.impressions" => self.metrics.as_ref().unwrap().impressions.to_string(),
            "metrics.impressions_from_store_reach" => self.metrics.as_ref().unwrap().impressions_from_store_reach.to_string(),
            "metrics.interaction_rate" => self.metrics.as_ref().unwrap().interaction_rate.to_string(),
            "metrics.interactions" => self.metrics.as_ref().unwrap().interactions.to_string(),
            "metrics.invalid_click_rate" => self.metrics.as_ref().unwrap().invalid_click_rate.to_string(),
            "metrics.invalid_clicks" => self.metrics.as_ref().unwrap().invalid_clicks.to_string(),
            "metrics.message_chat_rate" => self.metrics.as_ref().unwrap().message_chat_rate.to_string(),
            "metrics.message_chats" => self.metrics.as_ref().unwrap().message_chats.to_string(),
            "metrics.message_impressions" => self.metrics.as_ref().unwrap().message_impressions.to_string(),
            "metrics.mobile_friendly_clicks_percentage" => self.metrics.as_ref().unwrap().mobile_friendly_clicks_percentage.to_string(),
            "metrics.optimization_score_uplift" => self.metrics.as_ref().unwrap().optimization_score_uplift.to_string(),
            "metrics.optimization_score_url" => self.metrics.as_ref().unwrap().optimization_score_url.to_string(),
            "metrics.organic_clicks" => self.metrics.as_ref().unwrap().organic_clicks.to_string(),
            "metrics.organic_clicks_per_query" => self.metrics.as_ref().unwrap().organic_clicks_per_query.to_string(),
            "metrics.organic_impressions" => self.metrics.as_ref().unwrap().organic_impressions.to_string(),
            "metrics.organic_impressions_per_query" => self.metrics.as_ref().unwrap().organic_impressions_per_query.to_string(),
            "metrics.organic_queries" => self.metrics.as_ref().unwrap().organic_queries.to_string(),
            "metrics.percent_new_visitors" => self.metrics.as_ref().unwrap().percent_new_visitors.to_string(),
            "metrics.phone_calls" => self.metrics.as_ref().unwrap().phone_calls.to_string(),
            "metrics.phone_impressions" => self.metrics.as_ref().unwrap().phone_impressions.to_string(),
            "metrics.phone_through_rate" => self.metrics.as_ref().unwrap().phone_through_rate.to_string(),
            "metrics.relative_ctr" => self.metrics.as_ref().unwrap().relative_ctr.to_string(),
            "metrics.search_absolute_top_impression_share" => self.metrics.as_ref().unwrap().search_absolute_top_impression_share.to_string(),
            "metrics.search_budget_lost_absolute_top_impression_share" => self.metrics.as_ref().unwrap().search_budget_lost_absolute_top_impression_share.to_string(),
            "metrics.search_budget_lost_impression_share" => self.metrics.as_ref().unwrap().search_budget_lost_impression_share.to_string(),
            "metrics.search_budget_lost_top_impression_share" => self.metrics.as_ref().unwrap().search_budget_lost_top_impression_share.to_string(),
            "metrics.search_click_share" => self.metrics.as_ref().unwrap().search_click_share.to_string(),
            "metrics.search_exact_match_impression_share" => self.metrics.as_ref().unwrap().search_exact_match_impression_share.to_string(),
            "metrics.search_impression_share" => self.metrics.as_ref().unwrap().search_impression_share.to_string(),
            "metrics.search_rank_lost_absolute_top_impression_share" => self.metrics.as_ref().unwrap().search_rank_lost_absolute_top_impression_share.to_string(),
            "metrics.search_rank_lost_impression_share" => self.metrics.as_ref().unwrap().search_rank_lost_impression_share.to_string(),
            "metrics.search_rank_lost_top_impression_share" => self.metrics.as_ref().unwrap().search_rank_lost_top_impression_share.to_string(),
            "metrics.search_top_impression_share" => self.metrics.as_ref().unwrap().search_top_impression_share.to_string(),
            "metrics.sk_ad_network_conversions" => self.metrics.as_ref().unwrap().sk_ad_network_conversions.to_string(),
            "metrics.speed_score" => self.metrics.as_ref().unwrap().speed_score.to_string(),
            "metrics.top_impression_percentage" => self.metrics.as_ref().unwrap().top_impression_percentage.to_string(),
            "metrics.valid_accelerated_mobile_pages_clicks_percentage" => self.metrics.as_ref().unwrap().valid_accelerated_mobile_pages_clicks_percentage.to_string(),
            "metrics.value_per_all_conversions" => self.metrics.as_ref().unwrap().value_per_all_conversions.to_string(),
            "metrics.value_per_all_conversions_by_conversion_date" => self.metrics.as_ref().unwrap().value_per_all_conversions_by_conversion_date.to_string(),
            "metrics.value_per_conversion" => self.metrics.as_ref().unwrap().value_per_conversion.to_string(),
            "metrics.value_per_conversions_by_conversion_date" => self.metrics.as_ref().unwrap().value_per_conversions_by_conversion_date.to_string(),
            "metrics.value_per_current_model_attributed_conversion" => self.metrics.as_ref().unwrap().value_per_current_model_attributed_conversion.to_string(),
            "metrics.video_quartile_p100_rate" => self.metrics.as_ref().unwrap().video_quartile_p100_rate.to_string(),
            "metrics.video_quartile_p25_rate" => self.metrics.as_ref().unwrap().video_quartile_p25_rate.to_string(),
            "metrics.video_quartile_p50_rate" => self.metrics.as_ref().unwrap().video_quartile_p50_rate.to_string(),
            "metrics.video_quartile_p75_rate" => self.metrics.as_ref().unwrap().video_quartile_p75_rate.to_string(),
            "metrics.video_view_rate" => self.metrics.as_ref().unwrap().video_view_rate.to_string(),
            "metrics.video_views" => self.metrics.as_ref().unwrap().video_views.to_string(),
            "metrics.view_through_conversions" => self.metrics.as_ref().unwrap().view_through_conversions.to_string(),
            "segments.ad_network_type" => self.segments.as_ref().unwrap().ad_network_type.to_string(),
            "segments.click_type" => self.segments.as_ref().unwrap().click_type.to_string(),
            "segments.date" => self.segments.as_ref().unwrap().date.to_string(),
            "segments.day_of_week" => self.segments.as_ref().unwrap().day_of_week.to_string(),
            "segments.device" => format!("{:?}", self.segments.as_ref().unwrap().device()),
            "segments.hour" => self.segments.as_ref().unwrap().hour.to_string(),
            "segments.month" => self.segments.as_ref().unwrap().month.to_string(),
            "segments.month_of_year" => self.segments.as_ref().unwrap().month_of_year.to_string(),
            "segments.product_channel" => self.segments.as_ref().unwrap().product_channel.to_string(),
            "segments.product_item_id" => self.segments.as_ref().unwrap().product_item_id.to_string(),
            "segments.week" => self.segments.as_ref().unwrap().week.to_string(),
            "segments.year" => self.segments.as_ref().unwrap().year.to_string(),
            _ => "unsupported".to_string()
        }
    }
}

use std::fmt;

impl fmt::Display for google::ads::googleads::v10::enums::advertising_channel_type_enum::AdvertisingChannelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for google::ads::googleads::v10::enums::advertising_channel_sub_type_enum::AdvertisingChannelSubType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

// impl fmt::Display for google::ads::googleads::v10::enums::device_enum::Device {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }
