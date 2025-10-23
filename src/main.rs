mod data;
mod error;

use std::io;

use csv::{ReaderBuilder, Trim, Writer};
use data::*;
use error::*;

fn apply(state: &mut State, input: Input) -> Result<()> {
    let id = input.client as usize;
    state.id[id] = Some(input.client);

    match state.transactions.get(&input.transaction_id) {
        Some(_) => {
            return Err(Error::DuplicateTransaction {
                id: input.transaction_id,
                amount: input.amount,
            });
        }
        None => {
            state.transactions.insert(
                input.transaction_id,
                Transaction {
                    amount: input.amount,
                    status: TransactionStatus::Open,
                },
            );
        }
    }

    match input.kind {
        InputType::Deposit => {
            state.available[id] += input.amount;
            state.total[id] += input.amount;
        }
        InputType::Withdrawal => {
            match state.available[id].checked_sub(input.amount) {
                Some(available) => {
                    state.available[id] = available;
                }
                None => {
                    return Err(Error::InsufficientBalance {
                        id: input.transaction_id,
                        expected: input.amount,
                        got: state.available[id],
                    });
                }
            }
        }
        InputType::Dispute => {
            todo!()
        }
        InputType::Resolve => {
            todo!()
        }
        InputType::Chargeback => {
            todo!()
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(io::stdin());
    let mut state = State::default();

    for input in reader.deserialize() {
        apply(&mut state, input?)?;
    }

    let mut writer = Writer::from_writer(io::stdout());

    for row in state {
        assert_eq!(row.available, row.total - row.held);
        assert_eq!(row.held, row.total - row.available);
        assert_eq!(row.total, row.available + row.held);

        writer.serialize(row)?;
    }

    writer.flush()?;

    Ok(())
}
