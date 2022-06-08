use crate::models::ClientAccount;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ProcessingOutput {
    pub clients: Vec<ClientAccount>,
}