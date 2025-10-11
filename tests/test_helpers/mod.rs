// Test helper module for building GoogleAdsRow test data
//
// This module provides builder patterns for constructing test instances
// of GoogleAdsRow and its nested resource types.

#![allow(dead_code)]
#![allow(clippy::needless_update)]
#![allow(clippy::field_reassign_with_default)]

use googleads_rs::google::ads::googleads::v19::services::GoogleAdsRow;
use googleads_rs::google::ads::googleads::v19::resources::{
    Campaign, AdGroup, AdGroupAd, AdGroupCriterion, CampaignCriterion, CampaignBudget,
    Customer, CustomerClient, Label, Ad, AccountBudget, AssetGroup, Audience, BiddingStrategy,
    SearchTermView, SmartCampaignSearchTermView, ChangeEvent, AdGroupAdAssetView, AssetFieldTypeView,
    ConversionAction, ConversionCustomVariable, Asset, CampaignAsset, AdGroupAsset, CustomerAsset,
    UserList, GeoTargetConstant, KeywordView, LandingPageView, GeographicView, ClickView,
    AssetGroupAsset, AssetGroupSignal, CampaignLabel, AdGroupLabel, AdGroupAdLabel,
    Recommendation, CampaignSharedSet, SharedSet, SharedCriterion,
};
use googleads_rs::google::ads::googleads::v19::common::{Metrics, Segments};
use googleads_rs::google::ads::googleads::v19::enums::{
    campaign_status_enum::CampaignStatus,
    ad_group_status_enum::AdGroupStatus,
    ad_group_type_enum::AdGroupType,
    advertising_channel_type_enum::AdvertisingChannelType,
    bidding_strategy_type_enum::BiddingStrategyType,
    device_enum::Device,
    day_of_week_enum::DayOfWeek,
};

/// Builder for GoogleAdsRow
pub struct GoogleAdsRowBuilder {
    campaign: Option<Campaign>,
    ad_group: Option<AdGroup>,
    ad_group_ad: Option<AdGroupAd>,
    ad_group_criterion: Option<AdGroupCriterion>,
    campaign_criterion: Option<CampaignCriterion>,
    campaign_budget: Option<CampaignBudget>,
    customer: Option<Customer>,
    customer_client: Option<CustomerClient>,
    label: Option<Label>,
    metrics: Option<Metrics>,
    segments: Option<Segments>,
    account_budget: Option<AccountBudget>,
    asset_group: Option<AssetGroup>,
    audience: Option<Audience>,
    bidding_strategy: Option<BiddingStrategy>,
    search_term_view: Option<SearchTermView>,
    smart_campaign_search_term_view: Option<SmartCampaignSearchTermView>,
    change_event: Option<ChangeEvent>,
    ad_group_ad_asset_view: Option<AdGroupAdAssetView>,
    asset_field_type_view: Option<AssetFieldTypeView>,
    conversion_action: Option<ConversionAction>,
    conversion_custom_variable: Option<ConversionCustomVariable>,
    asset: Option<Asset>,
    campaign_asset: Option<CampaignAsset>,
    ad_group_asset: Option<AdGroupAsset>,
    customer_asset: Option<CustomerAsset>,
    user_list: Option<UserList>,
    geo_target_constant: Option<GeoTargetConstant>,
    keyword_view: Option<KeywordView>,
    landing_page_view: Option<LandingPageView>,
    geographic_view: Option<GeographicView>,
    click_view: Option<ClickView>,
    asset_group_asset: Option<AssetGroupAsset>,
    asset_group_signal: Option<AssetGroupSignal>,
    campaign_label: Option<CampaignLabel>,
    ad_group_label: Option<AdGroupLabel>,
    ad_group_ad_label: Option<AdGroupAdLabel>,
    recommendation: Option<Recommendation>,
    campaign_shared_set: Option<CampaignSharedSet>,
    shared_set: Option<SharedSet>,
    shared_criterion: Option<SharedCriterion>,
}

