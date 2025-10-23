use std::vec::IntoIter;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InputType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "type")]
    pub kind: InputType,
    pub client: u16,
    #[serde(rename = "tx")]
    pub transaction_id: u32,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Row {
    pub id: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: Vec<Option<u16>>,
    pub available: Vec<Decimal>,
    pub held: Vec<Decimal>,
    pub total: Vec<Decimal>,
    pub locked: Vec<bool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            id: vec![None; u16::MAX as usize],
            available: vec![Decimal::ZERO; u16::MAX as usize],
            held: vec![Decimal::ZERO; u16::MAX as usize],
            total: vec![Decimal::ZERO; u16::MAX as usize],
            locked: vec![false; u16::MAX as usize],
        }
    }
}

pub struct StateIter {
    id: IntoIter<Option<u16>>,
    available: IntoIter<Decimal>,
    held: IntoIter<Decimal>,
    total: IntoIter<Decimal>,
    locked: IntoIter<bool>,
}

impl IntoIterator for State {
    type Item = Row;
    type IntoIter = StateIter;

    fn into_iter(self) -> Self::IntoIter {
        assert_eq!(self.id.len(), self.available.len());
        assert_eq!(self.id.len(), self.held.len());
        assert_eq!(self.id.len(), self.total.len());
        assert_eq!(self.id.len(), self.locked.len());

        StateIter {
            id: self.id.into_iter(),
            available: self.available.into_iter(),
            held: self.held.into_iter(),
            total: self.total.into_iter(),
            locked: self.locked.into_iter(),
        }
    }
}

impl Iterator for StateIter {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(id) = self.id.next() {
            // Safety: We pre-construct the State directly, so we **know** those are correct.
            let available = self.available.next().unwrap();
            let held = self.held.next().unwrap();
            let total = self.total.next().unwrap();
            let locked = self.locked.next().unwrap();

            if let Some(id) = id {
                return Some(Row {
                    id,
                    available,
                    held,
                    total,
                    locked,
                });
            }
        }

        None
    }
}
