pub mod accept;
pub mod enums;
pub mod get;
pub mod list;
pub mod types;
pub use self::{accept::*, enums::*, get::*, list::*, types::*};
pub struct Disputes<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Disputes<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Accept a dispute, conceding the chargeback.
    ///
    /// Closes the dispute with the processor (settling it as `lost`) and
    /// records the merchant's decision on the dispute's support case.
    ///
    /// **Scopes**: `disputes:write`
    pub fn accept(
        &self,
        id: crate::v2026_04::models::common::ids::DisputeId,
    ) -> crate::v2026_04::models::disputes::DisputesAccept<'a, ClientService> {
        crate::v2026_04::models::disputes::DisputesAccept {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a dispute by ID.
    ///
    /// **Scopes**: `disputes:read` `disputes:write`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::DisputeId,
    ) -> crate::v2026_04::models::disputes::DisputesGet<'a, ClientService> {
        crate::v2026_04::models::disputes::DisputesGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// List disputes.
    ///
    /// **Scopes**: `disputes:read` `disputes:write`
    pub fn list(&self) -> crate::v2026_04::models::disputes::DisputesList<'a, ClientService> {
        crate::v2026_04::models::disputes::DisputesList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }
}
