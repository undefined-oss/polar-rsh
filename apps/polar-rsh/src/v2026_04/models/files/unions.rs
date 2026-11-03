#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "service", deny_unknown_fields)]
pub enum FileCreate {
    #[serde(rename = "downloadable")]
    Downloadable {
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id:
            presence_rs::Presence<crate::v2026_04::models::common::ids::OrganizationId>,
        name: String,
        mime_type: String,
        size: i64,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        checksum_sha256_base64: presence_rs::Presence<String>,
        upload: crate::v2026_04::models::files::types::S3FileCreateMultipart,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        version: presence_rs::Presence<String>,
    },
    #[serde(rename = "product_media")]
    ProductMedia {
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id:
            presence_rs::Presence<crate::v2026_04::models::common::ids::OrganizationId>,
        name: String,
        /// MIME type of the file. Only images are supported for this type of
        /// file.
        mime_type: String,
        /// Size of the file. A maximum of 10 MB is allowed for this type of
        /// file.
        size: crate::v2026_04::models::files::newtypes::ProductMediaFileCreateSize,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        checksum_sha256_base64: presence_rs::Presence<String>,
        upload: crate::v2026_04::models::files::types::S3FileCreateMultipart,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        version: presence_rs::Presence<String>,
    },
    #[serde(rename = "organization_avatar")]
    OrganizationAvatar {
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id:
            presence_rs::Presence<crate::v2026_04::models::common::ids::OrganizationId>,
        name: String,
        /// MIME type of the file. Only images are supported for this type of
        /// file.
        mime_type: String,
        /// Size of the file. A maximum of 1 MB is allowed for this type of
        /// file.
        size: crate::v2026_04::models::files::newtypes::OrganizationAvatarFileCreateSize,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        checksum_sha256_base64: presence_rs::Presence<String>,
        upload: crate::v2026_04::models::files::types::S3FileCreateMultipart,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        version: presence_rs::Presence<String>,
    },
    #[serde(rename = "support_case_attachment")]
    SupportCaseAttachment {
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id:
            presence_rs::Presence<crate::v2026_04::models::common::ids::OrganizationId>,
        name: String,
        /// MIME type of the file. Images, videos, PDF, CSV, plain text, Word
        /// and Excel documents are supported.
        mime_type: String,
        /// Size of the file. A maximum of 250 MB is allowed for this type of
        /// file.
        size: crate::v2026_04::models::files::newtypes::SupportCaseAttachmentFileCreateSize,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        checksum_sha256_base64: presence_rs::Presence<String>,
        upload: crate::v2026_04::models::files::types::S3FileCreateMultipart,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        version: presence_rs::Presence<String>,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "service", deny_unknown_fields)]
pub enum FileRead {
    #[serde(rename = "downloadable")]
    Downloadable {
        /// The ID of the object.
        id: crate::v2026_04::models::common::ids::FileId,
        organization_id: crate::v2026_04::models::common::ids::OrganizationId,
        name: String,
        path: String,
        mime_type: String,
        size: i64,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        storage_version: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_etag: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_sha256_base64: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_sha256_hex: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        last_modified_at: Option<jiff::Timestamp>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        version: Option<String>,
        is_uploaded: bool,
        created_at: jiff::Timestamp,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        flagged_malicious_at: Option<jiff::Timestamp>,
        size_readable: String,
    },
    #[serde(rename = "product_media")]
    ProductMedia {
        /// The ID of the object.
        id: crate::v2026_04::models::common::ids::FileId,
        organization_id: crate::v2026_04::models::common::ids::OrganizationId,
        name: String,
        path: String,
        mime_type: String,
        size: i64,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        storage_version: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_etag: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_sha256_base64: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_sha256_hex: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        last_modified_at: Option<jiff::Timestamp>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        version: Option<String>,
        is_uploaded: bool,
        created_at: jiff::Timestamp,
        size_readable: String,
        public_url: String,
    },
    #[serde(rename = "organization_avatar")]
    OrganizationAvatar {
        /// The ID of the object.
        id: crate::v2026_04::models::common::ids::FileId,
        organization_id: crate::v2026_04::models::common::ids::OrganizationId,
        name: String,
        path: String,
        mime_type: String,
        size: i64,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        storage_version: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_etag: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_sha256_base64: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_sha256_hex: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        last_modified_at: Option<jiff::Timestamp>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        version: Option<String>,
        is_uploaded: bool,
        created_at: jiff::Timestamp,
        size_readable: String,
        public_url: String,
    },
    #[serde(rename = "support_case_attachment")]
    SupportCaseAttachment {
        /// The ID of the object.
        id: crate::v2026_04::models::common::ids::FileId,
        organization_id: crate::v2026_04::models::common::ids::OrganizationId,
        name: String,
        path: String,
        mime_type: String,
        size: i64,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        storage_version: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_etag: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_sha256_base64: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        checksum_sha256_hex: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        last_modified_at: Option<jiff::Timestamp>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        version: Option<String>,
        is_uploaded: bool,
        created_at: jiff::Timestamp,
        size_readable: String,
    },
}
