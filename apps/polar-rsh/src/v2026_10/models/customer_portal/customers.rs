pub mod add_payment_method;
pub mod check_email_update;
pub mod confirm_payment_method;
pub mod delete_payment_method;
pub mod get;
pub mod list_payment_methods;
pub mod request_email_update;
pub mod update;
pub mod verify_email_update;
pub use self::{
    add_payment_method::*, check_email_update::*, confirm_payment_method::*,
    delete_payment_method::*, get::*, list_payment_methods::*, request_email_update::*, update::*,
    verify_email_update::*,
};
pub struct CustomerPortalCustomers<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalCustomers<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Add a payment method to the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn add_payment_method(
        &self,
        payload: crate::v2026_10::models::customers::types::CustomerPaymentMethodCreate,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersAddPaymentMethod<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersAddPaymentMethod {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Check if an email change verification token is still valid.
    pub fn check_email_update(
        &self,
        params: crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersCheckEmailUpdateParams,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersCheckEmailUpdate<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersCheckEmailUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params,
        }
    }

    /// Confirm a payment method for the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn confirm_payment_method(
        &self,
        payload: crate::v2026_10::models::customers::types::CustomerPaymentMethodConfirm,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersConfirmPaymentMethod<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersConfirmPaymentMethod {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a payment method from the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn delete_payment_method(
        &self,
        id: crate::v2026_10::models::common::ids::PaymentMethodId,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersDeletePaymentMethod<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersDeletePaymentMethod {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// Get authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn get(
        &self,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersGet<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersGet {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
        }
    }

    /// Get saved payment methods of the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list_payment_methods(
        &self,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersListPaymentMethods<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersListPaymentMethods {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Request an email change for the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn request_email_update(
        &self,
        payload: crate::v2026_10::models::customers::types::CustomerEmailUpdateRequest,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersRequestEmailUpdate<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersRequestEmailUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Update authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn update(
        &self,
        payload: crate::v2026_10::models::customers::types::CustomerPortalCustomerUpdate,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersUpdate<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Verify an email change using the token from the verification email.
    pub fn verify_email_update(
        &self,
        payload: crate::v2026_10::models::customers::types::CustomerEmailUpdateVerifyRequest,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersVerifyEmailUpdate<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomersVerifyEmailUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }
}
