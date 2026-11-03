#[derive(Debug, Clone)]
pub struct MetricRequired {
    /// Unique identifier for the metric.
    pub slug: String,
    /// Human-readable name for the metric.
    pub display_name: String,
    /// Type of the metric, useful to know the unit or format of the value.
    pub r#type: crate::v2027_01::models::metrics::enums::MetricType,
}
/// Information about a metric.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Metric {
    /// Unique identifier for the metric.
    pub slug: String,
    /// Human-readable name for the metric.
    pub display_name: String,
    /// Type of the metric, useful to know the unit or format of the value.
    pub r#type: crate::v2027_01::models::metrics::enums::MetricType,
}
#[allow(clippy::type_complexity)]
impl Metric {
    pub fn new(required: MetricRequired) -> Self {
        Self { slug: required.slug, display_name: required.display_name, r#type: required.r#type }
    }
}
#[derive(Debug, Clone)]
pub struct MetricDashboardCreateRequired {
    /// Display name for the dashboard.
    pub name: crate::v2027_01::models::metrics::newtypes::MetricDashboardCreateName,
}
/// Schema for creating a metrics dashboard.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MetricDashboardCreate {
    /// Display name for the dashboard.
    pub name: crate::v2027_01::models::metrics::newtypes::MetricDashboardCreateName,
    /// List of metric slugs to display in this dashboard.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metrics: Option<Vec<String>>,
    /// The ID of the organization owning this dashboard. **Required unless you
    /// use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::OrganizationId>,
}
#[allow(clippy::type_complexity)]
impl MetricDashboardCreate {
    pub fn new(required: MetricDashboardCreateRequired) -> Self {
        Self { name: required.name, metrics: None, organization_id: presence_rs::Presence::Absent }
    }

    /// List of metric slugs to display in this dashboard.
    #[allow(clippy::should_implement_trait)]
    pub fn metrics(mut self, value: Vec<String>) -> Self {
        self.metrics = Some(value);
        self
    }

