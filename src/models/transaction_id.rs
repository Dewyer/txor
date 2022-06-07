#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct TransactionId(u32);

impl TransactionId {
    pub fn new(inner_id: u32) -> Self {
        Self(inner_id)
    }
}