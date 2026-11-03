use crate::{
    client::{PolarClient, RequestOptions},
    models::common::OrganizationId,
};

mod create;
mod enums;
mod errors;
mod get;
mod list;
mod types;
mod update;

#[cfg(test)]
mod page_stream_tests;

#[cfg(test)]
mod item_stream_tests;

#[cfg(test)]
mod create_tests;

#[cfg(test)]
mod complete_create_tests;

#[cfg(test)]
mod update_tests;

#[cfg(test)]
mod error_tests;

pub use create::{
    CreateOrganization, CreateOrganizationError, OrganizationCreate, OrganizationCreateRequired,
};
pub use enums::{
    OrganizationSocialPlatforms, OrganizationSubscriptionSettingsProrationBehavior,
    OrganizationSwitchingFrom,
};
pub use errors::{
    CannotCreateOrganizationError, CannotCreateOrganizationErrorKind, DisputeAutoAcceptNotEnabled,
    DisputeAutoAcceptNotEnabledKind, SsoEnforcementRequiresConnection,
    SsoEnforcementRequiresConnectionKind,
};
pub use get::{GetOrganization, GetOrganizationError};
pub use list::{
    ListOrganizations, ListOrganizationsError, ListOrganizationsParams, OrganizationSortField,
};
pub use types::{
    CustomerPortalCustomerSettings, CustomerPortalSubscriptionSettings,
    CustomerPortalUsageSettings, Organization, OrganizationCustomerEmailSettings,
    OrganizationCustomerPortalSettings, OrganizationDetails, OrganizationDisputeSettingsUpdate,
    OrganizationFeatureSettingsUpdate, OrganizationLegalEntity, OrganizationSocialLink,
    OrganizationSubscriptionSettings,
};
pub use update::{OrganizationUpdate, UpdateOrganization, UpdateOrganizationError};

pub struct Organizations<'a, ClientService> {
    client: &'a PolarClient<ClientService>,
}

impl<'a, ClientService> Organizations<'a, ClientService> {
    pub(crate) fn new(client: &'a PolarClient<ClientService>) -> Self {
        Self { client }
    }

    pub fn get(&self, organization_id: OrganizationId) -> GetOrganization<'a, ClientService> {
        GetOrganization {
            client: self.client,
            id: organization_id,
            options: RequestOptions::default(),
        }
    }

    pub fn list(&self) -> ListOrganizations<'a, ClientService> {
        ListOrganizations { client: self.client, options: RequestOptions::default(), params: None }
    }

    pub fn create(&self, body: OrganizationCreate) -> CreateOrganization<'a, ClientService> {
        CreateOrganization {
            client: self.client,
            options: RequestOptions::default(),
            payload: body,
        }
    }

    pub fn update(
        &self,
        organization_id: OrganizationId,
        payload: OrganizationUpdate,
    ) -> UpdateOrganization<'a, ClientService> {
        UpdateOrganization {
            client: self.client,
            id: organization_id,
            payload,
            options: RequestOptions::default(),
        }
    }
}
