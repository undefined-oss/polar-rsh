use derive_more::{Debug, Deref, Display, Eq, From, PartialEq};
use serde::{Deserialize, Serialize};

pub mod enums;
pub mod errors;

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct OrganizationId(uuid::Uuid);

impl ::std::default::Default for OrganizationId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct AccountId(uuid::Uuid);

impl ::std::default::Default for AccountId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct PayoutAccountId(uuid::Uuid);

impl ::std::default::Default for PayoutAccountId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
