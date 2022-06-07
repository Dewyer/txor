#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct ClientId(u16);

impl ClientId {
    pub fn new(inner_id: u16) -> Self {
        Self(inner_id)
    }
}