impl GoogleAdsRowBuilder {
    pub fn new() -> Self {
        Self {
            campaign: None,
            ad_group: None,
            ad_group_ad: None,
            ad_group_criterion: None,
            campaign_criterion: None,
            campaign_budget: None,
            customer: None,
            customer_client: None,
            label: None,
            metrics: None,
            segments: None,
            account_budget: None,
            asset_group: None,
            audience: None,
            bidding_strategy: None,
            search_term_view: None,
            smart_campaign_search_term_view: None,
            change_event: None,
            ad_group_ad_asset_view: None,
            asset_field_type_view: None,
            conversion_action: None,
            conversion_custom_variable: None,
            asset: None,
            campaign_asset: None,
            ad_group_asset: None,
            customer_asset: None,
            user_list: None,
            geo_target_constant: None,
            keyword_view: None,
            landing_page_view: None,
            geographic_view: None,
            click_view: None,
            asset_group_asset: None,
            asset_group_signal: None,
            campaign_label: None,
            ad_group_label: None,
            ad_group_ad_label: None,
            recommendation: None,
            campaign_shared_set: None,
            shared_set: None,
            shared_criterion: None,
        }
    }

    pub fn with_campaign(mut self, campaign: Campaign) -> Self {
        self.campaign = Some(campaign);
        self
    }

    pub fn with_ad_group(mut self, ad_group: AdGroup) -> Self {
        self.ad_group = Some(ad_group);
        self
    }

    pub fn with_ad_group_ad(mut self, ad_group_ad: AdGroupAd) -> Self {
        self.ad_group_ad = Some(ad_group_ad);
        self
    }

    pub fn with_ad_group_criterion(mut self, ad_group_criterion: AdGroupCriterion) -> Self {
        self.ad_group_criterion = Some(ad_group_criterion);
        self
    }

    pub fn with_campaign_criterion(mut self, campaign_criterion: CampaignCriterion) -> Self {
        self.campaign_criterion = Some(campaign_criterion);
        self
    }

    pub fn with_campaign_budget(mut self, campaign_budget: CampaignBudget) -> Self {
        self.campaign_budget = Some(campaign_budget);
        self
    }

    pub fn with_customer(mut self, customer: Customer) -> Self {
        self.customer = Some(customer);
        self
    }

