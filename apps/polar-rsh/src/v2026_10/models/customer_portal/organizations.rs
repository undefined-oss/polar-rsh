pub mod get;
pub use self::get::*;
pub struct CustomerPortalOrganizations<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalOrganizations<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Get a customer portal's organization by slug.
    pub fn get(
        &self,
        slug: String,
    ) -> crate::v2026_10::models::customer_portal::organizations::CustomerPortalOrganizationsGet<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::organizations::CustomerPortalOrganizationsGet {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            slug,
        }
    }
}
