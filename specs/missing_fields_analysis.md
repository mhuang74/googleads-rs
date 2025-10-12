# GoogleAdsRow Missing Fields Analysis

**Analysis Date**: 2025-10-10
**API Version**: Google Ads API V19
**Source Files**:
- `/Users/mhuang/Projects/Development/googleads/googleads-rs/specs/all_googleadsrow_fields_and_metrics_v19.md`
- `/Users/mhuang/Projects/Development/googleads/googleads-rs/src/lib.rs` (lines 215-502)

---

## Executive Summary

**Total Available Fields**: 568
- Resource Fields: 189
- Metrics Fields: 283
- Segments Fields: 96

**Currently Implemented**:
- Resource Fields: 18 resources (partially or fully implemented)
- Metrics Fields: 124 / 283 (43.8%)
- Segments Fields: 20 / 96 (20.8%)

**Missing**:
- Resource Fields: 171 resources (90.5% of resources not implemented)
- Metrics Fields: 159 / 283 (56.2%)
- Segments Fields: 76 / 96 (79.2%)

---

## 1. Implemented Resources Summary

### Currently Implemented Resources (18 total)

1. **account_budget** - 3 fields
   - id, name, status

2. **ad** - 2 fields (via ad_group_ad.ad)
   - id, name, type
   - responsive_search_ad.headlines, responsive_search_ad.descriptions, responsive_search_ad.path1, responsive_search_ad.path2

3. **ad_group** - 14 fields
   - id, name, status, type, labels
   - cpc_bid_micros, cpm_bid_micros, cpv_bid_micros
   - target_cpa_micros, target_cpm_micros, target_roas
   - effective_cpc_bid_micros, effective_target_cpa_micros, effective_target_cpa_source, effective_target_roas, effective_target_roas_source

4. **ad_group_ad** - 3 fields
   - status, labels
   - ad.* (see ad above)

5. **ad_group_ad_asset_view** - 5 fields
   - resource_name, asset, field_type, pinned_field, performance_label

6. **ad_group_criterion** - 19 fields
   - bid_modifier, criterion_id, status, type, labels
   - cpc_bid_micros, cpm_bid_micros, cpv_bid_micros
   - effective_cpc_bid_micros, effective_cpc_bid_source
   - effective_cpm_bid_micros, effective_cpm_bid_source
   - effective_cpv_bid_micros, effective_cpv_bid_source
   - effective_percent_cpc_bid_micros, effective_percent_cpc_bid_source
   - keyword.text, keyword.match_type

7. **asset_field_type_view** - 1 field
   - field_type

8. **asset_group** - 6 fields
   - id, name, status, resource_name, campaign, ad_strength

9. **audience** - 4 fields
   - description, id, name, status

10. **bidding_strategy** - 3 fields
    - id, name, status

11. **campaign** - 22 fields
    - id, name, status, serving_status, base_campaign, end_date, campaign_budget, labels, optimization_score
    - advertising_channel_type, advertising_channel_sub_type, experiment_type, bidding_strategy_type
    - network_settings.target_content_network, network_settings.target_google_search, network_settings.target_partner_search_network, network_settings.target_search_network
    - dynamic_search_ads_setting.domain_name, dynamic_search_ads_setting.language_code, dynamic_search_ads_setting.use_supplied_urls_only
    - performance_max_upgrade.performance_max_campaign, performance_max_upgrade.pre_upgrade_campaign, performance_max_upgrade.status

12. **campaign_budget** - 1 field (optional parent)
    - amount_micros

13. **campaign_criterion** - 7 fields (optional parent)
    - campaign, criterion_id, display_name, status, type
    - keyword.text
    - location.geo_target_constant

14. **change_event** - 8 fields
    - change_date_time, change_resource_type, change_resource_name, client_type, user_email, resource_change_operation, changed_fields, campaign

15. **customer** - 5 fields
    - id, descriptive_name, status, currency_code, time_zone

16. **customer_client** - 8 fields
    - client_customer, currency_code, descriptive_name, id, level, manager, status, time_zone

17. **label** - 3 fields
    - id, name, status

18. **search_term_view** - 3 fields
    - ad_group, search_term, status

19. **smart_campaign_search_term_view** - 2 fields
    - campaign, search_term

---

## 2. Missing Resource Fields

### 2.1 Completely Unimplemented Resources (171 resources)

#### A

1. **accessible_bidding_strategy** (Tag 169)
   - Type: AccessibleBiddingStrategy
   - Common fields: id, name, resource_name

2. **account_budget_proposal** (Tag 43)
   - Type: AccountBudgetProposal
   - Common fields: id, resource_name, status

3. **account_link** (Tag 143)
   - Type: AccountLink
   - Common fields: resource_name, status

4. **ad_group_ad_asset_combination_view** (Tag 193)
   - Type: AdGroupAdAssetCombinationView
   - Usage: Performance Max asset combinations

5. **ad_group_ad_label** (Tag 120)
   - Type: AdGroupAdLabel
   - Common fields: resource_name

6. **ad_group_asset** (Tag 154)
   - Type: AdGroupAsset
   - Common fields: resource_name, status, field_type

7. **ad_group_asset_set** (Tag 196)
   - Type: AdGroupAssetSet
   - Common fields: resource_name, status

8. **ad_group_audience_view** (Tag 57)
   - Type: AdGroupAudienceView
   - Common fields: resource_name

9. **ad_group_bid_modifier** (Tag 24)
   - Type: AdGroupBidModifier
   - Common fields: bid_modifier, resource_name

10. **ad_group_criterion_customizer** (Tag 187)
    - Type: AdGroupCriterionCustomizer
    - Common fields: resource_name, value