    pub fn with_customer_client(mut self, customer_client: CustomerClient) -> Self {
        self.customer_client = Some(customer_client);
        self
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_metrics(mut self, metrics: Metrics) -> Self {
        self.metrics = Some(metrics);
        self
    }

    pub fn with_segments(mut self, segments: Segments) -> Self {
        self.segments = Some(segments);
        self
    }

    pub fn with_account_budget(mut self, account_budget: AccountBudget) -> Self {
        self.account_budget = Some(account_budget);
        self
    }

    pub fn with_asset_group(mut self, asset_group: AssetGroup) -> Self {
        self.asset_group = Some(asset_group);
        self
    }

    pub fn with_audience(mut self, audience: Audience) -> Self {
        self.audience = Some(audience);
        self
    }

    pub fn with_bidding_strategy(mut self, bidding_strategy: BiddingStrategy) -> Self {
        self.bidding_strategy = Some(bidding_strategy);
        self
    }

    pub fn with_search_term_view(mut self, search_term_view: SearchTermView) -> Self {
        self.search_term_view = Some(search_term_view);
        self
    }

    pub fn with_smart_campaign_search_term_view(mut self, smart_campaign_search_term_view: SmartCampaignSearchTermView) -> Self {
        self.smart_campaign_search_term_view = Some(smart_campaign_search_term_view);
        self
    }

    pub fn with_change_event(mut self, change_event: ChangeEvent) -> Self {
        self.change_event = Some(change_event);
        self
    }

    pub fn with_ad_group_ad_asset_view(mut self, ad_group_ad_asset_view: AdGroupAdAssetView) -> Self {
        self.ad_group_ad_asset_view = Some(ad_group_ad_asset_view);
        self
    }

    pub fn with_asset_field_type_view(mut self, asset_field_type_view: AssetFieldTypeView) -> Self {
        self.asset_field_type_view = Some(asset_field_type_view);
        self
    }

    pub fn with_conversion_action(mut self, conversion_action: ConversionAction) -> Self {
        self.conversion_action = Some(conversion_action);
        self
    }

    pub fn with_conversion_custom_variable(mut self, conversion_custom_variable: ConversionCustomVariable) -> Self {
        self.conversion_custom_variable = Some(conversion_custom_variable);
        self
    }

    pub fn with_asset(mut self, asset: Asset) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn with_campaign_asset(mut self, campaign_asset: CampaignAsset) -> Self {
        self.campaign_asset = Some(campaign_asset);
        self
    }

    pub fn with_ad_group_asset(mut self, ad_group_asset: AdGroupAsset) -> Self {
        self.ad_group_asset = Some(ad_group_asset);
        self
    }

    pub fn with_customer_asset(mut self, customer_asset: CustomerAsset) -> Self {
        self.customer_asset = Some(customer_asset);
        self
    }

    pub fn with_user_list(mut self, user_list: UserList) -> Self {
        self.user_list = Some(user_list);
        self
    }

    pub fn with_geo_target_constant(mut self, geo_target_constant: GeoTargetConstant) -> Self {
        self.geo_target_constant = Some(geo_target_constant);
        self
    }

    pub fn with_keyword_view(mut self, keyword_view: KeywordView) -> Self {
        self.keyword_view = Some(keyword_view);
        self
    }

    pub fn with_landing_page_view(mut self, landing_page_view: LandingPageView) -> Self {
        self.landing_page_view = Some(landing_page_view);
        self
    }

    pub fn with_geographic_view(mut self, geographic_view: GeographicView) -> Self {
        self.geographic_view = Some(geographic_view);
        self
    }

    pub fn with_click_view(mut self, click_view: ClickView) -> Self {
        self.click_view = Some(click_view);
        self
    }

    pub fn with_asset_group_asset(mut self, asset_group_asset: AssetGroupAsset) -> Self {
        self.asset_group_asset = Some(asset_group_asset);
        self
    }

    pub fn with_asset_group_signal(mut self, asset_group_signal: AssetGroupSignal) -> Self {
        self.asset_group_signal = Some(asset_group_signal);
        self
    }

    pub fn with_campaign_label(mut self, campaign_label: CampaignLabel) -> Self {
        self.campaign_label = Some(campaign_label);
        self
    }

    pub fn with_ad_group_label(mut self, ad_group_label: AdGroupLabel) -> Self {
        self.ad_group_label = Some(ad_group_label);
        self
    }

    pub fn with_ad_group_ad_label(mut self, ad_group_ad_label: AdGroupAdLabel) -> Self {
        self.ad_group_ad_label = Some(ad_group_ad_label);
        self
    }

    pub fn with_recommendation(mut self, recommendation: Recommendation) -> Self {
        self.recommendation = Some(recommendation);
        self
    }

    pub fn with_campaign_shared_set(mut self, campaign_shared_set: CampaignSharedSet) -> Self {
        self.campaign_shared_set = Some(campaign_shared_set);
        self
    }

    pub fn with_shared_set(mut self, shared_set: SharedSet) -> Self {
        self.shared_set = Some(shared_set);
        self
    }

    pub fn with_shared_criterion(mut self, shared_criterion: SharedCriterion) -> Self {
        self.shared_criterion = Some(shared_criterion);
        self
    }

    pub fn build(self) -> GoogleAdsRow {
        GoogleAdsRow {
            campaign: self.campaign,
            ad_group: self.ad_group,
            ad_group_ad: self.ad_group_ad,
            ad_group_criterion: self.ad_group_criterion,
            campaign_criterion: self.campaign_criterion,
            campaign_budget: self.campaign_budget,
            customer: self.customer,
            customer_client: self.customer_client,
            label: self.label,
            metrics: self.metrics,
            segments: self.segments,
            account_budget: self.account_budget,
            asset_group: self.asset_group,
            audience: self.audience,
            bidding_strategy: self.bidding_strategy,
            search_term_view: self.search_term_view,
            smart_campaign_search_term_view: self.smart_campaign_search_term_view,
            change_event: self.change_event,
            ad_group_ad_asset_view: self.ad_group_ad_asset_view,
            asset_field_type_view: self.asset_field_type_view,
            conversion_action: self.conversion_action,
            conversion_custom_variable: self.conversion_custom_variable,
            asset: self.asset,
            campaign_asset: self.campaign_asset,
            ad_group_asset: self.ad_group_asset,
            customer_asset: self.customer_asset,
            user_list: self.user_list,
            geo_target_constant: self.geo_target_constant,
            keyword_view: self.keyword_view,
            landing_page_view: self.landing_page_view,
            geographic_view: self.geographic_view,
            click_view: self.click_view,
            asset_group_asset: self.asset_group_asset,
            asset_group_signal: self.asset_group_signal,
            campaign_label: self.campaign_label,
            ad_group_label: self.ad_group_label,
            ad_group_ad_label: self.ad_group_ad_label,
            recommendation: self.recommendation,
            campaign_shared_set: self.campaign_shared_set,
            shared_set: self.shared_set,
            shared_criterion: self.shared_criterion,
            ..Default::default()
        }
    }
}

impl Default for GoogleAdsRowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Campaign
pub struct CampaignBuilder {
    campaign: Campaign,
}

impl CampaignBuilder {
    pub fn new() -> Self {
        Self {
            campaign: Campaign::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.campaign.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.campaign.name = name.to_string();
        self
    }

    pub fn status(mut self, status: CampaignStatus) -> Self {
        self.campaign.status = status as i32;
        self
    }

    pub fn advertising_channel_type(mut self, channel_type: AdvertisingChannelType) -> Self {
        self.campaign.advertising_channel_type = channel_type as i32;
        self
    }

    pub fn bidding_strategy_type(mut self, bidding_type: BiddingStrategyType) -> Self {
        self.campaign.bidding_strategy_type = bidding_type as i32;
        self
    }

    pub fn campaign_budget(mut self, budget: &str) -> Self {
        self.campaign.campaign_budget = budget.to_string();
        self
    }

    pub fn end_date(mut self, date: &str) -> Self {
        self.campaign.end_date = date.to_string();
        self
    }

    pub fn optimization_score(mut self, score: f64) -> Self {
        self.campaign.optimization_score = score;
        self
    }

    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.campaign.labels = labels;
        self
    }

