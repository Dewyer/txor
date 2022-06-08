use std::fmt::{Display, Formatter};

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone,
)]
pub struct TransactionId(u32);

impl TransactionId {
    pub fn new(inner_id: u32) -> Self {
        Self(inner_id)
    }
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
