#[derive(Debug, Clone)]
pub struct CustomerWalletRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::WalletId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer that owns the wallet.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The current balance of the wallet, in cents.
    pub balance: i64,
    /// The currency of the wallet.
    pub currency: String,
}
/// A wallet represents your balance with an organization.
///
/// You can top-up your wallet and use the balance to pay for usage.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerWallet {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::WalletId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer that owns the wallet.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The current balance of the wallet, in cents.
    pub balance: i64,
    /// The currency of the wallet.
    pub currency: String,
}
#[allow(clippy::type_complexity)]
impl CustomerWallet {
    pub fn new(required: CustomerWalletRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            customer_id: required.customer_id,
            balance: required.balance,
            currency: required.currency,
        }
    }
}