    pub fn build(self) -> Campaign {
        self.campaign
    }
}

impl Default for CampaignBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AdGroup
pub struct AdGroupBuilder {
    ad_group: AdGroup,
}

impl AdGroupBuilder {
    pub fn new() -> Self {
        Self {
            ad_group: AdGroup::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.ad_group.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.ad_group.name = name.to_string();
        self
    }

    pub fn status(mut self, status: AdGroupStatus) -> Self {
        self.ad_group.status = status as i32;
        self
    }

    pub fn ad_group_type(mut self, ag_type: AdGroupType) -> Self {
        self.ad_group.r#type = ag_type as i32;
        self
    }

    pub fn cpc_bid_micros(mut self, bid: i64) -> Self {
        self.ad_group.cpc_bid_micros = bid;
        self
    }

    pub fn cpm_bid_micros(mut self, bid: i64) -> Self {
        self.ad_group.cpm_bid_micros = bid;
        self
    }

    pub fn target_cpa_micros(mut self, cpa: i64) -> Self {
        self.ad_group.target_cpa_micros = cpa;
        self
    }

    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.ad_group.labels = labels;
        self
    }

    pub fn build(self) -> AdGroup {
        self.ad_group
    }
}

impl Default for AdGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for CampaignBudget
pub struct CampaignBudgetBuilder {
    budget: CampaignBudget,
}

impl CampaignBudgetBuilder {
    pub fn new() -> Self {
        Self {
            budget: CampaignBudget::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.budget.id = id;
        self
    }

    pub fn amount_micros(mut self, amount: i64) -> Self {
        self.budget.amount_micros = amount;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.budget.name = name.to_string();
        self
    }

    pub fn build(self) -> CampaignBudget {
        self.budget
    }
}

impl Default for CampaignBudgetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Customer
pub struct CustomerBuilder {
    customer: Customer,
}

impl CustomerBuilder {
    pub fn new() -> Self {
        Self {
            customer: Customer::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.customer.id = id;
        self
    }

    pub fn descriptive_name(mut self, name: &str) -> Self {
        self.customer.descriptive_name = name.to_string();
        self
    }

    pub fn currency_code(mut self, code: &str) -> Self {
        self.customer.currency_code = code.to_string();
        self
    }

    pub fn time_zone(mut self, tz: &str) -> Self {
        self.customer.time_zone = tz.to_string();
        self
    }

    pub fn build(self) -> Customer {
        self.customer
    }
}

impl Default for CustomerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Metrics
pub struct MetricsBuilder {
    metrics: Metrics,
}

impl MetricsBuilder {
    pub fn new() -> Self {
        Self {
            metrics: Metrics::default(),
        }
    }

