use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum EventTypesSortProperty {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "occurrences")]
    Occurrences,
    #[serde(rename = "first_seen")]
    FirstSeen,
    #[serde(rename = "last_seen")]
    LastSeen,
}
impl EventTypesSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Label => "label",
            Self::Occurrences => "occurrences",
            Self::FirstSeen => "first_seen",
            Self::LastSeen => "last_seen",
        }
    }
}
impl ::std::fmt::Display for EventTypesSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for EventTypesSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
