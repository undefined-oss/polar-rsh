#[derive(Debug, Clone)]
pub struct BenefitGrantCustomWebhookRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    pub benefit: crate::v2026_04::models::common::types::BenefitCustom,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantCustomProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantCustomWebhook {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantError>,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2026_04::models::common::types::Member>,
    pub benefit: crate::v2026_04::models::common::types::BenefitCustom,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantCustomProperties,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_properties:
        presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantCustomProperties>,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantCustomWebhook {
    pub fn new(required: BenefitGrantCustomWebhookRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
            previous_properties: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2026_04::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn previous_properties(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantCustomProperties,
    ) -> Self {
        self.previous_properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantDiscordWebhookRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    pub benefit: crate::v2026_04::models::common::types::BenefitDiscord,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantDiscordProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantDiscordWebhook {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantError>,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2026_04::models::common::types::Member>,
    pub benefit: crate::v2026_04::models::common::types::BenefitDiscord,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantDiscordProperties,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::BenefitGrantDiscordProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantDiscordWebhook {
    pub fn new(required: BenefitGrantDiscordWebhookRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
            previous_properties: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2026_04::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn previous_properties(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantDiscordProperties,
    ) -> Self {
        self.previous_properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantDownloadablesWebhookRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    pub benefit: crate::v2026_04::models::common::types::BenefitDownloadables,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantDownloadablesProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantDownloadablesWebhook {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantError>,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2026_04::models::common::types::Member>,
    pub benefit: crate::v2026_04::models::common::types::BenefitDownloadables,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantDownloadablesProperties,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::BenefitGrantDownloadablesProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantDownloadablesWebhook {
    pub fn new(required: BenefitGrantDownloadablesWebhookRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
            previous_properties: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2026_04::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn previous_properties(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantDownloadablesProperties,
    ) -> Self {
        self.previous_properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantFeatureFlagWebhookRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    pub benefit: crate::v2026_04::models::common::types::BenefitFeatureFlag,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantFeatureFlagProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantFeatureFlagWebhook {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantError>,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2026_04::models::common::types::Member>,
    pub benefit: crate::v2026_04::models::common::types::BenefitFeatureFlag,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantFeatureFlagProperties,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::BenefitGrantFeatureFlagProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantFeatureFlagWebhook {
    pub fn new(required: BenefitGrantFeatureFlagWebhookRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
            previous_properties: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2026_04::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn previous_properties(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantFeatureFlagProperties,
    ) -> Self {
        self.previous_properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantGitHubRepositoryWebhookRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    pub benefit: crate::v2026_04::models::common::types::BenefitGitHubRepository,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantGitHubRepositoryProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantGitHubRepositoryWebhook {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantError>,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2026_04::models::common::types::Member>,
    pub benefit: crate::v2026_04::models::common::types::BenefitGitHubRepository,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantGitHubRepositoryProperties,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::BenefitGrantGitHubRepositoryProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantGitHubRepositoryWebhook {
    pub fn new(required: BenefitGrantGitHubRepositoryWebhookRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
            previous_properties: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2026_04::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn previous_properties(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantGitHubRepositoryProperties,
    ) -> Self {
        self.previous_properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantLicenseKeysWebhookRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    pub benefit: crate::v2026_04::models::common::types::BenefitLicenseKeys,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantLicenseKeysProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantLicenseKeysWebhook {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantError>,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2026_04::models::common::types::Member>,
    pub benefit: crate::v2026_04::models::common::types::BenefitLicenseKeys,
    pub properties: crate::v2026_04::models::common::types::BenefitGrantLicenseKeysProperties,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::BenefitGrantLicenseKeysProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantLicenseKeysWebhook {
    pub fn new(required: BenefitGrantLicenseKeysWebhookRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
            previous_properties: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2026_04::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn previous_properties(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantLicenseKeysProperties,
    ) -> Self {
        self.previous_properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantMeterCreditWebhookRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    pub benefit: crate::v2026_04::models::common::types::BenefitMeterCredit,
    pub properties:
        crate::v2026_04::models::benefit_grants::types::BenefitGrantMeterCreditProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantMeterCreditWebhook {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantError>,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2026_04::models::common::types::Member>,
    pub benefit: crate::v2026_04::models::common::types::BenefitMeterCredit,
    pub properties:
        crate::v2026_04::models::benefit_grants::types::BenefitGrantMeterCreditProperties,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_properties: presence_rs::Presence<
        crate::v2026_04::models::benefit_grants::types::BenefitGrantMeterCreditProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantMeterCreditWebhook {
    pub fn new(required: BenefitGrantMeterCreditWebhookRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
            previous_properties: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2026_04::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn previous_properties(
        mut self,
        value: crate::v2026_04::models::benefit_grants::types::BenefitGrantMeterCreditProperties,
    ) -> Self {
        self.previous_properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantSlackSharedChannelWebhookRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    pub benefit: crate::v2026_04::models::common::types::BenefitSlackSharedChannel,
    pub properties:
        crate::v2026_04::models::common::types::BenefitGrantSlackSharedChannelProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantSlackSharedChannelWebhook {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::BenefitGrantWebhookId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_04::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2026_04::models::common::types::BenefitGrantError>,
    pub customer: crate::v2026_04::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2026_04::models::common::types::Member>,
    pub benefit: crate::v2026_04::models::common::types::BenefitSlackSharedChannel,
    pub properties:
        crate::v2026_04::models::common::types::BenefitGrantSlackSharedChannelProperties,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::BenefitGrantSlackSharedChannelProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantSlackSharedChannelWebhook {
    pub fn new(required: BenefitGrantSlackSharedChannelWebhookRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
            previous_properties: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2026_04::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn previous_properties(
        mut self,
        value: crate::v2026_04::models::common::types::BenefitGrantSlackSharedChannelProperties,
    ) -> Self {
        self.previous_properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct WebhookBenefitCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Benefit,
}
/// Sent when a new benefit is created.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookBenefitCreatedPayload {
    pub r#type: monostate::MustBe!("benefit.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Benefit,
}
#[allow(clippy::type_complexity)]
impl WebhookBenefitCreatedPayload {
    pub fn new(required: WebhookBenefitCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookBenefitGrantCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::webhooks::unions::BenefitGrantWebhook,
}
/// Sent when a new benefit grant is created.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookBenefitGrantCreatedPayload {
    pub r#type: monostate::MustBe!("benefit_grant.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::webhooks::unions::BenefitGrantWebhook,
}
#[allow(clippy::type_complexity)]
impl WebhookBenefitGrantCreatedPayload {
    pub fn new(required: WebhookBenefitGrantCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookBenefitGrantCycledPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::webhooks::unions::BenefitGrantWebhook,
}
/// Sent when a benefit grant is cycled,
/// meaning the related subscription has been renewed for another period.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookBenefitGrantCycledPayload {
    pub r#type: monostate::MustBe!("benefit_grant.cycled"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::webhooks::unions::BenefitGrantWebhook,
}
#[allow(clippy::type_complexity)]
impl WebhookBenefitGrantCycledPayload {
    pub fn new(required: WebhookBenefitGrantCycledPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookBenefitGrantRevokedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::webhooks::unions::BenefitGrantWebhook,
}
/// Sent when a benefit grant is revoked.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookBenefitGrantRevokedPayload {
    pub r#type: monostate::MustBe!("benefit_grant.revoked"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::webhooks::unions::BenefitGrantWebhook,
}
#[allow(clippy::type_complexity)]
impl WebhookBenefitGrantRevokedPayload {
    pub fn new(required: WebhookBenefitGrantRevokedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookBenefitGrantUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::webhooks::unions::BenefitGrantWebhook,
}
/// Sent when a benefit grant is updated.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookBenefitGrantUpdatedPayload {
    pub r#type: monostate::MustBe!("benefit_grant.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::webhooks::unions::BenefitGrantWebhook,
}
#[allow(clippy::type_complexity)]
impl WebhookBenefitGrantUpdatedPayload {
    pub fn new(required: WebhookBenefitGrantUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookBenefitUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Benefit,
}
/// Sent when a benefit is updated.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookBenefitUpdatedPayload {
    pub r#type: monostate::MustBe!("benefit.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Benefit,
}
#[allow(clippy::type_complexity)]
impl WebhookBenefitUpdatedPayload {
    pub fn new(required: WebhookBenefitUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCheckoutCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::checkouts::types::Checkout,
}
/// Sent when a new checkout is created.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCheckoutCreatedPayload {
    pub r#type: monostate::MustBe!("checkout.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::checkouts::types::Checkout,
}
#[allow(clippy::type_complexity)]
impl WebhookCheckoutCreatedPayload {
    pub fn new(required: WebhookCheckoutCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCheckoutExpiredPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::checkouts::types::Checkout,
}
/// Sent when a checkout expires.
///
/// This event fires when a checkout reaches its expiration time without being
/// completed. Developers can use this to send reminder emails or track checkout
/// abandonment.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCheckoutExpiredPayload {
    pub r#type: monostate::MustBe!("checkout.expired"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::checkouts::types::Checkout,
}
#[allow(clippy::type_complexity)]
impl WebhookCheckoutExpiredPayload {
    pub fn new(required: WebhookCheckoutExpiredPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCheckoutUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::checkouts::types::Checkout,
}
/// Sent when a checkout is updated.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCheckoutUpdatedPayload {
    pub r#type: monostate::MustBe!("checkout.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::checkouts::types::Checkout,
}
#[allow(clippy::type_complexity)]
impl WebhookCheckoutUpdatedPayload {
    pub fn new(required: WebhookCheckoutUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCustomerCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Customer,
}
/// Sent when a new customer is created.
///
/// A customer can be created:
///
/// * After a successful checkout.
/// * Programmatically via the API.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCustomerCreatedPayload {
    pub r#type: monostate::MustBe!("customer.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Customer,
}
#[allow(clippy::type_complexity)]
impl WebhookCustomerCreatedPayload {
    pub fn new(required: WebhookCustomerCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCustomerDeletedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Customer,
}
/// Sent when a customer is deleted.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCustomerDeletedPayload {
    pub r#type: monostate::MustBe!("customer.deleted"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Customer,
}
#[allow(clippy::type_complexity)]
impl WebhookCustomerDeletedPayload {
    pub fn new(required: WebhookCustomerDeletedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCustomerSeatAssignedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::CustomerSeat,
}
/// Sent when a new customer seat is assigned.
///
/// This event is triggered when a seat is assigned to a customer by the
/// organization. The customer will receive an invitation email to claim the
/// seat.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCustomerSeatAssignedPayload {
    pub r#type: monostate::MustBe!("customer_seat.assigned"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::CustomerSeat,
}
#[allow(clippy::type_complexity)]
impl WebhookCustomerSeatAssignedPayload {
    pub fn new(required: WebhookCustomerSeatAssignedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCustomerSeatClaimedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::CustomerSeat,
}
/// Sent when a customer seat is claimed.
///
/// This event is triggered when a customer accepts the seat invitation and
/// claims their access.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCustomerSeatClaimedPayload {
    pub r#type: monostate::MustBe!("customer_seat.claimed"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::CustomerSeat,
}
#[allow(clippy::type_complexity)]
impl WebhookCustomerSeatClaimedPayload {
    pub fn new(required: WebhookCustomerSeatClaimedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCustomerSeatRevokedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::CustomerSeat,
}
/// Sent when a customer seat is revoked.
///
/// This event is triggered when access to a seat is revoked, either manually by
/// the organization or automatically when a subscription is canceled.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCustomerSeatRevokedPayload {
    pub r#type: monostate::MustBe!("customer_seat.revoked"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::CustomerSeat,
}
#[allow(clippy::type_complexity)]
impl WebhookCustomerSeatRevokedPayload {
    pub fn new(required: WebhookCustomerSeatRevokedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCustomerStateChangedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::customers::unions::CustomerState,
}
/// Sent when a customer state has changed.
///
/// It's triggered when:
///
/// * Customer is created, updated or deleted.
/// * A subscription is created or updated.
/// * A benefit is granted or revoked.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCustomerStateChangedPayload {
    pub r#type: monostate::MustBe!("customer.state_changed"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::customers::unions::CustomerState,
}
#[allow(clippy::type_complexity)]
impl WebhookCustomerStateChangedPayload {
    pub fn new(required: WebhookCustomerStateChangedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookCustomerUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Customer,
}
/// Sent when a customer is updated.
///
/// This event is fired when the customer details are updated.
///
/// If you want to be notified when a customer subscription or benefit state
/// changes, you should listen to the `customer_state_changed` event.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookCustomerUpdatedPayload {
    pub r#type: monostate::MustBe!("customer.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::unions::Customer,
}
#[allow(clippy::type_complexity)]
impl WebhookCustomerUpdatedPayload {
    pub fn new(required: WebhookCustomerUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookDeliveryRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::DeliveryId,
    /// Whether the delivery was successful.
    pub succeeded: bool,
    /// The HTTP code returned by the URL. `null` if the endpoint was
    /// unreachable.
    pub http_code: Option<i64>,
    /// The response body returned by the URL, or the error message if the
    /// endpoint was unreachable.
    pub response: Option<String>,
    /// The webhook event sent by this delivery.
    pub webhook_event: crate::v2026_04::models::webhooks::types::WebhookEvent,
}
/// A webhook delivery for a webhook event.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookDelivery {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::DeliveryId,
    /// Whether the delivery was successful.
    pub succeeded: bool,
    /// The HTTP code returned by the URL. `null` if the endpoint was
    /// unreachable.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub http_code: Option<i64>,
    /// The response body returned by the URL, or the error message if the
    /// endpoint was unreachable.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub response: Option<String>,
    /// The webhook event sent by this delivery.
    pub webhook_event: crate::v2026_04::models::webhooks::types::WebhookEvent,
}
#[allow(clippy::type_complexity)]
impl WebhookDelivery {
    pub fn new(required: WebhookDeliveryRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            succeeded: required.succeeded,
            http_code: required.http_code,
            response: required.response,
            webhook_event: required.webhook_event,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookDiscountCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::discounts::unions::Discount,
}
/// Sent when a new discount is created.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookDiscountCreatedPayload {
    pub r#type: monostate::MustBe!("discount.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::discounts::unions::Discount,
}
#[allow(clippy::type_complexity)]
impl WebhookDiscountCreatedPayload {
    pub fn new(required: WebhookDiscountCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookDiscountDeletedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::discounts::unions::Discount,
}
/// Sent when a discount is deleted.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookDiscountDeletedPayload {
    pub r#type: monostate::MustBe!("discount.deleted"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::discounts::unions::Discount,
}
#[allow(clippy::type_complexity)]
impl WebhookDiscountDeletedPayload {
    pub fn new(required: WebhookDiscountDeletedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookDiscountUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::discounts::unions::Discount,
}
/// Sent when a discount is updated.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookDiscountUpdatedPayload {
    pub r#type: monostate::MustBe!("discount.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::discounts::unions::Discount,
}
#[allow(clippy::type_complexity)]
impl WebhookDiscountUpdatedPayload {
    pub fn new(required: WebhookDiscountUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookEndpointRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::EndpointId,
    /// The URL where the webhook events will be sent.
    pub url: String,
    /// The API version that'll be used in event payloads.
    pub api_version: String,
    /// The format of the webhook payload.
    pub format: crate::v2026_04::models::webhooks::enums::WebhookFormat,
    /// The secret used to sign the webhook events.
    pub secret: String,
    /// The organization ID associated with the webhook endpoint.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// The events that will trigger the webhook.
    pub events: Vec<crate::v2026_04::models::webhooks::enums::WebhookEventType>,
    /// Whether the webhook endpoint is enabled and will receive events.
    pub enabled: bool,
    /// Whether Polar signs deliveries to this endpoint with Standard Webhooks.
    /// False means Polar's original HMAC over the UTF-8 bytes of the full
    /// secret.
    pub uses_standard_webhook_signature: bool,
}
/// A webhook endpoint.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookEndpoint {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::EndpointId,
    /// The URL where the webhook events will be sent.
    pub url: String,
    /// An optional name for the webhook endpoint to help organize and identify
    /// it.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<String>,
    /// The API version that'll be used in event payloads.
    pub api_version: String,
    /// The format of the webhook payload.
    pub format: crate::v2026_04::models::webhooks::enums::WebhookFormat,
    /// The secret used to sign the webhook events.
    pub secret: String,
    /// The organization ID associated with the webhook endpoint.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// The events that will trigger the webhook.
    pub events: Vec<crate::v2026_04::models::webhooks::enums::WebhookEventType>,
    /// Whether the webhook endpoint is enabled and will receive events.
    pub enabled: bool,
    /// Whether Polar signs deliveries to this endpoint with Standard Webhooks.
    /// False means Polar's original HMAC over the UTF-8 bytes of the full
    /// secret.
    pub uses_standard_webhook_signature: bool,
}
#[allow(clippy::type_complexity)]
impl WebhookEndpoint {
    pub fn new(required: WebhookEndpointRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            url: required.url,
            name: presence_rs::Presence::Absent,
            api_version: required.api_version,
            format: required.format,
            secret: required.secret,
            organization_id: required.organization_id,
            events: required.events,
            enabled: required.enabled,
            uses_standard_webhook_signature: required.uses_standard_webhook_signature,
        }
    }

    /// An optional name for the webhook endpoint to help organize and identify
    /// it.
    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct WebhookEndpointCreateRequired {
    /// The URL where the webhook events will be sent.
    pub url: crate::v2026_04::models::webhooks::newtypes::WebhookEndpointUrl,
    /// The format of the webhook payload.
    pub format: crate::v2026_04::models::webhooks::enums::WebhookFormat,
    /// The events that will trigger the webhook.
    pub events: Vec<crate::v2026_04::models::webhooks::enums::WebhookEventType>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookEndpointCreate {
    /// The URL where the webhook events will be sent.
    pub url: crate::v2026_04::models::webhooks::newtypes::WebhookEndpointUrl,
    /// An optional name for the webhook endpoint to help organize and identify
    /// it.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<String>,
    /// The API version that'll be used in event payloads.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub api_version:
        Option<crate::v2026_04::models::webhooks::enums::WebhookEndpointCreateApiVersion>,
    /// The format of the webhook payload.
    pub format: crate::v2026_04::models::webhooks::enums::WebhookFormat,
    /// The events that will trigger the webhook.
    pub events: Vec<crate::v2026_04::models::webhooks::enums::WebhookEventType>,
    /// The organization ID associated with the webhook endpoint. **Required
    /// unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2026_04::models::common::ids::OrganizationId>,
}
#[allow(clippy::type_complexity)]
impl WebhookEndpointCreate {
    pub fn new(required: WebhookEndpointCreateRequired) -> Self {
        Self {
            url: required.url,
            name: presence_rs::Presence::Absent,
            api_version: None,
            format: required.format,
            events: required.events,
            organization_id: presence_rs::Presence::Absent,
        }
    }

    /// An optional name for the webhook endpoint to help organize and identify
    /// it.
    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = presence_rs::Presence::Some(value.into());
        self
    }

    /// The API version that'll be used in event payloads.
    #[allow(clippy::should_implement_trait)]
    pub fn api_version(
        mut self,
        value: crate::v2026_04::models::webhooks::enums::WebhookEndpointCreateApiVersion,
    ) -> Self {
        self.api_version = Some(value);
        self
    }

    /// The organization ID associated with the webhook endpoint. **Required
    /// unless you use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct WebhookEndpointUpdate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub url: presence_rs::Presence<crate::v2026_04::models::webhooks::newtypes::WebhookEndpointUrl>,
    /// An optional name for the webhook endpoint to help organize and identify
    /// it.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<String>,
    /// The API version that'll be used in event payloads.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub api_version: presence_rs::Presence<
        crate::v2026_04::models::webhooks::enums::WebhookEndpointUpdateApiVersion,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub format: presence_rs::Presence<crate::v2026_04::models::webhooks::enums::WebhookFormat>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub events:
        presence_rs::Presence<Vec<crate::v2026_04::models::webhooks::enums::WebhookEventType>>,
    /// Whether the webhook endpoint is enabled.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub enabled: presence_rs::Presence<bool>,
}
#[allow(clippy::type_complexity)]
impl WebhookEndpointUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn url(
        mut self,
        value: crate::v2026_04::models::webhooks::newtypes::WebhookEndpointUrl,
    ) -> Self {
        self.url = presence_rs::Presence::Some(value);
        self
    }

    /// An optional name for the webhook endpoint to help organize and identify
    /// it.
    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = presence_rs::Presence::Some(value.into());
        self
    }

    /// The API version that'll be used in event payloads.
    #[allow(clippy::should_implement_trait)]
    pub fn api_version(
        mut self,
        value: crate::v2026_04::models::webhooks::enums::WebhookEndpointUpdateApiVersion,
    ) -> Self {
        self.api_version = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn format(
        mut self,
        value: crate::v2026_04::models::webhooks::enums::WebhookFormat,
    ) -> Self {
        self.format = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn events(
        mut self,
        value: Vec<crate::v2026_04::models::webhooks::enums::WebhookEventType>,
    ) -> Self {
        self.events = presence_rs::Presence::Some(value);
        self
    }

    /// Whether the webhook endpoint is enabled.
    #[allow(clippy::should_implement_trait)]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct WebhookEventRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::EventId,
    /// Whether this event was skipped because the webhook endpoint was
    /// disabled.
    pub skipped: bool,
    /// The API version used in the payload of this event.
    pub api_version: String,
    /// The payload of the webhook event.
    pub payload: Option<String>,
    /// The type of the webhook event.
    pub r#type: crate::v2026_04::models::webhooks::enums::WebhookEventType,
    /// Whether this event is archived. Archived events can't be redelivered,
    /// and the payload is not accessible anymore.
    pub is_archived: bool,
}
/// A webhook event.
///
/// An event represent something that happened in the system
/// that should be sent to the webhook endpoint.
///
/// It can be delivered multiple times until it's marked as succeeded,
/// each one creating a new delivery.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookEvent {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::EventId,
    /// Last HTTP code returned by the URL. `null` if no delviery has been
    /// attempted or if the endpoint was unreachable.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub last_http_code: presence_rs::Presence<i64>,
    /// Whether this event was successfully delivered. `null` if no delivery has
    /// been attempted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub succeeded: presence_rs::Presence<bool>,
    /// Whether this event was skipped because the webhook endpoint was
    /// disabled.
    pub skipped: bool,
    /// The API version used in the payload of this event.
    pub api_version: String,
    /// The payload of the webhook event.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub payload: Option<String>,
    /// The type of the webhook event.
    pub r#type: crate::v2026_04::models::webhooks::enums::WebhookEventType,
    /// Whether this event is archived. Archived events can't be redelivered,
    /// and the payload is not accessible anymore.
    pub is_archived: bool,
}
#[allow(clippy::type_complexity)]
impl WebhookEvent {
    pub fn new(required: WebhookEventRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            last_http_code: presence_rs::Presence::Absent,
            succeeded: presence_rs::Presence::Absent,
            skipped: required.skipped,
            api_version: required.api_version,
            payload: required.payload,
            r#type: required.r#type,
            is_archived: required.is_archived,
        }
    }

    /// Last HTTP code returned by the URL. `null` if no delviery has been
    /// attempted or if the endpoint was unreachable.
    #[allow(clippy::should_implement_trait)]
    pub fn last_http_code(mut self, value: i64) -> Self {
        self.last_http_code = presence_rs::Presence::Some(value);
        self
    }

    /// Whether this event was successfully delivered. `null` if no delivery has
    /// been attempted.
    #[allow(clippy::should_implement_trait)]
    pub fn succeeded(mut self, value: bool) -> Self {
        self.succeeded = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct WebhookMemberCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Member,
}
/// Sent when a new member is created.
///
/// A member represents an individual within a customer (team).
/// This event is triggered when a member is added to a customer,
/// either programmatically via the API or when an owner is automatically
/// created for a new customer.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookMemberCreatedPayload {
    pub r#type: monostate::MustBe!("member.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Member,
}
#[allow(clippy::type_complexity)]
impl WebhookMemberCreatedPayload {
    pub fn new(required: WebhookMemberCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookMemberDeletedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Member,
}
/// Sent when a member is deleted.
///
/// This event is triggered when a member is removed from a customer.
/// Any active seats assigned to the member will be automatically revoked.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookMemberDeletedPayload {
    pub r#type: monostate::MustBe!("member.deleted"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Member,
}
#[allow(clippy::type_complexity)]
impl WebhookMemberDeletedPayload {
    pub fn new(required: WebhookMemberDeletedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookMemberUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Member,
}
/// Sent when a member is updated.
///
/// This event is triggered when member details are updated,
/// such as their name or role within the customer.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookMemberUpdatedPayload {
    pub r#type: monostate::MustBe!("member.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Member,
}
#[allow(clippy::type_complexity)]
impl WebhookMemberUpdatedPayload {
    pub fn new(required: WebhookMemberUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookOrderCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::orders::types::Order,
}
/// Sent when a new order is created.
///
/// A new order is created when:
///
/// * A customer purchases a one-time product. In this case, `billing_reason` is
///   set to `purchase`.
/// * A customer starts a subscription. In this case, `billing_reason` is set to
///   `subscription_create`.
/// * A subscription is renewed. In this case, `billing_reason` is set to
///   `subscription_cycle`.
/// * A subscription is upgraded or downgraded with an immediate proration
///   invoice. In this case, `billing_reason` is set to `subscription_update`.
///
/// > [!WARNING]
/// > The order might not be paid yet, so the `status` field might be `pending`.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookOrderCreatedPayload {
    pub r#type: monostate::MustBe!("order.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::orders::types::Order,
}
#[allow(clippy::type_complexity)]
impl WebhookOrderCreatedPayload {
    pub fn new(required: WebhookOrderCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookOrderPaidPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::orders::types::Order,
}
/// Sent when an order is paid.
///
/// When you receive this event, the order is fully processed and payment has
/// been received.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookOrderPaidPayload {
    pub r#type: monostate::MustBe!("order.paid"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::orders::types::Order,
}
#[allow(clippy::type_complexity)]
impl WebhookOrderPaidPayload {
    pub fn new(required: WebhookOrderPaidPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookOrderRefundedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::orders::types::Order,
}
/// Sent when an order is fully or partially refunded.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookOrderRefundedPayload {
    pub r#type: monostate::MustBe!("order.refunded"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::orders::types::Order,
}
#[allow(clippy::type_complexity)]
impl WebhookOrderRefundedPayload {
    pub fn new(required: WebhookOrderRefundedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookOrderUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::orders::types::Order,
}
/// Sent when an order is updated.
///
/// An order is updated when:
///
/// * Its status changes, e.g. from `pending` to `paid`.
/// * It's refunded, partially or fully.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookOrderUpdatedPayload {
    pub r#type: monostate::MustBe!("order.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::orders::types::Order,
}
#[allow(clippy::type_complexity)]
impl WebhookOrderUpdatedPayload {
    pub fn new(required: WebhookOrderUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookOrganizationUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::organizations::types::Organization,
}
/// Sent when a organization is updated.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookOrganizationUpdatedPayload {
    pub r#type: monostate::MustBe!("organization.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::organizations::types::Organization,
}
#[allow(clippy::type_complexity)]
impl WebhookOrganizationUpdatedPayload {
    pub fn new(required: WebhookOrganizationUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookProductCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Product,
}
/// Sent when a new product is created.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookProductCreatedPayload {
    pub r#type: monostate::MustBe!("product.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Product,
}
#[allow(clippy::type_complexity)]
impl WebhookProductCreatedPayload {
    pub fn new(required: WebhookProductCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookProductUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Product,
}
/// Sent when a product is updated.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookProductUpdatedPayload {
    pub r#type: monostate::MustBe!("product.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::common::types::Product,
}
#[allow(clippy::type_complexity)]
impl WebhookProductUpdatedPayload {
    pub fn new(required: WebhookProductUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookRefundCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::refunds::types::Refund,
}
/// Sent when a refund is created regardless of status.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookRefundCreatedPayload {
    pub r#type: monostate::MustBe!("refund.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::refunds::types::Refund,
}
#[allow(clippy::type_complexity)]
impl WebhookRefundCreatedPayload {
    pub fn new(required: WebhookRefundCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookRefundUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::refunds::types::Refund,
}
/// Sent when a refund is updated.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookRefundUpdatedPayload {
    pub r#type: monostate::MustBe!("refund.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::refunds::types::Refund,
}
#[allow(clippy::type_complexity)]
impl WebhookRefundUpdatedPayload {
    pub fn new(required: WebhookRefundUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionActivePayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a subscription becomes active,
/// whether because it's a new paid subscription or because payment was
/// recovered.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionActivePayload {
    pub r#type: monostate::MustBe!("subscription.active"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionActivePayload {
    pub fn new(required: WebhookSubscriptionActivePayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionCanceledPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a subscription is canceled.
/// Customers might still have access until the end of the current period.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionCanceledPayload {
    pub r#type: monostate::MustBe!("subscription.canceled"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionCanceledPayload {
    pub fn new(required: WebhookSubscriptionCanceledPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionCreatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a new subscription is created.
///
/// When this event occurs, the subscription `status` might not be `active` yet,
/// as we can still have to wait for the first payment to be processed.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionCreatedPayload {
    pub r#type: monostate::MustBe!("subscription.created"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionCreatedPayload {
    pub fn new(required: WebhookSubscriptionCreatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionCycledPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a subscription enters a new billing period.
///
/// The payload carries the new `current_period_start` and `current_period_end`.
/// It fires when the period rolls over, before the renewal order exists and
/// regardless of whether the renewal payment succeeds — listen to `order.paid`
/// if you need the payment.
///
/// A trial converting to a paid subscription starts a new period, so it fires
/// there too. Read `status` to tell the two apart.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionCycledPayload {
    pub r#type: monostate::MustBe!("subscription.cycled"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionCycledPayload {
    pub fn new(required: WebhookSubscriptionCycledPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionMigratedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
    /// The billing provider the subscription was migrated from.
    pub provider: String,
    /// The identifier of the subscription on the billing provider.
    pub provider_subscription_id: String,
}
/// Sent when Polar takes over billing of a subscription migrated from another
/// provider.
///
/// This fires at cutover, once the subscription is live on Polar. `provider`
/// and `provider_subscription_id` identify the subscription on the billing
/// provider so you can correlate the two.
///
/// **Discord & Slack support:** Basic
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionMigratedPayload {
    pub r#type: monostate::MustBe!("subscription.migrated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
    /// The billing provider the subscription was migrated from.
    pub provider: String,
    /// The identifier of the subscription on the billing provider.
    pub provider_subscription_id: String,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionMigratedPayload {
    pub fn new(required: WebhookSubscriptionMigratedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
            provider: required.provider,
            provider_subscription_id: required.provider_subscription_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionPastDuePayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a subscription payment fails and the subscription enters
/// `past_due` status.
///
/// This is a recoverable state - the customer can update their payment method
/// to restore the subscription. Benefits may be revoked depending on the
/// organization's grace period settings.
///
/// If payment retries are exhausted, a `subscription.revoked` event will be
/// sent.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionPastDuePayload {
    pub r#type: monostate::MustBe!("subscription.past_due"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionPastDuePayload {
    pub fn new(required: WebhookSubscriptionPastDuePayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionPausedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a subscription is paused and the customer temporarily loses
/// access.
///
/// No order is created while paused. The subscription resumes either on its
/// scheduled resume date or when resumed manually, starting a new billing
/// period.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionPausedPayload {
    pub r#type: monostate::MustBe!("subscription.paused"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionPausedPayload {
    pub fn new(required: WebhookSubscriptionPausedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionResumedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a paused subscription resumes, restoring the customer's access.
///
/// Resuming starts a new billing period and charges the customer immediately.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionResumedPayload {
    pub r#type: monostate::MustBe!("subscription.resumed"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionResumedPayload {
    pub fn new(required: WebhookSubscriptionResumedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionRevokedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a subscription is revoked and the user loses access immediately.
/// Happens when the subscription is canceled or payment retries are exhausted
/// (status becomes `unpaid`).
///
/// For payment failures that can still be recovered, see
/// `subscription.past_due`.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionRevokedPayload {
    pub r#type: monostate::MustBe!("subscription.revoked"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionRevokedPayload {
    pub fn new(required: WebhookSubscriptionRevokedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionUncanceledPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a customer revokes a pending cancellation.
///
/// When a customer cancels with "at period end", they retain access until the
/// subscription would renew. During this time, they can change their mind and
/// undo the cancellation. This event is triggered when they do so.
///
/// **Discord & Slack support:** Full
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionUncanceledPayload {
    pub r#type: monostate::MustBe!("subscription.uncanceled"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionUncanceledPayload {
    pub fn new(required: WebhookSubscriptionUncanceledPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WebhookSubscriptionUpdatedPayloadRequired {
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
/// Sent when a subscription is updated. This event fires for all changes to the
/// subscription, including renewals.
///
/// If you want more specific events, you can listen to `subscription.active`,
/// `subscription.canceled`, `subscription.past_due`, and
/// `subscription.revoked`.
///
/// To listen specifically for renewals, listen to `subscription.cycled`.
///
/// **Discord & Slack support:** On cancellation, past due, and revocation.
/// Renewals are skipped.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebhookSubscriptionUpdatedPayload {
    pub r#type: monostate::MustBe!("subscription.updated"),
    pub timestamp: jiff::Timestamp,
    pub api_version: String,
    pub data: crate::v2026_04::models::subscriptions::types::Subscription,
}
#[allow(clippy::type_complexity)]
impl WebhookSubscriptionUpdatedPayload {
    pub fn new(required: WebhookSubscriptionUpdatedPayloadRequired) -> Self {
        Self {
            r#type: Default::default(),
            timestamp: required.timestamp,
            api_version: required.api_version,
            data: required.data,
        }
    }
}