    pub fn impressions(mut self, impressions: i64) -> Self {
        self.metrics.impressions = impressions;
        self
    }

    pub fn clicks(mut self, clicks: i64) -> Self {
        self.metrics.clicks = clicks;
        self
    }

    pub fn ctr(mut self, ctr: f64) -> Self {
        self.metrics.ctr = ctr;
        self
    }

    pub fn cost_micros(mut self, cost: i64) -> Self {
        self.metrics.cost_micros = cost;
        self
    }

    pub fn conversions(mut self, conversions: f64) -> Self {
        self.metrics.conversions = conversions;
        self
    }

    pub fn conversions_value(mut self, value: f64) -> Self {
        self.metrics.conversions_value = value;
        self
    }

    pub fn average_cpc(mut self, cpc: f64) -> Self {
        self.metrics.average_cpc = cpc;
        self
    }

    pub fn average_cpm(mut self, cpm: f64) -> Self {
        self.metrics.average_cpm = cpm;
        self
    }

    pub fn build(self) -> Metrics {
        self.metrics
    }
}

impl Default for MetricsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Segments
pub struct SegmentsBuilder {
    segments: Segments,
}

impl SegmentsBuilder {
    pub fn new() -> Self {
        Self {
            segments: Segments::default(),
        }
    }

    pub fn date(mut self, date: &str) -> Self {
        self.segments.date = date.to_string();
        self
    }

    pub fn device(mut self, device: Device) -> Self {
        self.segments.device = device as i32;
        self
    }

    pub fn day_of_week(mut self, day: DayOfWeek) -> Self {
        self.segments.day_of_week = day as i32;
        self
    }

    pub fn hour(mut self, hour: i32) -> Self {
        self.segments.hour = hour;
        self
    }

    pub fn month(mut self, month: &str) -> Self {
        self.segments.month = month.to_string();
        self
    }

    pub fn year(mut self, year: i32) -> Self {
        self.segments.year = year;
        self
    }

    pub fn build(self) -> Segments {
        self.segments
    }
}

impl Default for SegmentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AdGroupCriterion with oneof criterion support
pub struct AdGroupCriterionBuilder {
    criterion: AdGroupCriterion,
}

impl AdGroupCriterionBuilder {
    pub fn new() -> Self {
        Self {
            criterion: AdGroupCriterion::default(),
        }
    }

    pub fn criterion_id(mut self, id: i64) -> Self {
        self.criterion.criterion_id = id;
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.criterion.status = status;
        self
    }

    pub fn cpc_bid_micros(mut self, bid: i64) -> Self {
        self.criterion.cpc_bid_micros = bid;
        self
    }

    pub fn with_keyword(mut self, text: &str, match_type: i32) -> Self {
        use googleads_rs::google::ads::googleads::v19::common::KeywordInfo;
        use googleads_rs::google::ads::googleads::v19::resources::ad_group_criterion::Criterion;

        self.criterion.criterion = Some(Criterion::Keyword(KeywordInfo {
            text: text.to_string(),
            match_type,
        }));
        self
    }

    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.criterion.labels = labels;
        self
    }

    pub fn build(self) -> AdGroupCriterion {
        self.criterion
    }
}

impl Default for AdGroupCriterionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for CampaignCriterion with oneof criterion support
pub struct CampaignCriterionBuilder {
    criterion: CampaignCriterion,
}

impl CampaignCriterionBuilder {
    pub fn new() -> Self {
        Self {
            criterion: CampaignCriterion::default(),
        }
    }

    pub fn criterion_id(mut self, id: i64) -> Self {
        self.criterion.criterion_id = id;
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.criterion.status = status;
        self
    }

    pub fn display_name(mut self, name: &str) -> Self {
        self.criterion.display_name = name.to_string();
        self
    }

    pub fn with_keyword(mut self, text: &str) -> Self {
        use googleads_rs::google::ads::googleads::v19::common::KeywordInfo;
        use googleads_rs::google::ads::googleads::v19::resources::campaign_criterion::Criterion;

        self.criterion.criterion = Some(Criterion::Keyword(KeywordInfo {
            text: text.to_string(),
            match_type: 0,
        }));
        self
    }

