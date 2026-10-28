use derive_more::{Debug, Deref, Display, Eq, From, PartialEq};
use serde::{Deserialize, Serialize};

mod country_alpha2_input;
mod presentment_currency;

pub use country_alpha2_input::CountryAlpha2Input;
pub use presentment_currency::PresentmentCurrency;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TaxBehaviorOption {
    #[default]
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "inclusive")]
    Inclusive,
    #[serde(rename = "exclusive")]
    Exclusive,
}

impl TaxBehaviorOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Location => "location",
            Self::Inclusive => "inclusive",
            Self::Exclusive => "exclusive",
        }
    }
}
impl ::std::fmt::Display for TaxBehaviorOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for TaxBehaviorOption {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum SubscriptionProrationBehavior {
    #[default]
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "prorate")]
    Prorate,
    #[serde(rename = "next_period")]
    NextPeriod,
    #[serde(rename = "reset")]
    Reset,
}
impl SubscriptionProrationBehavior {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Prorate => "prorate",
            Self::NextPeriod => "next_period",
            Self::Reset => "reset",
        }
    }
}
impl ::std::fmt::Display for SubscriptionProrationBehavior {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for SubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