11. **ad_group_criterion_label** (Tag 121)
    - Type: AdGroupCriterionLabel
    - Common fields: resource_name

12. **ad_group_criterion_simulation** (Tag 110)
    - Type: AdGroupCriterionSimulation
    - Usage: Bid simulations

13. **ad_group_customizer** (Tag 185)
    - Type: AdGroupCustomizer
    - Common fields: resource_name, value

14. **ad_group_label** (Tag 115)
    - Type: AdGroupLabel
    - Common fields: resource_name

15. **ad_group_simulation** (Tag 107)
    - Type: AdGroupSimulation
    - Usage: Bid simulations

16. **ad_parameter** (Tag 130)
    - Type: AdParameter
    - Common fields: resource_name, insertion_text

17. **ad_schedule_view** (Tag 89)
    - Type: AdScheduleView
    - Common fields: resource_name

18. **age_range_view** (Tag 48)
    - Type: AgeRangeView
    - Common fields: resource_name

19. **android_privacy_shared_key_google_ad_group** (Tag 217)
    - Type: AndroidPrivacySharedKeyGoogleAdGroup
    - Usage: Android privacy sandbox

20. **android_privacy_shared_key_google_campaign** (Tag 218)
    - Type: AndroidPrivacySharedKeyGoogleCampaign
    - Usage: Android privacy sandbox

21. **android_privacy_shared_key_google_network_type** (Tag 219)
    - Type: AndroidPrivacySharedKeyGoogleNetworkType
    - Usage: Android privacy sandbox

22. **asset** (Tag 105)
    - Type: Asset
    - Common fields: id, name, resource_name, type, final_urls

23. **asset_group_asset** (Tag 173)
    - Type: AssetGroupAsset
    - Common fields: resource_name, field_type, status

24. **asset_group_listing_group_filter** (Tag 182)
    - Type: AssetGroupListingGroupFilter
    - Usage: Shopping filters

25. **asset_group_product_group_view** (Tag 189)
    - Type: AssetGroupProductGroupView
    - Usage: Shopping performance

26. **asset_group_signal** (Tag 191)
    - Type: AssetGroupSignal
    - Common fields: resource_name

27. **asset_group_top_combination_view** (Tag 199)
    - Type: AssetGroupTopCombinationView
    - Usage: Performance Max top combinations

28. **asset_set** (Tag 179)
    - Type: AssetSet
    - Common fields: id, name, resource_name, status

29. **asset_set_asset** (Tag 180)
    - Type: AssetSetAsset
    - Common fields: resource_name, status

30. **asset_set_type_view** (Tag 197)
    - Type: AssetSetTypeView
    - Common fields: resource_name

#### B

31. **batch_job** (Tag 139)
    - Type: BatchJob
    - Common fields: id, resource_name, status

32. **bidding_data_exclusion** (Tag 159)
    - Type: BiddingDataExclusion
    - Common fields: resource_name, status

33. **bidding_seasonality_adjustment** (Tag 160)
    - Type: BiddingSeasonalityAdjustment
    - Common fields: resource_name, status

34. **bidding_strategy_simulation** (Tag 158)
    - Type: BiddingStrategySimulation
    - Usage: Bid simulations

35. **billing_setup** (Tag 41)
    - Type: BillingSetup
    - Common fields: id, resource_name, status

#### C

36. **call_view** (Tag 152)
    - Type: CallView
    - Common fields: resource_name, caller_country_code

37. **campaign_aggregate_asset_view** (Tag 224)
    - Type: CampaignAggregateAssetView
    - Usage: Asset performance aggregation

38. **campaign_asset** (Tag 142)
    - Type: CampaignAsset
    - Common fields: resource_name, field_type, status

39. **campaign_asset_set** (Tag 181)
    - Type: CampaignAssetSet
    - Common fields: resource_name, status

40. **campaign_audience_view** (Tag 69)
    - Type: CampaignAudienceView
    - Common fields: resource_name

41. **campaign_bid_modifier** (Tag 26)
    - Type: CampaignBidModifier
    - Common fields: bid_modifier, resource_name

42. **campaign_conversion_goal** (Tag 175)
    - Type: CampaignConversionGoal
    - Common fields: resource_name

43. **campaign_customizer** (Tag 186)
    - Type: CampaignCustomizer
    - Common fields: resource_name, value

44. **campaign_draft** (Tag 49)
    - Type: CampaignDraft
    - Common fields: id, name, resource_name, status

45. **campaign_group** (Tag 25)
    - Type: CampaignGroup
    - Common fields: id, name, resource_name, status

46. **campaign_label** (Tag 108)
    - Type: CampaignLabel
    - Common fields: resource_name

47. **campaign_lifecycle_goal** (Tag 213)
    - Type: CampaignLifecycleGoal
    - Common fields: resource_name

48. **campaign_search_term_insight** (Tag 204)
    - Type: CampaignSearchTermInsight
    - Common fields: resource_name

49. **campaign_shared_set** (Tag 30)
    - Type: CampaignSharedSet
    - Common fields: resource_name, status

50. **campaign_simulation** (Tag 157)
    - Type: CampaignSimulation
    - Usage: Bid simulations

51. **carrier_constant** (Tag 66)
    - Type: CarrierConstant
    - Common fields: id, name, resource_name

52. **change_status** (Tag 37)
    - Type: ChangeStatus
    - Common fields: resource_name, resource_type, resource_status

53. **channel_aggregate_asset_view** (Tag 222)
    - Type: ChannelAggregateAssetView
    - Usage: Cross-channel asset performance

54. **click_view** (Tag 122)
    - Type: ClickView
    - Common fields: resource_name, gclid

