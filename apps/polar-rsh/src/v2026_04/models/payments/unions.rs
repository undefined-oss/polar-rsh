#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Payment {
    CardPayment(crate::v2026_04::models::payments::types::CardPayment),
    KrCardPayment(crate::v2026_04::models::payments::types::KrCardPayment),
    GenericPayment(crate::v2026_04::models::payments::types::GenericPayment),
}
