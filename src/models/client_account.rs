use crate::models::money_cents::MoneyCents;
use crate::models::ClientId;
use serde::{Deserialize, Serialize};
use crate::errors::ProcessorError;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ClientAccount {
    id: ClientId,

    available: MoneyCents,
    held: MoneyCents,

    locked: bool,
}

fn checked_add_money(lhs: MoneyCents, rhs: MoneyCents) -> Result<MoneyCents, ProcessorError> {
    lhs.checked_add(rhs)
        .ok_or(ProcessorError::ArithmeticOverflow)
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

    pub fn add_available(&mut self, amount: MoneyCents) -> Result<(), ProcessorError> {
        self.available = checked_add_money(self.available, amount)?;

        Ok(())
    }

    pub fn remove_available(&mut self, amount: MoneyCents) -> Result<(), ProcessorError> {
        self.add_available(-amount)
    }

    pub fn hold(&mut self, amount: MoneyCents) -> Result<(), ProcessorError> {
        self.available = checked_add_money(self.available, -amount)?;
        self.held = checked_add_money(self.held, amount)?;

        Ok(())
    }

    pub fn un_hold(&mut self, amount: MoneyCents) -> Result<(), ProcessorError> {
        self.hold(-amount)
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }
}
