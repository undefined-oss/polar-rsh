use std::num::NonZeroU64;

use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    pub max_page: u64,
    pub total_count: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListResource<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

impl Pagination {
    pub fn next_page(&self, current: NonZeroU64) -> Option<NonZeroU64> {
        let max_page = u64::try_from(self.max_page).ok()?;

        if current.get() >= max_page {
            return None;
        }

        current.checked_add(1)
    }
}

/// Number of items per page, between 1 and 100 inclusive.
#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, Deref, Deserialize, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct PageLimit(u8);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("page limit must be between 1 and 100 inclusive, got {0}")]
pub struct PageLimitValidationError(pub u8);

impl TryFrom<u8> for PageLimit {
    type Error = PageLimitValidationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 1 || value > 100 {
            return Err(PageLimitValidationError(value));
        }

        Ok(Self(value))
    }
}

impl From<PageLimit> for u8 {
    fn from(value: PageLimit) -> Self {
        value.0
    }
}

impl Default for PageLimit {
    fn default() -> Self {
        Self(10)
    }
}

#[cfg(test)]
mod page_limit_tests {
    use super::{PageLimit, PageLimitValidationError};

    #[test]
    fn accepts_only_limits_in_the_supported_range() {
        for value in u8::MIN..=u8::MAX {
            let result = PageLimit::try_from(value);
            if (1..=100).contains(&value) {
                let limit = result.unwrap();
                assert_eq!(*limit, value);
                assert_eq!(u8::from(limit), value);
                assert_eq!(limit.to_string(), value.to_string());
            } else {
                assert_eq!(result, Err(PageLimitValidationError(value)));
            }
        }
        assert_eq!(*PageLimit::default(), 10);
    }

    #[test]
    fn serde_validates_limits_and_preserves_numeric_encoding() {
        for value in [1, 10, 100] {
            let json = serde_json::json!(value);
            let limit: PageLimit = serde_json::from_value(json.clone()).unwrap();
            assert_eq!(*limit, value);
            assert_eq!(serde_json::to_value(limit).unwrap(), json);
        }
        for json in ["0", "101", "255", "256", "-1", "1.5", "null", "\"10\""] {
            assert!(serde_json::from_str::<PageLimit>(json).is_err(), "accepted {json}");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use super::Pagination;

    #[test]
    fn advances_until_the_last_page() {
        for (current, max_page, expected) in
            [(1, 3, Some(2)), (2, 3, Some(3)), (3, 3, None), (4, 3, None), (1, 0, None)]
        {
            let pagination = Pagination { max_page, total_count: 30 };
            let next = pagination.next_page(NonZeroU64::new(current).unwrap());

            assert_eq!(
                next.map(NonZeroU64::get),
                expected,
                "unexpected next page for current={current}, max_page={max_page}"
            );
        }
    }

    #[test]
    fn total_count_does_not_affect_page_advancement() {
        for total_count in [0, 1, 30, u64::MAX] {
            let pagination = Pagination { max_page: 3, total_count };

            assert_eq!(pagination.next_page(NonZeroU64::new(1).unwrap()), NonZeroU64::new(2));
            assert_eq!(pagination.next_page(NonZeroU64::new(3).unwrap()), None);
        }
    }

    #[test]
    fn handles_large_page_numbers_without_overflow() {
        let pagination = Pagination { max_page: u64::MAX, total_count: 0 };
        let last_page = u64::MAX;

        assert_eq!(
            pagination.next_page(NonZeroU64::new(last_page - 1).unwrap()),
            NonZeroU64::new(last_page)
        );
        assert_eq!(pagination.next_page(NonZeroU64::new(last_page).unwrap()), None);
        assert_eq!(pagination.next_page(NonZeroU64::new(u64::MAX).unwrap()), None);
    }
}
