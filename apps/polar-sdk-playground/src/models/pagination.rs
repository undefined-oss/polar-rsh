use std::num::NonZeroU64;

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
