pub type Id = u16;

pub enum InputType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

pub struct Input {
    pub kind: InputType,
    pub client: Id,
    pub transaction: Id,
    pub amount: u64,
}

pub struct State {
    pub id: Vec<Id>,
    pub available: Vec<u64>,
    pub held: Vec<u64>,
    pub locked: Vec<bool>,
}

fn apply(state: &mut State, input: Input) {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
