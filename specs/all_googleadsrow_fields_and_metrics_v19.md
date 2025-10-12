# Google Ads API V19 - Complete GoogleAdsRow Field Reference

This document provides a comprehensive reference of all fields and metrics available in the `GoogleAdsRow` struct returned by Google Ads API V19 query operations.

**Source**: Generated from `google.ads.googleads.v19.services.rs` and `google.ads.googleads.v19.common.rs`

**Total Fields**: 568
- Resource Fields: 189
- Metrics Fields: 283
- Segments Fields: 96

---

## Table of Contents

1. [Resource Fields](#1-resource-fields)
2. [Metrics Fields](#2-metrics-fields)
3. [Segments Fields](#3-segments-fields)

---

## 1. Resource Fields

The GoogleAdsRow struct contains 189 optional resource fields representing different Google Ads entities that can be queried.

### A

#### accessible_bidding_strategy
- **Type**: `Option<AccessibleBiddingStrategy>`
- **Tag**: 169
- **Description**: The accessible bidding strategy referenced in the query.

#### account_budget
- **Type**: `Option<AccountBudget>`
- **Tag**: 42
- **Description**: The account budget in the query.

#### account_budget_proposal
- **Type**: `Option<AccountBudgetProposal>`
- **Tag**: 43
- **Description**: The account budget proposal referenced in the query.

#### account_link
- **Type**: `Option<AccountLink>`
- **Tag**: 143
- **Description**: The AccountLink referenced in the query.

#### ad
- **Type**: `Option<Ad>`
- **Tag**: 227
- **Description**: The Ad referenced in the query.

#### ad_group
- **Type**: `Option<AdGroup>`
- **Tag**: 3
- **Description**: The ad group referenced in the query.

#### ad_group_ad
- **Type**: `Option<AdGroupAd>`
- **Tag**: 16
- **Description**: The ad referenced in the query.

#### ad_group_ad_asset_combination_view
- **Type**: `Option<AdGroupAdAssetCombinationView>`
- **Tag**: 193
- **Description**: The ad group ad asset combination view in the query.

#### ad_group_ad_asset_view
- **Type**: `Option<AdGroupAdAssetView>`
- **Tag**: 131
- **Description**: The ad group ad asset view in the query.

#### ad_group_ad_label
- **Type**: `Option<AdGroupAdLabel>`
- **Tag**: 120
- **Description**: The ad group ad label referenced in the query.

#### ad_group_asset
- **Type**: `Option<AdGroupAsset>`
- **Tag**: 154
- **Description**: The ad group asset referenced in the query.

#### ad_group_asset_set
- **Type**: `Option<AdGroupAssetSet>`
- **Tag**: 196
- **Description**: The ad group asset set referenced in the query.

#### ad_group_audience_view
- **Type**: `Option<AdGroupAudienceView>`
- **Tag**: 57
- **Description**: The ad group audience view referenced in the query.

#### ad_group_bid_modifier
- **Type**: `Option<AdGroupBidModifier>`
- **Tag**: 24
- **Description**: The bid modifier referenced in the query.

#### ad_group_criterion
- **Type**: `Option<AdGroupCriterion>`
- **Tag**: 17
- **Description**: The criterion referenced in the query.

#### ad_group_criterion_customizer
- **Type**: `Option<AdGroupCriterionCustomizer>`
- **Tag**: 187
- **Description**: The ad group criterion customizer referenced in the query.

#### ad_group_criterion_label
- **Type**: `Option<AdGroupCriterionLabel>`
- **Tag**: 121
- **Description**: The ad group criterion label referenced in the query.

#### ad_group_criterion_simulation
- **Type**: `Option<AdGroupCriterionSimulation>`
- **Tag**: 110
- **Description**: The ad group criterion simulation referenced in the query.

#### ad_group_customizer
- **Type**: `Option<AdGroupCustomizer>`
- **Tag**: 185
- **Description**: The ad group customizer referenced in the query.

#### ad_group_label
- **Type**: `Option<AdGroupLabel>`
- **Tag**: 115
- **Description**: The ad group label referenced in the query.

#### ad_group_simulation
- **Type**: `Option<AdGroupSimulation>`
- **Tag**: 107
- **Description**: The ad group simulation referenced in the query.

#### ad_parameter
- **Type**: `Option<AdParameter>`
- **Tag**: 130
- **Description**: The ad parameter referenced in the query.

#### ad_schedule_view
- **Type**: `Option<AdScheduleView>`
- **Tag**: 89
- **Description**: The ad schedule view referenced in the query.

#### age_range_view
- **Type**: `Option<AgeRangeView>`
- **Tag**: 48
- **Description**: The age range view referenced in the query.

#### android_privacy_shared_key_google_ad_group
- **Type**: `Option<AndroidPrivacySharedKeyGoogleAdGroup>`
- **Tag**: 217
- **Description**: The android privacy shared key google ad group referenced in the query.

#### android_privacy_shared_key_google_campaign
- **Type**: `Option<AndroidPrivacySharedKeyGoogleCampaign>`
- **Tag**: 218
- **Description**: The android privacy shared key google campaign referenced in the query.

#### android_privacy_shared_key_google_network_type
- **Type**: `Option<AndroidPrivacySharedKeyGoogleNetworkType>`
- **Tag**: 219
- **Description**: The android privacy shared key google network type referenced in the query.

#### asset
- **Type**: `Option<Asset>`
- **Tag**: 105
- **Description**: The asset referenced in the query.

#### asset_field_type_view
- **Type**: `Option<AssetFieldTypeView>`
- **Tag**: 168
- **Description**: The asset field type view referenced in the query.

#### asset_group
- **Type**: `Option<AssetGroup>`
- **Tag**: 172
- **Description**: The asset group referenced in the query.

#### asset_group_asset
- **Type**: `Option<AssetGroupAsset>`
- **Tag**: 173
- **Description**: The asset group asset referenced in the query.

#### asset_group_listing_group_filter
- **Type**: `Option<AssetGroupListingGroupFilter>`
- **Tag**: 182
- **Description**: The asset group listing group filter referenced in the query.

#### asset_group_product_group_view
- **Type**: `Option<AssetGroupProductGroupView>`
- **Tag**: 189
- **Description**: The asset group product group view referenced in the query.

#### asset_group_signal
- **Type**: `Option<AssetGroupSignal>`
- **Tag**: 191
- **Description**: The asset group signal referenced in the query.

#### asset_group_top_combination_view
- **Type**: `Option<AssetGroupTopCombinationView>`
- **Tag**: 199
- **Description**: The asset group top combination view referenced in the query.

#### asset_set
- **Type**: `Option<AssetSet>`
- **Tag**: 179
- **Description**: The asset set referenced in the query.

#### asset_set_asset
- **Type**: `Option<AssetSetAsset>`
- **Tag**: 180
- **Description**: The asset set asset referenced in the query.

#### asset_set_type_view
- **Type**: `Option<AssetSetTypeView>`
- **Tag**: 197
- **Description**: The asset set type view referenced in the query.

#### audience
- **Type**: `Option<Audience>`
- **Tag**: 190
- **Description**: The Audience referenced in the query.

### B

#### batch_job
- **Type**: `Option<BatchJob>`
- **Tag**: 139
- **Description**: The batch job referenced in the query.

#### bidding_data_exclusion
- **Type**: `Option<BiddingDataExclusion>`
- **Tag**: 159
- **Description**: The bidding data exclusion referenced in the query.

#### bidding_seasonality_adjustment
- **Type**: `Option<BiddingSeasonalityAdjustment>`
- **Tag**: 160
- **Description**: The bidding seasonality adjustment referenced in the query.

#### bidding_strategy
- **Type**: `Option<BiddingStrategy>`
- **Tag**: 18
- **Description**: The bidding strategy referenced in the query.

#### bidding_strategy_simulation
- **Type**: `Option<BiddingStrategySimulation>`
- **Tag**: 158
- **Description**: The bidding strategy simulation referenced in the query.

#### billing_setup
- **Type**: `Option<BillingSetup>`
- **Tag**: 41
- **Description**: The billing setup referenced in the query.

### C

#### call_view
- **Type**: `Option<CallView>`
- **Tag**: 152
- **Description**: The call view referenced in the query.

#### campaign
- **Type**: `Option<Campaign>`
- **Tag**: 2
- **Description**: The campaign referenced in the query.

#### campaign_aggregate_asset_view
- **Type**: `Option<CampaignAggregateAssetView>`
- **Tag**: 224
- **Description**: The campaign aggregate asset view referenced in the query.

#### campaign_asset
- **Type**: `Option<CampaignAsset>`
- **Tag**: 142
- **Description**: The campaign asset referenced in the query.

#### campaign_asset_set
- **Type**: `Option<CampaignAssetSet>`
- **Tag**: 181
- **Description**: The campaign asset set referenced in the query.

#### campaign_audience_view
- **Type**: `Option<CampaignAudienceView>`
- **Tag**: 69
- **Description**: The campaign audience view referenced in the query.

#### campaign_bid_modifier
- **Type**: `Option<CampaignBidModifier>`
- **Tag**: 26
- **Description**: The campaign bid modifier referenced in the query.

#### campaign_budget
- **Type**: `Option<CampaignBudget>`
- **Tag**: 19
- **Description**: The campaign budget referenced in the query.

#### campaign_conversion_goal
- **Type**: `Option<CampaignConversionGoal>`
- **Tag**: 175
- **Description**: The CampaignConversionGoal referenced in the query.

#### campaign_criterion
- **Type**: `Option<CampaignCriterion>`
- **Tag**: 20
- **Description**: The campaign criterion referenced in the query.

#### campaign_customizer
- **Type**: `Option<CampaignCustomizer>`
- **Tag**: 186
- **Description**: The campaign customizer referenced in the query.

#### campaign_draft
- **Type**: `Option<CampaignDraft>`
- **Tag**: 49
- **Description**: The campaign draft referenced in the query.

#### campaign_group
- **Type**: `Option<CampaignGroup>`
- **Tag**: 25
- **Description**: Campaign Group referenced in AWQL query.

#### campaign_label
- **Type**: `Option<CampaignLabel>`
- **Tag**: 108
- **Description**: The campaign label referenced in the query.

#### campaign_lifecycle_goal
- **Type**: `Option<CampaignLifecycleGoal>`
- **Tag**: 213
- **Description**: The campaign lifecycle goal referenced in the query.

#### campaign_search_term_insight
- **Type**: `Option<CampaignSearchTermInsight>`
- **Tag**: 204
- **Description**: The campaign search term insight referenced in the query.

#### campaign_shared_set
- **Type**: `Option<CampaignSharedSet>`
- **Tag**: 30
- **Description**: Campaign Shared Set referenced in AWQL query.

#### campaign_simulation
- **Type**: `Option<CampaignSimulation>`
- **Tag**: 157
- **Description**: The campaign simulation referenced in the query.

#### carrier_constant
- **Type**: `Option<CarrierConstant>`
- **Tag**: 66
- **Description**: The carrier constant referenced in the query.

#### change_event
- **Type**: `Option<ChangeEvent>`
- **Tag**: 145
- **Description**: The ChangeEvent referenced in the query.

#### change_status
- **Type**: `Option<ChangeStatus>`
- **Tag**: 37
- **Description**: The ChangeStatus referenced in the query.

#### channel_aggregate_asset_view
- **Type**: `Option<ChannelAggregateAssetView>`
- **Tag**: 222
- **Description**: The channel aggregate asset view referenced in the query.

#### click_view
- **Type**: `Option<ClickView>`
- **Tag**: 122
- **Description**: The ClickView referenced in the query.

#### combined_audience
- **Type**: `Option<CombinedAudience>`
- **Tag**: 148
- **Description**: The CombinedAudience referenced in the query.

#### content_criterion_view
- **Type**: `Option<ContentCriterionView>`
- **Tag**: 232
- **Description**: The content criterion view referenced in the query.

#### conversion_action
- **Type**: `Option<ConversionAction>`
- **Tag**: 103
- **Description**: The conversion action referenced in the query.

#### conversion_custom_variable
- **Type**: `Option<ConversionCustomVariable>`
- **Tag**: 153
- **Description**: The conversion custom variable referenced in the query.

#### conversion_goal_campaign_config
- **Type**: `Option<ConversionGoalCampaignConfig>`
- **Tag**: 177
- **Description**: The ConversionGoalCampaignConfig referenced in the query.

#### conversion_value_rule
- **Type**: `Option<ConversionValueRule>`
- **Tag**: 164
- **Description**: The conversion value rule referenced in the query.

#### conversion_value_rule_set
- **Type**: `Option<ConversionValueRuleSet>`
- **Tag**: 165
- **Description**: The conversion value rule set referenced in the query.

#### currency_constant
- **Type**: `Option<CurrencyConstant>`
- **Tag**: 134
- **Description**: The currency constant referenced in the query.

#### custom_audience
- **Type**: `Option<CustomAudience>`
- **Tag**: 147
- **Description**: The CustomAudience referenced in the query.

#### custom_conversion_goal
- **Type**: `Option<CustomConversionGoal>`
- **Tag**: 176
- **Description**: The CustomConversionGoal referenced in the query.

#### custom_interest
- **Type**: `Option<CustomInterest>`
- **Tag**: 104
- **Description**: The CustomInterest referenced in the query.

#### customer
- **Type**: `Option<Customer>`
- **Tag**: 1
- **Description**: The customer referenced in the query.

#### customer_asset
- **Type**: `Option<CustomerAsset>`
- **Tag**: 155
- **Description**: The customer asset referenced in the query.

#### customer_asset_set
- **Type**: `Option<CustomerAssetSet>`
- **Tag**: 195
- **Description**: The customer asset set referenced in the query.

#### customer_client
- **Type**: `Option<CustomerClient>`
- **Tag**: 70
- **Description**: The CustomerClient referenced in the query.

#### customer_client_link
- **Type**: `Option<CustomerClientLink>`
- **Tag**: 62
- **Description**: The CustomerClientLink referenced in the query.

#### customer_conversion_goal
- **Type**: `Option<CustomerConversionGoal>`
- **Tag**: 174
- **Description**: The CustomerConversionGoal referenced in the query.

#### customer_customizer
- **Type**: `Option<CustomerCustomizer>`
- **Tag**: 184
- **Description**: The customer customizer referenced in the query.

#### customer_label
- **Type**: `Option<CustomerLabel>`
- **Tag**: 124
- **Description**: The customer label referenced in the query.

#### customer_lifecycle_goal
- **Type**: `Option<CustomerLifecycleGoal>`
- **Tag**: 212
- **Description**: The customer lifecycle goal referenced in the query.

#### customer_manager_link
- **Type**: `Option<CustomerManagerLink>`
- **Tag**: 61
- **Description**: The CustomerManagerLink referenced in the query.

#### customer_negative_criterion
- **Type**: `Option<CustomerNegativeCriterion>`
- **Tag**: 88
- **Description**: The customer negative criterion referenced in the query.

#### customer_search_term_insight
- **Type**: `Option<CustomerSearchTermInsight>`
- **Tag**: 205
- **Description**: The customer search term insight referenced in the query.

#### customer_user_access
- **Type**: `Option<CustomerUserAccess>`
- **Tag**: 146
- **Description**: The CustomerUserAccess referenced in the query.

#### customer_user_access_invitation
- **Type**: `Option<CustomerUserAccessInvitation>`
- **Tag**: 150
- **Description**: The CustomerUserAccessInvitation referenced in the query.

#### customizer_attribute
- **Type**: `Option<CustomizerAttribute>`
- **Tag**: 178
- **Description**: The customizer attribute referenced in the query.

### D

#### data_link
- **Type**: `Option<DataLink>`
- **Tag**: 230
- **Description**: The data link referenced in the query.

#### detail_placement_view
- **Type**: `Option<DetailPlacementView>`
- **Tag**: 118
- **Description**: The detail placement view referenced in the query.

#### detailed_demographic
- **Type**: `Option<DetailedDemographic>`
- **Tag**: 166
- **Description**: The detailed demographic referenced in the query.

#### display_keyword_view
- **Type**: `Option<DisplayKeywordView>`
- **Tag**: 47
- **Description**: The display keyword view referenced in the query.

#### distance_view
- **Type**: `Option<DistanceView>`
- **Tag**: 132
- **Description**: The distance view referenced in the query.

#### domain_category
- **Type**: `Option<DomainCategory>`
- **Tag**: 91
- **Description**: The domain category referenced in the query.

#### dynamic_search_ads_search_term_view
- **Type**: `Option<DynamicSearchAdsSearchTermView>`
- **Tag**: 106
- **Description**: The dynamic search ads search term view referenced in the query.

### E

#### expanded_landing_page_view
- **Type**: `Option<ExpandedLandingPageView>`
- **Tag**: 128
- **Description**: The expanded landing page view referenced in the query.

#### experiment
- **Type**: `Option<Experiment>`
- **Tag**: 133
- **Description**: The experiment referenced in the query.

#### experiment_arm
- **Type**: `Option<ExperimentArm>`
- **Tag**: 183
- **Description**: The experiment arm referenced in the query.

### G

#### gender_view
- **Type**: `Option<GenderView>`
- **Tag**: 40
- **Description**: The gender view referenced in the query.

#### geo_target_constant
- **Type**: `Option<GeoTargetConstant>`
- **Tag**: 23
- **Description**: The geo target constant referenced in the query.

#### geographic_view
- **Type**: `Option<GeographicView>`
- **Tag**: 125
- **Description**: The geographic view referenced in the query.

#### group_placement_view
- **Type**: `Option<GroupPlacementView>`
- **Tag**: 119
- **Description**: The group placement view referenced in the query.

### H

#### hotel_group_view
- **Type**: `Option<HotelGroupView>`
- **Tag**: 51
- **Description**: The hotel group view referenced in the query.

#### hotel_performance_view
- **Type**: `Option<HotelPerformanceView>`
- **Tag**: 71
- **Description**: The hotel performance view referenced in the query.

#### hotel_reconciliation
- **Type**: `Option<HotelReconciliation>`
- **Tag**: 188
- **Description**: The hotel reconciliation referenced in the query.

### I

#### income_range_view
- **Type**: `Option<IncomeRangeView>`
- **Tag**: 138
- **Description**: The income range view referenced in the query.

### K

#### keyword_plan
- **Type**: `Option<KeywordPlan>`
- **Tag**: 32
- **Description**: The keyword plan referenced in the query.

#### keyword_plan_ad_group
- **Type**: `Option<KeywordPlanAdGroup>`
- **Tag**: 35
- **Description**: The keyword plan ad group referenced in the query.

#### keyword_plan_ad_group_keyword
- **Type**: `Option<KeywordPlanAdGroupKeyword>`
- **Tag**: 141
- **Description**: The keyword plan ad group keyword referenced in the query.

#### keyword_plan_campaign
- **Type**: `Option<KeywordPlanCampaign>`
- **Tag**: 33
- **Description**: The keyword plan campaign referenced in the query.

#### keyword_plan_campaign_keyword
- **Type**: `Option<KeywordPlanCampaignKeyword>`
- **Tag**: 140
- **Description**: The keyword plan campaign keyword referenced in the query.

#### keyword_theme_constant
- **Type**: `Option<KeywordThemeConstant>`
- **Tag**: 163
- **Description**: The keyword theme constant referenced in the query.

#### keyword_view
- **Type**: `Option<KeywordView>`
- **Tag**: 21
- **Description**: The keyword view referenced in the query.

### L

#### label
- **Type**: `Option<Label>`
- **Tag**: 52
- **Description**: The label referenced in the query.

#### landing_page_view
- **Type**: `Option<LandingPageView>`
- **Tag**: 126
- **Description**: The landing page view referenced in the query.

#### language_constant
- **Type**: `Option<LanguageConstant>`
- **Tag**: 55
- **Description**: The language constant referenced in the query.

#### lead_form_submission_data
- **Type**: `Option<LeadFormSubmissionData>`
- **Tag**: 192
- **Description**: The lead form user submission referenced in the query.

#### life_event
- **Type**: `Option<LifeEvent>`
- **Tag**: 161
- **Description**: The life event referenced in the query.

#### local_services_employee
- **Type**: `Option<LocalServicesEmployee>`
- **Tag**: 223
- **Description**: The local services employee referenced in the query.

#### local_services_lead
- **Type**: `Option<LocalServicesLead>`
- **Tag**: 210
- **Description**: The local services lead referenced in the query.

#### local_services_lead_conversation
- **Type**: `Option<LocalServicesLeadConversation>`
- **Tag**: 214
- **Description**: The local services lead conversation referenced in the query.

#### local_services_verification_artifact
- **Type**: `Option<LocalServicesVerificationArtifact>`
- **Tag**: 211
- **Description**: The local services verification artifact referenced in the query.

#### location_view
- **Type**: `Option<LocationView>`
- **Tag**: 123
- **Description**: The location view referenced in the query.

### M

#### managed_placement_view
- **Type**: `Option<ManagedPlacementView>`
- **Tag**: 53
- **Description**: The managed placement view referenced in the query.

#### media_file
- **Type**: `Option<MediaFile>`
- **Tag**: 90
- **Description**: The media file referenced in the query.

#### metrics
- **Type**: `Option<Metrics>`
- **Tag**: 4
- **Description**: The metrics. See [Metrics Fields](#2-metrics-fields) section.

#### mobile_app_category_constant
- **Type**: `Option<MobileAppCategoryConstant>`
- **Tag**: 87
- **Description**: The mobile app category constant referenced in the query.

#### mobile_device_constant
- **Type**: `Option<MobileDeviceConstant>`
- **Tag**: 98
- **Description**: The mobile device constant referenced in the query.

### O

#### offline_conversion_upload_client_summary
- **Type**: `Option<OfflineConversionUploadClientSummary>`
- **Tag**: 216
- **Description**: Offline conversion upload summary at customer level.

#### offline_conversion_upload_conversion_action_summary
- **Type**: `Option<OfflineConversionUploadConversionActionSummary>`
- **Tag**: 228
- **Description**: Offline conversion upload summary at conversion type level.

#### offline_user_data_job
- **Type**: `Option<OfflineUserDataJob>`
- **Tag**: 137
- **Description**: The offline user data job referenced in the query.

#### operating_system_version_constant
- **Type**: `Option<OperatingSystemVersionConstant>`
- **Tag**: 86
- **Description**: The operating system version constant referenced in the query.

### P

#### paid_organic_search_term_view
- **Type**: `Option<PaidOrganicSearchTermView>`
- **Tag**: 129
- **Description**: The paid organic search term view referenced in the query.

#### parental_status_view
- **Type**: `Option<ParentalStatusView>`
- **Tag**: 45
- **Description**: The parental status view referenced in the query.

#### per_store_view
- **Type**: `Option<PerStoreView>`
- **Tag**: 198
- **Description**: The per store view referenced in the query.

#### performance_max_placement_view
- **Type**: `Option<PerformanceMaxPlacementView>`
- **Tag**: 233
- **Description**: The performance max placement view referenced in the query.

#### product_category_constant
- **Type**: `Option<ProductCategoryConstant>`
- **Tag**: 208
- **Description**: The product category referenced in the query.

#### product_group_view
- **Type**: `Option<ProductGroupView>`
- **Tag**: 54
- **Description**: The product group view referenced in the query.

#### product_link
- **Type**: `Option<ProductLink>`
- **Tag**: 194
- **Description**: The product link referenced in the query.

#### product_link_invitation
- **Type**: `Option<ProductLinkInvitation>`
- **Tag**: 209
- **Description**: The product link invitation in the query.

### Q

#### qualifying_question
- **Type**: `Option<QualifyingQuestion>`
- **Tag**: 202
- **Description**: The qualifying question referenced in the query.

### R

#### recommendation
- **Type**: `Option<Recommendation>`
- **Tag**: 22
- **Description**: The recommendation referenced in the query.

#### recommendation_subscription
- **Type**: `Option<RecommendationSubscription>`
- **Tag**: 220
- **Description**: The recommendation subscription referenced in the query.

#### remarketing_action
- **Type**: `Option<RemarketingAction>`
- **Tag**: 60
- **Description**: The remarketing action referenced in the query.

### S

#### search_term_view
- **Type**: `Option<SearchTermView>`
- **Tag**: 68
- **Description**: The search term view referenced in the query.

#### segments
- **Type**: `Option<Segments>`
- **Tag**: 102
- **Description**: The segments. See [Segments Fields](#3-segments-fields) section.

#### shared_criterion
- **Type**: `Option<SharedCriterion>`
- **Tag**: 29
- **Description**: The shared criterion referenced in the query.

#### shared_set
- **Type**: `Option<SharedSet>`
- **Tag**: 27
- **Description**: The shared set referenced in the query.

#### shopping_performance_view
- **Type**: `Option<ShoppingPerformanceView>`
- **Tag**: 117
- **Description**: The shopping performance view referenced in the query.

#### shopping_product
- **Type**: `Option<ShoppingProduct>`
- **Tag**: 226
- **Description**: The shopping product referenced in the query.

#### smart_campaign_search_term_view
- **Type**: `Option<SmartCampaignSearchTermView>`
- **Tag**: 170
- **Description**: The Smart campaign search term view referenced in the query.

#### smart_campaign_setting
- **Type**: `Option<SmartCampaignSetting>`
- **Tag**: 167
- **Description**: The Smart campaign setting referenced in the query.

### T

#### third_party_app_analytics_link
- **Type**: `Option<ThirdPartyAppAnalyticsLink>`
- **Tag**: 144
- **Description**: The AccountLink referenced in the query.

#### topic_constant
- **Type**: `Option<TopicConstant>`
- **Tag**: 31
- **Description**: The topic constant referenced in the query.

#### topic_view
- **Type**: `Option<TopicView>`
- **Tag**: 44
- **Description**: The topic view referenced in the query.

#### travel_activity_group_view
- **Type**: `Option<TravelActivityGroupView>`
- **Tag**: 201
- **Description**: The travel activity group view referenced in the query.

#### travel_activity_performance_view
- **Type**: `Option<TravelActivityPerformanceView>`
- **Tag**: 200
- **Description**: The travel activity performance view referenced in the query.

### U

#### user_interest
- **Type**: `Option<UserInterest>`
- **Tag**: 59
- **Description**: The user interest referenced in the query.

#### user_list
- **Type**: `Option<UserList>`
- **Tag**: 38
- **Description**: The user list referenced in the query.

#### user_list_customer_type
- **Type**: `Option<UserListCustomerType>`
- **Tag**: 225
- **Description**: The user list customer type in the query.

#### user_location_view
- **Type**: `Option<UserLocationView>`
- **Tag**: 135
- **Description**: The user location view referenced in the query.

### V

#### video
- **Type**: `Option<Video>`
- **Tag**: 39
- **Description**: The video referenced in the query.

### W

#### webpage_view
- **Type**: `Option<WebpageView>`
- **Tag**: 162
- **Description**: The webpage view referenced in the query.

---

## 2. Metrics Fields

The Metrics struct contains 283 performance metric fields. These are accessed via `row.metrics.field_name`.

#### absolute_top_impression_percentage
- **Type**: `f64`
- **Tag**: 183
- **Description**: Search absolute top impression share is the percentage of your Search ad impressions that are shown in the most prominent Search position.

#### active_view_cpm
- **Type**: `f64`
- **Tag**: 184
- **Description**: Average cost of viewable impressions (`active_view_impressions`).

#### active_view_ctr
- **Type**: `f64`
- **Tag**: 185
- **Description**: Active view measurable clicks divided by active view viewable impressions. This metric is reported only for the Display Network.

#### active_view_impressions
- **Type**: `i64`
- **Tag**: 186
- **Description**: A measurement of how often your ad has become viewable on a Display Network site.

#### active_view_measurability
- **Type**: `f64`
- **Tag**: 187
- **Description**: The ratio of impressions that could be measured by Active View over the number of served impressions.

#### active_view_measurable_cost_micros
- **Type**: `i64`
- **Tag**: 188
- **Description**: The cost of the impressions you received that were measurable by Active View.

#### active_view_measurable_impressions
- **Type**: `i64`
- **Tag**: 189
- **Description**: The number of times your ads are appearing on placements in positions where they can be seen.

#### active_view_viewability
- **Type**: `f64`
- **Tag**: 190
- **Description**: The percentage of time when your ad appeared on an Active View enabled site (measurable impressions) and was viewable (viewable impressions).

#### all_conversions
- **Type**: `f64`
- **Tag**: 193
- **Description**: The total number of conversions. This includes all conversions regardless of the value of include_in_conversions_metric.

#### all_conversions_by_conversion_date
- **Type**: `f64`
- **Tag**: 241
- **Description**: The total number of conversions. When this column is selected with date, the values in date column means the conversion date.

#### all_conversions_from_click_to_call
- **Type**: `f64`
- **Tag**: 195
- **Description**: The number of times people clicked the "Call" button to call a store during or after clicking an ad. This metric applies to feed items only.

#### all_conversions_from_directions
- **Type**: `f64`
- **Tag**: 196
- **Description**: The number of times people clicked a "Get directions" button to navigate to a store after clicking an ad. This metric applies to feed items only.

#### all_conversions_from_interactions_rate
- **Type**: `f64`
- **Tag**: 191
- **Description**: All conversions from interactions (as opposed to view through conversions) divided by the number of ad interactions.

#### all_conversions_from_interactions_value_per_interaction
- **Type**: `f64`
- **Tag**: 197
- **Description**: The value of all conversions from interactions divided by the total number of interactions.

#### all_conversions_from_location_asset_click_to_call
- **Type**: `f64`
- **Tag**: 267
- **Description**: Number of call button clicks on any location surface. From Asset based location.

#### all_conversions_from_location_asset_directions
- **Type**: `f64`
- **Tag**: 268
- **Description**: Number of driving directions clicks. From Asset based location.

#### all_conversions_from_location_asset_menu
- **Type**: `f64`
- **Tag**: 269
- **Description**: Number of menu link clicks. From Asset based location.

#### all_conversions_from_location_asset_order
- **Type**: `f64`
- **Tag**: 270
- **Description**: Number of order clicks. From Asset based location.

#### all_conversions_from_location_asset_other_engagement
- **Type**: `f64`
- **Tag**: 271
- **Description**: Number of other types of local action clicks. From Asset based location.

#### all_conversions_from_location_asset_store_visits
- **Type**: `f64`
- **Tag**: 272
- **Description**: Estimated number of visits to the store. From Asset based location.

#### all_conversions_from_location_asset_website
- **Type**: `f64`
- **Tag**: 273
- **Description**: Number of website URL clicks. From Asset based location.

#### all_conversions_from_menu
- **Type**: `f64`
- **Tag**: 198
- **Description**: The number of times people clicked a link to view a store's menu after clicking an ad. This metric applies to feed items only.

#### all_conversions_from_order
- **Type**: `f64`
- **Tag**: 199
- **Description**: The number of times people placed an order at a store after clicking an ad. This metric applies to feed items only.

#### all_conversions_from_other_engagement
- **Type**: `f64`
- **Tag**: 200
- **Description**: The number of other conversions (for example, posting a review or saving a location for a store) that occurred after people clicked an ad. This metric applies to feed items only.

#### all_conversions_from_store_visit
- **Type**: `f64`
- **Tag**: 201
- **Description**: Estimated number of times people visited a store after clicking an ad. This metric applies to feed items only.

#### all_conversions_from_store_website
- **Type**: `f64`
- **Tag**: 202
- **Description**: The number of times that people were taken to a store's URL after clicking an ad. This metric applies to feed items only.

#### all_conversions_value
- **Type**: `f64`
- **Tag**: 192
- **Description**: The value of all conversions.

#### all_conversions_value_by_conversion_date
- **Type**: `f64`
- **Tag**: 240
- **Description**: The value of all conversions. When this column is selected with date, the values in date column means the conversion date.

#### all_conversions_value_per_cost
- **Type**: `f64`
- **Tag**: 194
- **Description**: The value of all conversions divided by the total cost of ad interactions (such as clicks for text ads or views for video ads).

#### all_new_customer_lifetime_value
- **Type**: `f64`
- **Tag**: 294
- **Description**: All of new customers' lifetime conversion value. If you have set up customer acquisition goal at either account level or campaign level, this will include the additional conversion value from new customers for both biddable and non-biddable conversions.

#### asset_best_performance_cost_percentage
- **Type**: `f64`
- **Tag**: 359
- **Description**: Percentage of cost the asset received in ads with AssetPerformanceLabel.BEST.

#### asset_best_performance_impression_percentage
- **Type**: `f64`
- **Tag**: 354
- **Description**: Percentage of impressions the asset received in ads with AssetPerformanceLabel.BEST.

#### asset_good_performance_cost_percentage
- **Type**: `f64`
- **Tag**: 360
- **Description**: Percentage of cost the asset received in ads with AssetPerformanceLabel.GOOD.

#### asset_good_performance_impression_percentage
- **Type**: `f64`
- **Tag**: 355
- **Description**: Percentage of impressions the asset received in ads with AssetPerformanceLabel.GOOD.

#### asset_learning_performance_cost_percentage
- **Type**: `f64`
- **Tag**: 362
- **Description**: Percentage of cost the asset received in ads with AssetPerformanceLabel.LEARNING.

#### asset_learning_performance_impression_percentage
- **Type**: `f64`
- **Tag**: 357
- **Description**: Percentage of impressions the asset received in ads with AssetPerformanceLabel.LEARNING.

#### asset_low_performance_cost_percentage
- **Type**: `f64`
- **Tag**: 361
- **Description**: Percentage of cost the asset received in ads with AssetPerformanceLabel.LOW.

#### asset_low_performance_impression_percentage
- **Type**: `f64`
- **Tag**: 356
- **Description**: Percentage of impressions the asset received in ads with AssetPerformanceLabel.LOW.

#### asset_pinned_as_description_position_one_count
- **Type**: `i64`
- **Tag**: 352
- **Description**: Number of entities in which the asset is pinned to description 1.

#### asset_pinned_as_description_position_two_count
- **Type**: `i64`
- **Tag**: 353
- **Description**: Number of entities in which the asset is pinned to description 2.

#### asset_pinned_as_headline_position_one_count
- **Type**: `i64`
- **Tag**: 349
- **Description**: Number of entities in which the asset is pinned to headline 1.

#### asset_pinned_as_headline_position_three_count
- **Type**: `i64`
- **Tag**: 351
- **Description**: Number of entities in which the asset is pinned to headline 3.

#### asset_pinned_as_headline_position_two_count
- **Type**: `i64`
- **Tag**: 350
- **Description**: Number of entities in which the asset is pinned to headline 2.

#### asset_pinned_total_count
- **Type**: `i64`
- **Tag**: 348
- **Description**: Number of total usages in which the asset is pinned.

#### asset_unrated_performance_cost_percentage
- **Type**: `f64`
- **Tag**: 363
- **Description**: Percentage of cost the asset received in ads with other performance labels.

#### asset_unrated_performance_impression_percentage
- **Type**: `f64`
- **Tag**: 358
- **Description**: Percentage of impressions the asset received in ads with other performance labels.

#### auction_insight_search_absolute_top_impression_percentage
- **Type**: `f64`
- **Tag**: 258
- **Description**: This metric is part of the Auction Insights report, and tells how often the ads of another participant showed in the most prominent position on the search results page. This metric is not publicly available.

#### auction_insight_search_impression_share
- **Type**: `f64`
- **Tag**: 259
- **Description**: This metric is part of the Auction Insights report, and tells the percentage of impressions that another participant obtained, over the total number of impressions that your ads were eligible for. This metric is not publicly available.

#### auction_insight_search_outranking_share
- **Type**: `f64`
- **Tag**: 260
- **Description**: This metric is part of the Auction Insights report, and tells the percentage of impressions that your ads outranked (showed above) another participant in the auction. This metric is not publicly available.

#### auction_insight_search_overlap_rate
- **Type**: `f64`
- **Tag**: 261
- **Description**: This metric is part of the Auction Insights report, and tells how often another participant's ad received an impression when your ad also received an impression. This metric is not publicly available.

#### auction_insight_search_position_above_rate
- **Type**: `f64`
- **Tag**: 262
- **Description**: This metric is part of the Auction Insights report, and tells how often another participant's ad was shown in a higher position than yours. This metric is not publicly available.

#### auction_insight_search_top_impression_percentage
- **Type**: `f64`
- **Tag**: 263
- **Description**: This metric is part of the Auction Insights report, and tells how often the ads of another participant showed adjacent to the top organic search results. This metric is not publicly available.

#### average_cart_size
- **Type**: `f64`
- **Tag**: 298
- **Description**: Average cart size is the average number of products in each order attributed to your ads.

#### average_cost
- **Type**: `f64`
- **Tag**: 203
- **Description**: The average amount you pay per interaction. This amount is the total cost of your ads divided by the total number of interactions.

#### average_cpc
- **Type**: `f64`
- **Tag**: 204
- **Description**: The total cost of all clicks divided by the total number of clicks received.

#### average_cpe
- **Type**: `f64`
- **Tag**: 205
- **Description**: The average amount that you've been charged for an ad engagement. This amount is the total cost of all ad engagements divided by the total number of ad engagements.

#### average_cpm
- **Type**: `f64`
- **Tag**: 206
- **Description**: Average cost-per-thousand impressions (CPM).

#### average_cpv
- **Type**: `f64`
- **Tag**: 207
- **Description**: The average amount you pay each time someone views your ad. The average CPV is defined by the total cost of all ad views divided by the number of views.

#### average_impression_frequency_per_user
- **Type**: `f64`
- **Tag**: 320
- **Description**: The average number of times a unique user saw your ad during the requested time period.

#### average_order_value_micros
- **Type**: `i64`
- **Tag**: 297
- **Description**: Average order value is the average revenue you made per order attributed to your ads.

#### average_page_views
- **Type**: `f64`
- **Tag**: 208
- **Description**: Average number of pages viewed per session.

#### average_target_cpa_micros
- **Type**: `i64`
- **Tag**: 290
- **Description**: The average Target CPA.

#### average_target_roas
- **Type**: `f64`
- **Tag**: 250
- **Description**: The average Target ROAS.

#### average_time_on_site
- **Type**: `f64`
- **Tag**: 209
- **Description**: Total duration of all sessions (in seconds) / number of sessions. Imported from Google Analytics.

#### benchmark_average_max_cpc
- **Type**: `f64`
- **Tag**: 210
- **Description**: An indication of how other advertisers are bidding on similar products.

#### benchmark_ctr
- **Type**: `f64`
- **Tag**: 211
- **Description**: An indication on how other advertisers' Shopping ads for similar products are performing based on how often people who see their ad click on it.

#### biddable_app_install_conversions
- **Type**: `f64`
- **Tag**: 254
- **Description**: Number of app installs.

#### biddable_app_post_install_conversions
- **Type**: `f64`
- **Tag**: 255
- **Description**: Number of in-app actions.

#### bounce_rate
- **Type**: `f64`
- **Tag**: 212
- **Description**: Percentage of clicks where the user only visited a single page on your site. Imported from Google Analytics.

#### clicks
- **Type**: `i64`
- **Tag**: 131
- **Description**: The number of clicks.

#### combined_clicks
- **Type**: `i64`
- **Tag**: 156
- **Description**: The number of times your ad or your site's listing in the unpaid results was clicked.

#### combined_clicks_per_query
- **Type**: `f64`
- **Tag**: 157
- **Description**: The number of times your ad or your site's listing in the unpaid results was clicked (combined_clicks) divided by combined_queries.

#### combined_queries
- **Type**: `i64`
- **Tag**: 158
- **Description**: The number of searches that returned pages from your site in the unpaid results or showed one of your text ads.

#### content_budget_lost_impression_share
- **Type**: `f64`
- **Tag**: 159
- **Description**: The estimated percent of times that your ad was eligible to show on the Display Network but didn't because your budget was too low.

#### content_impression_share
- **Type**: `f64`
- **Tag**: 160
- **Description**: The impressions you've received on the Display Network divided by the estimated number of impressions you were eligible to receive.

#### content_rank_lost_impression_share
- **Type**: `f64`
- **Tag**: 163
- **Description**: The estimated percentage of impressions on the Display Network that your ads didn't receive due to poor Ad Rank.

#### conversion_last_conversion_date
- **Type**: `String`
- **Tag**: 162
- **Description**: The date of the most recent conversion for this conversion action. The date is in the customer's time zone.

#### conversion_last_received_request_date_time
- **Type**: `String`
- **Tag**: 161
- **Description**: The last date/time a conversion tag for this conversion action successfully fired and was seen by Google Ads.

#### conversions
- **Type**: `f64`
- **Tag**: 168
- **Description**: The number of conversions. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### conversions_by_conversion_date
- **Type**: `f64`
- **Tag**: 243
- **Description**: The number of conversions by conversion date. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### conversions_from_interactions_rate
- **Type**: `f64`
- **Tag**: 164
- **Description**: Conversions from interactions divided by the number of ad interactions. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### conversions_from_interactions_value_per_interaction
- **Type**: `f64`
- **Tag**: 167
- **Description**: The value of conversions from interactions divided by the number of ad interactions. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### conversions_value
- **Type**: `f64`
- **Tag**: 165
- **Description**: The value of conversions. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### conversions_value_by_conversion_date
- **Type**: `f64`
- **Tag**: 242
- **Description**: The value of conversions by conversion date. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### conversions_value_per_cost
- **Type**: `f64`
- **Tag**: 166
- **Description**: The value of conversions divided by the cost of ad interactions. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### cost_micros
- **Type**: `i64`
- **Tag**: 169
- **Description**: The sum of your cost-per-click (CPC) and cost-per-thousand impressions (CPM) costs during this period.

#### cost_of_goods_sold_micros
- **Type**: `i64`
- **Tag**: 299
- **Description**: Cost of goods sold (COGS) is the total cost of the products you sold in orders attributed to your ads.

#### cost_per_all_conversions
- **Type**: `f64`
- **Tag**: 170
- **Description**: The cost of ad interactions divided by all conversions.

#### cost_per_conversion
- **Type**: `f64`
- **Tag**: 171
- **Description**: The cost of ad interactions divided by conversions. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### cost_per_current_model_attributed_conversion
- **Type**: `f64`
- **Tag**: 172
- **Description**: The cost of ad interactions divided by current model attributed conversions. This only includes conversion actions which include_in_conversions_metric attribute is set to true.

#### cross_device_conversions
- **Type**: `f64`
- **Tag**: 173
- **Description**: Conversions from when a customer clicks on a Google Ads ad on one device, then converts on a different device or browser. Cross-device conversions are already included in all_conversions.

#### cross_device_conversions_value_micros
- **Type**: `i64`
- **Tag**: 312
- **Description**: The sum of the value of cross-device conversions, in micros.

#### cross_sell_cost_of_goods_sold_micros
- **Type**: `i64`
- **Tag**: 304
- **Description**: Cross-sell cost of goods sold (COGS) is the total cost of products sold as a result of advertising a different product.

#### cross_sell_gross_profit_micros
- **Type**: `i64`
- **Tag**: 305
- **Description**: Cross-sell gross profit is the profit you made from products sold as a result of advertising a different product, minus cost of goods sold (COGS).

#### cross_sell_revenue_micros
- **Type**: `i64`
- **Tag**: 306
- **Description**: Cross-sell revenue is the total amount you made from products sold as a result of advertising a different product.

#### cross_sell_units_sold
- **Type**: `f64`
- **Tag**: 307
- **Description**: Cross-sell units sold is the total number of products sold as a result of advertising a different product.

#### ctr
- **Type**: `f64`
- **Tag**: 174
- **Description**: The number of clicks your ad receives (Clicks) divided by the number of times your ad is shown (Impressions).

#### current_model_attributed_conversions
- **Type**: `f64`
- **Tag**: 175
- **Description**: Shows how your historic conversions data would look under the attribution model you've currently selected.

#### current_model_attributed_conversions_from_interactions_rate
- **Type**: `f64`
- **Tag**: 176
- **Description**: Current model attributed conversions from interactions divided by the number of ad interactions.

#### current_model_attributed_conversions_from_interactions_value_per_interaction
- **Type**: `f64`
- **Tag**: 177
- **Description**: The value of current model attributed conversions from interactions divided by the number of ad interactions.

#### current_model_attributed_conversions_value
- **Type**: `f64`
- **Tag**: 178
- **Description**: The value of current model attributed conversions.

#### current_model_attributed_conversions_value_per_cost
- **Type**: `f64`
- **Tag**: 179
- **Description**: The value of current model attributed conversions divided by the cost of ad interactions.

#### eligible_impressions_from_location_asset_store_reach
- **Type**: `i64`
- **Tag**: 274
- **Description**: Number of impressions in which the store location was shown. From Asset based location.

#### engagement_rate
- **Type**: `f64`
- **Tag**: 180
- **Description**: How often people engage with your ad after it's shown to them.

#### engagements
- **Type**: `i64`
- **Tag**: 181
- **Description**: The number of engagements.

#### general_invalid_click_rate
- **Type**: `f64`
- **Tag**: 370
- **Description**: The percentage of clicks that have been filtered out due to being general invalid clicks.

#### general_invalid_clicks
- **Type**: `i64`
- **Tag**: 371
- **Description**: Number of general invalid clicks.

#### gmail_forwards
- **Type**: `i64`
- **Tag**: 217
- **Description**: The number of times the ad was forwarded to someone else as a message.

#### gmail_saves
- **Type**: `i64`
- **Tag**: 218
- **Description**: The number of times someone has saved your Gmail ad to their inbox as a message.

#### gmail_secondary_clicks
- **Type**: `i64`
- **Tag**: 219
- **Description**: The number of clicks to the landing page on the expanded state of Gmail ads.

#### gross_profit_margin
- **Type**: `f64`
- **Tag**: 301
- **Description**: Gross profit margin is the percentage gross profit you made from orders attributed to your ads, after taking out the cost of goods sold (COGS).

#### gross_profit_micros
- **Type**: `i64`
- **Tag**: 300
- **Description**: Gross profit is the profit you made from orders attributed to your ads minus the cost of goods sold (COGS).

#### historical_creative_quality_score
- **Type**: `i32` (enum)
- **Tag**: 80
- **Description**: The creative historical quality score.

#### historical_landing_page_quality_score
- **Type**: `i32` (enum)
- **Tag**: 81
- **Description**: The quality of historical landing page experience.

#### historical_quality_score
- **Type**: `i64`
- **Tag**: 216
- **Description**: The historical quality score.

#### historical_search_predicted_ctr
- **Type**: `i32` (enum)
- **Tag**: 83
- **Description**: The historical search predicted click through rate (CTR).

#### hotel_average_lead_value_micros
- **Type**: `f64`
- **Tag**: 213
- **Description**: Average lead value based on clicks.

#### hotel_commission_rate_micros
- **Type**: `i64`
- **Tag**: 256
- **Description**: Commission bid rate in micros. A 20% commission is represented as 200,000.

#### hotel_eligible_impressions
- **Type**: `i64`
- **Tag**: 215
- **Description**: The number of impressions that hotel partners could have had given their feed performance.

#### hotel_expected_commission_cost
- **Type**: `f64`
- **Tag**: 257
- **Description**: Expected commission cost.

#### hotel_price_difference_percentage
- **Type**: `f64`
- **Tag**: 214
- **Description**: The average price difference between the price offered by reporting hotel advertiser and the cheapest price offered by the competing advertiser.

#### impressions
- **Type**: `i64`
- **Tag**: 221
- **Description**: Count of how often your ad has appeared on a search results page or website on the Google Network.

#### impressions_from_store_reach
- **Type**: `i64`
- **Tag**: 220
- **Description**: The number of times a store's location-based ad was shown. This metric applies to feed items only.

#### interaction_event_types
- **Type**: `Vec<i32>` (enum)
- **Tag**: 100
- **Description**: The types of payable and free interactions.

#### interaction_rate
- **Type**: `f64`
- **Tag**: 222
- **Description**: How often people interact with your ad after it is shown to them.

#### interactions
- **Type**: `i64`
- **Tag**: 223
- **Description**: The number of interactions.

#### invalid_click_rate
- **Type**: `f64`
- **Tag**: 224
- **Description**: The percentage of clicks filtered out of your total number of clicks.

#### invalid_clicks
- **Type**: `i64`
- **Tag**: 225
- **Description**: Number of clicks Google considers illegitimate and doesn't charge you for.

#### lead_cost_of_goods_sold_micros
- **Type**: `i64`
- **Tag**: 308
- **Description**: Lead cost of goods sold (COGS) is the total cost of products sold as a result of advertising the same product.

#### lead_gross_profit_micros
- **Type**: `i64`
- **Tag**: 309
- **Description**: Lead gross profit is the profit you made from products sold as a result of advertising the same product, minus cost of goods sold (COGS).

#### lead_revenue_micros
- **Type**: `i64`
- **Tag**: 310
- **Description**: Lead revenue is the total amount you made from products sold as a result of advertising the same product.

#### lead_units_sold
- **Type**: `f64`
- **Tag**: 311
- **Description**: Lead units sold is the total number of products sold as a result of advertising the same product.

#### linked_entities_count
- **Type**: `i64`
- **Tag**: 341
- **Description**: Number of linked resources in which the asset is used.

#### linked_sample_entities
- **Type**: `Vec<String>`
- **Tag**: 342
- **Description**: A list of up to 20 sample linked resources in which the asset is used.

#### message_chat_rate
- **Type**: `f64`
- **Tag**: 228
- **Description**: Number of message chats initiated divided by the number of message impressions.

#### message_chats
- **Type**: `i64`
- **Tag**: 226
- **Description**: Number of message chats initiated for Click To Message impressions.

#### message_impressions
- **Type**: `i64`
- **Tag**: 227
- **Description**: Number of Click To Message impressions that were message tracking eligible.

#### mobile_friendly_clicks_percentage
- **Type**: `f64`
- **Tag**: 229
- **Description**: The percentage of mobile clicks that go to a mobile-friendly page.

#### new_customer_lifetime_value
- **Type**: `f64`
- **Tag**: 293
- **Description**: New customers' lifetime conversion value. If you have set up customer acquisition goal at either account level or campaign level, this will include the additional conversion value from new customers for biddable conversions.

#### optimization_score_uplift
- **Type**: `f64`
- **Tag**: 247
- **Description**: Total optimization score uplift of all recommendations.

#### optimization_score_url
- **Type**: `String`
- **Tag**: 248
- **Description**: URL for the optimization score page in the Google Ads web interface.

#### orders
- **Type**: `f64`
- **Tag**: 296
- **Description**: Orders is the total number of purchase conversions you received attributed to your ads.

#### organic_clicks
- **Type**: `i64`
- **Tag**: 230
- **Description**: The number of times someone clicked your site's listing in the unpaid results.

#### organic_clicks_per_query
- **Type**: `f64`
- **Tag**: 231
- **Description**: The number of organic clicks divided by organic queries.

#### organic_impressions
- **Type**: `i64`
- **Tag**: 232
- **Description**: The number of listings for your site in the unpaid search results.

#### organic_impressions_per_query
- **Type**: `f64`
- **Tag**: 233
- **Description**: Organic impressions divided by organic queries.

#### organic_queries
- **Type**: `i64`
- **Tag**: 234
- **Description**: The total number of searches that returned your site's listing in the unpaid results.

#### percent_new_visitors
- **Type**: `f64`
- **Tag**: 235
- **Description**: Percentage of first-time sessions.

#### phone_calls
- **Type**: `i64`
- **Tag**: 236
- **Description**: Number of offline phone calls.

#### phone_impressions
- **Type**: `i64`
- **Tag**: 237
- **Description**: Number of offline phone impressions.

#### phone_through_rate
- **Type**: `f64`
- **Tag**: 238
- **Description**: Number of phone calls received divided by the number of times your phone number is shown.

#### publisher_organic_clicks
- **Type**: `i64`
- **Tag**: 265
- **Description**: Clicks from properties for which the traffic the publisher has not paid for.

#### publisher_purchased_clicks
- **Type**: `i64`
- **Tag**: 264
- **Description**: Clicks from properties not owned by the publisher.

#### publisher_unknown_clicks
- **Type**: `i64`
- **Tag**: 266
- **Description**: Clicks from traffic which is not identified as "Publisher Purchased" or "Publisher Organic".

#### relative_ctr
- **Type**: `f64`
- **Tag**: 239
- **Description**: Your clickthrough rate divided by the average clickthrough rate of all advertisers.

#### results_conversions_purchase
- **Type**: `f64`
- **Tag**: 366
- **Description**: The purchase conversion stats for the unified goals results.

#### revenue_micros
- **Type**: `i64`
- **Tag**: 302
- **Description**: Revenue is the total amount you made from orders attributed to your ads.

#### sample_best_performance_entities
- **Type**: `Vec<String>`
- **Tag**: 343
- **Description**: A list of up to 20 sample linked resources with BEST performance label.

#### sample_good_performance_entities
- **Type**: `Vec<String>`
- **Tag**: 344
- **Description**: A list of up to 20 sample linked resources with GOOD performance label.

#### sample_learning_performance_entities
- **Type**: `Vec<String>`
- **Tag**: 346
- **Description**: A list of up to 20 sample linked resources with LEARNING performance label.

#### sample_low_performance_entities
- **Type**: `Vec<String>`
- **Tag**: 345
- **Description**: A list of up to 20 sample linked resources with LOW performance label.

#### sample_unrated_performance_entities
- **Type**: `Vec<String>`
- **Tag**: 347
- **Description**: A list of up to 20 sample linked resources with other performance labels.

#### search_absolute_top_impression_share
- **Type**: `f64`
- **Tag**: 136
- **Description**: The percentage of the customer's Shopping or Search ad impressions shown in the most prominent Shopping position.

#### search_budget_lost_absolute_top_impression_share
- **Type**: `f64`
- **Tag**: 137
- **Description**: The number estimating how often your ad wasn't the very first ad among the top ads due to a low budget.

#### search_budget_lost_impression_share
- **Type**: `f64`
- **Tag**: 138
- **Description**: The estimated percent of times that your ad was eligible to show on the Search Network but didn't because your budget was too low.

#### search_budget_lost_top_impression_share
- **Type**: `f64`
- **Tag**: 139
- **Description**: The number estimating how often your ad didn't show adjacent to the top organic search results due to a low budget.

#### search_click_share
- **Type**: `f64`
- **Tag**: 140
- **Description**: The number of clicks you've received on the Search Network divided by the estimated number of clicks you were eligible to receive.

#### search_exact_match_impression_share
- **Type**: `f64`
- **Tag**: 141
- **Description**: The impressions you've received divided by the estimated number of impressions for exact keyword matches.

#### search_impression_share
- **Type**: `f64`
- **Tag**: 142
- **Description**: The impressions you've received on the Search Network divided by the estimated number of impressions you were eligible to receive.

#### search_rank_lost_absolute_top_impression_share
- **Type**: `f64`
- **Tag**: 143
- **Description**: The number estimating how often your ad wasn't the very first ad among the top ads due to poor Ad Rank.

#### search_rank_lost_impression_share
- **Type**: `f64`
- **Tag**: 144
- **Description**: The estimated percentage of impressions on the Search Network that your ads didn't receive due to poor Ad Rank.

#### search_rank_lost_top_impression_share
- **Type**: `f64`
- **Tag**: 145
- **Description**: The number estimating how often your ad didn't show adjacent to the top organic search results due to poor Ad Rank.

#### search_top_impression_share
- **Type**: `f64`
- **Tag**: 146
- **Description**: The impressions you've received among the top ads compared to the estimated number of impressions you were eligible to receive among the top ads.

#### search_volume
- **Type**: `Option<SearchVolumeRange>`
- **Tag**: 295
- **Description**: Search volume range for a search term insight category.

#### sk_ad_network_installs
- **Type**: `i64`
- **Tag**: 246
- **Description**: The number of iOS Store Kit Ad Network conversions.

#### sk_ad_network_total_conversions
- **Type**: `i64`
- **Tag**: 292
- **Description**: The total number of iOS Store Kit Ad Network conversions.

#### speed_score
- **Type**: `i64`
- **Tag**: 147
- **Description**: A measure of how quickly your page loads after clicks on your mobile ads.

#### store_visits_last_click_model_attributed_conversions
- **Type**: `f64`
- **Tag**: 365
- **Description**: The amount of store visits attributed by the last click model.

#### top_impression_percentage
- **Type**: `f64`
- **Tag**: 148
- **Description**: The percent of your ad impressions that are shown adjacent to the top organic search results.

#### unique_users
- **Type**: `i64`
- **Tag**: 319
- **Description**: The number of unique users who saw your ad during the requested time period.

#### units_sold
- **Type**: `f64`
- **Tag**: 303
- **Description**: Units sold is the total number of products sold from orders attributed to your ads.

#### valid_accelerated_mobile_pages_clicks_percentage
- **Type**: `f64`
- **Tag**: 149
- **Description**: The percentage of ad clicks to AMP landing pages that reach a valid AMP page.

#### value_per_all_conversions
- **Type**: `f64`
- **Tag**: 150
- **Description**: The value of all conversions divided by the number of all conversions.

#### value_per_all_conversions_by_conversion_date
- **Type**: `f64`
- **Tag**: 244
- **Description**: The value of all conversions divided by the number of all conversions by conversion date.

#### value_per_conversion
- **Type**: `f64`
- **Tag**: 151
- **Description**: The value of conversions divided by the number of conversions.

#### value_per_conversions_by_conversion_date
- **Type**: `f64`
- **Tag**: 245
- **Description**: The value of conversions divided by the number of conversions by conversion date.

#### value_per_current_model_attributed_conversion
- **Type**: `f64`
- **Tag**: 152
- **Description**: The value of current model attributed conversions divided by the number of the conversions.

#### video_quartile_p100_rate
- **Type**: `f64`
- **Tag**: 132
- **Description**: Percentage of impressions where the viewer watched all of your video.

#### video_quartile_p25_rate
- **Type**: `f64`
- **Tag**: 133
- **Description**: Percentage of impressions where the viewer watched 25% of your video.

#### video_quartile_p50_rate
- **Type**: `f64`
- **Tag**: 134
- **Description**: Percentage of impressions where the viewer watched 50% of your video.

#### video_quartile_p75_rate
- **Type**: `f64`
- **Tag**: 135
- **Description**: Percentage of impressions where the viewer watched 75% of your video.

#### video_view_rate
- **Type**: `f64`
- **Tag**: 153
- **Description**: The number of views your TrueView video ad receives divided by its number of impressions.

#### video_view_rate_in_feed
- **Type**: `f64`
- **Tag**: 367
- **Description**: The number of video views divided by number of impressions for in-feed formats.

#### video_view_rate_in_stream
- **Type**: `f64`
- **Tag**: 368
- **Description**: The number of video views divided by number of impressions for in-stream formats.

#### video_view_rate_shorts
- **Type**: `f64`
- **Tag**: 369
- **Description**: The number of video views divided by number of impressions for in shorts formats.

#### video_views
- **Type**: `i64`
- **Tag**: 154
- **Description**: The number of times your video ads were viewed.

#### view_through_conversions
- **Type**: `i64`
- **Tag**: 155
- **Description**: The total number of view-through conversions.

#### view_through_conversions_from_location_asset_click_to_call
- **Type**: `f64`
- **Tag**: 275
- **Description**: Number of call button clicks after an impression. From Asset based location.

#### view_through_conversions_from_location_asset_directions
- **Type**: `f64`
- **Tag**: 276
- **Description**: Number of driving directions clicks after an impression. From Asset based location.

#### view_through_conversions_from_location_asset_menu
- **Type**: `f64`
- **Tag**: 277
- **Description**: Number of menu link clicks after an impression. From Asset based location.

#### view_through_conversions_from_location_asset_order
- **Type**: `f64`
- **Tag**: 278
- **Description**: Number of order clicks after an impression. From Asset based location.

#### view_through_conversions_from_location_asset_other_engagement
- **Type**: `f64`
- **Tag**: 279
- **Description**: Number of other types of local action clicks after an impression. From Asset based location.

#### view_through_conversions_from_location_asset_store_visits
- **Type**: `f64`
- **Tag**: 280
- **Description**: Estimated number of visits to the store after an impression. From Asset based location.

#### view_through_conversions_from_location_asset_website
- **Type**: `f64`
- **Tag**: 281
- **Description**: Number of website URL clicks after an impression. From Asset based location.

---

## 3. Segments Fields

The Segments struct contains 96 dimension fields used to segment and break down metrics. These are accessed via `row.segments.field_name`.

#### activity_account_id
- **Type**: `i64`
- **Tag**: 148
- **Description**: Activity account ID.

#### activity_city
- **Type**: `String`
- **Tag**: 185
- **Description**: The city where the travel activity is available.

#### activity_country
- **Type**: `String`
- **Tag**: 186
- **Description**: The country where the travel activity is available.

#### activity_rating
- **Type**: `i64`
- **Tag**: 149
- **Description**: Activity rating.

#### activity_state
- **Type**: `String`
- **Tag**: 187
- **Description**: The state where the travel activity is available.

#### ad_destination_type
- **Type**: `i32` (enum: AdDestinationType)
- **Tag**: 136
- **Description**: Ad Destination type.

#### ad_format_type
- **Type**: `i32` (enum: AdFormatType)
- **Tag**: 191
- **Description**: Ad Format type.

#### ad_group
- **Type**: `String`
- **Tag**: 158
- **Description**: Resource name of the ad group.

#### ad_network_type
- **Type**: `i32` (enum: AdNetworkType)
- **Tag**: 3
- **Description**: Ad network type.

#### asset_group
- **Type**: `String`
- **Tag**: 159
- **Description**: Resource name of the asset group.

#### asset_interaction_target
- **Type**: `Option<AssetInteractionTarget>`
- **Tag**: 139
- **Description**: Indicates whether the interaction metrics occurred on the asset itself or a different asset or ad unit.

#### auction_insight_domain
- **Type**: `String`
- **Tag**: 145
- **Description**: Domain (visible URL) of a participant in the Auction Insights report.

#### budget_campaign_association_status
- **Type**: `Option<BudgetCampaignAssociationStatus>`
- **Tag**: 134
- **Description**: Budget campaign association status.

#### campaign
- **Type**: `String`
- **Tag**: 157
- **Description**: Resource name of the campaign.

#### click_type
- **Type**: `i32` (enum: ClickType)
- **Tag**: 26
- **Description**: Click type.

#### conversion_action
- **Type**: `String`
- **Tag**: 113
- **Description**: Resource name of the conversion action.

#### conversion_action_category
- **Type**: `i32` (enum: ConversionActionCategory)
- **Tag**: 53
- **Description**: Conversion action category.

#### conversion_action_name
- **Type**: `String`
- **Tag**: 114
- **Description**: Conversion action name.

#### conversion_adjustment
- **Type**: `bool`
- **Tag**: 115
- **Description**: This segments your conversion columns by the original conversion and conversion value versus the delta if conversions were adjusted.

#### conversion_attribution_event_type
- **Type**: `i32` (enum: ConversionAttributionEventType)
- **Tag**: 2
- **Description**: Conversion attribution event type.

#### conversion_lag_bucket
- **Type**: `i32` (enum: ConversionLagBucket)
- **Tag**: 50
- **Description**: An enum value representing the number of days between the impression and the conversion.

#### conversion_or_adjustment_lag_bucket
- **Type**: `i32` (enum: ConversionOrAdjustmentLagBucket)
- **Tag**: 51
- **Description**: An enum value representing the number of days between the impression and the conversion or between the impression and adjustments to the conversion.

#### conversion_value_rule_primary_dimension
- **Type**: `i32` (enum: ConversionValueRulePrimaryDimension)
- **Tag**: 138
- **Description**: Primary dimension of applied conversion value rules.

#### date
- **Type**: `String`
- **Tag**: 79
- **Description**: Date to which metrics apply. yyyy-MM-dd format, for example, 2018-04-17.

#### day_of_week
- **Type**: `i32` (enum: DayOfWeek)
- **Tag**: 5
- **Description**: Day of the week, for example, MONDAY.

#### device
- **Type**: `i32` (enum: Device)
- **Tag**: 1
- **Description**: Device to which metrics apply.

#### external_activity_id
- **Type**: `String`
- **Tag**: 150
- **Description**: Advertiser supplied activity ID.

#### external_conversion_source
- **Type**: `i32` (enum: ExternalConversionSource)
- **Tag**: 55
- **Description**: External conversion source.

#### geo_target_airport
- **Type**: `String`
- **Tag**: 116
- **Description**: Resource name of the geo target constant that represents an airport.

#### geo_target_canton
- **Type**: `String`
- **Tag**: 117
- **Description**: Resource name of the geo target constant that represents a canton.

#### geo_target_city
- **Type**: `String`
- **Tag**: 118
- **Description**: Resource name of the geo target constant that represents a city.

#### geo_target_country
- **Type**: `String`
- **Tag**: 119
- **Description**: Resource name of the geo target constant that represents a country.

#### geo_target_county
- **Type**: `String`
- **Tag**: 120
- **Description**: Resource name of the geo target constant that represents a county.

#### geo_target_district
- **Type**: `String`
- **Tag**: 121
- **Description**: Resource name of the geo target constant that represents a district.

#### geo_target_metro
- **Type**: `String`
- **Tag**: 122
- **Description**: Resource name of the geo target constant that represents a metro.

#### geo_target_most_specific_location
- **Type**: `String`
- **Tag**: 123
- **Description**: Resource name of the geo target constant that represents the most specific location.

#### geo_target_postal_code
- **Type**: `String`
- **Tag**: 124
- **Description**: Resource name of the geo target constant that represents a postal code.

#### geo_target_province
- **Type**: `String`
- **Tag**: 125
- **Description**: Resource name of the geo target constant that represents a province.

#### geo_target_region
- **Type**: `String`
- **Tag**: 126
- **Description**: Resource name of the geo target constant that represents a region.

#### geo_target_state
- **Type**: `String`
- **Tag**: 127
- **Description**: Resource name of the geo target constant that represents a state.

#### hotel_booking_window_days
- **Type**: `i64`
- **Tag**: 135
- **Description**: Hotel booking window in days.

#### hotel_center_id
- **Type**: `i64`
- **Tag**: 80
- **Description**: Hotel center ID.

#### hotel_check_in_date
- **Type**: `String`
- **Tag**: 81
- **Description**: Hotel check-in date. Formatted as yyyy-MM-dd.

#### hotel_check_in_day_of_week
- **Type**: `i32` (enum: DayOfWeek)
- **Tag**: 9
- **Description**: Hotel check-in day of week.

#### hotel_city
- **Type**: `String`
- **Tag**: 82
- **Description**: Hotel city.

#### hotel_class
- **Type**: `i32`
- **Tag**: 83
- **Description**: Hotel class.

#### hotel_country
- **Type**: `String`
- **Tag**: 84
- **Description**: Hotel country.

#### hotel_date_selection_type
- **Type**: `i32` (enum: HotelDateSelectionType)
- **Tag**: 13
- **Description**: Hotel date selection type.

#### hotel_length_of_stay
- **Type**: `i32`
- **Tag**: 85
- **Description**: Hotel length of stay.

#### hotel_price_bucket
- **Type**: `i32` (enum: HotelPriceBucket)
- **Tag**: 78
- **Description**: Hotel price bucket.

#### hotel_rate_rule_id
- **Type**: `String`
- **Tag**: 86
- **Description**: Hotel rate rule ID.

#### hotel_rate_type
- **Type**: `i32` (enum: HotelRateType)
- **Tag**: 74
- **Description**: Hotel rate type.

#### hotel_state
- **Type**: `String`
- **Tag**: 87
- **Description**: Hotel state.

#### hour
- **Type**: `i32`
- **Tag**: 88
- **Description**: Hour of day as a number between 0 and 23, inclusive.

#### interaction_on_this_extension
- **Type**: `bool`
- **Tag**: 89
- **Description**: Only used with feed item metrics. Indicates whether the interaction metrics occurred on the feed item itself or a different extension or ad unit.

#### keyword
- **Type**: `Option<Keyword>`
- **Tag**: 61
- **Description**: Keyword criterion.

#### month
- **Type**: `String`
- **Tag**: 90
- **Description**: Month as represented by the date of the first day of a month. Formatted as yyyy-MM-dd.

#### month_of_year
- **Type**: `i32` (enum: MonthOfYear)
- **Tag**: 18
- **Description**: Month of the year, for example, January.

#### new_versus_returning_customers
- **Type**: `i32` (enum: ConvertingUserPriorEngagementTypeAndLtvBucket)
- **Tag**: 160
- **Description**: This is for segmenting conversions by whether the user is a new customer or a returning customer.

#### partner_hotel_id
- **Type**: `String`
- **Tag**: 91
- **Description**: Partner hotel ID.

#### product_aggregator_id
- **Type**: `i64`
- **Tag**: 132
- **Description**: Aggregator ID of the product.

#### product_brand
- **Type**: `String`
- **Tag**: 97
- **Description**: Brand of the product.

#### product_category_level1
- **Type**: `String`
- **Tag**: 161
- **Description**: Category (level 1) of the product.

#### product_category_level2
- **Type**: `String`
- **Tag**: 162
- **Description**: Category (level 2) of the product.

#### product_category_level3
- **Type**: `String`
- **Tag**: 163
- **Description**: Category (level 3) of the product.

#### product_category_level4
- **Type**: `String`
- **Tag**: 164
- **Description**: Category (level 4) of the product.

#### product_category_level5
- **Type**: `String`
- **Tag**: 165
- **Description**: Category (level 5) of the product.

#### product_channel
- **Type**: `i32` (enum: ProductChannel)
- **Tag**: 30
- **Description**: Channel of the product.

#### product_channel_exclusivity
- **Type**: `i32` (enum: ProductChannelExclusivity)
- **Tag**: 31
- **Description**: Channel exclusivity of the product.

#### product_condition
- **Type**: `i32` (enum: ProductCondition)
- **Tag**: 32
- **Description**: Condition of the product.

#### product_country
- **Type**: `String`
- **Tag**: 98
- **Description**: Resource name of the geo target constant for the country of sale of the product.

#### product_custom_attribute0
- **Type**: `String`
- **Tag**: 99
- **Description**: Custom attribute 0 of the product.

#### product_custom_attribute1
- **Type**: `String`
- **Tag**: 100
- **Description**: Custom attribute 1 of the product.

#### product_custom_attribute2
- **Type**: `String`
- **Tag**: 101
- **Description**: Custom attribute 2 of the product.

#### product_custom_attribute3
- **Type**: `String`
- **Tag**: 102
- **Description**: Custom attribute 3 of the product.

#### product_custom_attribute4
- **Type**: `String`
- **Tag**: 103
- **Description**: Custom attribute 4 of the product.

#### product_feed_label
- **Type**: `String`
- **Tag**: 147
- **Description**: Feed label of the product.

#### product_item_id
- **Type**: `String`
- **Tag**: 104
- **Description**: Item ID of the product.

#### product_language
- **Type**: `String`
- **Tag**: 105
- **Description**: Resource name of the language constant for the language of the product.

#### product_merchant_id
- **Type**: `i64`
- **Tag**: 133
- **Description**: Merchant ID of the product.

#### product_store_id
- **Type**: `String`
- **Tag**: 106
- **Description**: Store ID of the product.

#### product_title
- **Type**: `String`
- **Tag**: 107
- **Description**: Title of the product.

#### product_type_l1
- **Type**: `String`
- **Tag**: 108
- **Description**: Type (level 1) of the product.

#### product_type_l2
- **Type**: `String`
- **Tag**: 109
- **Description**: Type (level 2) of the product.

#### product_type_l3
- **Type**: `String`
- **Tag**: 110
- **Description**: Type (level 3) of the product.

#### product_type_l4
- **Type**: `String`
- **Tag**: 111
- **Description**: Type (level 4) of the product.

#### product_type_l5
- **Type**: `String`
- **Tag**: 112
- **Description**: Type (level 5) of the product.

#### quarter
- **Type**: `String`
- **Tag**: 128
- **Description**: Quarter as represented by the date of the first day of a quarter. Uses the calendar year for quarters. Formatted as yyyy-MM-dd.

#### recommendation_type
- **Type**: `i32` (enum: RecommendationType)
- **Tag**: 140
- **Description**: Recommendation type.

#### search_engine_results_page_type
- **Type**: `i32` (enum: SearchEngineResultsPageType)
- **Tag**: 70
- **Description**: Type of the search engine results page.

#### search_subcategory
- **Type**: `String`
- **Tag**: 155
- **Description**: A search term subcategory. An empty string denotes the catch-all subcategory for search terms that didn't fit into another subcategory.

#### search_term
- **Type**: `String`
- **Tag**: 156
- **Description**: A search term.

#### search_term_match_type
- **Type**: `i32` (enum: SearchTermMatchType)
- **Tag**: 22
- **Description**: Match type of the keyword that triggered the ad, including variants.

#### sk_ad_network_ad_event_type
- **Type**: `i32` (enum: SkAdNetworkAdEventType)
- **Tag**: 142
- **Description**: iOS Store Kit Ad Network ad event type.

#### sk_ad_network_attribution_credit
- **Type**: `i32` (enum: SkAdNetworkAttributionCredit)
- **Tag**: 144
- **Description**: iOS Store Kit Ad Network attribution credit.

#### sk_ad_network_coarse_conversion_value
- **Type**: `i32` (enum: SkAdNetworkCoarseConversionValue)
- **Tag**: 151
- **Description**: iOS Store Kit Ad Network coarse conversion value.

#### sk_ad_network_fine_conversion_value
- **Type**: `i64`
- **Tag**: 137
- **Description**: iOS Store Kit Ad Network conversion value. Null value means this segment is not applicable, for example, non-iOS campaign.

#### sk_ad_network_postback_sequence_index
- **Type**: `i64`
- **Tag**: 154
- **Description**: iOS Store Kit Ad Network postback sequence index.

#### sk_ad_network_redistributed_fine_conversion_value
- **Type**: `i64`
- **Tag**: 190
- **Description**: iOS Store Kit Ad Network redistributed fine conversion value. Google uses modeling on observed conversion values to calculate conversions from SKAN postbacks where NULLs are returned.

#### sk_ad_network_source_app
- **Type**: `Option<SkAdNetworkSourceApp>`
- **Tag**: 143
- **Description**: App where the ad that drove the iOS Store Kit Ad Network install was shown. Null value means this segment is not applicable or was not present in any postbacks sent by Apple.

#### sk_ad_network_source_domain
- **Type**: `String`
- **Tag**: 152
- **Description**: Website where the ad that drove the iOS Store Kit Ad Network install was shown.

#### sk_ad_network_source_type
- **Type**: `i32` (enum: SkAdNetworkSourceType)
- **Tag**: 153
- **Description**: The source type where the ad that drove the iOS Store Kit Ad Network install was shown.

#### sk_ad_network_user_type
- **Type**: `i32` (enum: SkAdNetworkUserType)
- **Tag**: 141
- **Description**: iOS Store Kit Ad Network user type.

#### sk_ad_network_version
- **Type**: `String`
- **Tag**: 192
- **Description**: The version of the SKAdNetwork API used.

#### slot
- **Type**: `i32` (enum: Slot)
- **Tag**: 23
- **Description**: Position of the ad.

#### travel_destination_city
- **Type**: `String`
- **Tag**: 193
- **Description**: The city the user is searching for at query time.

#### travel_destination_country
- **Type**: `String`
- **Tag**: 194
- **Description**: The country the user is searching for at query time.

#### travel_destination_region
- **Type**: `String`
- **Tag**: 195
- **Description**: The region the user is searching for at query time.

#### webpage
- **Type**: `String`
- **Tag**: 129
- **Description**: Resource name of the ad group criterion that represents webpage criterion.

#### week
- **Type**: `String`
- **Tag**: 130
- **Description**: Week as defined as Monday through Sunday, and represented by the date of Monday. Formatted as yyyy-MM-dd.

#### year
- **Type**: `i32`
- **Tag**: 131
- **Description**: Year, formatted as yyyy.

---

## Usage Notes

### Accessing Fields

To access these fields in your Rust code:

```rust
// Resource fields
if let Some(campaign) = &row.campaign {
    println!("Campaign: {:?}", campaign);
}

// Metrics fields
if let Some(metrics) = &row.metrics {
    println!("Clicks: {}", metrics.clicks);
    println!("Impressions: {}", metrics.impressions);
    println!("Cost (micros): {}", metrics.cost_micros);
}

// Segments fields
if let Some(segments) = &row.segments {
    println!("Date: {}", segments.date);
    println!("Device: {:?}", segments.device);
}
```

### Query Example

When constructing Google Ads queries, you reference these fields using their snake_case names:

```sql
SELECT
  campaign.id,
  campaign.name,
  metrics.clicks,
  metrics.impressions,
  metrics.cost_micros,
  segments.date
FROM campaign
WHERE segments.date DURING LAST_30_DAYS
```

### Cost Fields

Note that many cost-related fields use micros (millionths of the currency unit):
- `cost_micros` - divide by 1,000,000 to get actual cost
- `average_cpc` - already in standard currency units

### Date Formats

Date segment fields use `yyyy-MM-dd` format (e.g., "2024-10-10").

---

**Document Version**: 1.0
**API Version**: Google Ads API V19
**Generated**: October 2025
