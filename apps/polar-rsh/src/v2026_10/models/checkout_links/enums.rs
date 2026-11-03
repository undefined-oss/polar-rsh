use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CheckoutLinkSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "success_url")]
    SuccessUrl,
    #[serde(rename = "allow_discount_codes")]
    AllowDiscountCodes,
}
impl CheckoutLinkSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Label => "label",
            Self::SuccessUrl => "success_url",
            Self::AllowDiscountCodes => "allow_discount_codes",
        }
    }
}
impl ::std::fmt::Display for CheckoutLinkSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CheckoutLinkSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
