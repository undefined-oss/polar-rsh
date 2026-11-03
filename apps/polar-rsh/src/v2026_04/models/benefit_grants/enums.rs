use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BenefitGrantSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "granted_at")]
    GrantedAt,
    #[serde(rename = "revoked_at")]
    RevokedAt,
}
impl BenefitGrantSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::GrantedAt => "granted_at",
            Self::RevokedAt => "revoked_at",
        }
    }
}
impl ::std::fmt::Display for BenefitGrantSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BenefitGrantSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerBenefitGrantSortProperty {
    #[serde(rename = "granted_at")]
    GrantedAt,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "product_benefit")]
    ProductBenefit,
}
impl CustomerBenefitGrantSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GrantedAt => "granted_at",
            Self::Type => "type",
            Self::Organization => "organization",
            Self::ProductBenefit => "product_benefit",
        }
    }
}
impl ::std::fmt::Display for CustomerBenefitGrantSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerBenefitGrantSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
