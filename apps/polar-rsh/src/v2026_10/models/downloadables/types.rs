#[derive(Debug, Clone)]
pub struct DownloadableReadRequired {
    pub id: crate::v2026_10::models::common::ids::DownloadableId,
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub file: crate::v2026_10::models::downloadables::types::FileDownload,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DownloadableRead {
    pub id: crate::v2026_10::models::common::ids::DownloadableId,
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub file: crate::v2026_10::models::downloadables::types::FileDownload,
}
#[allow(clippy::type_complexity)]
impl DownloadableRead {
    pub fn new(required: DownloadableReadRequired) -> Self {
        Self { id: required.id, benefit_id: required.benefit_id, file: required.file }
    }
}
#[derive(Debug, Clone)]
pub struct FileDownloadRequired {
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
    pub download: crate::v2026_10::models::downloadables::types::S3DownloadUrl,
    pub version: Option<String>,
    pub is_uploaded: bool,
    pub service: crate::v2026_10::models::common::enums::FileServiceTypes,
    pub size_readable: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct FileDownload {
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
    pub download: crate::v2026_10::models::downloadables::types::S3DownloadUrl,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub version: Option<String>,
    pub is_uploaded: bool,
    pub service: crate::v2026_10::models::common::enums::FileServiceTypes,
    pub size_readable: String,
}
#[allow(clippy::type_complexity)]
impl FileDownload {
    pub fn new(required: FileDownloadRequired) -> Self {
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
            download: required.download,
            version: required.version,
            is_uploaded: required.is_uploaded,
            service: required.service,
            size_readable: required.size_readable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct S3DownloadUrlRequired {
    pub url: String,
    pub expires_at: jiff::Timestamp,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct S3DownloadUrl {
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub headers: Option<std::collections::BTreeMap<String, String>>,
    pub expires_at: jiff::Timestamp,
}
#[allow(clippy::type_complexity)]
impl S3DownloadUrl {
    pub fn new(required: S3DownloadUrlRequired) -> Self {
        Self { url: required.url, headers: None, expires_at: required.expires_at }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn headers(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }
}