    /// The ID of the organization owning this dashboard. **Required unless you
    /// use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MetricDashboardSchemaRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DashboardId,
    /// Display name for the dashboard.
    pub name: String,
    /// List of metric slugs displayed in this dashboard.
    pub metrics: Vec<String>,
    /// The ID of the organization owning this dashboard.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
/// A user-defined metrics dashboard.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MetricDashboardSchema {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DashboardId,
    /// Display name for the dashboard.
    pub name: String,
    /// List of metric slugs displayed in this dashboard.
    pub metrics: Vec<String>,
    /// The ID of the organization owning this dashboard.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[allow(clippy::type_complexity)]
impl MetricDashboardSchema {
    pub fn new(required: MetricDashboardSchemaRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
            metrics: required.metrics,
            organization_id: required.organization_id,
        }
    }
}
/// Schema for updating a metrics dashboard.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct MetricDashboardUpdate {
    /// Display name for the dashboard.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2027_01::models::metrics::newtypes::MetricDashboardUpdateName,
    >,
    /// List of metric slugs to display in this dashboard.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub metrics: presence_rs::Presence<Vec<String>>,
}
#[allow(clippy::type_complexity)]
impl MetricDashboardUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// Display name for the dashboard.
    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2027_01::models::metrics::newtypes::MetricDashboardUpdateName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    /// List of metric slugs to display in this dashboard.
    #[allow(clippy::should_implement_trait)]
    pub fn metrics(mut self, value: Vec<String>) -> Self {
        self.metrics = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MetricPeriodRequired {
    /// Timestamp of this period data.
    pub timestamp: jiff::Timestamp,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MetricPeriod {
    ///Timestamp of this period data.
    pub timestamp: jiff::Timestamp,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub active_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodActiveSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCommittedSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub monthly_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodMonthlyRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_monthly_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodTrialMonthlyRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_monthly_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCommittedMonthlyRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_committed_monthly_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodTrialCommittedMonthlyRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_revenue_per_user: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodAverageRevenuePerUserUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checkouts: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCheckoutsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub succeeded_checkouts: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodSucceededCheckoutsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churned_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodChurnedSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churn_rate: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodChurnRateUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_total: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodSeatsTotalUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_claimed: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodSeatsClaimedUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_pending: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodSeatsPendingUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seat_customers: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodSeatCustomersUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_seat_customers: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodNewSeatCustomersUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churned_seat_customers: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodChurnedSeatCustomersUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub orders: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodOrdersUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodNetRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cumulative_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCumulativeRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_cumulative_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodNetCumulativeRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub costs: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCostsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cumulative_costs: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCumulativeCostsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_order_value: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodAverageOrderValueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_average_order_value: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodNetAverageOrderValueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cost_per_user: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCostPerUserUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub active_user_by_event: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodActiveUserByEventUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodOneTimeProductsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodOneTimeProductsRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products_net_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodOneTimeProductsNetRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodNewSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodNewSubscriptionsRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions_net_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodNewSubscriptionsNetRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodRenewedSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodRenewedSubscriptionsRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions_net_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodRenewedSubscriptionsNetRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_customer_service: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsCustomerServiceUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_low_quality: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsLowQualityUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_missing_features: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsMissingFeaturesUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_switched_service: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsSwitchedServiceUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_too_complex: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsTooComplexUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_too_expensive: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsTooExpensiveUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_unused: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsUnusedUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_other: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsOtherUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub annual_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodAnnualRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_annual_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCommittedAnnualRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checkouts_conversion: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCheckoutsConversionUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub ltv: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodLtvUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub gross_margin: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodGrossMarginUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub gross_margin_percentage: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodGrossMarginPercentageUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cashflow: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodCashflowUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_seats_per_customer: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodAverageSeatsPerCustomerUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seat_utilization_rate: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricPeriodSeatUtilizationRateUnion,
    >,
}
#[allow(clippy::type_complexity)]
impl MetricPeriod {
    pub fn new(required: MetricPeriodRequired) -> Self {
        Self {
            timestamp: required.timestamp,
            active_subscriptions: presence_rs::Presence::Absent,
            committed_subscriptions: presence_rs::Presence::Absent,
            monthly_recurring_revenue: presence_rs::Presence::Absent,
            trial_monthly_recurring_revenue: presence_rs::Presence::Absent,
            committed_monthly_recurring_revenue: presence_rs::Presence::Absent,
            trial_committed_monthly_recurring_revenue: presence_rs::Presence::Absent,
            average_revenue_per_user: presence_rs::Presence::Absent,
            checkouts: presence_rs::Presence::Absent,
            succeeded_checkouts: presence_rs::Presence::Absent,
            churned_subscriptions: presence_rs::Presence::Absent,
            churn_rate: presence_rs::Presence::Absent,
            seats_total: presence_rs::Presence::Absent,
            seats_claimed: presence_rs::Presence::Absent,
            seats_pending: presence_rs::Presence::Absent,
            seat_customers: presence_rs::Presence::Absent,
            new_seat_customers: presence_rs::Presence::Absent,
            churned_seat_customers: presence_rs::Presence::Absent,
            orders: presence_rs::Presence::Absent,
            revenue: presence_rs::Presence::Absent,
            net_revenue: presence_rs::Presence::Absent,
            cumulative_revenue: presence_rs::Presence::Absent,
            net_cumulative_revenue: presence_rs::Presence::Absent,
            costs: presence_rs::Presence::Absent,
            cumulative_costs: presence_rs::Presence::Absent,
            average_order_value: presence_rs::Presence::Absent,
            net_average_order_value: presence_rs::Presence::Absent,
            cost_per_user: presence_rs::Presence::Absent,
            active_user_by_event: presence_rs::Presence::Absent,
            one_time_products: presence_rs::Presence::Absent,
            one_time_products_revenue: presence_rs::Presence::Absent,
            one_time_products_net_revenue: presence_rs::Presence::Absent,
            new_subscriptions: presence_rs::Presence::Absent,
            new_subscriptions_revenue: presence_rs::Presence::Absent,
            new_subscriptions_net_revenue: presence_rs::Presence::Absent,
            renewed_subscriptions: presence_rs::Presence::Absent,
            renewed_subscriptions_revenue: presence_rs::Presence::Absent,
            renewed_subscriptions_net_revenue: presence_rs::Presence::Absent,
            canceled_subscriptions: presence_rs::Presence::Absent,
            canceled_subscriptions_customer_service: presence_rs::Presence::Absent,
            canceled_subscriptions_low_quality: presence_rs::Presence::Absent,
            canceled_subscriptions_missing_features: presence_rs::Presence::Absent,
            canceled_subscriptions_switched_service: presence_rs::Presence::Absent,
            canceled_subscriptions_too_complex: presence_rs::Presence::Absent,
            canceled_subscriptions_too_expensive: presence_rs::Presence::Absent,
            canceled_subscriptions_unused: presence_rs::Presence::Absent,
            canceled_subscriptions_other: presence_rs::Presence::Absent,
            annual_recurring_revenue: presence_rs::Presence::Absent,
            committed_annual_recurring_revenue: presence_rs::Presence::Absent,
            checkouts_conversion: presence_rs::Presence::Absent,
            ltv: presence_rs::Presence::Absent,
            gross_margin: presence_rs::Presence::Absent,
            gross_margin_percentage: presence_rs::Presence::Absent,
            cashflow: presence_rs::Presence::Absent,
            average_seats_per_customer: presence_rs::Presence::Absent,
            seat_utilization_rate: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn active_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodActiveSubscriptionsUnion,
    ) -> Self {
        self.active_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCommittedSubscriptionsUnion,
    ) -> Self {
        self.committed_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodMonthlyRecurringRevenueUnion,
    ) -> Self {
        self.monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn trial_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodTrialMonthlyRecurringRevenueUnion,
    ) -> Self {
        self.trial_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCommittedMonthlyRecurringRevenueUnion,
    ) -> Self {
        self.committed_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn trial_committed_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodTrialCommittedMonthlyRecurringRevenueUnion,
    ) -> Self {
        self.trial_committed_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_revenue_per_user(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodAverageRevenuePerUserUnion,
    ) -> Self {
        self.average_revenue_per_user = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checkouts(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCheckoutsUnion,
    ) -> Self {
        self.checkouts = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn succeeded_checkouts(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodSucceededCheckoutsUnion,
    ) -> Self {
        self.succeeded_checkouts = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churned_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodChurnedSubscriptionsUnion,
    ) -> Self {
        self.churned_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churn_rate(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodChurnRateUnion,
    ) -> Self {
        self.churn_rate = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_total(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodSeatsTotalUnion,
    ) -> Self {
        self.seats_total = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_claimed(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodSeatsClaimedUnion,
    ) -> Self {
        self.seats_claimed = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_pending(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodSeatsPendingUnion,
    ) -> Self {
        self.seats_pending = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodSeatCustomersUnion,
    ) -> Self {
        self.seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodNewSeatCustomersUnion,
    ) -> Self {
        self.new_seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churned_seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodChurnedSeatCustomersUnion,
    ) -> Self {
        self.churned_seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn orders(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodOrdersUnion,
    ) -> Self {
        self.orders = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodRevenueUnion,
    ) -> Self {
        self.revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodNetRevenueUnion,
    ) -> Self {
        self.net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cumulative_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCumulativeRevenueUnion,
    ) -> Self {
        self.cumulative_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_cumulative_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodNetCumulativeRevenueUnion,
    ) -> Self {
        self.net_cumulative_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn costs(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCostsUnion,
    ) -> Self {
        self.costs = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cumulative_costs(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCumulativeCostsUnion,
    ) -> Self {
        self.cumulative_costs = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_order_value(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodAverageOrderValueUnion,
    ) -> Self {
        self.average_order_value = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_average_order_value(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodNetAverageOrderValueUnion,
    ) -> Self {
        self.net_average_order_value = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cost_per_user(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCostPerUserUnion,
    ) -> Self {
        self.cost_per_user = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn active_user_by_event(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodActiveUserByEventUnion,
    ) -> Self {
        self.active_user_by_event = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodOneTimeProductsUnion,
    ) -> Self {
        self.one_time_products = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodOneTimeProductsRevenueUnion,
    ) -> Self {
        self.one_time_products_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodOneTimeProductsNetRevenueUnion,
    ) -> Self {
        self.one_time_products_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodNewSubscriptionsUnion,
    ) -> Self {
        self.new_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodNewSubscriptionsRevenueUnion,
    ) -> Self {
        self.new_subscriptions_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodNewSubscriptionsNetRevenueUnion,
    ) -> Self {
        self.new_subscriptions_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodRenewedSubscriptionsUnion,
    ) -> Self {
        self.renewed_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodRenewedSubscriptionsRevenueUnion,
    ) -> Self {
        self.renewed_subscriptions_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodRenewedSubscriptionsNetRevenueUnion,
    ) -> Self {
        self.renewed_subscriptions_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsUnion,
    ) -> Self {
        self.canceled_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_customer_service(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsCustomerServiceUnion,
    ) -> Self {
        self.canceled_subscriptions_customer_service = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_low_quality(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsLowQualityUnion,
    ) -> Self {
        self.canceled_subscriptions_low_quality = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_missing_features(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsMissingFeaturesUnion,
    ) -> Self {
        self.canceled_subscriptions_missing_features = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_switched_service(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsSwitchedServiceUnion,
    ) -> Self {
        self.canceled_subscriptions_switched_service = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_too_complex(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsTooComplexUnion,
    ) -> Self {
        self.canceled_subscriptions_too_complex = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_too_expensive(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsTooExpensiveUnion,
    ) -> Self {
        self.canceled_subscriptions_too_expensive = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_unused(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsUnusedUnion,
    ) -> Self {
        self.canceled_subscriptions_unused = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_other(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCanceledSubscriptionsOtherUnion,
    ) -> Self {
        self.canceled_subscriptions_other = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn annual_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodAnnualRecurringRevenueUnion,
    ) -> Self {
        self.annual_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_annual_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCommittedAnnualRecurringRevenueUnion,
    ) -> Self {
        self.committed_annual_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checkouts_conversion(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCheckoutsConversionUnion,
    ) -> Self {
        self.checkouts_conversion = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn ltv(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodLtvUnion,
    ) -> Self {
        self.ltv = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn gross_margin(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodGrossMarginUnion,
    ) -> Self {
        self.gross_margin = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn gross_margin_percentage(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodGrossMarginPercentageUnion,
    ) -> Self {
        self.gross_margin_percentage = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cashflow(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodCashflowUnion,
    ) -> Self {
        self.cashflow = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_seats_per_customer(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodAverageSeatsPerCustomerUnion,
    ) -> Self {
        self.average_seats_per_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seat_utilization_rate(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricPeriodSeatUtilizationRateUnion,
    ) -> Self {
        self.seat_utilization_rate = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct Metrics {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub active_subscriptions:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_subscriptions:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub monthly_recurring_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_monthly_recurring_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_monthly_recurring_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_committed_monthly_recurring_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_revenue_per_user:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checkouts: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub succeeded_checkouts: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churned_subscriptions:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churn_rate: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_total: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_claimed: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_pending: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seat_customers: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_seat_customers: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churned_seat_customers:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub orders: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revenue: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_revenue: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cumulative_revenue: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_cumulative_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub costs: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cumulative_costs: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_order_value: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_average_order_value:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cost_per_user: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub active_user_by_event:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products_net_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions_net_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions_net_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_customer_service:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_low_quality:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_missing_features:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_switched_service:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_too_complex:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_too_expensive:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_unused:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_other:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub annual_recurring_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_annual_recurring_revenue:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checkouts_conversion:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub ltv: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub gross_margin: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub gross_margin_percentage:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cashflow: presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_seats_per_customer:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seat_utilization_rate:
        presence_rs::Presence<crate::v2027_01::models::metrics::types::Metric>,
}
#[allow(clippy::type_complexity)]
impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn active_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.active_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.committed_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn trial_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.trial_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.committed_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn trial_committed_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.trial_committed_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_revenue_per_user(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.average_revenue_per_user = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checkouts(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.checkouts = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn succeeded_checkouts(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.succeeded_checkouts = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churned_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.churned_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churn_rate(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.churn_rate = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_total(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.seats_total = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_claimed(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.seats_claimed = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_pending(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.seats_pending = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.new_seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churned_seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.churned_seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn orders(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.orders = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn revenue(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_revenue(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cumulative_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.cumulative_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_cumulative_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.net_cumulative_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn costs(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.costs = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cumulative_costs(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.cumulative_costs = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_order_value(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.average_order_value = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_average_order_value(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.net_average_order_value = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cost_per_user(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.cost_per_user = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn active_user_by_event(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.active_user_by_event = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.one_time_products = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.one_time_products_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.one_time_products_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.new_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.new_subscriptions_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.new_subscriptions_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.renewed_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.renewed_subscriptions_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.renewed_subscriptions_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_customer_service(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions_customer_service = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_low_quality(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions_low_quality = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_missing_features(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions_missing_features = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_switched_service(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions_switched_service = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_too_complex(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions_too_complex = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_too_expensive(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions_too_expensive = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_unused(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions_unused = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_other(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.canceled_subscriptions_other = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn annual_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.annual_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_annual_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.committed_annual_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checkouts_conversion(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.checkouts_conversion = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn ltv(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.ltv = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn gross_margin(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.gross_margin = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn gross_margin_percentage(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.gross_margin_percentage = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cashflow(mut self, value: crate::v2027_01::models::metrics::types::Metric) -> Self {
        self.cashflow = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_seats_per_customer(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.average_seats_per_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seat_utilization_rate(
        mut self,
        value: crate::v2027_01::models::metrics::types::Metric,
    ) -> Self {
        self.seat_utilization_rate = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MetricsIntervalLimitRequired {
    /// Minimum number of days for this interval.
    pub min_days: i64,
    /// Maximum number of days for this interval.
    pub max_days: i64,
}
/// Date interval limit to get metrics for a given interval.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MetricsIntervalLimit {
    /// Minimum number of days for this interval.
    pub min_days: i64,
    /// Maximum number of days for this interval.
    pub max_days: i64,
}
#[allow(clippy::type_complexity)]
impl MetricsIntervalLimit {
    pub fn new(required: MetricsIntervalLimitRequired) -> Self {
        Self { min_days: required.min_days, max_days: required.max_days }
    }
}
#[derive(Debug, Clone)]
pub struct MetricsIntervalsLimitsRequired {
    /// Limits for the hour interval.
    pub hour: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
    /// Limits for the day interval.
    pub day: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
    /// Limits for the week interval.
    pub week: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
    /// Limits for the month interval.
    pub month: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
    /// Limits for the year interval.
    pub year: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
}
/// Date interval limits to get metrics for each interval.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MetricsIntervalsLimits {
    /// Limits for the hour interval.
    pub hour: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
    /// Limits for the day interval.
    pub day: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
    /// Limits for the week interval.
    pub week: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
    /// Limits for the month interval.
    pub month: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
    /// Limits for the year interval.
    pub year: crate::v2027_01::models::metrics::types::MetricsIntervalLimit,
}
#[allow(clippy::type_complexity)]
impl MetricsIntervalsLimits {
    pub fn new(required: MetricsIntervalsLimitsRequired) -> Self {
        Self {
            hour: required.hour,
            day: required.day,
            week: required.week,
            month: required.month,
            year: required.year,
        }
    }
}
#[derive(Debug, Clone)]
pub struct MetricsLimitsRequired {
    /// Minimum date to get metrics.
    pub min_date: jiff::civil::Date,
    /// Limits for each interval.
    pub intervals: crate::v2027_01::models::metrics::types::MetricsIntervalsLimits,
}
/// Date limits to get metrics.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MetricsLimits {
    /// Minimum date to get metrics.
    pub min_date: jiff::civil::Date,
    /// Limits for each interval.
    pub intervals: crate::v2027_01::models::metrics::types::MetricsIntervalsLimits,
}
#[allow(clippy::type_complexity)]
impl MetricsLimits {
    pub fn new(required: MetricsLimitsRequired) -> Self {
        Self { min_date: required.min_date, intervals: required.intervals }
    }
}
#[derive(Debug, Clone)]
pub struct MetricsResponseRequired {
    /// List of data for each timestamp.
    pub periods: Vec<crate::v2027_01::models::metrics::types::MetricPeriod>,
    /// Totals for the whole selected period.
    pub totals: crate::v2027_01::models::metrics::types::MetricsTotals,
    /// Information about the returned metrics.
    pub metrics: crate::v2027_01::models::metrics::types::Metrics,
}
/// Metrics response schema.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MetricsResponse {
    /// List of data for each timestamp.
    pub periods: Vec<crate::v2027_01::models::metrics::types::MetricPeriod>,
    /// Totals for the whole selected period.
    pub totals: crate::v2027_01::models::metrics::types::MetricsTotals,
    /// Information about the returned metrics.
    pub metrics: crate::v2027_01::models::metrics::types::Metrics,
}
#[allow(clippy::type_complexity)]
impl MetricsResponse {
    pub fn new(required: MetricsResponseRequired) -> Self {
        Self { periods: required.periods, totals: required.totals, metrics: required.metrics }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct MetricsTotals {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub active_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsActiveSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCommittedSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub monthly_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsMonthlyRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_monthly_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsTrialMonthlyRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_monthly_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCommittedMonthlyRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_committed_monthly_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsTrialCommittedMonthlyRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_revenue_per_user: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsAverageRevenuePerUserUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checkouts: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCheckoutsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub succeeded_checkouts: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsSucceededCheckoutsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churned_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsChurnedSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churn_rate: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsChurnRateUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_total: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsSeatsTotalUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_claimed: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsSeatsClaimedUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats_pending: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsSeatsPendingUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seat_customers: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsSeatCustomersUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_seat_customers: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsNewSeatCustomersUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub churned_seat_customers: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsChurnedSeatCustomersUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub orders: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsOrdersUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsNetRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cumulative_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCumulativeRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_cumulative_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsNetCumulativeRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub costs: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCostsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cumulative_costs: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCumulativeCostsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_order_value: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsAverageOrderValueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub net_average_order_value: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsNetAverageOrderValueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cost_per_user: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCostPerUserUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub active_user_by_event: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsActiveUserByEventUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsOneTimeProductsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsOneTimeProductsRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub one_time_products_net_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsOneTimeProductsNetRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsNewSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsNewSubscriptionsRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub new_subscriptions_net_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsNewSubscriptionsNetRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsRenewedSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsRenewedSubscriptionsRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub renewed_subscriptions_net_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsRenewedSubscriptionsNetRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_customer_service: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsCustomerServiceUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_low_quality: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsLowQualityUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_missing_features: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsMissingFeaturesUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_switched_service: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsSwitchedServiceUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_too_complex: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsTooComplexUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_too_expensive: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsTooExpensiveUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_unused: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsUnusedUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub canceled_subscriptions_other: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsOtherUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub annual_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsAnnualRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub committed_annual_recurring_revenue: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCommittedAnnualRecurringRevenueUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checkouts_conversion: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCheckoutsConversionUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub ltv: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsLtvUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub gross_margin: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsGrossMarginUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub gross_margin_percentage: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsGrossMarginPercentageUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cashflow: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsCashflowUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub average_seats_per_customer: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsAverageSeatsPerCustomerUnion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seat_utilization_rate: presence_rs::Presence<
        crate::v2027_01::models::metrics::unions::MetricsTotalsSeatUtilizationRateUnion,
    >,
}
#[allow(clippy::type_complexity)]
impl MetricsTotals {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn active_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsActiveSubscriptionsUnion,
    ) -> Self {
        self.active_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCommittedSubscriptionsUnion,
    ) -> Self {
        self.committed_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsMonthlyRecurringRevenueUnion,
    ) -> Self {
        self.monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn trial_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsTrialMonthlyRecurringRevenueUnion,
    ) -> Self {
        self.trial_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCommittedMonthlyRecurringRevenueUnion,
    ) -> Self {
        self.committed_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn trial_committed_monthly_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsTrialCommittedMonthlyRecurringRevenueUnion,
    ) -> Self {
        self.trial_committed_monthly_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_revenue_per_user(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsAverageRevenuePerUserUnion,
    ) -> Self {
        self.average_revenue_per_user = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checkouts(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCheckoutsUnion,
    ) -> Self {
        self.checkouts = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn succeeded_checkouts(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsSucceededCheckoutsUnion,
    ) -> Self {
        self.succeeded_checkouts = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churned_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsChurnedSubscriptionsUnion,
    ) -> Self {
        self.churned_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churn_rate(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsChurnRateUnion,
    ) -> Self {
        self.churn_rate = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_total(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsSeatsTotalUnion,
    ) -> Self {
        self.seats_total = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_claimed(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsSeatsClaimedUnion,
    ) -> Self {
        self.seats_claimed = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats_pending(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsSeatsPendingUnion,
    ) -> Self {
        self.seats_pending = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsSeatCustomersUnion,
    ) -> Self {
        self.seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsNewSeatCustomersUnion,
    ) -> Self {
        self.new_seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn churned_seat_customers(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsChurnedSeatCustomersUnion,
    ) -> Self {
        self.churned_seat_customers = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn orders(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsOrdersUnion,
    ) -> Self {
        self.orders = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsRevenueUnion,
    ) -> Self {
        self.revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsNetRevenueUnion,
    ) -> Self {
        self.net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cumulative_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCumulativeRevenueUnion,
    ) -> Self {
        self.cumulative_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_cumulative_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsNetCumulativeRevenueUnion,
    ) -> Self {
        self.net_cumulative_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn costs(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCostsUnion,
    ) -> Self {
        self.costs = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cumulative_costs(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCumulativeCostsUnion,
    ) -> Self {
        self.cumulative_costs = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_order_value(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsAverageOrderValueUnion,
    ) -> Self {
        self.average_order_value = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_average_order_value(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsNetAverageOrderValueUnion,
    ) -> Self {
        self.net_average_order_value = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cost_per_user(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCostPerUserUnion,
    ) -> Self {
        self.cost_per_user = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn active_user_by_event(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsActiveUserByEventUnion,
    ) -> Self {
        self.active_user_by_event = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsOneTimeProductsUnion,
    ) -> Self {
        self.one_time_products = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsOneTimeProductsRevenueUnion,
    ) -> Self {
        self.one_time_products_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn one_time_products_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsOneTimeProductsNetRevenueUnion,
    ) -> Self {
        self.one_time_products_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsNewSubscriptionsUnion,
    ) -> Self {
        self.new_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsNewSubscriptionsRevenueUnion,
    ) -> Self {
        self.new_subscriptions_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_subscriptions_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsNewSubscriptionsNetRevenueUnion,
    ) -> Self {
        self.new_subscriptions_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsRenewedSubscriptionsUnion,
    ) -> Self {
        self.renewed_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsRenewedSubscriptionsRevenueUnion,
    ) -> Self {
        self.renewed_subscriptions_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn renewed_subscriptions_net_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsRenewedSubscriptionsNetRevenueUnion,
    ) -> Self {
        self.renewed_subscriptions_net_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsUnion,
    ) -> Self {
        self.canceled_subscriptions = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_customer_service(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsCustomerServiceUnion,
    ) -> Self {
        self.canceled_subscriptions_customer_service = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_low_quality(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsLowQualityUnion,
    ) -> Self {
        self.canceled_subscriptions_low_quality = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_missing_features(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsMissingFeaturesUnion,
    ) -> Self {
        self.canceled_subscriptions_missing_features = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_switched_service(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsSwitchedServiceUnion,
    ) -> Self {
        self.canceled_subscriptions_switched_service = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_too_complex(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsTooComplexUnion,
    ) -> Self {
        self.canceled_subscriptions_too_complex = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_too_expensive(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsTooExpensiveUnion,
    ) -> Self {
        self.canceled_subscriptions_too_expensive = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_unused(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsUnusedUnion,
    ) -> Self {
        self.canceled_subscriptions_unused = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn canceled_subscriptions_other(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCanceledSubscriptionsOtherUnion,
    ) -> Self {
        self.canceled_subscriptions_other = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn annual_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsAnnualRecurringRevenueUnion,
    ) -> Self {
        self.annual_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn committed_annual_recurring_revenue(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCommittedAnnualRecurringRevenueUnion,
    ) -> Self {
        self.committed_annual_recurring_revenue = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checkouts_conversion(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCheckoutsConversionUnion,
    ) -> Self {
        self.checkouts_conversion = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn ltv(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsLtvUnion,
    ) -> Self {
        self.ltv = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn gross_margin(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsGrossMarginUnion,
    ) -> Self {
        self.gross_margin = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn gross_margin_percentage(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsGrossMarginPercentageUnion,
    ) -> Self {
        self.gross_margin_percentage = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cashflow(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsCashflowUnion,
    ) -> Self {
        self.cashflow = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn average_seats_per_customer(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsAverageSeatsPerCustomerUnion,
    ) -> Self {
        self.average_seats_per_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seat_utilization_rate(
        mut self,
        value: crate::v2027_01::models::metrics::unions::MetricsTotalsSeatUtilizationRateUnion,
    ) -> Self {
        self.seat_utilization_rate = presence_rs::Presence::Some(value);
        self
    }
}
