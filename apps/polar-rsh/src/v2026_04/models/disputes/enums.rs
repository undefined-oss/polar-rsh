use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum DisputeSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "amount")]
    Amount,
}
impl DisputeSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Amount => "amount",
        }
    }
}
impl ::std::fmt::Display for DisputeSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for DisputeSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
