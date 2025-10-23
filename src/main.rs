mod data;
mod error;

use std::{env, fs, io};

use csv::{ReaderBuilder, Trim, Writer};
use data::*;
use error::*;

fn apply(state: &mut State, input: Input) -> Result<()> {
    let id = input.client as usize;
    state.id[id] = Some(input.client);

    if state.locked[id] {
        return Err(Error::LockedUser {
            client_id: input.client,
            transaction_id: input.transaction_id,
        });
    }

    let amount = input.amount.unwrap_or_default();

    match input.kind {
        InputType::Deposit => {
            match state.transactions.get(&input.transaction_id) {
                Some(_) => {
                    return Err(Error::DuplicateTransaction {
                        transaction_id: input.transaction_id,
                        amount,
                    });
                }
                None => {
                    state.transactions.insert(
                        input.transaction_id,
                        Transaction {
                            amount,
                            status: TransactionStatus::Open,
                        },
                    );
                }
            }

            state.available[id] += amount;
            state.total[id] += amount;
        }
        InputType::Withdrawal => {
            match state.transactions.get(&input.transaction_id) {
                Some(_) => {
                    return Err(Error::DuplicateTransaction {
                        transaction_id: input.transaction_id,
                        amount: amount,
                    });
                }
                None => {
                    state.transactions.insert(
                        input.transaction_id,
                        Transaction {
                            amount,
                            status: TransactionStatus::Open,
                        },
                    );
                }
            }

            match (
                state.available[id].checked_sub(amount),
                state.total[id].checked_sub(amount),
            ) {
                (Some(available), Some(total)) => {
                    state.available[id] = available;
                    state.total[id] -= total;
                }
                _ => {
                    return Err(Error::InsufficientBalance {
                        transaction_id: input.transaction_id,
                        expected: amount,
                        got: state.available[id],
                    });
                }
            }
        }
        InputType::Dispute => {
            if let Some(tx) = state.transactions.get_mut(&input.transaction_id)
                && tx.status == TransactionStatus::Open
            {
                tx.status = TransactionStatus::Disputed;
            } else {
                return Ok(());
            }

            match state.available[id].checked_sub(amount) {
                Some(available) => {
                    state.available[id] = available;
                    state.held[id] += amount;
                }
                _ => {
                    return Err(Error::InsufficientBalance {
                        transaction_id: input.transaction_id,
                        expected: amount,
                        got: state.available[id],
                    });
                }
            }
        }
        InputType::Resolve => {
            if let Some(tx) = state.transactions.get_mut(&input.transaction_id)
            {
                if tx.status != TransactionStatus::Disputed {
                    return Ok(());
                }

                tx.status = TransactionStatus::Resolved;

                match state.held[id].checked_sub(tx.amount) {
                    Some(held) => {
                        state.held[id] -= held;
                        state.available[id] += held;
                    }
                    None => {
                        return Err(Error::InsufficientBalance {
                            transaction_id: input.transaction_id,
                            expected: tx.amount,
                            got: state.held[id],
                        });
                    }
                }
            }
        }
        InputType::Chargeback => {
            if let Some(tx) = state.transactions.get_mut(&input.transaction_id)
                && tx.status == TransactionStatus::Disputed
            {
                tx.status = TransactionStatus::Chargedback;

                if state.held[id] >= tx.amount {
                    state.held[id] -= tx.amount;
                    state.total[id] -= tx.amount;
                    state.locked[id] = true;
                } else {
                    return Err(Error::InsufficientBalance {
                        transaction_id: input.transaction_id,
                        expected: tx.amount,
                        got: state.held[id],
                    });
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let file = args
        .first()
        .ok_or(Error::MissingInputFile)
        .and_then(|f| fs::File::open(f).map_err(Error::from))?;

    let mut reader = ReaderBuilder::new().trim(Trim::All).from_reader(file);

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
