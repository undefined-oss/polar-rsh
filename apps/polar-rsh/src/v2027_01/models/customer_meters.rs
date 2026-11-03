pub mod credits;
pub mod enums;
pub mod get;
pub mod list;
pub mod newtypes;
pub mod reverse_credit;
pub mod types;
pub mod unions;
pub use self::{
    credits::*, enums::*, get::*, list::*, newtypes::*, reverse_credit::*, types::*, unions::*,
};
pub struct CustomerMeters<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerMeters<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Manually grant credits on a customer meter.
    ///
    /// Returns the `meter.credited` event. The customer meter balance is
    /// updated before the response is returned, unless the meter is being
    /// updated concurrently, in which case it catches up shortly after.
    ///
    /// **Scopes**: `customers:write`
    pub fn credits(
        &self,
        payload: crate::v2027_01::models::customer_meters::unions::CustomerMetersCreditsRequestUnion,
    ) -> crate::v2027_01::models::customer_meters::CustomerMetersCredits<'a, ClientService> {
        crate::v2027_01::models::customer_meters::CustomerMetersCredits {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Get a customer meter by ID.
    ///
    /// **Scopes**: `customer_meters:read`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::CustomerMeterId,
    ) -> crate::v2027_01::models::customer_meters::CustomerMetersGet<'a, ClientService> {
        crate::v2027_01::models::customer_meters::CustomerMetersGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// List customer meters.
    ///
    /// **Scopes**: `customer_meters:read`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::customer_meters::CustomerMetersList<'a, ClientService> {
        crate::v2027_01::models::customer_meters::CustomerMetersList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Reverse a manual credit, removing the units the customer hasn't used
    /// yet.
    ///
    /// Works once per credit, within 7 days and in the current billing period.
    /// Returns a `meter.credited` event with negative units and
    /// `parent_id` set to the reversed credit.
    ///
    /// **Scopes**: `customers:write`
    pub fn reverse_credit(
        &self,
        id: crate::v2027_01::models::common::ids::CreditId,
    ) -> crate::v2027_01::models::customer_meters::CustomerMetersReverseCredit<'a, ClientService>
    {
        crate::v2027_01::models::customer_meters::CustomerMetersReverseCredit {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }
}
