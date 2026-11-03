pub mod create;
pub mod delete;
pub mod list;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update;
pub mod uploaded;
pub use self::{
    create::*, delete::*, list::*, newtypes::*, types::*, unions::*, update::*, uploaded::*,
};
pub struct Files<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Files<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a file.
    ///
    /// **Scopes**: `files:write`
    pub fn create(
        &self,
        payload: crate::v2026_10::models::files::unions::FileCreate,
    ) -> crate::v2026_10::models::files::FilesCreate<'a, ClientService> {
        crate::v2026_10::models::files::FilesCreate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a file.
    ///
    /// **Scopes**: `files:write`
    pub fn delete(
        &self,
        id: crate::v2026_10::models::common::ids::FileId,
    ) -> crate::v2026_10::models::files::FilesDelete<'a, ClientService> {
        crate::v2026_10::models::files::FilesDelete {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// List files.
    ///
    /// **Scopes**: `files:read` `files:write`
    pub fn list(&self) -> crate::v2026_10::models::files::FilesList<'a, ClientService> {
        crate::v2026_10::models::files::FilesList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a file.
    ///
    /// **Scopes**: `files:write`
    pub fn update(
        &self,
        id: crate::v2026_10::models::common::ids::FileId,
        payload: crate::v2026_10::models::files::types::FilePatch,
    ) -> crate::v2026_10::models::files::FilesUpdate<'a, ClientService> {
        crate::v2026_10::models::files::FilesUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
            payload,
        }
    }

    /// Complete a file upload.
    ///
    /// **Scopes**: `files:write`
    pub fn uploaded(
        &self,
        id: crate::v2026_10::models::common::ids::FileId,
        payload: crate::v2026_10::models::files::types::FileUploadCompleted,
    ) -> crate::v2026_10::models::files::FilesUploaded<'a, ClientService> {
        crate::v2026_10::models::files::FilesUploaded {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
