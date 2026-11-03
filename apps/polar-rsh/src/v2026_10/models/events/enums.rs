use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum EventNamesSortProperty {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "occurrences")]
    Occurrences,
    #[serde(rename = "first_seen")]
    FirstSeen,
    #[serde(rename = "last_seen")]
    LastSeen,
}
impl EventNamesSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Occurrences => "occurrences",
            Self::FirstSeen => "first_seen",
            Self::LastSeen => "last_seen",
        }
    }
}
impl ::std::fmt::Display for EventNamesSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for EventNamesSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum EventSortProperty {
    #[serde(rename = "timestamp")]
    Timestamp,
}
impl EventSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Timestamp => "timestamp",
        }
    }
}
impl ::std::fmt::Display for EventSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for EventSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
