#[derive(Debug, Clone)]
pub struct CustomerCustomerSessionRequired {
    pub expires_at: jiff::Timestamp,
    pub return_url: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerCustomerSession {
    pub expires_at: jiff::Timestamp,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub return_url: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerCustomerSession {
    pub fn new(required: CustomerCustomerSessionRequired) -> Self {
        Self { expires_at: required.expires_at, return_url: required.return_url }
    }
}
#[derive(Debug, Clone)]
pub struct PortalAuthenticatedUserRequired {
    /// Type of authenticated user: 'customer' or 'member'
    pub r#type: String,
    /// User's name, if available.
    pub name: Option<String>,
    /// User's email address.
    pub email: String,
    /// Associated customer ID.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
}
/// Information about the authenticated portal user.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct PortalAuthenticatedUser {
    /// Type of authenticated user: 'customer' or 'member'
    pub r#type: String,
    /// User's name, if available.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// User's email address.
    pub email: String,
    /// Associated customer ID.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// Member ID. Only set for members.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// Member role (owner, billing_manager, member). Only set for members.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub role: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl PortalAuthenticatedUser {
    pub fn new(required: PortalAuthenticatedUserRequired) -> Self {
        Self {
            r#type: required.r#type,
            name: required.name,
            email: required.email,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            role: presence_rs::Presence::Absent,
        }
    }

    /// Member ID. Only set for members.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// Member role (owner, billing_manager, member). Only set for members.
    #[allow(clippy::should_implement_trait)]
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = presence_rs::Presence::Some(value.into());
        self
    }
}
