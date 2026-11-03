use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum DiscountSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "redemptions_count")]
    RedemptionsCount,
    #[serde(rename = "ends_at")]
    EndsAt,
}
impl DiscountSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Name => "name",
            Self::Code => "code",
            Self::RedemptionsCount => "redemptions_count",
            Self::EndsAt => "ends_at",
        }
    }
}
impl ::std::fmt::Display for DiscountSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for DiscountSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
