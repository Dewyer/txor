use crate::models::money_cents::MoneyCents;
use crate::models::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ClientAccount {
    id: ClientId,

    available: MoneyCents,
    held: MoneyCents,

    locked: bool,
}

impl ClientAccount {
    pub fn new(id: ClientId) -> Self {
        Self {
            id,
            available: 0,
            held: 0,
            locked: false,
        }
    }

    pub fn get_total(&self) -> MoneyCents {
        self.available + self.held
    }

    pub fn get_available(&self) -> MoneyCents {
        self.available
    }

    pub fn get_held(&self) -> MoneyCents {
        self.held
    }

    pub fn get_id(&self) -> ClientId {
        self.id
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn add_available(&mut self, amount: MoneyCents) {
        self.available += amount;
    }

    pub fn remove_available(&mut self, amount: MoneyCents) {
        self.add_available(-amount);
    }

    pub fn hold(&mut self, amount: MoneyCents) {
        self.available -= amount;
        self.held += amount;
    }

    pub fn un_hold(&mut self, amount: MoneyCents) {
        self.hold(-amount);
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }
}
