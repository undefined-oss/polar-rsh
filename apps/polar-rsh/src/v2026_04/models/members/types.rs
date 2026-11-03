#[derive(Debug, Clone)]
pub struct CustomerPortalMemberRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::MemberId,
    /// The email address of the member.
    pub email: String,
    /// The name of the member.
    pub name: Option<String>,
    /// The role of the member within the team.
    pub role: crate::v2026_04::models::common::enums::MemberRole,
}
/// A member of the customer's team as seen in the customer portal.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPortalMember {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::MemberId,
    /// The email address of the member.
    pub email: String,
    /// The name of the member.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The role of the member within the team.
    pub role: crate::v2026_04::models::common::enums::MemberRole,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalMember {
    pub fn new(required: CustomerPortalMemberRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            email: required.email,
            name: required.name,
            role: required.role,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPortalMemberCreateRequired {
    /// The email address of the new member.
    pub email: String,
}
/// Schema for adding a new member to the customer's team.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPortalMemberCreate {
    /// The email address of the new member.
    pub email: String,
    /// The name of the new member (optional).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<String>,
    /// The role for the new member. Defaults to 'member'.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub role: Option<crate::v2026_04::models::common::enums::MemberRole>,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalMemberCreate {
    pub fn new(required: CustomerPortalMemberCreateRequired) -> Self {
        Self { email: required.email, name: presence_rs::Presence::Absent, role: None }
    }

    /// The name of the new member (optional).
    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = presence_rs::Presence::Some(value.into());
        self
    }

    /// The role for the new member. Defaults to 'member'.
    #[allow(clippy::should_implement_trait)]
    pub fn role(mut self, value: crate::v2026_04::models::common::enums::MemberRole) -> Self {
        self.role = Some(value);
        self
    }
}
/// Schema for updating a member in the customer portal.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerPortalMemberUpdate {
    /// The new name for the member.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name:
        presence_rs::Presence<crate::v2026_04::models::members::newtypes::CustomerPortalMemberName>,
    /// The new role for the member.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub role: presence_rs::Presence<crate::v2026_04::models::common::enums::MemberRole>,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalMemberUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// The new name for the member.
    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::members::newtypes::CustomerPortalMemberName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    /// The new role for the member.
    #[allow(clippy::should_implement_trait)]
    pub fn role(mut self, value: crate::v2026_04::models::common::enums::MemberRole) -> Self {
        self.role = presence_rs::Presence::Some(value);
        self
    }
}
