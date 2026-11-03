pub mod oauth2_create_client;
pub mod oauth2_delete_client;
pub mod oauth2_get_client;
pub mod oauth2_update_client;
pub use self::{
    oauth2_create_client::*, oauth2_delete_client::*, oauth2_get_client::*, oauth2_update_client::*,
};
pub struct Oauth2Clients<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Oauth2Clients<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create an OAuth2 client.
    pub fn oauth2_create_client(
        &self,
        payload: crate::v2026_04::models::oauth2::types::OAuth2ClientConfiguration,
    ) -> crate::v2026_04::models::oauth2::clients::Oauth2ClientsOauth2CreateClient<'a, ClientService>
    {
        crate::v2026_04::models::oauth2::clients::Oauth2ClientsOauth2CreateClient {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete an OAuth2 client.
    pub fn oauth2_delete_client(
        &self,
        client_id: String,
    ) -> crate::v2026_04::models::oauth2::clients::Oauth2ClientsOauth2DeleteClient<'a, ClientService>
    {
        crate::v2026_04::models::oauth2::clients::Oauth2ClientsOauth2DeleteClient {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            client_id,
        }
    }

    /// Get an OAuth2 client by Client ID.
    pub fn oauth2_get_client(
        &self,
        client_id: String,
    ) -> crate::v2026_04::models::oauth2::clients::Oauth2ClientsOauth2GetClient<'a, ClientService>
    {
        crate::v2026_04::models::oauth2::clients::Oauth2ClientsOauth2GetClient {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            client_id,
        }
    }

    /// Update an OAuth2 client.
    pub fn oauth2_update_client(
        &self,
        client_id: String,
        payload: crate::v2026_04::models::oauth2::types::OAuth2ClientConfigurationUpdate,
    ) -> crate::v2026_04::models::oauth2::clients::Oauth2ClientsOauth2UpdateClient<'a, ClientService>
    {
        crate::v2026_04::models::oauth2::clients::Oauth2ClientsOauth2UpdateClient {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            client_id,
            payload,
        }
    }
}
