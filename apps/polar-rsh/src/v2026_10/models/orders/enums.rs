use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerOrderSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "net_amount")]
    NetAmount,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "subscription")]
    Subscription,
}
impl CustomerOrderSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Amount => "amount",
            Self::NetAmount => "net_amount",
            Self::Product => "product",
            Self::Subscription => "subscription",
        }
    }
}
impl ::std::fmt::Display for CustomerOrderSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerOrderSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrderBillingReason {
    #[serde(rename = "purchase")]
    Purchase,
    #[serde(rename = "subscription_create")]
    SubscriptionCreate,
    #[serde(rename = "subscription_cycle")]
    SubscriptionCycle,
    #[serde(rename = "subscription_update")]
    SubscriptionUpdate,
    #[serde(rename = "subscription_meter_cycle")]
    SubscriptionMeterCycle,
}
impl OrderBillingReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Purchase => "purchase",
            Self::SubscriptionCreate => "subscription_create",
            Self::SubscriptionCycle => "subscription_cycle",
            Self::SubscriptionUpdate => "subscription_update",
            Self::SubscriptionMeterCycle => "subscription_meter_cycle",
        }
    }
}
impl ::std::fmt::Display for OrderBillingReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrderBillingReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrderExportColumn {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "net_amount")]
    NetAmount,
    #[serde(rename = "currency")]
    Currency,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "invoice_number")]
    InvoiceNumber,
    #[serde(rename = "customer_name")]
    CustomerName,
    #[serde(rename = "billing_name")]
    BillingName,
    #[serde(rename = "billing_country")]
    BillingCountry,
    #[serde(rename = "subtotal_amount")]
    SubtotalAmount,
    #[serde(rename = "discount_amount")]
    DiscountAmount,
    #[serde(rename = "tax_amount")]
    TaxAmount,
    #[serde(rename = "total_amount")]
    TotalAmount,
    #[serde(rename = "refunded_amount")]
    RefundedAmount,
    #[serde(rename = "billing_reason")]
    BillingReason,
}
impl OrderExportColumn {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::CreatedAt => "created_at",
            Self::Product => "product",
            Self::NetAmount => "net_amount",
            Self::Currency => "currency",
            Self::Status => "status",
            Self::InvoiceNumber => "invoice_number",
            Self::CustomerName => "customer_name",
            Self::BillingName => "billing_name",
            Self::BillingCountry => "billing_country",
            Self::SubtotalAmount => "subtotal_amount",
            Self::DiscountAmount => "discount_amount",
            Self::TaxAmount => "tax_amount",
            Self::TotalAmount => "total_amount",
            Self::RefundedAmount => "refunded_amount",
            Self::BillingReason => "billing_reason",
        }
    }
}
impl ::std::fmt::Display for OrderExportColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrderExportColumn {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrderSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "invoice_number")]
    InvoiceNumber,
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "net_amount")]
    NetAmount,
    #[serde(rename = "customer")]
    Customer,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "discount")]
    Discount,
    #[serde(rename = "subscription")]
    Subscription,
}
impl OrderSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Status => "status",
            Self::InvoiceNumber => "invoice_number",
            Self::Amount => "amount",
            Self::NetAmount => "net_amount",
            Self::Customer => "customer",
            Self::Product => "product",
            Self::Discount => "discount",
            Self::Subscription => "subscription",
        }
    }
}
impl ::std::fmt::Display for OrderSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrderSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrderStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "refunded")]
    Refunded,
    #[serde(rename = "partially_refunded")]
    PartiallyRefunded,
    #[serde(rename = "void")]
    Void,
}
impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Pending => "pending",
            Self::Paid => "paid",
            Self::Refunded => "refunded",
            Self::PartiallyRefunded => "partially_refunded",
            Self::Void => "void",
        }
    }
}
impl ::std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
