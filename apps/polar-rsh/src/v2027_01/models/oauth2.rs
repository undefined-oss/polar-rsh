pub mod authorize;
pub mod clients;
pub mod enums;
pub mod newtypes;
pub mod oauth2_introspect_token;
pub mod oauth2_request_token;
pub mod oauth2_revoke_token;
pub mod types;
pub mod unions;
pub mod userinfo;
pub use self::{
    authorize::*, clients::Oauth2Clients, enums::*, newtypes::*, oauth2_introspect_token::*,
    oauth2_request_token::*, oauth2_revoke_token::*, types::*, unions::*, userinfo::*,
};
pub struct Oauth2<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Oauth2<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Authorize
    pub fn authorize(&self) -> crate::v2027_01::models::oauth2::Oauth2Authorize<'a, ClientService> {
        crate::v2027_01::models::oauth2::Oauth2Authorize {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
        }
    }

    /// Get information about an access token.
    pub fn oauth2_introspect_token(
        &self,
        payload: crate::v2027_01::models::oauth2::types::IntrospectTokenRequest,
    ) -> crate::v2027_01::models::oauth2::Oauth2Oauth2IntrospectToken<'a, ClientService> {
        crate::v2027_01::models::oauth2::Oauth2Oauth2IntrospectToken {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Request an access token using a valid grant.
    pub fn oauth2_request_token(
        &self,
        payload: crate::v2027_01::models::oauth2::unions::Oauth2Oauth2RequestTokenRequestUnion,
    ) -> crate::v2027_01::models::oauth2::Oauth2Oauth2RequestToken<'a, ClientService> {
        crate::v2027_01::models::oauth2::Oauth2Oauth2RequestToken {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Revoke an access token or a refresh token.
    pub fn oauth2_revoke_token(
        &self,
        payload: crate::v2027_01::models::oauth2::types::RevokeTokenRequest,
    ) -> crate::v2027_01::models::oauth2::Oauth2Oauth2RevokeToken<'a, ClientService> {
        crate::v2027_01::models::oauth2::Oauth2Oauth2RevokeToken {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Get information about the authenticated user.
    pub fn userinfo(&self) -> crate::v2027_01::models::oauth2::Oauth2Userinfo<'a, ClientService> {
        crate::v2027_01::models::oauth2::Oauth2Userinfo {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
        }
    }

    pub fn clients(
        &self,
    ) -> crate::v2027_01::models::oauth2::clients::Oauth2Clients<'a, ClientService> {
        crate::v2027_01::models::oauth2::clients::Oauth2Clients::new(self.client)
    }
}
