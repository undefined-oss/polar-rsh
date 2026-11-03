#[derive(Debug, Clone)]
pub struct CustomerSessionRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::CustomerSessionId,
    pub token: String,
    pub expires_at: jiff::Timestamp,
    pub return_url: Option<String>,
    pub customer_portal_url: String,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
}
/// A customer session that can be used to authenticate as a customer.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSession {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::CustomerSessionId,
    pub token: String,
    pub expires_at: jiff::Timestamp,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub return_url: Option<String>,
    pub customer_portal_url: String,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub customer: crate::v2026_04::models::common::unions::Customer,
}
#[allow(clippy::type_complexity)]
impl CustomerSession {
    pub fn new(required: CustomerSessionRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            token: required.token,
            expires_at: required.expires_at,
            return_url: required.return_url,
            customer_portal_url: required.customer_portal_url,
            customer_id: required.customer_id,
            customer: required.customer,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSessionCustomerExternalIdCreateRequired {
    /// External ID of the customer to create a session for.
    pub external_customer_id: String,
}
/// Schema for creating a customer session using an external customer ID.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSessionCustomerExternalIdCreate {
    ///ID of the member to create a session for. When not provided and the organization has `member_model_enabled`, the owner member of the customer will be used for individual customers.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    ///External ID of the member to create a session for. Alternative to `member_id`.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    ///When set, a back button will be shown in the customer portal to return to this URL.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub return_url: presence_rs::Presence<
        crate::v2026_04::models::customer_sessions::newtypes::CustomerSessionCustomerExternalIdCreateReturnUrl,
    >,
    ///External ID of the customer to create a session for.
    pub external_customer_id: String,
}
#[allow(clippy::type_complexity)]
impl CustomerSessionCustomerExternalIdCreate {
    pub fn new(required: CustomerSessionCustomerExternalIdCreateRequired) -> Self {
        Self {
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            return_url: presence_rs::Presence::Absent,
            external_customer_id: required.external_customer_id,
        }
    }

    /// ID of the member to create a session for. When not provided and the
    /// organization has `member_model_enabled`, the owner member of the
    /// customer will be used for individual customers.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// External ID of the member to create a session for. Alternative to
    /// `member_id`.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// When set, a back button will be shown in the customer portal to return
    /// to this URL.
    #[allow(clippy::should_implement_trait)]
    pub fn return_url(
        mut self,
        value: crate::v2026_04::models::customer_sessions::newtypes::CustomerSessionCustomerExternalIdCreateReturnUrl,
    ) -> Self {
        self.return_url = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSessionCustomerIdCreateRequired {
    /// ID of the customer to create a session for.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
}
/// Schema for creating a customer session using a customer ID.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSessionCustomerIdCreate {
    ///ID of the member to create a session for. When not provided and the organization has `member_model_enabled`, the owner member of the customer will be used for individual customers.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2026_04::models::common::ids::MemberId>,
    ///External ID of the member to create a session for. Alternative to `member_id`.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    ///When set, a back button will be shown in the customer portal to return to this URL.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub return_url: presence_rs::Presence<
        crate::v2026_04::models::customer_sessions::newtypes::CustomerSessionCustomerIdCreateReturnUrl,
    >,
    ///ID of the customer to create a session for.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
}
#[allow(clippy::type_complexity)]
impl CustomerSessionCustomerIdCreate {
    pub fn new(required: CustomerSessionCustomerIdCreateRequired) -> Self {
        Self {
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            return_url: presence_rs::Presence::Absent,
            customer_id: required.customer_id,
        }
    }

    /// ID of the member to create a session for. When not provided and the
    /// organization has `member_model_enabled`, the owner member of the
    /// customer will be used for individual customers.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2026_04::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// External ID of the member to create a session for. Alternative to
    /// `member_id`.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// When set, a back button will be shown in the customer portal to return
    /// to this URL.
    #[allow(clippy::should_implement_trait)]
    pub fn return_url(
        mut self,
        value: crate::v2026_04::models::customer_sessions::newtypes::CustomerSessionCustomerIdCreateReturnUrl,
    ) -> Self {
        self.return_url = presence_rs::Presence::Some(value);
        self
    }
}
