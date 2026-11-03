#[derive(Debug, Clone)]
pub struct DownloadableFileCreateRequired {
    pub name: String,
    pub mime_type: String,
    pub size: i64,
    pub upload: crate::v2026_10::models::files::types::S3FileCreateMultipart,
}
/// Schema to create a file to be associated with the downloadables benefit.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DownloadableFileCreate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2026_10::models::common::ids::OrganizationId>,
    pub name: String,
    pub mime_type: String,
    pub size: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checksum_sha256_base64: presence_rs::Presence<String>,
    pub upload: crate::v2026_10::models::files::types::S3FileCreateMultipart,
    pub service: monostate::MustBe!("downloadable"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub version: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl DownloadableFileCreate {
    pub fn new(required: DownloadableFileCreateRequired) -> Self {
        Self {
            organization_id: presence_rs::Presence::Absent,
            name: required.name,
            mime_type: required.mime_type,
            size: required.size,
            checksum_sha256_base64: presence_rs::Presence::Absent,
            upload: required.upload,
            service: Default::default(),
            version: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_10::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checksum_sha256_base64(mut self, value: impl Into<String>) -> Self {
        self.checksum_sha256_base64 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct DownloadableFileReadRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::FileId,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub storage_version: Option<String>,
    pub checksum_etag: Option<String>,
    pub checksum_sha256_base64: Option<String>,
    pub checksum_sha256_hex: Option<String>,
    pub last_modified_at: Option<jiff::Timestamp>,
    pub version: Option<String>,
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    pub flagged_malicious_at: Option<jiff::Timestamp>,
    pub size_readable: String,
}
/// File to be associated with the downloadables benefit.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DownloadableFileRead {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::FileId,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub storage_version: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_etag: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_base64: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_hex: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_modified_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub version: Option<String>,
    pub service: monostate::MustBe!("downloadable"),
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub flagged_malicious_at: Option<jiff::Timestamp>,
    pub size_readable: String,
}
#[allow(clippy::type_complexity)]
impl DownloadableFileRead {
    pub fn new(required: DownloadableFileReadRequired) -> Self {
        Self {
            id: required.id,
            organization_id: required.organization_id,
            name: required.name,
            path: required.path,
            mime_type: required.mime_type,
            size: required.size,
            storage_version: required.storage_version,
            checksum_etag: required.checksum_etag,
            checksum_sha256_base64: required.checksum_sha256_base64,
            checksum_sha256_hex: required.checksum_sha256_hex,
            last_modified_at: required.last_modified_at,
            version: required.version,
            service: Default::default(),
            is_uploaded: required.is_uploaded,
            created_at: required.created_at,
            flagged_malicious_at: required.flagged_malicious_at,
            size_readable: required.size_readable,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct FilePatch {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub version: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl FilePatch {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct FileUploadRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::FileUploadId,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub storage_version: Option<String>,
    pub checksum_etag: Option<String>,
    pub checksum_sha256_base64: Option<String>,
    pub checksum_sha256_hex: Option<String>,
    pub last_modified_at: Option<jiff::Timestamp>,
    pub upload: crate::v2026_10::models::files::types::S3FileUploadMultipart,
    pub version: Option<String>,
    pub service: crate::v2026_10::models::common::enums::FileServiceTypes,
    pub size_readable: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct FileUpload {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::FileUploadId,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub storage_version: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_etag: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_base64: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_hex: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_modified_at: Option<jiff::Timestamp>,
    pub upload: crate::v2026_10::models::files::types::S3FileUploadMultipart,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub version: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub is_uploaded: Option<bool>,
    pub service: crate::v2026_10::models::common::enums::FileServiceTypes,
    pub size_readable: String,
}
#[allow(clippy::type_complexity)]
impl FileUpload {
    pub fn new(required: FileUploadRequired) -> Self {
        Self {
            id: required.id,
            organization_id: required.organization_id,
            name: required.name,
            path: required.path,
            mime_type: required.mime_type,
            size: required.size,
            storage_version: required.storage_version,
            checksum_etag: required.checksum_etag,
            checksum_sha256_base64: required.checksum_sha256_base64,
            checksum_sha256_hex: required.checksum_sha256_hex,
            last_modified_at: required.last_modified_at,
            upload: required.upload,
            version: required.version,
            is_uploaded: None,
            service: required.service,
            size_readable: required.size_readable,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn is_uploaded(mut self, value: bool) -> Self {
        self.is_uploaded = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct FileUploadCompletedRequired {
    pub id: String,
    pub path: String,
    pub parts: Vec<crate::v2026_10::models::files::types::S3FileUploadCompletedPart>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct FileUploadCompleted {
    pub id: String,
    pub path: String,
    pub parts: Vec<crate::v2026_10::models::files::types::S3FileUploadCompletedPart>,
}
#[allow(clippy::type_complexity)]
impl FileUploadCompleted {
    pub fn new(required: FileUploadCompletedRequired) -> Self {
        Self { id: required.id, path: required.path, parts: required.parts }
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationAvatarFileCreateRequired {
    pub name: String,
    /// MIME type of the file. Only images are supported for this type of file.
    pub mime_type: String,
    /// Size of the file. A maximum of 1 MB is allowed for this type of file.
    pub size: crate::v2026_10::models::files::newtypes::OrganizationAvatarFileCreateSize,
    pub upload: crate::v2026_10::models::files::types::S3FileCreateMultipart,
}
/// Schema to create a file to be used as an organization avatar.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationAvatarFileCreate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2026_10::models::common::ids::OrganizationId>,
    pub name: String,
    /// MIME type of the file. Only images are supported for this type of file.
    pub mime_type: String,
    /// Size of the file. A maximum of 1 MB is allowed for this type of file.
    pub size: crate::v2026_10::models::files::newtypes::OrganizationAvatarFileCreateSize,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checksum_sha256_base64: presence_rs::Presence<String>,
    pub upload: crate::v2026_10::models::files::types::S3FileCreateMultipart,
    pub service: monostate::MustBe!("organization_avatar"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub version: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl OrganizationAvatarFileCreate {
    pub fn new(required: OrganizationAvatarFileCreateRequired) -> Self {
        Self {
            organization_id: presence_rs::Presence::Absent,
            name: required.name,
            mime_type: required.mime_type,
            size: required.size,
            checksum_sha256_base64: presence_rs::Presence::Absent,
            upload: required.upload,
            service: Default::default(),
            version: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_10::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checksum_sha256_base64(mut self, value: impl Into<String>) -> Self {
        self.checksum_sha256_base64 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationAvatarFileReadRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::FileId,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub storage_version: Option<String>,
    pub checksum_etag: Option<String>,
    pub checksum_sha256_base64: Option<String>,
    pub checksum_sha256_hex: Option<String>,
    pub last_modified_at: Option<jiff::Timestamp>,
    pub version: Option<String>,
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    pub size_readable: String,
    pub public_url: String,
}
/// File to be used as an organization avatar.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationAvatarFileRead {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::FileId,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub storage_version: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_etag: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_base64: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_hex: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_modified_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub version: Option<String>,
    pub service: monostate::MustBe!("organization_avatar"),
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    pub size_readable: String,
    pub public_url: String,
}
#[allow(clippy::type_complexity)]
impl OrganizationAvatarFileRead {
    pub fn new(required: OrganizationAvatarFileReadRequired) -> Self {
        Self {
            id: required.id,
            organization_id: required.organization_id,
            name: required.name,
            path: required.path,
            mime_type: required.mime_type,
            size: required.size,
            storage_version: required.storage_version,
            checksum_etag: required.checksum_etag,
            checksum_sha256_base64: required.checksum_sha256_base64,
            checksum_sha256_hex: required.checksum_sha256_hex,
            last_modified_at: required.last_modified_at,
            version: required.version,
            service: Default::default(),
            is_uploaded: required.is_uploaded,
            created_at: required.created_at,
            size_readable: required.size_readable,
            public_url: required.public_url,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductMediaFileCreateRequired {
    pub name: String,
    /// MIME type of the file. Only images are supported for this type of file.
    pub mime_type: String,
    /// Size of the file. A maximum of 10 MB is allowed for this type of file.
    pub size: crate::v2026_10::models::files::newtypes::ProductMediaFileCreateSize,
    pub upload: crate::v2026_10::models::files::types::S3FileCreateMultipart,
}
/// Schema to create a file to be used as a product media file.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductMediaFileCreate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2026_10::models::common::ids::OrganizationId>,
    pub name: String,
    /// MIME type of the file. Only images are supported for this type of file.
    pub mime_type: String,
    /// Size of the file. A maximum of 10 MB is allowed for this type of file.
    pub size: crate::v2026_10::models::files::newtypes::ProductMediaFileCreateSize,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checksum_sha256_base64: presence_rs::Presence<String>,
    pub upload: crate::v2026_10::models::files::types::S3FileCreateMultipart,
    pub service: monostate::MustBe!("product_media"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub version: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl ProductMediaFileCreate {
    pub fn new(required: ProductMediaFileCreateRequired) -> Self {
        Self {
            organization_id: presence_rs::Presence::Absent,
            name: required.name,
            mime_type: required.mime_type,
            size: required.size,
            checksum_sha256_base64: presence_rs::Presence::Absent,
            upload: required.upload,
            service: Default::default(),
            version: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_10::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checksum_sha256_base64(mut self, value: impl Into<String>) -> Self {
        self.checksum_sha256_base64 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct S3FileCreateMultipartRequired {
    pub parts: Vec<crate::v2026_10::models::files::types::S3FileCreatePart>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct S3FileCreateMultipart {
    pub parts: Vec<crate::v2026_10::models::files::types::S3FileCreatePart>,
}
#[allow(clippy::type_complexity)]
impl S3FileCreateMultipart {
    pub fn new(required: S3FileCreateMultipartRequired) -> Self {
        Self { parts: required.parts }
    }
}
#[derive(Debug, Clone)]
pub struct S3FileCreatePartRequired {
    pub number: i64,
    pub chunk_start: i64,
    pub chunk_end: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct S3FileCreatePart {
    pub number: i64,
    pub chunk_start: i64,
    pub chunk_end: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checksum_sha256_base64: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl S3FileCreatePart {
    pub fn new(required: S3FileCreatePartRequired) -> Self {
        Self {
            number: required.number,
            chunk_start: required.chunk_start,
            chunk_end: required.chunk_end,
            checksum_sha256_base64: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checksum_sha256_base64(mut self, value: impl Into<String>) -> Self {
        self.checksum_sha256_base64 = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct S3FileUploadCompletedPartRequired {
    pub number: i64,
    pub checksum_etag: String,
    pub checksum_sha256_base64: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct S3FileUploadCompletedPart {
    pub number: i64,
    pub checksum_etag: String,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_base64: Option<String>,
}
#[allow(clippy::type_complexity)]
impl S3FileUploadCompletedPart {
    pub fn new(required: S3FileUploadCompletedPartRequired) -> Self {
        Self {
            number: required.number,
            checksum_etag: required.checksum_etag,
            checksum_sha256_base64: required.checksum_sha256_base64,
        }
    }
}
#[derive(Debug, Clone)]
pub struct S3FileUploadMultipartRequired {
    pub id: String,
    pub path: String,
    pub parts: Vec<crate::v2026_10::models::files::types::S3FileUploadPart>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct S3FileUploadMultipart {
    pub id: String,
    pub path: String,
    pub parts: Vec<crate::v2026_10::models::files::types::S3FileUploadPart>,
}
#[allow(clippy::type_complexity)]
impl S3FileUploadMultipart {
    pub fn new(required: S3FileUploadMultipartRequired) -> Self {
        Self { id: required.id, path: required.path, parts: required.parts }
    }
}
#[derive(Debug, Clone)]
pub struct S3FileUploadPartRequired {
    pub number: i64,
    pub chunk_start: i64,
    pub chunk_end: i64,
    pub url: String,
    pub expires_at: jiff::Timestamp,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct S3FileUploadPart {
    pub number: i64,
    pub chunk_start: i64,
    pub chunk_end: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checksum_sha256_base64: presence_rs::Presence<String>,
    pub url: String,
    pub expires_at: jiff::Timestamp,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub headers: Option<std::collections::BTreeMap<String, String>>,
}
#[allow(clippy::type_complexity)]
impl S3FileUploadPart {
    pub fn new(required: S3FileUploadPartRequired) -> Self {
        Self {
            number: required.number,
            chunk_start: required.chunk_start,
            chunk_end: required.chunk_end,
            checksum_sha256_base64: presence_rs::Presence::Absent,
            url: required.url,
            expires_at: required.expires_at,
            headers: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checksum_sha256_base64(mut self, value: impl Into<String>) -> Self {
        self.checksum_sha256_base64 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn headers(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SupportCaseAttachmentFileCreateRequired {
    pub name: String,
    /// MIME type of the file. Images, videos, PDF, CSV, plain text, Word and
    /// Excel documents are supported.
    pub mime_type: String,
    /// Size of the file. A maximum of 250 MB is allowed for this type of file.
    pub size: crate::v2026_10::models::files::newtypes::SupportCaseAttachmentFileCreateSize,
    pub upload: crate::v2026_10::models::files::types::S3FileCreateMultipart,
}
/// Schema to create a file attached to a support case.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SupportCaseAttachmentFileCreate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2026_10::models::common::ids::OrganizationId>,
    pub name: String,
    /// MIME type of the file. Images, videos, PDF, CSV, plain text, Word and
    /// Excel documents are supported.
    pub mime_type: String,
    /// Size of the file. A maximum of 250 MB is allowed for this type of file.
    pub size: crate::v2026_10::models::files::newtypes::SupportCaseAttachmentFileCreateSize,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checksum_sha256_base64: presence_rs::Presence<String>,
    pub upload: crate::v2026_10::models::files::types::S3FileCreateMultipart,
    pub service: monostate::MustBe!("support_case_attachment"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub version: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl SupportCaseAttachmentFileCreate {
    pub fn new(required: SupportCaseAttachmentFileCreateRequired) -> Self {
        Self {
            organization_id: presence_rs::Presence::Absent,
            name: required.name,
            mime_type: required.mime_type,
            size: required.size,
            checksum_sha256_base64: presence_rs::Presence::Absent,
            upload: required.upload,
            service: Default::default(),
            version: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_10::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn checksum_sha256_base64(mut self, value: impl Into<String>) -> Self {
        self.checksum_sha256_base64 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct SupportCaseAttachmentFileReadRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::FileId,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub storage_version: Option<String>,
    pub checksum_etag: Option<String>,
    pub checksum_sha256_base64: Option<String>,
    pub checksum_sha256_hex: Option<String>,
    pub last_modified_at: Option<jiff::Timestamp>,
    pub version: Option<String>,
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    pub size_readable: String,
}
/// File attached to a support case (private; fetched via presigned URL).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SupportCaseAttachmentFileRead {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::FileId,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub storage_version: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_etag: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_base64: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_hex: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_modified_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub version: Option<String>,
    pub service: monostate::MustBe!("support_case_attachment"),
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    pub size_readable: String,
}
#[allow(clippy::type_complexity)]
impl SupportCaseAttachmentFileRead {
    pub fn new(required: SupportCaseAttachmentFileReadRequired) -> Self {
        Self {
            id: required.id,
            organization_id: required.organization_id,
            name: required.name,
            path: required.path,
            mime_type: required.mime_type,
            size: required.size,
            storage_version: required.storage_version,
            checksum_etag: required.checksum_etag,
            checksum_sha256_base64: required.checksum_sha256_base64,
            checksum_sha256_hex: required.checksum_sha256_hex,
            last_modified_at: required.last_modified_at,
            version: required.version,
            service: Default::default(),
            is_uploaded: required.is_uploaded,
            created_at: required.created_at,
            size_readable: required.size_readable,
        }
    }
}
