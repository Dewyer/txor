#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClientId(u16);

impl ClientId {
    pub fn new(inner_id: u16) -> Self {
        Self(inner_id)
    }
}