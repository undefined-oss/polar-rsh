pub mod enums;
pub mod list;
pub mod types;
pub mod unions;
pub use self::{enums::*, list::*, types::*, unions::*};
pub struct BenefitGrants<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> BenefitGrants<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// List benefit grants across all benefits accessible to the authenticated
    /// subject.
    ///
    /// **Scopes**: `benefits:read` `benefits:write`
    pub fn list(
        &self,
    ) -> crate::v2026_10::models::benefit_grants::BenefitGrantsList<'a, ClientService> {
        crate::v2026_10::models::benefit_grants::BenefitGrantsList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }
}
