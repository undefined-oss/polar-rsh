use derive_more::{Debug, Deref, Display, Eq, From, PartialEq};
use serde::{Deserialize, Serialize};
/// ID of the transactions account.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct AccountId(uuid::Uuid);
impl ::std::default::Default for AccountId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ActivationId(uuid::Uuid);
impl ::std::default::Default for ActivationId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct AuthorizeUserId(uuid::Uuid);
impl ::std::default::Default for AuthorizeUserId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct BenefitDownloadablesCreatePropertiesArchivedId(uuid::Uuid);
impl ::std::default::Default for BenefitDownloadablesCreatePropertiesArchivedId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct BenefitDownloadablesPropertiesArchivedId(uuid::Uuid);
impl ::std::default::Default for BenefitDownloadablesPropertiesArchivedId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct BenefitDownloadablesSubscriberPropertiesActiveFilesId(uuid::Uuid);
impl ::std::default::Default for BenefitDownloadablesSubscriberPropertiesActiveFilesId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the grant.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct BenefitGrantId(uuid::Uuid);
impl ::std::default::Default for BenefitGrantId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the grant.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct BenefitGrantWebhookId(uuid::Uuid);
impl ::std::default::Default for BenefitGrantWebhookId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the benefit.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct BenefitId(uuid::Uuid);
impl ::std::default::Default for BenefitId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the support case for this dispute, if one was opened.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CaseId(uuid::Uuid);
impl ::std::default::Default for CaseId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the checkout session associated with this payment.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CheckoutId(uuid::Uuid);
impl ::std::default::Default for CheckoutId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The checkout link ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CheckoutLinkId(uuid::Uuid);
impl ::std::default::Default for CheckoutLinkId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// Mapping of product IDs to their list of prices.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CheckoutPricesId(uuid::Uuid);
impl ::std::default::Default for CheckoutPricesId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// Optional mapping of product IDs to a list of ad-hoc prices to create for
/// that product. If not set, catalog prices of the product will be used.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CheckoutProductsCreatePricesId(uuid::Uuid);
impl ::std::default::Default for CheckoutProductsCreatePricesId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CheckoutPublicConfirmedId(uuid::Uuid);
impl ::std::default::Default for CheckoutPublicConfirmedId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// Mapping of product IDs to their list of prices.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CheckoutPublicConfirmedPricesId(uuid::Uuid);
impl ::std::default::Default for CheckoutPublicConfirmedPricesId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ClientSecretId(uuid::Uuid);
impl ::std::default::Default for ClientSecretId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// Mapping of product IDs to their list of prices.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ClientSecretPricesId(uuid::Uuid);
impl ::std::default::Default for ClientSecretPricesId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// ID of the custom field.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CustomFieldId(uuid::Uuid);
impl ::std::default::Default for CustomFieldId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// ID of the customer in your Polar organization associated with the event.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CustomerId(uuid::Uuid);
impl ::std::default::Default for CustomerId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The customer meter ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CustomerMeterId(uuid::Uuid);
impl ::std::default::Default for CustomerMeterId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CustomerSessionId(uuid::Uuid);
impl ::std::default::Default for CustomerSessionId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the grant.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CustomerStateBenefitGrantId(uuid::Uuid);
impl ::std::default::Default for CustomerStateBenefitGrantId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct CustomerStateMeterId(uuid::Uuid);
impl ::std::default::Default for CustomerStateMeterId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The metric dashboard ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct DashboardId(uuid::Uuid);
impl ::std::default::Default for DashboardId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the customer's default payment method, if any. Use the payment
/// methods endpoint to retrieve its details.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct DefaultPaymentMethodId(uuid::Uuid);
impl ::std::default::Default for DefaultPaymentMethodId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct DeliveryId(uuid::Uuid);
impl ::std::default::Default for DeliveryId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// ID of the discount applied to the checkout.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct DiscountId(uuid::Uuid);
impl ::std::default::Default for DiscountId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The dispute ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct DisputeId(uuid::Uuid);
impl ::std::default::Default for DisputeId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct DownloadableId(uuid::Uuid);
impl ::std::default::Default for DownloadableId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// Filter by webhook endpoint ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct EndpointId(uuid::Uuid);
impl ::std::default::Default for EndpointId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the parent event.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct EventId(uuid::Uuid);
impl ::std::default::Default for EventId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the event type. Null for system event types.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct EventTypeId(uuid::Uuid);
impl ::std::default::Default for EventTypeId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// Filter by specific parent event ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct EventTypesListId(uuid::Uuid);
impl ::std::default::Default for EventTypesListId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The event ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct EventsListId(uuid::Uuid);
impl ::std::default::Default for EventsListId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The benefit ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ExcludeId(uuid::Uuid);
impl ::std::default::Default for ExcludeId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ExistingProductPriceId(uuid::Uuid);
impl ::std::default::Default for ExistingProductPriceId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// Filter by file ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct FileId(uuid::Uuid);
impl ::std::default::Default for FileId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct FileUploadId(uuid::Uuid);
impl ::std::default::Default for FileUploadId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct LicenseKeyActivationCreatedId(uuid::Uuid);
impl ::std::default::Default for LicenseKeyActivationCreatedId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct LicenseKeyId(uuid::Uuid);
impl ::std::default::Default for LicenseKeyId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct LicenseKeyUserId(uuid::Uuid);
impl ::std::default::Default for LicenseKeyUserId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// ID of the member within the customer's organization who performed the action
/// inside B2B.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct MemberId(uuid::Uuid);
impl ::std::default::Default for MemberId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the meter.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct MeterId(uuid::Uuid);
impl ::std::default::Default for MeterId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the order that granted this benefit.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct OrderId(uuid::Uuid);
impl ::std::default::Default for OrderId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct OrderItemSchemaId(uuid::Uuid);
impl ::std::default::Default for OrderItemSchemaId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct OrderUserId(uuid::Uuid);
impl ::std::default::Default for OrderUserId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the organization owning the event.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct OrganizationId(uuid::Uuid);
impl ::std::default::Default for OrganizationId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the payment associated with the dispute.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct PaymentId(uuid::Uuid);
impl ::std::default::Default for PaymentId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// ID of an existing saved payment method.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct PaymentMethodId(uuid::Uuid);
impl ::std::default::Default for PaymentMethodId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// ID of the payout account.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct PayoutAccountId(uuid::Uuid);
impl ::std::default::Default for PayoutAccountId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct PendingSubscriptionUpdateId(uuid::Uuid);
impl ::std::default::Default for PendingSubscriptionUpdateId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// List of file IDs. Each one must be on the same organization as the product,
/// of type `product_media` and correctly uploaded.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ProductCreateMediasId(uuid::Uuid);
impl ::std::default::Default for ProductCreateMediasId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// ID of the product to checkout.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ProductId(uuid::Uuid);
impl ::std::default::Default for ProductId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// ID of the product price to checkout.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ProductPriceId(uuid::Uuid);
impl ::std::default::Default for ProductPriceId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// List of file IDs. Each one must be on the same organization as the product,
/// of type `product_media` and correctly uploaded.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ProductUpdateMediasId(uuid::Uuid);
impl ::std::default::Default for ProductUpdateMediasId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The refund ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct RefundId(uuid::Uuid);
impl ::std::default::Default for RefundId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct RotatedLicenseKeyId(uuid::Uuid);
impl ::std::default::Default for RotatedLicenseKeyId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The seat ID
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct SeatId(uuid::Uuid);
impl ::std::default::Default for SeatId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// Polar Slack integration to use for this benefit.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct SlackIntegrationId(uuid::Uuid);
impl ::std::default::Default for SlackIntegrationId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the subscription that granted this benefit.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct SubscriptionId(uuid::Uuid);
impl ::std::default::Default for SubscriptionId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct SubscriptionUserId(uuid::Uuid);
impl ::std::default::Default for SubscriptionUserId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The ID of the object.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct ValidatedLicenseKeyId(uuid::Uuid);
impl ::std::default::Default for ValidatedLicenseKeyId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
/// The wallet ID.
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct WalletId(uuid::Uuid);
impl ::std::default::Default for WalletId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct WebTokenRequestSubId(uuid::Uuid);
impl ::std::default::Default for WebTokenRequestSubId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
