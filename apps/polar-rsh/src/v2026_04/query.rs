/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sorting {
    Asc,
    Desc,
}
impl Sorting {
    /// Prefix the field with `-` for descending order.
    pub fn encode(self, field: &str) -> String {
        match self {
            Self::Asc => field.to_owned(),
            Self::Desc => format!("-{field}"),
        }
    }
}
/// Encode operation parameters as an application/x-www-form-urlencoded query.
///
/// Implementations supply unescaped values through the hooks below. The encoder
/// preserves their order and repeated keys, emitting scalar parameters first,
/// then sorting values, then metadata. Default hooks omit their parameters.
pub trait EncodeQuery {
    /// Return scalar parameter names and their unescaped string values.
    /// Empty strings are included; omit a pair to leave a parameter absent.
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
    /// Return sorting values in priority order, optionally using
    /// [`Sorting::encode`]. Each value becomes a separate `sorting` query
    /// parameter.
    fn sorting_values(&self) -> Vec<String> {
        Vec::new()
    }
    /// Return metadata keys and their unescaped values.
    /// Each value produces a `metadata[key]` parameter. Empty vectors are
    /// omitted; empty string values are preserved.
    fn metadata_params(&self) -> Vec<(String, Vec<String>)> {
        Vec::new()
    }
    /// Encode all parameters without a leading `?`, escaping keys and values
    /// once.
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
