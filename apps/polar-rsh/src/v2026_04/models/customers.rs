pub mod create;
pub mod delete;
pub mod delete_external;
pub mod enums;
pub mod export;
pub mod get;
pub mod get_external;
pub mod get_state;
pub mod get_state_external;
pub mod list;
pub mod list_payment_methods;
pub mod list_payment_methods_external;
pub mod members;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update;
pub mod update_external;
pub use self::{
    create::*, delete::*, delete_external::*, enums::*, export::*, get::*, get_external::*,
    get_state::*, get_state_external::*, list::*, list_payment_methods::*,
    list_payment_methods_external::*, members::CustomersMembers, newtypes::*, types::*, unions::*,
    update::*, update_external::*,
};
pub struct Customers<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Customers<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a customer.
    ///
    /// Set `external_id` to the customer's ID in your system to look it up by
    /// that ID.
    ///
    /// **Scopes**: `customers:write`
    pub fn create(
        &self,
        payload: crate::v2026_04::models::customers::unions::CustomerCreate,
    ) -> crate::v2026_04::models::customers::CustomersCreate<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersCreate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a customer.
    ///
    /// This action cannot be undone and will immediately:
    /// - Cancel any active subscriptions for the customer
    /// - Revoke all their benefits
    /// - Clear any `external_id`
    ///
    /// Use it only in the context of deleting a user within your
    /// own service. Otherwise, use more granular API endpoints to cancel
    /// a specific subscription or revoke certain benefits.
    ///
    /// Note: The customers information will nonetheless be retained for
    /// historic orders and subscriptions.
    ///
    /// Set `anonymize=true` to also anonymize PII for GDPR compliance.
    ///
    /// **Scopes**: `customers:write`
    pub fn delete(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
    ) -> crate::v2026_04::models::customers::CustomersDelete<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersDelete {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
            id,
        }
    }

    /// Delete a customer by external ID.
    ///
    /// Immediately cancels any active subscriptions and revokes any active
    /// benefits.
    ///
    /// Set `anonymize=true` to also anonymize PII for GDPR compliance.
    ///
    /// **Scopes**: `customers:write`
    pub fn delete_external(
        &self,
        external_id: String,
    ) -> crate::v2026_04::models::customers::CustomersDeleteExternal<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersDeleteExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
            external_id,
        }
    }

    /// Export customers as a CSV file.
    ///
    /// **Scopes**: `customers:read` `customers:write`
    pub fn export(&self) -> crate::v2026_04::models::customers::CustomersExport<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersExport {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Get a customer by ID.
    ///
    /// **Scopes**: `customers:read` `customers:write`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
    ) -> crate::v2026_04::models::customers::CustomersGet<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a customer by external ID.
    ///
    /// **Scopes**: `customers:read` `customers:write`
    pub fn get_external(
        &self,
        external_id: String,
    ) -> crate::v2026_04::models::customers::CustomersGetExternal<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersGetExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            external_id,
        }
    }

    /// Get a customer state by ID.
    ///
    /// The customer state includes information about
    /// the customer's active subscriptions and benefits.
    ///
    /// It's the ideal endpoint to use when you need to get a full overview
    /// of a customer's status.
    ///
    /// **Scopes**: `customers:read` `customers:write`
    pub fn get_state(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
    ) -> crate::v2026_04::models::customers::CustomersGetState<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersGetState {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a customer state by external ID.
    ///
    /// The customer state includes information about
    /// the customer's active subscriptions and benefits.
    ///
    /// It's the ideal endpoint to use when you need to get a full overview
    /// of a customer's status.
    ///
    /// **Scopes**: `customers:read` `customers:write`
    pub fn get_state_external(
        &self,
        external_id: String,
    ) -> crate::v2026_04::models::customers::CustomersGetStateExternal<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersGetStateExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            external_id,
        }
    }

    /// List customers.
    ///
    /// **Scopes**: `customers:read` `customers:write`
    pub fn list(&self) -> crate::v2026_04::models::customers::CustomersList<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Get saved payment methods of a customer.
    ///
    /// **Scopes**: `customers:read` `customers:write`
    pub fn list_payment_methods(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
    ) -> crate::v2026_04::models::customers::CustomersListPaymentMethods<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersListPaymentMethods {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
            id,
        }
    }

    /// Get saved payment methods of a customer by external ID.
    ///
    /// **Scopes**: `customers:read` `customers:write`
    pub fn list_payment_methods_external(
        &self,
        external_id: String,
    ) -> crate::v2026_04::models::customers::CustomersListPaymentMethodsExternal<'a, ClientService>
    {
        crate::v2026_04::models::customers::CustomersListPaymentMethodsExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
            external_id,
        }
    }

    /// Update a customer.
    ///
    /// **Scopes**: `customers:write`
    pub fn update(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
        payload: crate::v2026_04::models::customers::types::CustomerUpdate,
    ) -> crate::v2026_04::models::customers::CustomersUpdate<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersUpdate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }

    /// Update a customer by external ID.
    ///
    /// **Scopes**: `customers:write`
    pub fn update_external(
        &self,
        external_id: String,
        payload: crate::v2026_04::models::customers::types::CustomerUpdateExternalId,
    ) -> crate::v2026_04::models::customers::CustomersUpdateExternal<'a, ClientService> {
        crate::v2026_04::models::customers::CustomersUpdateExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            external_id,
            payload,
        }
    }

    pub fn members(
        &self,
    ) -> crate::v2026_04::models::customers::members::CustomersMembers<'a, ClientService> {
        crate::v2026_04::models::customers::members::CustomersMembers::new(self.client)
    }
}