55. **combined_audience** (Tag 148)
    - Type: CombinedAudience
    - Common fields: id, name, resource_name, status

56. **content_criterion_view** (Tag 232)
    - Type: ContentCriterionView
    - Common fields: resource_name

57. **conversion_action** (Tag 103)
    - Type: ConversionAction
    - Common fields: id, name, resource_name, status, type, category

58. **conversion_custom_variable** (Tag 153)
    - Type: ConversionCustomVariable
    - Common fields: id, name, resource_name

59. **conversion_goal_campaign_config** (Tag 177)
    - Type: ConversionGoalCampaignConfig
    - Common fields: resource_name

60. **conversion_value_rule** (Tag 164)
    - Type: ConversionValueRule
    - Common fields: id, resource_name

61. **conversion_value_rule_set** (Tag 165)
    - Type: ConversionValueRuleSet
    - Common fields: id, resource_name

62. **currency_constant** (Tag 134)
    - Type: CurrencyConstant
    - Common fields: code, name, resource_name

63. **custom_audience** (Tag 147)
    - Type: CustomAudience
    - Common fields: id, name, resource_name, status

64. **custom_conversion_goal** (Tag 176)
    - Type: CustomConversionGoal
    - Common fields: id, name, resource_name, status

65. **custom_interest** (Tag 104)
    - Type: CustomInterest
    - Common fields: id, name, resource_name, status

66. **customer_asset** (Tag 155)
    - Type: CustomerAsset
    - Common fields: resource_name, field_type, status

67. **customer_asset_set** (Tag 195)
    - Type: CustomerAssetSet
    - Common fields: resource_name, status

68. **customer_client_link** (Tag 62)
    - Type: CustomerClientLink
    - Common fields: resource_name, status

69. **customer_conversion_goal** (Tag 174)
    - Type: CustomerConversionGoal
    - Common fields: resource_name

70. **customer_customizer** (Tag 184)
    - Type: CustomerCustomizer
    - Common fields: resource_name, value

71. **customer_label** (Tag 124)
    - Type: CustomerLabel
    - Common fields: resource_name

72. **customer_lifecycle_goal** (Tag 212)
    - Type: CustomerLifecycleGoal
    - Common fields: resource_name

73. **customer_manager_link** (Tag 61)
    - Type: CustomerManagerLink
    - Common fields: resource_name, status

74. **customer_negative_criterion** (Tag 88)
    - Type: CustomerNegativeCriterion
    - Common fields: resource_name, type

75. **customer_search_term_insight** (Tag 205)
    - Type: CustomerSearchTermInsight
    - Common fields: resource_name

76. **customer_user_access** (Tag 146)
    - Type: CustomerUserAccess
    - Common fields: resource_name, email_address, access_role

77. **customer_user_access_invitation** (Tag 150)
    - Type: CustomerUserAccessInvitation
    - Common fields: resource_name, email_address, access_role

78. **customizer_attribute** (Tag 178)
    - Type: CustomizerAttribute
    - Common fields: id, name, resource_name, type

#### D

79. **data_link** (Tag 230)
    - Type: DataLink
    - Common fields: resource_name

80. **detail_placement_view** (Tag 118)
    - Type: DetailPlacementView
    - Common fields: resource_name, placement

81. **detailed_demographic** (Tag 166)
    - Type: DetailedDemographic
    - Common fields: id, name, resource_name

82. **display_keyword_view** (Tag 47)
    - Type: DisplayKeywordView
    - Common fields: resource_name

83. **distance_view** (Tag 132)
    - Type: DistanceView
    - Common fields: resource_name, distance_bucket

84. **domain_category** (Tag 91)
    - Type: DomainCategory
    - Common fields: resource_name, category, domain

85. **dynamic_search_ads_search_term_view** (Tag 106)
    - Type: DynamicSearchAdsSearchTermView
    - Common fields: resource_name, search_term

#### E

86. **expanded_landing_page_view** (Tag 128)
    - Type: ExpandedLandingPageView
    - Common fields: resource_name, expanded_final_url

87. **experiment** (Tag 133)
    - Type: Experiment
    - Common fields: id, name, resource_name, status, type

88. **experiment_arm** (Tag 183)
    - Type: ExperimentArm
    - Common fields: resource_name, name

#### G

89. **gender_view** (Tag 40)
    - Type: GenderView
    - Common fields: resource_name

90. **geo_target_constant** (Tag 23)
    - Type: GeoTargetConstant
    - Common fields: id, name, resource_name, country_code

91. **geographic_view** (Tag 125)
    - Type: GeographicView
    - Common fields: resource_name, location_type

92. **group_placement_view** (Tag 119)
    - Type: GroupPlacementView
    - Common fields: resource_name, placement

#### H

93. **hotel_group_view** (Tag 51)
    - Type: HotelGroupView
    - Common fields: resource_name

94. **hotel_performance_view** (Tag 71)
    - Type: HotelPerformanceView
    - Common fields: resource_name

95. **hotel_reconciliation** (Tag 188)
    - Type: HotelReconciliation
    - Common fields: resource_name

#### I

96. **income_range_view** (Tag 138)
    - Type: IncomeRangeView
    - Common fields: resource_name

#### K

97. **keyword_plan** (Tag 32)
    - Type: KeywordPlan
    - Common fields: id, name, resource_name

98. **keyword_plan_ad_group** (Tag 35)
    - Type: KeywordPlanAdGroup
    - Common fields: id, name, resource_name

99. **keyword_plan_ad_group_keyword** (Tag 141)
    - Type: KeywordPlanAdGroupKeyword
    - Common fields: id, resource_name, text

