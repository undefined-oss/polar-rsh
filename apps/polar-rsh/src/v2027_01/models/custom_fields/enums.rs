use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomFieldSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "slug")]
    Slug,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "type")]
    Type,
}
impl CustomFieldSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Slug => "slug",
            Self::Name => "name",
            Self::Type => "type",
        }
    }
}
impl ::std::fmt::Display for CustomFieldSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomFieldSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomFieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "select")]
    Select,
}
impl CustomFieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Date => "date",
            Self::Checkbox => "checkbox",
            Self::Select => "select",
        }
    }
}
impl ::std::fmt::Display for CustomFieldType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomFieldType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