    pub fn with_location(mut self, geo_target_constant: &str) -> Self {
        use googleads_rs::google::ads::googleads::v19::common::LocationInfo;
        use googleads_rs::google::ads::googleads::v19::resources::campaign_criterion::Criterion;

        self.criterion.criterion = Some(Criterion::Location(LocationInfo {
            geo_target_constant: geo_target_constant.to_string(),
        }));
        self
    }

    pub fn build(self) -> CampaignCriterion {
        self.criterion
    }
}

impl Default for CampaignCriterionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Ad with oneof ad_data support
pub struct AdBuilder {
    ad: Ad,
}

impl AdBuilder {
    pub fn new() -> Self {
        Self {
            ad: Ad::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.ad.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.ad.name = name.to_string();
        self
    }

    pub fn ad_type(mut self, ad_type: i32) -> Self {
        self.ad.r#type = ad_type;
        self
    }

    pub fn with_responsive_search_ad(
        mut self,
        headlines: Vec<&str>,
        descriptions: Vec<&str>,
        path1: Option<&str>,
        path2: Option<&str>,
    ) -> Self {
        use googleads_rs::google::ads::googleads::v19::common::{
            ResponsiveSearchAdInfo, AdTextAsset,
        };
        use googleads_rs::google::ads::googleads::v19::resources::ad::AdData;

        let headline_assets: Vec<AdTextAsset> = headlines
            .iter()
            .map(|text| AdTextAsset {
                text: text.to_string(),
                ..Default::default()
            })
            .collect();

        let description_assets: Vec<AdTextAsset> = descriptions
            .iter()
            .map(|text| AdTextAsset {
                text: text.to_string(),
                ..Default::default()
            })
            .collect();

        self.ad.ad_data = Some(AdData::ResponsiveSearchAd(ResponsiveSearchAdInfo {
            headlines: headline_assets,
            descriptions: description_assets,
            path1: path1.unwrap_or("").to_string(),
            path2: path2.unwrap_or("").to_string(),
            ..Default::default()
        }));
        self
    }

    pub fn build(self) -> Ad {
        self.ad
    }
}

impl Default for AdBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AdGroupAd
pub struct AdGroupAdBuilder {
    ad_group_ad: AdGroupAd,
}

impl AdGroupAdBuilder {
    pub fn new() -> Self {
        Self {
            ad_group_ad: AdGroupAd::default(),
        }
    }

    pub fn status(mut self, status: i32) -> Self {
        self.ad_group_ad.status = status;
        self
    }

    pub fn with_ad(mut self, ad: Ad) -> Self {
        self.ad_group_ad.ad = Some(ad);
        self
    }

    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.ad_group_ad.labels = labels;
        self
    }

    pub fn build(self) -> AdGroupAd {
        self.ad_group_ad
    }
}

impl Default for AdGroupAdBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Campaign with nested message support
impl CampaignBuilder {
    pub fn with_network_settings(
        mut self,
        target_search: bool,
        target_content: bool,
        target_partner: bool,
        target_google_search: bool,
    ) -> Self {
        use googleads_rs::google::ads::googleads::v19::resources::campaign::NetworkSettings;

        self.campaign.network_settings = Some(NetworkSettings {
            target_search_network: target_search,
            target_content_network: target_content,
            target_partner_search_network: target_partner,
            target_google_search,
            ..Default::default()
        });
        self
    }

