pub type Index = u32;

pub enum InputType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

pub struct Input {
    pub kind: InputType,
    pub client: u16,
    pub transaction: Index,
    pub amount: u64,
}

#[derive(Debug)]
pub struct Row {
    pub id: u16,
    pub available: u64,
    pub held: u64,
    pub total: u64,
    pub locked: bool,
}

#[derive(Debug)]
pub struct State {
    pub id: Vec<Option<u16>>,
    pub available: Vec<u64>,
    pub held: Vec<u64>,
    pub total: Vec<u64>,
    pub locked: Vec<bool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            id: vec![None; u16::MAX as usize],
            available: vec![0; u16::MAX as usize],
            held: vec![0; u16::MAX as usize],
            total: vec![0; u16::MAX as usize],
            locked: vec![false; u16::MAX as usize],
        }
    }
}
