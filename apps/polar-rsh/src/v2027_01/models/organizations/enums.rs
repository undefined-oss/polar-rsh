use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrganizationDetailsInputSwitchingFrom {
    #[serde(rename = "paddle")]
    Paddle,
    #[serde(rename = "lemon_squeezy")]
    LemonSqueezy,
    #[serde(rename = "gumroad")]
    Gumroad,
    #[serde(rename = "stripe")]
    Stripe,
    #[serde(rename = "other")]
    Other,
}
impl OrganizationDetailsInputSwitchingFrom {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Paddle => "paddle",
            Self::LemonSqueezy => "lemon_squeezy",
            Self::Gumroad => "gumroad",
            Self::Stripe => "stripe",
            Self::Other => "other",
        }
    }
}
impl ::std::fmt::Display for OrganizationDetailsInputSwitchingFrom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrganizationDetailsInputSwitchingFrom {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrganizationSocialPlatforms {
    #[serde(rename = "x")]
    X,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "youtube")]
    Youtube,
    #[serde(rename = "tiktok")]
    Tiktok,
    #[serde(rename = "linkedin")]
    Linkedin,
    #[serde(rename = "threads")]
    Threads,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "other")]
    Other,
}
impl OrganizationSocialPlatforms {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Github => "github",
            Self::Facebook => "facebook",
            Self::Instagram => "instagram",
            Self::Youtube => "youtube",
            Self::Tiktok => "tiktok",
            Self::Linkedin => "linkedin",
            Self::Threads => "threads",
            Self::Discord => "discord",
            Self::Other => "other",
        }
    }
}
impl ::std::fmt::Display for OrganizationSocialPlatforms {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrganizationSocialPlatforms {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrganizationSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "slug")]
    Slug,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "next_review_threshold")]
    NextReviewThreshold,
    #[serde(rename = "days_in_status")]
    DaysInStatus,
}
impl OrganizationSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Slug => "slug",
            Self::Name => "name",
            Self::NextReviewThreshold => "next_review_threshold",
            Self::DaysInStatus => "days_in_status",
        }
    }
}
impl ::std::fmt::Display for OrganizationSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrganizationSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrganizationStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "snoozed")]
    Snoozed,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "offboarding")]
    Offboarding,
    #[serde(rename = "offboarded")]
    Offboarded,
}
impl OrganizationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Review => "review",
            Self::Snoozed => "snoozed",
            Self::Denied => "denied",
            Self::Active => "active",
            Self::Blocked => "blocked",
            Self::Offboarding => "offboarding",
            Self::Offboarded => "offboarded",
        }
    }
}
impl ::std::fmt::Display for OrganizationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrganizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrganizationSubscriptionSettingsProrationBehavior {
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "prorate")]
    Prorate,
    #[serde(rename = "next_period")]
    NextPeriod,
}
impl OrganizationSubscriptionSettingsProrationBehavior {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Prorate => "prorate",
            Self::NextPeriod => "next_period",
        }
    }
}
impl ::std::fmt::Display for OrganizationSubscriptionSettingsProrationBehavior {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrganizationSubscriptionSettingsProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
