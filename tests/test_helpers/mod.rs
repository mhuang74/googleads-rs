// Test helper module for building GoogleAdsRow test data
//
// This module provides builder patterns for constructing test instances
// of GoogleAdsRow and its nested resource types.

use googleads_rs::google::ads::googleads::v19::services::GoogleAdsRow;
use googleads_rs::google::ads::googleads::v19::resources::{
    Campaign, AdGroup, AdGroupAd, AdGroupCriterion, CampaignCriterion, CampaignBudget,
    Customer, CustomerClient, Label, Ad,
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
