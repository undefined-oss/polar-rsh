pub mod activate;
pub mod deactivate;
pub mod get;
pub mod list;
pub mod rotate;
pub mod validate;
pub use self::{activate::*, deactivate::*, get::*, list::*, rotate::*, validate::*};
pub struct CustomerPortalLicenseKeys<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalLicenseKeys<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Activate a license key instance.
    ///
    /// > This endpoint doesn't require authentication and can be safely used on
    /// > a public
    /// > client, like a desktop application or a mobile app.
    /// > If you plan to validate a license key on a server, use the
    /// > `/v1/license-keys/activate`
    /// > endpoint instead.
    pub fn activate(
        &self,
        payload: crate::v2027_01::models::license_keys::types::LicenseKeyActivate,
    ) -> crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysActivate<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysActivate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Deactivate a license key instance.
    ///
    /// > This endpoint doesn't require authentication and can be safely used on
    /// > a public
    /// > client, like a desktop application or a mobile app.
    /// > If you plan to validate a license key on a server, use the
    /// > `/v1/license-keys/deactivate`
    /// > endpoint instead.
    pub fn deactivate(
        &self,
        payload: crate::v2027_01::models::license_keys::types::LicenseKeyDeactivate,
    ) -> crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysDeactivate<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysDeactivate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Get a license key.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::LicenseKeyId,
    ) -> crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysGet<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysList<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysList {
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
    /// **Scopes**: `customer_portal:write`
    pub fn rotate(
        &self,
        id: crate::v2027_01::models::common::ids::LicenseKeyId,
    ) -> crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysRotate<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysRotate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Validate a license key.
    ///
    /// > This endpoint doesn't require authentication and can be safely used on
    /// > a public
    /// > client, like a desktop application or a mobile app.
    /// > If you plan to validate a license key on a server, use the
    /// > `/v1/license-keys/validate`
    /// > endpoint instead.
    pub fn validate(
        &self,
        payload: crate::v2027_01::models::license_keys::types::LicenseKeyValidate,
    ) -> crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysValidate<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::license_keys::CustomerPortalLicenseKeysValidate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }
}
