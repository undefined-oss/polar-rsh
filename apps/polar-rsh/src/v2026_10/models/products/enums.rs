use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum ProductSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "price_amount_type")]
    PriceAmountType,
    #[serde(rename = "price_amount")]
    PriceAmount,
}
impl ProductSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Name => "name",
            Self::PriceAmountType => "price_amount_type",
            Self::PriceAmount => "price_amount",
        }
    }
}
impl ::std::fmt::Display for ProductSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for ProductSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