    pub fn with_dynamic_search_ads_setting(
        mut self,
        domain_name: &str,
        language_code: &str,
        use_supplied_urls_only: bool,
    ) -> Self {
        use googleads_rs::google::ads::googleads::v19::resources::campaign::DynamicSearchAdsSetting;

        self.campaign.dynamic_search_ads_setting = Some(DynamicSearchAdsSetting {
            domain_name: domain_name.to_string(),
            language_code: language_code.to_string(),
            use_supplied_urls_only,
            ..Default::default()
        });
        self
    }
}

// ============================================================================
// Phase 3 Resource Builders
// ============================================================================

/// Builder for AccountBudget
pub struct AccountBudgetBuilder {
    account_budget: AccountBudget,
}

impl AccountBudgetBuilder {
    pub fn new() -> Self {
        Self {
            account_budget: AccountBudget::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.account_budget.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.account_budget.name = name.to_string();
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.account_budget.status = status;
        self
    }

    pub fn build(self) -> AccountBudget {
        self.account_budget
    }
}

impl Default for AccountBudgetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AssetGroup
pub struct AssetGroupBuilder {
    asset_group: AssetGroup,
}

impl AssetGroupBuilder {
    pub fn new() -> Self {
        Self {
            asset_group: AssetGroup::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.asset_group.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.asset_group.name = name.to_string();
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.asset_group.status = status;
        self
    }

    pub fn resource_name(mut self, name: &str) -> Self {
        self.asset_group.resource_name = name.to_string();
        self
    }

    pub fn campaign(mut self, campaign: &str) -> Self {
        self.asset_group.campaign = campaign.to_string();
        self
    }

    pub fn ad_strength(mut self, strength: i32) -> Self {
        self.asset_group.ad_strength = strength;
        self
    }

    pub fn build(self) -> AssetGroup {
        self.asset_group
    }
}

impl Default for AssetGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Audience
pub struct AudienceBuilder {
    audience: Audience,
}

impl AudienceBuilder {
    pub fn new() -> Self {
        Self {
            audience: Audience::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.audience.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.audience.name = name.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.audience.description = description.to_string();
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.audience.status = status;
        self
    }

    pub fn build(self) -> Audience {
        self.audience
    }
}

impl Default for AudienceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for BiddingStrategy
pub struct BiddingStrategyBuilder {
    bidding_strategy: BiddingStrategy,
}

impl BiddingStrategyBuilder {
    pub fn new() -> Self {
        Self {
            bidding_strategy: BiddingStrategy::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.bidding_strategy.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.bidding_strategy.name = name.to_string();
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.bidding_strategy.status = status;
        self
    }

    pub fn build(self) -> BiddingStrategy {
        self.bidding_strategy
    }
}

impl Default for BiddingStrategyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for Label (extension to existing)
pub struct LabelBuilder {
    label: Label,
}

impl LabelBuilder {
    pub fn new() -> Self {
        Self {
            label: Label::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.label.id = id;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.label.name = name.to_string();
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.label.status = status;
        self
    }

    pub fn build(self) -> Label {
        self.label
    }
}

impl Default for LabelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for CustomerClient (extension to existing)
pub struct CustomerClientBuilder {
    customer_client: CustomerClient,
}

impl CustomerClientBuilder {
    pub fn new() -> Self {
        Self {
            customer_client: CustomerClient::default(),
        }
    }

    pub fn id(mut self, id: i64) -> Self {
        self.customer_client.id = id;
        self
    }

    pub fn client_customer(mut self, client: &str) -> Self {
        self.customer_client.client_customer = client.to_string();
        self
    }

    pub fn currency_code(mut self, code: &str) -> Self {
        self.customer_client.currency_code = code.to_string();
        self
    }

    pub fn descriptive_name(mut self, name: &str) -> Self {
        self.customer_client.descriptive_name = name.to_string();
        self
    }

    pub fn level(mut self, level: i64) -> Self {
        self.customer_client.level = level;
        self
    }

    pub fn manager(mut self, manager: bool) -> Self {
        self.customer_client.manager = manager;
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.customer_client.status = status;
        self
    }

    pub fn time_zone(mut self, tz: &str) -> Self {
        self.customer_client.time_zone = tz.to_string();
        self
    }

    pub fn build(self) -> CustomerClient {
        self.customer_client
    }
}

impl Default for CustomerClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for SearchTermView
pub struct SearchTermViewBuilder {
    search_term_view: SearchTermView,
}

impl SearchTermViewBuilder {
    pub fn new() -> Self {
        Self {
            search_term_view: SearchTermView::default(),
        }
    }

    pub fn ad_group(mut self, ad_group: &str) -> Self {
        self.search_term_view.ad_group = ad_group.to_string();
        self
    }

    pub fn search_term(mut self, term: &str) -> Self {
        self.search_term_view.search_term = term.to_string();
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.search_term_view.status = status;
        self
    }

