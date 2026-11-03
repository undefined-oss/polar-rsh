pub mod create_webhook_endpoint;
pub mod delete_webhook_endpoint;
pub mod enums;
pub mod events;
pub mod get_webhook_endpoint;
pub mod list_webhook_deliveries;
pub mod list_webhook_endpoints;
pub mod newtypes;
pub mod redeliver_webhook_event;
pub mod reset_webhook_endpoint_secret;
pub mod types;
pub mod unions;
pub mod update_webhook_endpoint;
pub mod verification;
pub use self::{
    create_webhook_endpoint::*, delete_webhook_endpoint::*, enums::*, events::*,
    get_webhook_endpoint::*, list_webhook_deliveries::*, list_webhook_endpoints::*, newtypes::*,
    redeliver_webhook_event::*, reset_webhook_endpoint_secret::*, types::*, unions::*,
    update_webhook_endpoint::*, verification::*,
};
pub struct Webhooks<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Webhooks<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a webhook endpoint.
    ///
    /// **Scopes**: `webhooks:write`
    pub fn create_webhook_endpoint(
        &self,
        payload: crate::v2026_10::models::webhooks::types::WebhookEndpointCreate,
    ) -> crate::v2026_10::models::webhooks::WebhooksCreateWebhookEndpoint<'a, ClientService> {
        crate::v2026_10::models::webhooks::WebhooksCreateWebhookEndpoint {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a webhook endpoint.
    ///
    /// **Scopes**: `webhooks:write`
    pub fn delete_webhook_endpoint(
        &self,
        id: crate::v2026_10::models::common::ids::EndpointId,
    ) -> crate::v2026_10::models::webhooks::WebhooksDeleteWebhookEndpoint<'a, ClientService> {
        crate::v2026_10::models::webhooks::WebhooksDeleteWebhookEndpoint {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a webhook endpoint by ID.
    ///
    /// **Scopes**: `webhooks:read` `webhooks:write`
    pub fn get_webhook_endpoint(
        &self,
        id: crate::v2026_10::models::common::ids::EndpointId,
    ) -> crate::v2026_10::models::webhooks::WebhooksGetWebhookEndpoint<'a, ClientService> {
        crate::v2026_10::models::webhooks::WebhooksGetWebhookEndpoint {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// List webhook deliveries.
    ///
    /// Deliveries are all the attempts to deliver a webhook event to an
    /// endpoint.
    ///
    /// **Scopes**: `webhooks:read` `webhooks:write`
    pub fn list_webhook_deliveries(
        &self,
    ) -> crate::v2026_10::models::webhooks::WebhooksListWebhookDeliveries<'a, ClientService> {
        crate::v2026_10::models::webhooks::WebhooksListWebhookDeliveries {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// List webhook endpoints.
    ///
    /// **Scopes**: `webhooks:read` `webhooks:write`
    pub fn list_webhook_endpoints(
        &self,
    ) -> crate::v2026_10::models::webhooks::WebhooksListWebhookEndpoints<'a, ClientService> {
        crate::v2026_10::models::webhooks::WebhooksListWebhookEndpoints {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Schedule the re-delivery of a webhook event.
    ///
    /// **Scopes**: `webhooks:write`
    pub fn redeliver_webhook_event(
        &self,
        id: crate::v2026_10::models::common::ids::EventId,
    ) -> crate::v2026_10::models::webhooks::WebhooksRedeliverWebhookEvent<'a, ClientService> {
        crate::v2026_10::models::webhooks::WebhooksRedeliverWebhookEvent {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// Regenerate a webhook endpoint secret.
    ///
    /// **Scopes**: `webhooks:write`
    pub fn reset_webhook_endpoint_secret(
        &self,
        id: crate::v2026_10::models::common::ids::EndpointId,
    ) -> crate::v2026_10::models::webhooks::WebhooksResetWebhookEndpointSecret<'a, ClientService>
    {
        crate::v2026_10::models::webhooks::WebhooksResetWebhookEndpointSecret {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// Update a webhook endpoint.
    ///
    /// **Scopes**: `webhooks:write`
    pub fn update_webhook_endpoint(
        &self,
        id: crate::v2026_10::models::common::ids::EndpointId,
        payload: crate::v2026_10::models::webhooks::types::WebhookEndpointUpdate,
    ) -> crate::v2026_10::models::webhooks::WebhooksUpdateWebhookEndpoint<'a, ClientService> {
        crate::v2026_10::models::webhooks::WebhooksUpdateWebhookEndpoint {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
