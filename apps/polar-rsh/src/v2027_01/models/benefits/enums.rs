use serde::{Deserialize, Serialize};
/// The permission level to grant. Read more about roles and their permissions on [GitHub documentation](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization#permissions-for-each-role).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BenefitGitHubRepositoryCreatePropertiesPermission {
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "maintain")]
    Maintain,
    #[serde(rename = "admin")]
    Admin,
}
impl BenefitGitHubRepositoryCreatePropertiesPermission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pull => "pull",
            Self::Triage => "triage",
            Self::Push => "push",
            Self::Maintain => "maintain",
            Self::Admin => "admin",
        }
    }
}
impl ::std::fmt::Display for BenefitGitHubRepositoryCreatePropertiesPermission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BenefitGitHubRepositoryCreatePropertiesPermission {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BenefitSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "user_order")]
    UserOrder,
}
impl BenefitSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Description => "description",
            Self::Type => "type",
            Self::UserOrder => "user_order",
        }
    }
}
impl ::std::fmt::Display for BenefitSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BenefitSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