    pub fn build(self) -> SearchTermView {
        self.search_term_view
    }
}

impl Default for SearchTermViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for SmartCampaignSearchTermView
pub struct SmartCampaignSearchTermViewBuilder {
    smart_campaign_search_term_view: SmartCampaignSearchTermView,
}

impl SmartCampaignSearchTermViewBuilder {
    pub fn new() -> Self {
        Self {
            smart_campaign_search_term_view: SmartCampaignSearchTermView::default(),
        }
    }

    pub fn campaign(mut self, campaign: &str) -> Self {
        self.smart_campaign_search_term_view.campaign = campaign.to_string();
        self
    }

    pub fn search_term(mut self, term: &str) -> Self {
        self.smart_campaign_search_term_view.search_term = term.to_string();
        self
    }

    pub fn build(self) -> SmartCampaignSearchTermView {
        self.smart_campaign_search_term_view
    }
}

impl Default for SmartCampaignSearchTermViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for ChangeEvent
pub struct ChangeEventBuilder {
    change_event: ChangeEvent,
}

impl ChangeEventBuilder {
    pub fn new() -> Self {
        Self {
            change_event: ChangeEvent::default(),
        }
    }

    pub fn change_date_time(mut self, date_time: &str) -> Self {
        self.change_event.change_date_time = date_time.to_string();
        self
    }

    pub fn change_resource_type(mut self, resource_type: i32) -> Self {
        self.change_event.change_resource_type = resource_type;
        self
    }

    pub fn change_resource_name(mut self, resource_name: &str) -> Self {
        self.change_event.change_resource_name = resource_name.to_string();
        self
    }

    pub fn client_type(mut self, client_type: i32) -> Self {
        self.change_event.client_type = client_type;
        self
    }

    pub fn user_email(mut self, email: &str) -> Self {
        self.change_event.user_email = email.to_string();
        self
    }

    pub fn resource_change_operation(mut self, operation: i32) -> Self {
        self.change_event.resource_change_operation = operation;
        self
    }

    pub fn changed_fields(mut self, paths: Vec<&str>) -> Self {
        use prost_types::FieldMask;
        let paths_vec: Vec<String> = paths.iter().map(|s| s.to_string()).collect();
        self.change_event.changed_fields = Some(FieldMask { paths: paths_vec });
        self
    }

    pub fn campaign(mut self, campaign: &str) -> Self {
        self.change_event.campaign = campaign.to_string();
        self
    }

    pub fn build(self) -> ChangeEvent {
        self.change_event
    }
}

impl Default for ChangeEventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AdGroupAdAssetView
pub struct AdGroupAdAssetViewBuilder {
    ad_group_ad_asset_view: AdGroupAdAssetView,
}

impl AdGroupAdAssetViewBuilder {
    pub fn new() -> Self {
        Self {
            ad_group_ad_asset_view: AdGroupAdAssetView::default(),
        }
    }

    pub fn resource_name(mut self, name: &str) -> Self {
        self.ad_group_ad_asset_view.resource_name = name.to_string();
        self
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.ad_group_ad_asset_view.asset = asset.to_string();
        self
    }

    pub fn field_type(mut self, field_type: i32) -> Self {
        self.ad_group_ad_asset_view.field_type = field_type;
        self
    }

    pub fn pinned_field(mut self, pinned: i32) -> Self {
        self.ad_group_ad_asset_view.pinned_field = pinned;
        self
    }

    pub fn performance_label(mut self, label: i32) -> Self {
        self.ad_group_ad_asset_view.performance_label = label;
        self
    }

    pub fn build(self) -> AdGroupAdAssetView {
        self.ad_group_ad_asset_view
    }
}

impl Default for AdGroupAdAssetViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AssetFieldTypeView
pub struct AssetFieldTypeViewBuilder {
    asset_field_type_view: AssetFieldTypeView,
}

impl AssetFieldTypeViewBuilder {
    pub fn new() -> Self {
        Self {
            asset_field_type_view: AssetFieldTypeView::default(),
        }
    }

    pub fn field_type(mut self, field_type: i32) -> Self {
        self.asset_field_type_view.field_type = field_type;
        self
    }

    pub fn build(self) -> AssetFieldTypeView {
        self.asset_field_type_view
    }
}

impl Default for AssetFieldTypeViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}