100. **keyword_plan_campaign** (Tag 33)
     - Type: KeywordPlanCampaign
     - Common fields: id, name, resource_name

101. **keyword_plan_campaign_keyword** (Tag 140)
     - Type: KeywordPlanCampaignKeyword
     - Common fields: id, resource_name, text

102. **keyword_theme_constant** (Tag 163)
     - Type: KeywordThemeConstant
     - Common fields: resource_name, country_code

103. **keyword_view** (Tag 21)
     - Type: KeywordView
     - Common fields: resource_name

#### L

104. **landing_page_view** (Tag 126)
     - Type: LandingPageView
     - Common fields: resource_name, unexpanded_final_url

105. **language_constant** (Tag 55)
     - Type: LanguageConstant
     - Common fields: id, code, name, resource_name

106. **lead_form_submission_data** (Tag 192)
     - Type: LeadFormSubmissionData
     - Common fields: resource_name, id

107. **life_event** (Tag 161)
     - Type: LifeEvent
     - Common fields: id, name, resource_name

108. **local_services_employee** (Tag 223)
     - Type: LocalServicesEmployee
     - Common fields: resource_name

109. **local_services_lead** (Tag 210)
     - Type: LocalServicesLead
     - Common fields: resource_name, id

110. **local_services_lead_conversation** (Tag 214)
     - Type: LocalServicesLeadConversation
     - Common fields: resource_name, id

111. **local_services_verification_artifact** (Tag 211)
     - Type: LocalServicesVerificationArtifact
     - Common fields: resource_name, id

112. **location_view** (Tag 123)
     - Type: LocationView
     - Common fields: resource_name

#### M

113. **managed_placement_view** (Tag 53)
     - Type: ManagedPlacementView
     - Common fields: resource_name

114. **media_file** (Tag 90)
     - Type: MediaFile
     - Common fields: id, name, resource_name, type

115. **mobile_app_category_constant** (Tag 87)
     - Type: MobileAppCategoryConstant
     - Common fields: id, name, resource_name

116. **mobile_device_constant** (Tag 98)
     - Type: MobileDeviceConstant
     - Common fields: id, name, resource_name

#### O

117. **offline_conversion_upload_client_summary** (Tag 216)
     - Type: OfflineConversionUploadClientSummary
     - Common fields: resource_name

118. **offline_conversion_upload_conversion_action_summary** (Tag 228)
     - Type: OfflineConversionUploadConversionActionSummary
     - Common fields: resource_name

119. **offline_user_data_job** (Tag 137)
     - Type: OfflineUserDataJob
     - Common fields: id, resource_name, status, type

120. **operating_system_version_constant** (Tag 86)
     - Type: OperatingSystemVersionConstant
     - Common fields: id, name, resource_name

#### P

121. **paid_organic_search_term_view** (Tag 129)
     - Type: PaidOrganicSearchTermView
     - Common fields: resource_name, search_term

122. **parental_status_view** (Tag 45)
     - Type: ParentalStatusView
     - Common fields: resource_name

123. **per_store_view** (Tag 198)
     - Type: PerStoreView
     - Common fields: resource_name

124. **performance_max_placement_view** (Tag 233)
     - Type: PerformanceMaxPlacementView
     - Common fields: resource_name

125. **product_category_constant** (Tag 208)
     - Type: ProductCategoryConstant
     - Common fields: resource_name, category_id

126. **product_group_view** (Tag 54)
     - Type: ProductGroupView
     - Common fields: resource_name

127. **product_link** (Tag 194)
     - Type: ProductLink
     - Common fields: resource_name, type

128. **product_link_invitation** (Tag 209)
     - Type: ProductLinkInvitation
     - Common fields: resource_name

#### Q

129. **qualifying_question** (Tag 202)
     - Type: QualifyingQuestion
     - Common fields: resource_name

#### R

130. **recommendation** (Tag 22)
     - Type: Recommendation
     - Common fields: resource_name, type

131. **recommendation_subscription** (Tag 220)
     - Type: RecommendationSubscription
     - Common fields: resource_name, type

132. **remarketing_action** (Tag 60)
     - Type: RemarketingAction
     - Common fields: id, name, resource_name

#### S

133. **shared_criterion** (Tag 29)
     - Type: SharedCriterion
     - Common fields: resource_name, type

134. **shared_set** (Tag 27)
     - Type: SharedSet
     - Common fields: id, name, resource_name, status, type

135. **shopping_performance_view** (Tag 117)
     - Type: ShoppingPerformanceView
     - Common fields: resource_name

136. **shopping_product** (Tag 226)
     - Type: ShoppingProduct
     - Common fields: resource_name

#### T

137. **third_party_app_analytics_link** (Tag 144)
     - Type: ThirdPartyAppAnalyticsLink
     - Common fields: resource_name

138. **topic_constant** (Tag 31)
     - Type: TopicConstant
     - Common fields: id, resource_name

139. **topic_view** (Tag 44)
     - Type: TopicView
     - Common fields: resource_name

140. **travel_activity_group_view** (Tag 201)
     - Type: TravelActivityGroupView
     - Common fields: resource_name

141. **travel_activity_performance_view** (Tag 200)
     - Type: TravelActivityPerformanceView
     - Common fields: resource_name

#### U

142. **user_interest** (Tag 59)
     - Type: UserInterest
     - Common fields: id, name, resource_name

143. **user_list** (Tag 38)
     - Type: UserList
     - Common fields: id, name, resource_name, type, size_for_display, size_for_search

144. **user_list_customer_type** (Tag 225)
     - Type: UserListCustomerType
     - Common fields: resource_name

