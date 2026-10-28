#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sorting {
    Asc,
    Desc,
}

impl Sorting {
    pub fn encode(self, field: &str) -> String {
        match self {
            Self::Asc => field.to_owned(),
            Self::Desc => format!("-{field}"),
        }
    }
}

pub trait EncodeQuery {
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn sorting_values(&self) -> Vec<String> {
        Vec::new()
    }

    fn metadata_params(&self) -> Vec<(String, Vec<String>)> {
        Vec::new()
    }

    fn encode_query(&self) -> String {
        let mut encoder = form_urlencoded::Serializer::new(String::new());

        for (key, value) in self.scalar_params() {
            encoder.append_pair(key, &value);
        }

        for value in self.sorting_values() {
            encoder.append_pair("sorting", &value);
        }

        for (key, values) in self.metadata_params() {
            let parameter = format!("metadata[{key}]");

            for value in values {
                encoder.append_pair(&parameter, &value);
            }
        }

        encoder.finish()
    }
}

#[cfg(test)]
mod query_test {
    use crate::query::{EncodeQuery, Sorting};

    struct ExampleQuery;

    impl EncodeQuery for ExampleQuery {
        fn scalar_params(&self) -> Vec<(&'static str, String)> {
            vec![("slug", "acme & sons".into())]
        }

        fn sorting_values(&self) -> Vec<String> {
            vec![Sorting::Desc.encode("name"), Sorting::Asc.encode("created_at")]
        }
    }

    #[test]
    fn encodes_scalars_and_preserves_sorting_priority() {
        assert_eq!(
            ExampleQuery.encode_query(),
            "slug=acme+%26+sons&sorting=-name&sorting=created_at"
        );
    }

    #[test]
    fn default_hooks_produce_an_empty_query() {
        struct EmptyQuery;
        impl EncodeQuery for EmptyQuery {}

        assert_eq!(EmptyQuery.encode_query(), "");
    }

    #[test]
    fn escapes_scalar_values_once_and_omits_empty_sorting() {
        struct ScalarQuery;

        impl EncodeQuery for ScalarQuery {
            fn scalar_params(&self) -> Vec<(&'static str, String)> {
                vec![("slug", "a+b & 50%".into())]
            }
        }

        assert_eq!(ScalarQuery.encode_query(), "slug=a%2Bb+%26+50%25");
    }

    struct MetadataQuery(Vec<(String, Vec<String>)>);

    impl EncodeQuery for MetadataQuery {
        fn metadata_params(&self) -> Vec<(String, Vec<String>)> {
            self.0.clone()
        }
    }

    #[test]
    fn encodes_metadata_scalars_and_preserves_repeated_value_order() {
        let query = MetadataQuery(vec![
            ("plan".into(), vec!["pro".into(), "enterprise".into()]),
            ("active".into(), vec![true.to_string()]),
            ("attempts".into(), vec![3.to_string()]),
            ("unused".into(), vec![]),
        ]);

        assert_eq!(
            query.encode_query(),
            "metadata%5Bplan%5D=pro&metadata%5Bplan%5D=enterprise&metadata%5Bactive%5D=true&\
             metadata%5Battempts%5D=3"
        );
    }

    #[test]
    fn omits_empty_metadata_and_empty_value_vectors() {
        assert_eq!(MetadataQuery(vec![]).encode_query(), "");
        assert_eq!(MetadataQuery(vec![("unused".into(), vec![])]).encode_query(), "");
    }

    #[test]
    fn preserves_empty_metadata_string_values() {
        let query = MetadataQuery(vec![("key".into(), vec![String::new()])]);

        assert_eq!(query.encode_query(), "metadata%5Bkey%5D=");
    }

    #[test]
    fn escapes_metadata_keys_and_values_once() {
        let query = MetadataQuery(vec![("a+b & 50%".into(), vec!["c+d & 25%".into()])]);

        assert_eq!(query.encode_query(), "metadata%5Ba%2Bb+%26+50%25%5D=c%2Bd+%26+25%25");
    }

    #[test]
    fn combines_scalars_sorting_and_metadata() {
        struct CombinedQuery;

        impl EncodeQuery for CombinedQuery {
            fn scalar_params(&self) -> Vec<(&'static str, String)> {
                ExampleQuery.scalar_params()
            }

            fn sorting_values(&self) -> Vec<String> {
                ExampleQuery.sorting_values()
            }

            fn metadata_params(&self) -> Vec<(String, Vec<String>)> {
                vec![("plan".into(), vec!["pro".into(), "enterprise".into()])]
            }
        }

        assert_eq!(
            CombinedQuery.encode_query(),
            "slug=acme+%26+sons&sorting=-name&sorting=created_at&metadata%5Bplan%5D=pro&metadata%\
             5Bplan%5D=enterprise"
        );
    }
}
