#[derive(Debug, Clone)]
pub struct AuthorizationCodeTokenRequestRequired {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri:
        crate::v2026_10::models::oauth2::newtypes::AuthorizationCodeTokenRequestRedirectUri,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationCodeTokenRequest {
    pub grant_type: monostate::MustBe!("authorization_code"),
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri:
        crate::v2026_10::models::oauth2::newtypes::AuthorizationCodeTokenRequestRedirectUri,
}
#[allow(clippy::type_complexity)]
impl AuthorizationCodeTokenRequest {
    pub fn new(required: AuthorizationCodeTokenRequestRequired) -> Self {
        Self {
            grant_type: Default::default(),
            client_id: required.client_id,
            client_secret: required.client_secret,
            code: required.code,
            redirect_uri: required.redirect_uri,
        }
    }
}
#[derive(Debug, Clone)]
pub struct AuthorizeOrganizationRequired {
    pub id: crate::v2026_10::models::common::ids::OrganizationId,
    pub slug: String,
    pub name: String,
    pub avatar_url: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AuthorizeOrganization {
    pub id: crate::v2026_10::models::common::ids::OrganizationId,
    pub slug: String,
    pub name: String,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
}
#[allow(clippy::type_complexity)]
impl AuthorizeOrganization {
    pub fn new(required: AuthorizeOrganizationRequired) -> Self {
        Self {
            id: required.id,
            slug: required.slug,
            name: required.name,
            avatar_url: required.avatar_url,
        }
    }
}
#[derive(Debug, Clone)]
pub struct AuthorizeResponseOrganizationRequired {
    pub client: crate::v2026_10::models::oauth2::types::OAuth2ClientPublic,
    pub sub: Option<crate::v2026_10::models::oauth2::types::AuthorizeOrganization>,
    pub scopes: Vec<crate::v2026_10::models::oauth2::enums::Scope>,
    pub organizations: Vec<crate::v2026_10::models::oauth2::types::AuthorizeOrganization>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AuthorizeResponseOrganization {
    pub client: crate::v2026_10::models::oauth2::types::OAuth2ClientPublic,
    pub sub_type: monostate::MustBe!("organization"),
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub sub: Option<crate::v2026_10::models::oauth2::types::AuthorizeOrganization>,
    pub scopes: Vec<crate::v2026_10::models::oauth2::enums::Scope>,
    pub organizations: Vec<crate::v2026_10::models::oauth2::types::AuthorizeOrganization>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub requires_single_organization: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub scope_display_names: Option<std::collections::BTreeMap<String, String>>,
}
#[allow(clippy::type_complexity)]
impl AuthorizeResponseOrganization {
    pub fn new(required: AuthorizeResponseOrganizationRequired) -> Self {
        Self {
            client: required.client,
            sub_type: Default::default(),
            sub: required.sub,
            scopes: required.scopes,
            organizations: required.organizations,
            requires_single_organization: None,
            scope_display_names: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn requires_single_organization(mut self, value: bool) -> Self {
        self.requires_single_organization = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn scope_display_names(
        mut self,
        value: std::collections::BTreeMap<String, String>,
    ) -> Self {
        self.scope_display_names = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct AuthorizeResponseUserRequired {
    pub client: crate::v2026_10::models::oauth2::types::OAuth2ClientPublic,
    pub sub: Option<crate::v2026_10::models::oauth2::types::AuthorizeUser>,
    pub scopes: Vec<crate::v2026_10::models::oauth2::enums::Scope>,
    pub organizations: Vec<crate::v2026_10::models::oauth2::types::AuthorizeOrganization>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AuthorizeResponseUser {
    pub client: crate::v2026_10::models::oauth2::types::OAuth2ClientPublic,
    pub sub_type: monostate::MustBe!("user"),
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub sub: Option<crate::v2026_10::models::oauth2::types::AuthorizeUser>,
    pub scopes: Vec<crate::v2026_10::models::oauth2::enums::Scope>,
    pub organizations: Vec<crate::v2026_10::models::oauth2::types::AuthorizeOrganization>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub requires_single_organization: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub scope_display_names: Option<std::collections::BTreeMap<String, String>>,
}
#[allow(clippy::type_complexity)]
impl AuthorizeResponseUser {
    pub fn new(required: AuthorizeResponseUserRequired) -> Self {
        Self {
            client: required.client,
            sub_type: Default::default(),
            sub: required.sub,
            scopes: required.scopes,
            organizations: required.organizations,
            requires_single_organization: None,
            scope_display_names: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn requires_single_organization(mut self, value: bool) -> Self {
        self.requires_single_organization = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn scope_display_names(
        mut self,
        value: std::collections::BTreeMap<String, String>,
    ) -> Self {
        self.scope_display_names = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct AuthorizeUserRequired {
    pub id: crate::v2026_10::models::common::ids::AuthorizeUserId,
    pub email: String,
    pub avatar_url: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AuthorizeUser {
    pub id: crate::v2026_10::models::common::ids::AuthorizeUserId,
    pub email: String,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
}
#[allow(clippy::type_complexity)]
impl AuthorizeUser {
    pub fn new(required: AuthorizeUserRequired) -> Self {
        Self { id: required.id, email: required.email, avatar_url: required.avatar_url }
    }
}
#[derive(Debug, Clone)]
pub struct IntrospectTokenRequestRequired {
    pub token: String,
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct IntrospectTokenRequest {
    pub token: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub token_type_hint: presence_rs::Presence<
        crate::v2026_10::models::oauth2::enums::IntrospectTokenRequestTokenTypeHint,
    >,
    pub client_id: String,
    pub client_secret: String,
}
#[allow(clippy::type_complexity)]
impl IntrospectTokenRequest {
    pub fn new(required: IntrospectTokenRequestRequired) -> Self {
        Self {
            token: required.token,
            token_type_hint: presence_rs::Presence::Absent,
            client_id: required.client_id,
            client_secret: required.client_secret,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn token_type_hint(
        mut self,
        value: crate::v2026_10::models::oauth2::enums::IntrospectTokenRequestTokenTypeHint,
    ) -> Self {
        self.token_type_hint = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct IntrospectTokenResponseRequired {
    pub active: bool,
    pub client_id: String,
    pub token_type: crate::v2026_10::models::oauth2::enums::IntrospectTokenResponseTokenType,
    pub scope: String,
    pub sub_type: crate::v2026_10::models::oauth2::enums::SubType,
    pub sub: String,
    pub organizations: Vec<String>,
    pub aud: String,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct IntrospectTokenResponse {
    pub active: bool,
    pub client_id: String,
    pub token_type: crate::v2026_10::models::oauth2::enums::IntrospectTokenResponseTokenType,
    pub scope: String,
    pub sub_type: crate::v2026_10::models::oauth2::enums::SubType,
    pub sub: String,
    pub organizations: Vec<String>,
    pub aud: String,
    pub iss: String,
    pub exp: i64,
    pub iat: i64,
}
#[allow(clippy::type_complexity)]
impl IntrospectTokenResponse {
    pub fn new(required: IntrospectTokenResponseRequired) -> Self {
        Self {
            active: required.active,
            client_id: required.client_id,
            token_type: required.token_type,
            scope: required.scope,
            sub_type: required.sub_type,
            sub: required.sub,
            organizations: required.organizations,
            aud: required.aud,
            iss: required.iss,
            exp: required.exp,
            iat: required.iat,
        }
    }
}
#[derive(Debug, Clone)]
pub struct OAuth2ClientConfigurationRequired {
    pub redirect_uris:
        Vec<crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationRedirectUrisItem>,
    pub client_name: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OAuth2ClientConfiguration {
    pub redirect_uris:
        Vec<crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationRedirectUrisItem>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub token_endpoint_auth_method: Option<
        crate::v2026_10::models::oauth2::enums::OAuth2ClientConfigurationTokenEndpointAuthMethod,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub grant_types: Option<
        Vec<crate::v2026_10::models::oauth2::enums::OAuth2ClientConfigurationGrantTypesItems>,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub response_types: Option<Vec<monostate::MustBe!("code")>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub scope: Option<String>,
    pub client_name: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub client_uri: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub logo_uri: presence_rs::Presence<
        crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationLogoUri,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tos_uri: presence_rs::Presence<
        crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationTosUri,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub policy_uri: presence_rs::Presence<
        crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationPolicyUri,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub default_sub_type: Option<crate::v2026_10::models::oauth2::enums::SubType>,
}
#[allow(clippy::type_complexity)]
impl OAuth2ClientConfiguration {
    pub fn new(required: OAuth2ClientConfigurationRequired) -> Self {
        Self {
            redirect_uris: required.redirect_uris,
            token_endpoint_auth_method: None,
            grant_types: None,
            response_types: None,
            scope: None,
            client_name: required.client_name,
            client_uri: presence_rs::Presence::Absent,
            logo_uri: presence_rs::Presence::Absent,
            tos_uri: presence_rs::Presence::Absent,
            policy_uri: presence_rs::Presence::Absent,
            default_sub_type: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn token_endpoint_auth_method(
        mut self,
        value: crate::v2026_10::models::oauth2::enums::OAuth2ClientConfigurationTokenEndpointAuthMethod,
    ) -> Self {
        self.token_endpoint_auth_method = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn grant_types(
        mut self,
        value: Vec<
            crate::v2026_10::models::oauth2::enums::OAuth2ClientConfigurationGrantTypesItems,
        >,
    ) -> Self {
        self.grant_types = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn response_types(mut self, value: Vec<monostate::MustBe!("code")>) -> Self {
        self.response_types = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn client_uri(mut self, value: impl Into<String>) -> Self {
        self.client_uri = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn logo_uri(
        mut self,
        value: crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationLogoUri,
    ) -> Self {
        self.logo_uri = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tos_uri(
        mut self,
        value: crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationTosUri,
    ) -> Self {
        self.tos_uri = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn policy_uri(
        mut self,
        value: crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationPolicyUri,
    ) -> Self {
        self.policy_uri = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default_sub_type(
        mut self,
        value: crate::v2026_10::models::oauth2::enums::SubType,
    ) -> Self {
        self.default_sub_type = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OAuth2ClientConfigurationUpdateRequired {
    pub redirect_uris:
        Vec<crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationRedirectUrisItem>,
    pub client_name: String,
    pub client_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OAuth2ClientConfigurationUpdate {
    pub redirect_uris: Vec<
        crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationRedirectUrisItem,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub token_endpoint_auth_method: Option<
        crate::v2026_10::models::oauth2::enums::OAuth2ClientConfigurationUpdateTokenEndpointAuthMethod,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub grant_types: Option<
        Vec<
            crate::v2026_10::models::oauth2::enums::OAuth2ClientConfigurationUpdateGrantTypesItems,
        >,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub response_types: Option<Vec<monostate::MustBe!("code")>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub scope: Option<String>,
    pub client_name: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub client_uri: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub logo_uri: presence_rs::Presence<
        crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationLogoUri,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tos_uri: presence_rs::Presence<
        crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationTosUri,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub policy_uri: presence_rs::Presence<
        crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationPolicyUri,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub default_sub_type: Option<crate::v2026_10::models::oauth2::enums::SubType>,
    pub client_id: String,
}
#[allow(clippy::type_complexity)]
impl OAuth2ClientConfigurationUpdate {
    pub fn new(required: OAuth2ClientConfigurationUpdateRequired) -> Self {
        Self {
            redirect_uris: required.redirect_uris,
            token_endpoint_auth_method: None,
            grant_types: None,
            response_types: None,
            scope: None,
            client_name: required.client_name,
            client_uri: presence_rs::Presence::Absent,
            logo_uri: presence_rs::Presence::Absent,
            tos_uri: presence_rs::Presence::Absent,
            policy_uri: presence_rs::Presence::Absent,
            default_sub_type: None,
            client_id: required.client_id,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn token_endpoint_auth_method(
        mut self,
        value: crate::v2026_10::models::oauth2::enums::OAuth2ClientConfigurationUpdateTokenEndpointAuthMethod,
    ) -> Self {
        self.token_endpoint_auth_method = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn grant_types(
        mut self,
        value: Vec<
            crate::v2026_10::models::oauth2::enums::OAuth2ClientConfigurationUpdateGrantTypesItems,
        >,
    ) -> Self {
        self.grant_types = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn response_types(mut self, value: Vec<monostate::MustBe!("code")>) -> Self {
        self.response_types = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn client_uri(mut self, value: impl Into<String>) -> Self {
        self.client_uri = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn logo_uri(
        mut self,
        value: crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationLogoUri,
    ) -> Self {
        self.logo_uri = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tos_uri(
        mut self,
        value: crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationTosUri,
    ) -> Self {
        self.tos_uri = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn policy_uri(
        mut self,
        value: crate::v2026_10::models::oauth2::newtypes::OAuth2ClientConfigurationPolicyUri,
    ) -> Self {
        self.policy_uri = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default_sub_type(
        mut self,
        value: crate::v2026_10::models::oauth2::enums::SubType,
    ) -> Self {
        self.default_sub_type = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OAuth2ClientPublicRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub client_id: String,
    pub client_name: Option<String>,
    pub client_uri: Option<String>,
    pub logo_uri: Option<String>,
    pub tos_uri: Option<String>,
    pub policy_uri: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OAuth2ClientPublic {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub client_id: String,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub client_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub client_uri: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub logo_uri: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tos_uri: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub policy_uri: Option<String>,
}
#[allow(clippy::type_complexity)]
impl OAuth2ClientPublic {
    pub fn new(required: OAuth2ClientPublicRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            client_id: required.client_id,
            client_name: required.client_name,
            client_uri: required.client_uri,
            logo_uri: required.logo_uri,
            tos_uri: required.tos_uri,
            policy_uri: required.policy_uri,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RefreshTokenRequestRequired {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct RefreshTokenRequest {
    pub grant_type: monostate::MustBe!("refresh_token"),
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}
#[allow(clippy::type_complexity)]
impl RefreshTokenRequest {
    pub fn new(required: RefreshTokenRequestRequired) -> Self {
        Self {
            grant_type: Default::default(),
            client_id: required.client_id,
            client_secret: required.client_secret,
            refresh_token: required.refresh_token,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RevokeTokenRequestRequired {
    pub token: String,
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct RevokeTokenRequest {
    pub token: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub token_type_hint: presence_rs::Presence<
        crate::v2026_10::models::oauth2::enums::RevokeTokenRequestTokenTypeHint,
    >,
    pub client_id: String,
    pub client_secret: String,
}
#[allow(clippy::type_complexity)]
impl RevokeTokenRequest {
    pub fn new(required: RevokeTokenRequestRequired) -> Self {
        Self {
            token: required.token,
            token_type_hint: presence_rs::Presence::Absent,
            client_id: required.client_id,
            client_secret: required.client_secret,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn token_type_hint(
        mut self,
        value: crate::v2026_10::models::oauth2::enums::RevokeTokenRequestTokenTypeHint,
    ) -> Self {
        self.token_type_hint = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct RevokeTokenResponse {}
#[allow(clippy::type_complexity)]
impl RevokeTokenResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct TokenResponseRequired {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: monostate::MustBe!("Bearer"),
    pub expires_in: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub refresh_token: presence_rs::Presence<String>,
    pub scope: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub id_token: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl TokenResponse {
    pub fn new(required: TokenResponseRequired) -> Self {
        Self {
            access_token: required.access_token,
            token_type: Default::default(),
            expires_in: required.expires_in,
            refresh_token: presence_rs::Presence::Absent,
            scope: required.scope,
            id_token: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn refresh_token(mut self, value: impl Into<String>) -> Self {
        self.refresh_token = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn id_token(mut self, value: impl Into<String>) -> Self {
        self.id_token = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct UserInfoOrganizationRequired {
    pub sub: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct UserInfoOrganization {
    pub sub: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl UserInfoOrganization {
    pub fn new(required: UserInfoOrganizationRequired) -> Self {
        Self { sub: required.sub, name: presence_rs::Presence::Absent }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct UserInfoUserRequired {
    pub sub: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct UserInfoUser {
    pub sub: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email_verified: presence_rs::Presence<bool>,
}
#[allow(clippy::type_complexity)]
impl UserInfoUser {
    pub fn new(required: UserInfoUserRequired) -> Self {
        Self {
            sub: required.sub,
            name: presence_rs::Presence::Absent,
            email: presence_rs::Presence::Absent,
            email_verified: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn email_verified(mut self, value: bool) -> Self {
        self.email_verified = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct WebTokenRequestRequired {
    pub client_id: String,
    pub client_secret: String,
    pub session_token: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct WebTokenRequest {
    pub grant_type: monostate::MustBe!("web"),
    pub client_id: String,
    pub client_secret: String,
    pub session_token: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub sub_type: Option<crate::v2026_10::models::oauth2::enums::WebTokenRequestSubType>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub sub: presence_rs::Presence<crate::v2026_10::models::common::ids::WebTokenRequestSubId>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub scope: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl WebTokenRequest {
    pub fn new(required: WebTokenRequestRequired) -> Self {
        Self {
            grant_type: Default::default(),
            client_id: required.client_id,
            client_secret: required.client_secret,
            session_token: required.session_token,
            sub_type: None,
            sub: presence_rs::Presence::Absent,
            scope: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn sub_type(
        mut self,
        value: crate::v2026_10::models::oauth2::enums::WebTokenRequestSubType,
    ) -> Self {
        self.sub_type = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn sub(
        mut self,
        value: crate::v2026_10::models::common::ids::WebTokenRequestSubId,
    ) -> Self {
        self.sub = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = presence_rs::Presence::Some(value.into());
        self
    }
}