145. **user_location_view** (Tag 135)
     - Type: UserLocationView
     - Common fields: resource_name

#### V

146. **video** (Tag 39)
     - Type: Video
     - Common fields: id, resource_name, channel_id

#### W

147. **webpage_view** (Tag 162)
     - Type: WebpageView
     - Common fields: resource_name

---

## 3. Missing Metrics Fields (159 / 283)

### 3.1 Currently Implemented Metrics (124 fields)

The following metrics are currently implemented:
- absolute_top_impression_percentage
- active_view_cpm, active_view_ctr, active_view_impressions, active_view_measurability, active_view_measurable_cost_micros, active_view_measurable_impressions, active_view_viewability
- all_conversions, all_conversions_by_conversion_date, all_conversions_from_click_to_call, all_conversions_from_directions, all_conversions_from_interactions_rate, all_conversions_from_interactions_value_per_interaction, all_conversions_from_menu, all_conversions_from_order, all_conversions_from_other_engagement, all_conversions_from_store_visit, all_conversions_from_store_website, all_conversions_value, all_conversions_value_by_conversion_date, all_conversions_value_per_cost
- average_cost, average_cpc, average_cpe, average_cpm, average_cpv, average_page_views, average_time_on_site
- benchmark_average_max_cpc, benchmark_ctr
- biddable_app_install_conversions, biddable_app_post_install_conversions, bounce_rate
- clicks, combined_clicks, combined_clicks_per_query, combined_queries
- content_budget_lost_impression_share, content_impression_share, content_rank_lost_impression_share
- conversion_last_conversion_date, conversion_last_received_request_date_time
- conversions, conversions_by_conversion_date, conversions_from_interactions_rate, conversions_from_interactions_value_per_interaction, conversions_value, conversions_value_by_conversion_date, conversions_value_per_cost
- cost_micros, cost_per_all_conversions, cost_per_conversion, cost_per_current_model_attributed_conversion
- cross_device_conversions, ctr
- current_model_attributed_conversions, current_model_attributed_conversions_from_interactions_rate, current_model_attributed_conversions_from_interactions_value_per_interaction, current_model_attributed_conversions_value, current_model_attributed_conversions_value_per_cost
- engagement_rate, engagements
- gmail_forwards, gmail_saves, gmail_secondary_clicks
- historical_creative_quality_score, historical_landing_page_quality_score, historical_quality_score, historical_search_predicted_ctr
- hotel_average_lead_value_micros, hotel_commission_rate_micros, hotel_eligible_impressions, hotel_expected_commission_cost, hotel_price_difference_percentage
- impressions, impressions_from_store_reach, interaction_rate, interactions
- invalid_click_rate, invalid_clicks
- message_chat_rate, message_chats, message_impressions
- mobile_friendly_clicks_percentage
- optimization_score_uplift, optimization_score_url
- organic_clicks, organic_clicks_per_query, organic_impressions, organic_impressions_per_query, organic_queries
- percent_new_visitors, phone_calls, phone_impressions, phone_through_rate, relative_ctr
- search_absolute_top_impression_share, search_budget_lost_absolute_top_impression_share, search_budget_lost_impression_share, search_budget_lost_top_impression_share, search_click_share, search_exact_match_impression_share, search_impression_share, search_rank_lost_absolute_top_impression_share, search_rank_lost_impression_share, search_rank_lost_top_impression_share, search_top_impression_share
- sk_ad_network_installs, speed_score, top_impression_percentage
- valid_accelerated_mobile_pages_clicks_percentage
- value_per_all_conversions, value_per_all_conversions_by_conversion_date, value_per_conversion, value_per_conversions_by_conversion_date, value_per_current_model_attributed_conversion
- video_quartile_p100_rate, video_quartile_p25_rate, video_quartile_p50_rate, video_quartile_p75_rate, video_view_rate, video_views
- view_through_conversions

### 3.2 Missing Metrics (159 fields)

#### Asset Performance Metrics (12 fields)
- all_new_customer_lifetime_value
- asset_best_performance_cost_percentage
- asset_best_performance_impression_percentage
- asset_good_performance_cost_percentage
- asset_good_performance_impression_percentage
- asset_learning_performance_cost_percentage
- asset_learning_performance_impression_percentage
- asset_low_performance_cost_percentage
- asset_low_performance_impression_percentage
- asset_unrated_performance_cost_percentage
- asset_unrated_performance_impression_percentage

#### Asset Pinning Metrics (7 fields)
- asset_pinned_as_description_position_one_count
- asset_pinned_as_description_position_two_count
- asset_pinned_as_headline_position_one_count
- asset_pinned_as_headline_position_three_count
- asset_pinned_as_headline_position_two_count
- asset_pinned_total_count

#### Asset Sample Entities (6 fields)
- linked_entities_count
- linked_sample_entities
- sample_best_performance_entities
- sample_good_performance_entities
- sample_learning_performance_entities
- sample_low_performance_entities
- sample_unrated_performance_entities

#### Location Asset Conversion Metrics (14 fields)
- all_conversions_from_location_asset_click_to_call
- all_conversions_from_location_asset_directions
- all_conversions_from_location_asset_menu
- all_conversions_from_location_asset_order
- all_conversions_from_location_asset_other_engagement
- all_conversions_from_location_asset_store_visits
- all_conversions_from_location_asset_website
- eligible_impressions_from_location_asset_store_reach
- view_through_conversions_from_location_asset_click_to_call
- view_through_conversions_from_location_asset_directions
- view_through_conversions_from_location_asset_menu
- view_through_conversions_from_location_asset_order
- view_through_conversions_from_location_asset_other_engagement
- view_through_conversions_from_location_asset_store_visits
- view_through_conversions_from_location_asset_website

