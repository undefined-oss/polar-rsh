//! Typed query parameters shared by ordinary and paginated operations.

pub struct QueryConfig {
    pub params_name: String,
    pub fields: Vec<QueryField>,
    pub sort_type: Option<String>,
    pub sorting_description: Option<String>,
}

impl QueryConfig {
    pub fn required(&self) -> bool {
        self.fields.iter().any(|field| field.required)
    }
}

pub struct QueryField {
    pub rust_name: String,
    pub wire_name: String,
    pub rust_type: String,
    pub description: Option<String>,
    pub encoding: QueryEncoding,
    pub deprecated: bool,
    pub required: bool,
}

pub enum QueryEncoding {
    Scalar,
    Repeated,
    Metadata { value_type: String, variants: Vec<MetadataVariant> },
}

pub struct MetadataVariant {
    pub name: String,
    pub repeated: bool,
}
