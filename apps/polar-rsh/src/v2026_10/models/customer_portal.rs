pub mod benefit_grants;
pub mod customer_meters;
pub mod customer_session;
pub mod customers;
pub mod downloadables;
pub mod license_keys;
pub mod members;
pub mod orders;
pub mod organizations;
pub mod seats;
pub mod subscriptions;
pub mod wallets;
pub use self::{
    benefit_grants::CustomerPortalBenefitGrants, customer_meters::CustomerPortalCustomerMeters,
    customer_session::CustomerPortalCustomerSession, customers::CustomerPortalCustomers,
    downloadables::CustomerPortalDownloadables, license_keys::CustomerPortalLicenseKeys,
    members::CustomerPortalMembers, orders::CustomerPortalOrders,
    organizations::CustomerPortalOrganizations, seats::CustomerPortalSeats,
    subscriptions::CustomerPortalSubscriptions, wallets::CustomerPortalWallets,
};
pub struct CustomerPortal<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortal<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    pub fn benefit_grants(
        &self,
    ) -> crate::v2026_10::models::customer_portal::benefit_grants::CustomerPortalBenefitGrants<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::benefit_grants::CustomerPortalBenefitGrants::new(
            self.client,
        )
    }

    pub fn customer_meters(
        &self,
    ) -> crate::v2026_10::models::customer_portal::customer_meters::CustomerPortalCustomerMeters<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::customer_meters::CustomerPortalCustomerMeters::new(
            self.client,
        )
    }

    pub fn customer_session(
        &self,
    ) -> crate::v2026_10::models::customer_portal::customer_session::CustomerPortalCustomerSession<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customer_session::CustomerPortalCustomerSession::new(
            self.client,
        )
    }

    pub fn customers(
        &self,
    ) -> crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomers<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::customers::CustomerPortalCustomers::new(
            self.client,
        )
    }

    pub fn downloadables(
        &self,
    ) -> crate::v2026_10::models::customer_portal::downloadables::CustomerPortalDownloadables<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::downloadables::CustomerPortalDownloadables::new(
            self.client,
        )
    }

    pub fn license_keys(
        &self,
    ) -> crate::v2026_10::models::customer_portal::license_keys::CustomerPortalLicenseKeys<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::license_keys::CustomerPortalLicenseKeys::new(
            self.client,
        )
    }

    pub fn members(
        &self,
    ) -> crate::v2026_10::models::customer_portal::members::CustomerPortalMembers<'a, ClientService>
    {
        crate::v2026_10::models::customer_portal::members::CustomerPortalMembers::new(self.client)
    }

    pub fn orders(
        &self,
    ) -> crate::v2026_10::models::customer_portal::orders::CustomerPortalOrders<'a, ClientService>
    {
        crate::v2026_10::models::customer_portal::orders::CustomerPortalOrders::new(self.client)
    }

    pub fn organizations(
        &self,
    ) -> crate::v2026_10::models::customer_portal::organizations::CustomerPortalOrganizations<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::organizations::CustomerPortalOrganizations::new(
            self.client,
        )
    }

    pub fn seats(
        &self,
    ) -> crate::v2026_10::models::customer_portal::seats::CustomerPortalSeats<'a, ClientService>
    {
        crate::v2026_10::models::customer_portal::seats::CustomerPortalSeats::new(self.client)
    }

    pub fn subscriptions(
        &self,
    ) -> crate::v2026_10::models::customer_portal::subscriptions::CustomerPortalSubscriptions<
        'a,
        ClientService,
    > {
        crate::v2026_10::models::customer_portal::subscriptions::CustomerPortalSubscriptions::new(
            self.client,
        )
    }

    pub fn wallets(
        &self,
    ) -> crate::v2026_10::models::customer_portal::wallets::CustomerPortalWallets<'a, ClientService>
    {
        crate::v2026_10::models::customer_portal::wallets::CustomerPortalWallets::new(self.client)
    }
}
