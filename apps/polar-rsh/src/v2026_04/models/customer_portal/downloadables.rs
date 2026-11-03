pub mod list;
pub use self::list::*;
pub struct CustomerPortalDownloadables<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalDownloadables<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list(
        &self,
    ) -> crate::v2026_04::models::customer_portal::downloadables::CustomerPortalDownloadablesList<
        'a,
        ClientService,
    >{
        crate::v2026_04::models::customer_portal::downloadables::CustomerPortalDownloadablesList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }
}
