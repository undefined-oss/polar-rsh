#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "sub_type", deny_unknown_fields)]
pub enum Oauth2AuthorizeResponse200Union {
    #[serde(rename = "user")]
    User {
        client: crate::v2027_01::models::oauth2::types::OAuth2ClientPublic,
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        sub: Option<crate::v2027_01::models::oauth2::types::AuthorizeUser>,
        scopes: Vec<crate::v2027_01::models::oauth2::enums::Scope>,
        organizations: Vec<crate::v2027_01::models::oauth2::types::AuthorizeOrganization>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        requires_single_organization: Option<bool>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        scope_display_names: Option<std::collections::BTreeMap<String, String>>,
    },
    #[serde(rename = "organization")]
    Organization {
        client: crate::v2027_01::models::oauth2::types::OAuth2ClientPublic,
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        sub: Option<crate::v2027_01::models::oauth2::types::AuthorizeOrganization>,
        scopes: Vec<crate::v2027_01::models::oauth2::enums::Scope>,
        organizations: Vec<crate::v2027_01::models::oauth2::types::AuthorizeOrganization>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        requires_single_organization: Option<bool>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        scope_display_names: Option<std::collections::BTreeMap<String, String>>,
    },
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Oauth2Oauth2RequestTokenRequestUnion {
    AuthorizationCodeTokenRequest(
        crate::v2027_01::models::oauth2::types::AuthorizationCodeTokenRequest,
    ),
    RefreshTokenRequest(crate::v2027_01::models::oauth2::types::RefreshTokenRequest),
    WebTokenRequest(crate::v2027_01::models::oauth2::types::WebTokenRequest),
}
impl<'de> serde::Deserialize<'de> for Oauth2Oauth2RequestTokenRequestUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::oauth2::types::AuthorizationCodeTokenRequest,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "Oauth2Oauth2RequestTokenRequestUnion"
                )));
            }
            matched = Some(Self::AuthorizationCodeTokenRequest(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::oauth2::types::RefreshTokenRequest,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "Oauth2Oauth2RequestTokenRequestUnion"
                )));
            }
            matched = Some(Self::RefreshTokenRequest(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::oauth2::types::WebTokenRequest,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "Oauth2Oauth2RequestTokenRequestUnion"
                )));
            }
            matched = Some(Self::WebTokenRequest(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "Oauth2Oauth2RequestTokenRequestUnion"
            ))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Oauth2UserinfoResponse200Union {
    UserInfoUser(crate::v2027_01::models::oauth2::types::UserInfoUser),
    UserInfoOrganization(crate::v2027_01::models::oauth2::types::UserInfoOrganization),
}