#### E-commerce Metrics (12 fields)
- average_cart_size
- average_order_value_micros
- cost_of_goods_sold_micros
- cross_sell_cost_of_goods_sold_micros
- cross_sell_gross_profit_micros
- cross_sell_revenue_micros
- cross_sell_units_sold
- gross_profit_margin
- gross_profit_micros
- lead_cost_of_goods_sold_micros
- lead_gross_profit_micros
- lead_revenue_micros
- lead_units_sold
- orders
- revenue_micros
- units_sold

#### Auction Insights Metrics (6 fields)
- auction_insight_search_absolute_top_impression_percentage
- auction_insight_search_impression_share
- auction_insight_search_outranking_share
- auction_insight_search_overlap_rate
- auction_insight_search_position_above_rate
- auction_insight_search_top_impression_percentage

#### Bidding Strategy Metrics (2 fields)
- average_target_cpa_micros
- average_target_roas

#### Customer Lifetime Value Metrics (1 field)
- new_customer_lifetime_value

#### Cross-Device Metrics (1 field)
- cross_device_conversions_value_micros

#### Invalid Click Metrics (2 fields)
- general_invalid_click_rate
- general_invalid_clicks

#### Impression Frequency Metrics (2 fields)
- average_impression_frequency_per_user
- unique_users

#### Publisher Metrics (3 fields)
- publisher_organic_clicks
- publisher_purchased_clicks
- publisher_unknown_clicks

#### Search Volume (1 field)
- search_volume (Option<SearchVolumeRange>)

#### SKAdNetwork Metrics (1 field)
- sk_ad_network_total_conversions

#### Store Visits (1 field)
- store_visits_last_click_model_attributed_conversions

#### Video Metrics (3 fields)
- video_view_rate_in_feed
- video_view_rate_in_stream
- video_view_rate_shorts

#### Results Conversions (1 field)
- results_conversions_purchase

---

## 4. Missing Segments Fields (76 / 96)

### 4.1 Currently Implemented Segments (20 fields)

- ad_destination_type
- ad_network_type
- click_type
- conversion_action
- conversion_action_category
- conversion_action_name
- date
- day_of_week
- device
- hour
- month
- month_of_year
- product_channel
- product_channel_exclusivity
- product_item_id
- quarter
- search_term_match_type
- slot
- week
- year

### 4.2 Missing Segments (76 fields)

#### Activity Segments (5 fields)
- activity_account_id
- activity_city
- activity_country
- activity_rating
- activity_state

#### Ad Format (1 field)
- ad_format_type

#### Ad/Asset Group (2 fields)
- ad_group
- asset_group

#### Asset Interaction (1 field)
- asset_interaction_target

#### Auction Insights (1 field)
- auction_insight_domain

#### Budget (1 field)
- budget_campaign_association_status

#### Campaign (1 field)
- campaign

#### Conversion Segments (5 fields)
- conversion_adjustment
- conversion_attribution_event_type
- conversion_lag_bucket
- conversion_or_adjustment_lag_bucket
- conversion_value_rule_primary_dimension

#### External Segments (2 fields)
- external_activity_id
- external_conversion_source

#### Geo Target Segments (11 fields)
- geo_target_airport
- geo_target_canton
- geo_target_city
- geo_target_country
- geo_target_county
- geo_target_district
- geo_target_metro
- geo_target_most_specific_location
- geo_target_postal_code
- geo_target_province
- geo_target_region
- geo_target_state

#### Hotel Segments (10 fields)
- hotel_booking_window_days
- hotel_center_id
- hotel_check_in_date
- hotel_check_in_day_of_week
- hotel_city
- hotel_class
- hotel_country
- hotel_date_selection_type
- hotel_length_of_stay
- hotel_price_bucket
- hotel_rate_rule_id
- hotel_rate_type
- hotel_state

#### Interaction (1 field)
- interaction_on_this_extension

#### Keyword (1 field)
- keyword

#### Customer Type (1 field)
- new_versus_returning_customers

#### Partner Hotel (1 field)
- partner_hotel_id

#### Product Segments (18 fields)
- product_aggregator_id
- product_brand
- product_category_level1
- product_category_level2
- product_category_level3
- product_category_level4
- product_category_level5
- product_condition
- product_country
- product_custom_attribute0
- product_custom_attribute1
- product_custom_attribute2
- product_custom_attribute3
- product_custom_attribute4
- product_feed_label
- product_language
- product_merchant_id
- product_store_id
- product_title
- product_type_l1
- product_type_l2
- product_type_l3
- product_type_l4
- product_type_l5

#### Recommendation (1 field)
- recommendation_type

#### Search Segments (3 fields)
- search_engine_results_page_type
- search_subcategory
- search_term

#### SKAdNetwork Segments (9 fields)
- sk_ad_network_ad_event_type
- sk_ad_network_attribution_credit
- sk_ad_network_coarse_conversion_value
- sk_ad_network_fine_conversion_value
- sk_ad_network_postback_sequence_index
- sk_ad_network_redistributed_fine_conversion_value
- sk_ad_network_source_app
- sk_ad_network_source_domain
- sk_ad_network_source_type
- sk_ad_network_user_type
- sk_ad_network_version

#### Travel Segments (3 fields)
- travel_destination_city
- travel_destination_country
- travel_destination_region

#### Webpage (1 field)
- webpage

---

## 5. Implementation Priority Recommendations

### 5.1 High Priority - Commonly Used Resources

These resources are frequently queried in typical Google Ads workflows:

