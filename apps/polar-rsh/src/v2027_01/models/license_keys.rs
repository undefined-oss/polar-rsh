pub mod activate;
pub mod deactivate;
pub mod enums;
pub mod get;
pub mod get_activation;
pub mod list;
pub mod newtypes;
pub mod rotate;
pub mod types;
pub mod unions;
pub mod update;
pub mod validate;
pub use self::{
    activate::*, deactivate::*, enums::*, get::*, get_activation::*, list::*, newtypes::*,
    rotate::*, types::*, unions::*, update::*, validate::*,
};
pub struct LicenseKeys<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> LicenseKeys<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Activate a license key instance.
    ///
    /// **Scopes**: `license_keys:write`
    pub fn activate(
        &self,
        payload: crate::v2027_01::models::license_keys::types::LicenseKeyActivate,
    ) -> crate::v2027_01::models::license_keys::LicenseKeysActivate<'a, ClientService> {
        crate::v2027_01::models::license_keys::LicenseKeysActivate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Deactivate a license key instance.
    ///
    /// **Scopes**: `license_keys:write`
    pub fn deactivate(
        &self,
        payload: crate::v2027_01::models::license_keys::types::LicenseKeyDeactivate,
    ) -> crate::v2027_01::models::license_keys::LicenseKeysDeactivate<'a, ClientService> {
        crate::v2027_01::models::license_keys::LicenseKeysDeactivate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Get a license key.
    ///
    /// **Scopes**: `license_keys:read` `license_keys:write`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::LicenseKeyId,
    ) -> crate::v2027_01::models::license_keys::LicenseKeysGet<'a, ClientService> {
        crate::v2027_01::models::license_keys::LicenseKeysGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a license key activation.
    ///
    /// **Scopes**: `license_keys:read` `license_keys:write`
    pub fn get_activation(
        &self,
        id: crate::v2027_01::models::common::ids::LicenseKeyId,
        activation_id: crate::v2027_01::models::common::ids::ActivationId,
    ) -> crate::v2027_01::models::license_keys::LicenseKeysGetActivation<'a, ClientService> {
        crate::v2027_01::models::license_keys::LicenseKeysGetActivation {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
            activation_id,
        }
    }

    /// Get license keys connected to the given organization & filters.
    ///
    /// **Scopes**: `license_keys:read` `license_keys:write`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::license_keys::LicenseKeysList<'a, ClientService> {
        crate::v2027_01::models::license_keys::LicenseKeysList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Rotate a license key.
    ///
    /// Generates a new key string for the same license key record. The previous
    /// key string immediately stops validating. Status, usage, limits, expiry,
    /// and activations are preserved.
    ///
    /// **Scopes**: `license_keys:write`
    pub fn rotate(
        &self,
        id: crate::v2027_01::models::common::ids::LicenseKeyId,
    ) -> crate::v2027_01::models::license_keys::LicenseKeysRotate<'a, ClientService> {
        crate::v2027_01::models::license_keys::LicenseKeysRotate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Update a license key.
    ///
    /// **Scopes**: `license_keys:write`
    pub fn update(
        &self,
        id: crate::v2027_01::models::common::ids::LicenseKeyId,
        payload: crate::v2027_01::models::license_keys::types::LicenseKeyUpdate,
    ) -> crate::v2027_01::models::license_keys::LicenseKeysUpdate<'a, ClientService> {
        crate::v2027_01::models::license_keys::LicenseKeysUpdate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
            payload,
        }
    }

    /// Validate a license key.
    ///
    /// **Scopes**: `license_keys:write`
    pub fn validate(
        &self,
        payload: crate::v2027_01::models::license_keys::types::LicenseKeyValidate,
    ) -> crate::v2027_01::models::license_keys::LicenseKeysValidate<'a, ClientService> {
        crate::v2027_01::models::license_keys::LicenseKeysValidate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }
}
