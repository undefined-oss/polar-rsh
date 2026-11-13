use serde::{Deserialize, Serialize};
/// Reason for the refund.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RefundCreateReason {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "fraudulent")]
    Fraudulent,
    #[serde(rename = "customer_request")]
    CustomerRequest,
    #[serde(rename = "service_disruption")]
    ServiceDisruption,
    #[serde(rename = "satisfaction_guarantee")]
    SatisfactionGuarantee,
    #[serde(rename = "other")]
    Other,
}
impl RefundCreateReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::CustomerRequest => "customer_request",
            Self::ServiceDisruption => "service_disruption",
            Self::SatisfactionGuarantee => "satisfaction_guarantee",
            Self::Other => "other",
        }
    }
}
impl ::std::fmt::Display for RefundCreateReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for RefundCreateReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RefundReason {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "fraudulent")]
    Fraudulent,
    #[serde(rename = "customer_request")]
    CustomerRequest,
    #[serde(rename = "service_disruption")]
    ServiceDisruption,
    #[serde(rename = "satisfaction_guarantee")]
    SatisfactionGuarantee,
    #[serde(rename = "dispute_prevention")]
    DisputePrevention,
    #[serde(rename = "tax_refund")]
    TaxRefund,
    #[serde(rename = "vat_reverse_charge_correction")]
    VatReverseChargeCorrection,
    #[serde(rename = "other")]
    Other,
}
impl RefundReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::CustomerRequest => "customer_request",
            Self::ServiceDisruption => "service_disruption",
            Self::SatisfactionGuarantee => "satisfaction_guarantee",
            Self::DisputePrevention => "dispute_prevention",
            Self::TaxRefund => "tax_refund",
            Self::VatReverseChargeCorrection => "vat_reverse_charge_correction",
            Self::Other => "other",
        }
    }
}
impl ::std::fmt::Display for RefundReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for RefundReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RefundSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "amount")]
    Amount,
}
impl RefundSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Amount => "amount",
        }
    }
}
impl ::std::fmt::Display for RefundSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for RefundSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RefundStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "canceled")]
    Canceled,
}
impl RefundStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
        }
    }
}
impl ::std::fmt::Display for RefundStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for RefundStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