1. **conversion_action** (Priority: CRITICAL)
   - Fields: id, name, status, type, category, include_in_conversions_metric
   - Reason: Essential for conversion tracking and reporting
   - Implementation: Use attr_str and method_str macros

2. **asset** (Priority: HIGH)
   - Fields: id, name, type, resource_name, final_urls
   - Reason: Core to Performance Max and responsive ads
   - Implementation: Use attr_str and method_str macros

3. **campaign_asset** (Priority: HIGH)
   - Fields: resource_name, field_type, status
   - Reason: Managing assets at campaign level
   - Implementation: Use attr_str and method_str macros

4. **ad_group_asset** (Priority: HIGH)
   - Fields: resource_name, field_type, status
   - Reason: Managing assets at ad group level
   - Implementation: Use attr_str and method_str macros

5. **user_list** (Priority: HIGH)
   - Fields: id, name, type, size_for_display, size_for_search, membership_status
   - Reason: Audience targeting
   - Implementation: Use attr_str and method_str macros

6. **geo_target_constant** (Priority: HIGH)
   - Fields: id, name, country_code, target_type
   - Reason: Location targeting
   - Implementation: Use attr_str and method_str macros

7. **keyword_view** (Priority: HIGH)
   - Fields: resource_name
   - Reason: Keyword performance reporting
   - Implementation: Simple attr_str

8. **campaign_group** (Priority: MEDIUM)
   - Fields: id, name, status
   - Reason: Campaign organization
   - Implementation: Use attr_str and method_str macros

### 5.2 High Priority - Missing Metrics

1. **E-commerce Metrics** (Priority: CRITICAL for Shopping campaigns)
   - orders, revenue_micros, average_order_value_micros
   - cost_of_goods_sold_micros, gross_profit_micros, gross_profit_margin
   - units_sold, average_cart_size
   - Implementation: All use attr_str macro (scalar fields)

2. **Location Asset Metrics** (Priority: HIGH for local businesses)
   - all_conversions_from_location_asset_* (7 fields)
   - view_through_conversions_from_location_asset_* (7 fields)
   - eligible_impressions_from_location_asset_store_reach
   - Implementation: All use attr_str macro

3. **Asset Performance Metrics** (Priority: HIGH for Performance Max)
   - asset_*_performance_cost_percentage (5 fields)
   - asset_*_performance_impression_percentage (5 fields)
   - Implementation: All use attr_str macro

4. **Customer Lifetime Value** (Priority: MEDIUM)
   - new_customer_lifetime_value
   - all_new_customer_lifetime_value
   - Implementation: Use attr_str macro

### 5.3 High Priority - Missing Segments

1. **Product Segments** (Priority: CRITICAL for Shopping)
   - product_brand, product_title, product_item_id (already implemented)
   - product_category_level1-5 (5 fields)
   - product_type_l1-5 (5 fields)
   - product_custom_attribute0-4 (5 fields)
   - product_condition, product_country, product_language
   - Implementation: All use attr_str macro (String fields)

2. **Geo Target Segments** (Priority: HIGH)
   - geo_target_country, geo_target_state, geo_target_city
   - geo_target_metro, geo_target_postal_code
   - geo_target_most_specific_location
   - Implementation: All use attr_str macro (String resource names)

3. **Hotel Segments** (Priority: MEDIUM for Hotel campaigns)
   - hotel_center_id, hotel_check_in_date, hotel_city, hotel_country
   - hotel_class, hotel_length_of_stay, hotel_price_bucket
   - Implementation: Mix of attr_str and method_str

4. **Search Segments** (Priority: MEDIUM)
   - search_term (use existing search_term_view instead)
   - search_subcategory
   - search_engine_results_page_type
   - Implementation: attr_str for String, method_str for enum

5. **Campaign/Ad Group Segments** (Priority: HIGH)
   - campaign (String resource name)
   - ad_group (String resource name)
   - Implementation: Both use attr_str macro

### 5.4 Medium Priority - Reporting Resources

9. **click_view** (Priority: MEDIUM)
   - Fields: resource_name, gclid, ad_group_ad, campaign_location_target
   - Reason: Click-level reporting
   - Implementation: Use attr_str macro

10. **geographic_view** (Priority: MEDIUM)
    - Fields: resource_name, location_type
    - Reason: Geographic performance
    - Implementation: Use attr_str and method_str macros

11. **landing_page_view** (Priority: MEDIUM)
    - Fields: resource_name, unexpanded_final_url
    - Reason: Landing page performance
    - Implementation: Use attr_str macro

### 5.5 Low Priority - Specialized Resources

The following are lower priority as they're used in specific scenarios:

- **android_privacy_* resources** - Only for Android privacy sandbox
- **hotel_* resources** - Only for Hotel campaigns
- **local_services_* resources** - Only for Local Services Ads
- **travel_* resources** - Only for Travel campaigns
- **batch_job** - Batch operations only
- **experiment resources** - Campaign experiments only
- **simulation resources** - Bid simulation only

---

## 6. Implementation Roadmap

### Phase 1: Core Resources (Week 1-2)
Implement the most commonly used resources:
1. conversion_action
2. asset
3. campaign_asset
4. ad_group_asset
5. user_list
6. geo_target_constant
7. keyword_view
8. campaign_group

**Estimated fields**: ~50-60 new match arms

### Phase 2: E-commerce & Location Metrics (Week 3)
Add all e-commerce and location metrics:
1. E-commerce metrics (12 fields)
2. Location asset conversion metrics (14 fields)
3. Asset performance metrics (12 fields)

**Estimated fields**: ~38 new match arms

