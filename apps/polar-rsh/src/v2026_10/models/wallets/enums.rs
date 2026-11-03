use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerWalletSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "balance")]
    Balance,
}
impl CustomerWalletSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Balance => "balance",
        }
    }
}
impl ::std::fmt::Display for CustomerWalletSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerWalletSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
