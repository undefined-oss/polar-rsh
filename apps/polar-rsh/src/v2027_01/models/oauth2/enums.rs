use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum IntrospectTokenRequestTokenTypeHint {
    #[serde(rename = "access_token")]
    AccessToken,
    #[serde(rename = "refresh_token")]
    RefreshToken,
}
impl IntrospectTokenRequestTokenTypeHint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AccessToken => "access_token",
            Self::RefreshToken => "refresh_token",
        }
    }
}
impl ::std::fmt::Display for IntrospectTokenRequestTokenTypeHint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for IntrospectTokenRequestTokenTypeHint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum IntrospectTokenResponseTokenType {
    #[serde(rename = "access_token")]
    AccessToken,
    #[serde(rename = "refresh_token")]
    RefreshToken,
}
impl IntrospectTokenResponseTokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AccessToken => "access_token",
            Self::RefreshToken => "refresh_token",
        }
    }
}
impl ::std::fmt::Display for IntrospectTokenResponseTokenType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for IntrospectTokenResponseTokenType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OAuth2ClientConfigurationGrantTypesItems {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "refresh_token")]
    RefreshToken,
}
impl OAuth2ClientConfigurationGrantTypesItems {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizationCode => "authorization_code",
            Self::RefreshToken => "refresh_token",
        }
    }
}
impl ::std::fmt::Display for OAuth2ClientConfigurationGrantTypesItems {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OAuth2ClientConfigurationGrantTypesItems {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default,
)]
pub enum OAuth2ClientConfigurationTokenEndpointAuthMethod {
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
    #[default]
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,
    #[serde(rename = "none")]
    None,
}
impl OAuth2ClientConfigurationTokenEndpointAuthMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ClientSecretBasic => "client_secret_basic",
            Self::ClientSecretPost => "client_secret_post",
            Self::None => "none",
        }
    }
}
impl ::std::fmt::Display for OAuth2ClientConfigurationTokenEndpointAuthMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OAuth2ClientConfigurationTokenEndpointAuthMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OAuth2ClientConfigurationUpdateGrantTypesItems {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "refresh_token")]
    RefreshToken,
}
impl OAuth2ClientConfigurationUpdateGrantTypesItems {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorizationCode => "authorization_code",
            Self::RefreshToken => "refresh_token",
        }
    }
}
impl ::std::fmt::Display for OAuth2ClientConfigurationUpdateGrantTypesItems {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OAuth2ClientConfigurationUpdateGrantTypesItems {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default,
)]
pub enum OAuth2ClientConfigurationUpdateTokenEndpointAuthMethod {
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
    #[default]
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,
    #[serde(rename = "none")]
    None,
}
impl OAuth2ClientConfigurationUpdateTokenEndpointAuthMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ClientSecretBasic => "client_secret_basic",
            Self::ClientSecretPost => "client_secret_post",
            Self::None => "none",
        }
    }
}
impl ::std::fmt::Display for OAuth2ClientConfigurationUpdateTokenEndpointAuthMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OAuth2ClientConfigurationUpdateTokenEndpointAuthMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RevokeTokenRequestTokenTypeHint {
    #[serde(rename = "access_token")]
    AccessToken,
    #[serde(rename = "refresh_token")]
    RefreshToken,
}
impl RevokeTokenRequestTokenTypeHint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AccessToken => "access_token",
            Self::RefreshToken => "refresh_token",
        }
    }
}
impl ::std::fmt::Display for RevokeTokenRequestTokenTypeHint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for RevokeTokenRequestTokenTypeHint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Scope {
    #[serde(rename = "openid")]
    Openid,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "user:read")]
    UserRead,
    #[serde(rename = "user:write")]
    UserWrite,
    #[serde(rename = "organizations:read")]
    OrganizationsRead,
    #[serde(rename = "organizations:write")]
    OrganizationsWrite,
    #[serde(rename = "custom_fields:read")]
    CustomFieldsRead,
    #[serde(rename = "custom_fields:write")]
    CustomFieldsWrite,
    #[serde(rename = "discounts:read")]
    DiscountsRead,
    #[serde(rename = "discounts:write")]
    DiscountsWrite,
    #[serde(rename = "checkout_links:read")]
    CheckoutLinksRead,
    #[serde(rename = "checkout_links:write")]
    CheckoutLinksWrite,
    #[serde(rename = "checkouts:read")]
    CheckoutsRead,
    #[serde(rename = "checkouts:write")]
    CheckoutsWrite,
    #[serde(rename = "transactions:read")]
    TransactionsRead,
    #[serde(rename = "transactions:write")]
    TransactionsWrite,
    #[serde(rename = "payouts:read")]
    PayoutsRead,
    #[serde(rename = "payouts:write")]
    PayoutsWrite,
    #[serde(rename = "products:read")]
    ProductsRead,
    #[serde(rename = "products:write")]
    ProductsWrite,
    #[serde(rename = "benefits:read")]
    BenefitsRead,
    #[serde(rename = "benefits:write")]
    BenefitsWrite,
    #[serde(rename = "events:read")]
    EventsRead,
    #[serde(rename = "events:write")]
    EventsWrite,
    #[serde(rename = "meters:read")]
    MetersRead,
    #[serde(rename = "meters:write")]
    MetersWrite,
    #[serde(rename = "files:read")]
    FilesRead,
    #[serde(rename = "files:write")]
    FilesWrite,
    #[serde(rename = "subscriptions:read")]
    SubscriptionsRead,
    #[serde(rename = "subscriptions:write")]
    SubscriptionsWrite,
    #[serde(rename = "customers:read")]
    CustomersRead,
    #[serde(rename = "customers:write")]
    CustomersWrite,
    #[serde(rename = "members:read")]
    MembersRead,
    #[serde(rename = "members:write")]
    MembersWrite,
    #[serde(rename = "wallets:read")]
    WalletsRead,
    #[serde(rename = "wallets:write")]
    WalletsWrite,
    #[serde(rename = "disputes:read")]
    DisputesRead,
    #[serde(rename = "disputes:write")]
    DisputesWrite,
    #[serde(rename = "customer_meters:read")]
    CustomerMetersRead,
    #[serde(rename = "customer_sessions:write")]
    CustomerSessionsWrite,
    #[serde(rename = "member_sessions:write")]
    MemberSessionsWrite,
    #[serde(rename = "customer_seats:read")]
    CustomerSeatsRead,
    #[serde(rename = "customer_seats:write")]
    CustomerSeatsWrite,
    #[serde(rename = "orders:read")]
    OrdersRead,
    #[serde(rename = "orders:write")]
    OrdersWrite,
    #[serde(rename = "refunds:read")]
    RefundsRead,
    #[serde(rename = "refunds:write")]
    RefundsWrite,
    #[serde(rename = "payments:read")]
    PaymentsRead,
    #[serde(rename = "metrics:read")]
    MetricsRead,
    #[serde(rename = "metrics:write")]
    MetricsWrite,
    #[serde(rename = "webhooks:read")]
    WebhooksRead,
    #[serde(rename = "webhooks:write")]
    WebhooksWrite,
    #[serde(rename = "license_keys:read")]
    LicenseKeysRead,
    #[serde(rename = "license_keys:write")]
    LicenseKeysWrite,
    #[serde(rename = "customer_portal:read")]
    CustomerPortalRead,
    #[serde(rename = "customer_portal:write")]
    CustomerPortalWrite,
    #[serde(rename = "notifications:read")]
    NotificationsRead,
    #[serde(rename = "notifications:write")]
    NotificationsWrite,
    #[serde(rename = "notification_recipients:read")]
    NotificationRecipientsRead,
    #[serde(rename = "notification_recipients:write")]
    NotificationRecipientsWrite,
    #[serde(rename = "organization_access_tokens:read")]
    OrganizationAccessTokensRead,
    #[serde(rename = "organization_access_tokens:write")]
    OrganizationAccessTokensWrite,
}
impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Openid => "openid",
            Self::Profile => "profile",
            Self::Email => "email",
            Self::UserRead => "user:read",
            Self::UserWrite => "user:write",
            Self::OrganizationsRead => "organizations:read",
            Self::OrganizationsWrite => "organizations:write",
            Self::CustomFieldsRead => "custom_fields:read",
            Self::CustomFieldsWrite => "custom_fields:write",
            Self::DiscountsRead => "discounts:read",
            Self::DiscountsWrite => "discounts:write",
            Self::CheckoutLinksRead => "checkout_links:read",
            Self::CheckoutLinksWrite => "checkout_links:write",
            Self::CheckoutsRead => "checkouts:read",
            Self::CheckoutsWrite => "checkouts:write",
            Self::TransactionsRead => "transactions:read",
            Self::TransactionsWrite => "transactions:write",
            Self::PayoutsRead => "payouts:read",
            Self::PayoutsWrite => "payouts:write",
            Self::ProductsRead => "products:read",
            Self::ProductsWrite => "products:write",
            Self::BenefitsRead => "benefits:read",
            Self::BenefitsWrite => "benefits:write",
            Self::EventsRead => "events:read",
            Self::EventsWrite => "events:write",
            Self::MetersRead => "meters:read",
            Self::MetersWrite => "meters:write",
            Self::FilesRead => "files:read",
            Self::FilesWrite => "files:write",
            Self::SubscriptionsRead => "subscriptions:read",
            Self::SubscriptionsWrite => "subscriptions:write",
            Self::CustomersRead => "customers:read",
            Self::CustomersWrite => "customers:write",
            Self::MembersRead => "members:read",
            Self::MembersWrite => "members:write",
            Self::WalletsRead => "wallets:read",
            Self::WalletsWrite => "wallets:write",
            Self::DisputesRead => "disputes:read",
            Self::DisputesWrite => "disputes:write",
            Self::CustomerMetersRead => "customer_meters:read",
            Self::CustomerSessionsWrite => "customer_sessions:write",
            Self::MemberSessionsWrite => "member_sessions:write",
            Self::CustomerSeatsRead => "customer_seats:read",
            Self::CustomerSeatsWrite => "customer_seats:write",
            Self::OrdersRead => "orders:read",
            Self::OrdersWrite => "orders:write",
            Self::RefundsRead => "refunds:read",
            Self::RefundsWrite => "refunds:write",
            Self::PaymentsRead => "payments:read",
            Self::MetricsRead => "metrics:read",
            Self::MetricsWrite => "metrics:write",
            Self::WebhooksRead => "webhooks:read",
            Self::WebhooksWrite => "webhooks:write",
            Self::LicenseKeysRead => "license_keys:read",
            Self::LicenseKeysWrite => "license_keys:write",
            Self::CustomerPortalRead => "customer_portal:read",
            Self::CustomerPortalWrite => "customer_portal:write",
            Self::NotificationsRead => "notifications:read",
            Self::NotificationsWrite => "notifications:write",
            Self::NotificationRecipientsRead => "notification_recipients:read",
            Self::NotificationRecipientsWrite => "notification_recipients:write",
            Self::OrganizationAccessTokensRead => "organization_access_tokens:read",
            Self::OrganizationAccessTokensWrite => "organization_access_tokens:write",
        }
    }
}
impl ::std::fmt::Display for Scope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum SubType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "organization")]
    Organization,
}
impl SubType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Organization => "organization",
        }
    }
}
impl ::std::fmt::Display for SubType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for SubType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default,
)]
pub enum WebTokenRequestSubType {
    #[default]
    #[serde(rename = "user")]
    User,
    #[serde(rename = "organization")]
    Organization,
}
impl WebTokenRequestSubType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Organization => "organization",
        }
    }
}
impl ::std::fmt::Display for WebTokenRequestSubType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for WebTokenRequestSubType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