### Phase 3: Product & Geo Segments (Week 4)
Add Shopping and geographic segments:
1. Product segments (18 fields)
2. Geo target segments (11 fields)
3. Campaign/Ad Group segments (2 fields)

**Estimated fields**: ~31 new match arms

### Phase 4: Reporting Views (Week 5)
Add common reporting views:
1. click_view
2. geographic_view
3. landing_page_view
4. age_range_view
5. gender_view
6. parental_status_view

**Estimated fields**: ~15-20 new match arms

### Phase 5: Asset & Label Resources (Week 6)
Add asset and label management:
1. campaign_label
2. ad_group_label
3. ad_group_ad_label
4. customer_label
5. asset_set
6. asset_set_asset

**Estimated fields**: ~15-20 new match arms

### Phase 6: Remaining High-Value Resources (Week 7-8)
Add remaining commonly used resources:
1. shared_set, shared_criterion
2. remarketing_action
3. custom_audience, combined_audience
4. media_file
5. feed resources
6. customer linking resources

**Estimated fields**: ~40-50 new match arms

### Phase 7: Specialized Metrics & Segments (Week 9-10)
Add specialized but useful fields:
1. Remaining metrics (SKAdNetwork, auction insights, etc.)
2. Remaining segments (hotel, travel, etc.)

**Estimated fields**: ~60-70 new match arms

### Phase 8: Specialized Resources (As needed)
Add specialized resources based on user demand:
1. Hotel resources
2. Travel resources
3. Local Services resources
4. Experiment resources
5. Simulation resources

**Estimated fields**: ~100+ new match arms

---

## 7. Technical Implementation Notes

### 7.1 Macro Selection Guide

Based on `/Users/mhuang/Projects/Development/googleads/googleads-rs/specs/how_to_implement_get_matcharms.md`:

1. **Scalar fields (String, i64, f64, bool)**
   - Non-optional parent: `attr_str!([parent], field)`
   - Optional parent: `optional_attr_str!(parent, field)`
   - Nested: `attr_str!([parent, nested], field)`

2. **Enum fields with accessor methods**
   - Non-optional parent: `method_str!([parent], status)` or `method_str!([parent], r#type)`
   - Optional parent: `optional_method_str!(parent, status)`

3. **Oneof fields**
   - Non-optional parent: `enum_match_str!([parent], criterion, Keyword, text)`
   - Optional parent: `optional_enum_match_str!(parent, criterion, Keyword, text)`

4. **Repeated nested fields**
   - `enum_match_iterator_str!([parent], ad_data, ResponsiveSearchAd, headlines, text)`

### 7.2 Common Patterns

Most new fields will follow these patterns:

```rust
// Simple resource fields
"resource.id" => attr_str!([resource], id),
"resource.name" => attr_str!([resource], name),
"resource.status" => method_str!([resource], status),

// Optional resource fields
"optional_resource.id" => optional_attr_str!(optional_resource, id),
"optional_resource.status" => optional_method_str!(optional_resource, status),

// Nested fields
"resource.nested.field" => attr_str!([resource, nested], field),

// Oneof fields
"resource.criterion.keyword.text" => enum_match_str!([resource], criterion, Keyword, text),
```

### 7.3 Required Imports

New resources may require adding imports at the top of the file:

```rust
use crate::google::ads::googleads::v19::resources::{
    // Add new oneof variant imports here
};
```

---

## 8. Testing Strategy

### 8.1 Unit Test Coverage

For each phase, add tests covering:
1. Resource fields with valid data
2. Optional resources (present and absent)
3. Enum fields (various enum values)
4. Oneof fields (different variants)
5. Nested fields
6. Edge cases (empty strings, zero values)

### 8.2 Integration Test Queries

Create GAQL queries that select:
1. All fields for a given resource
2. Mixed resource types
3. Metrics + segments combinations
4. Complex nested paths

### 8.3 Validation

Verify that:
1. Field paths match Google Ads API documentation
2. Enum values render correctly
3. Optional fields handle None gracefully
4. Oneof variants match correctly
5. No panics on unwrap()

---

## Appendix A: Field Count Summary

| Category | Total Available | Implemented | Missing | % Complete |
|----------|----------------|-------------|---------|------------|
| **Resources** | 189 | 18 | 171 | 9.5% |
| **Metrics** | 283 | 124 | 159 | 43.8% |
| **Segments** | 96 | 20 | 76 | 20.8% |
| **TOTAL** | 568 | 162 | 406 | 28.5% |

### Resources by Implementation Status

- **Fully/Partially Implemented**: 18 resources
- **Not Implemented**: 171 resources

### Priority Distribution

- **Critical Priority**: ~20 resources (conversion_action, asset, e-commerce metrics, product segments)
- **High Priority**: ~30 resources (asset groups, user lists, location resources, geo segments)
- **Medium Priority**: ~40 resources (reporting views, remaining metrics/segments)
- **Low Priority**: ~100+ resources (specialized/scenario-specific)

---

## Appendix B: Quick Reference - Most Common Missing Fields

### Top 20 Most Commonly Queried Missing Fields

1. `conversion_action.name`
2. `conversion_action.id`
3. `conversion_action.status`
4. `asset.id`
5. `asset.name`
6. `asset.type`
7. `metrics.orders`
8. `metrics.revenue_micros`
9. `metrics.average_order_value_micros`
10. `segments.product_brand`
11. `segments.product_title`
12. `segments.product_category_level1`
13. `segments.geo_target_country`
14. `segments.campaign`
15. `segments.ad_group`
16. `user_list.id`
17. `user_list.name`
18. `campaign_asset.field_type`
19. `ad_group_asset.field_type`
20. `keyword_view.resource_name`

---

**End of Analysis**
