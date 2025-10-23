mod data;
mod error;

use std::{env, fs, io};

use csv::{ReaderBuilder, Trim, WriterBuilder};
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
            if amount.is_zero() {
                return Err(Error::InvalidTransaction {
                    transaction_id: input.transaction_id,
                    reason: "Deposit amount can not be zero",
                });
            }

            if state.transactions.contains_key(&input.transaction_id) {
                return Err(Error::DuplicateTransaction {
                    transaction_id: input.transaction_id,
                    amount,
                });
            }

            state.available[id] += amount;
            state.total[id] += amount;

            state.transactions.insert(
                input.transaction_id,
                Transaction {
                    amount,
                    status: TransactionStatus::Open,
                },
            );
        }
        InputType::Withdrawal => {
            if amount.is_zero() {
                return Err(Error::InvalidTransaction {
                    transaction_id: input.transaction_id,
                    reason: "Withdrawal amount can not be zero",
                });
            }

            if state.transactions.contains_key(&input.transaction_id) {
                return Err(Error::DuplicateTransaction {
                    transaction_id: input.transaction_id,
                    amount,
                });
            }

            if state.available[id] >= amount && state.total[id] >= amount {
                state.available[id] -= amount;
                state.total[id] -= amount;

                state.transactions.insert(
                    input.transaction_id,
                    Transaction {
                        amount,
                        status: TransactionStatus::Open,
                    },
                );
            }
        }
        InputType::Dispute => {
            let tx = match state.transactions.get_mut(&input.transaction_id) {
                Some(tx) if tx.status == TransactionStatus::Open => tx,
                _ => {
                    return Ok(());
                }
            };

            if state.available[id] >= tx.amount {
                state.available[id] -= tx.amount;
                state.held[id] += tx.amount;

                tx.status = TransactionStatus::Disputed;
            } else {
                return Err(Error::InsufficientBalance {
                    transaction_id: input.transaction_id,
                    expected: tx.amount,
                    got: state.available[id],
                });
            }
        }
        InputType::Resolve => {
            let tx = match state.transactions.get_mut(&input.transaction_id) {
                Some(tx) if tx.status == TransactionStatus::Disputed => tx,
                _ => {
                    return Ok(());
                }
            };

            if state.held[id] >= tx.amount {
                state.held[id] -= tx.amount;
                state.available[id] += tx.amount;

                tx.status = TransactionStatus::Resolved;
            } else {
                return Err(Error::InsufficientBalance {
                    transaction_id: input.transaction_id,
                    expected: tx.amount,
                    got: state.held[id],
                });
            }
        }
        InputType::Chargeback => {
            let tx = match state.transactions.get_mut(&input.transaction_id) {
                Some(tx) if tx.status == TransactionStatus::Disputed => tx,
                _ => {
                    return Ok(());
                }
            };

            tx.status = TransactionStatus::Chargedback;

            if state.held[id] >= tx.amount && state.total[id] >= tx.amount {
                state.held[id] -= tx.amount;
                state.total[id] -= tx.amount;
                state.locked[id] = true;

                tx.status = TransactionStatus::Chargedback;
            } else {
                return Err(Error::InsufficientBalance {
                    transaction_id: input.transaction_id,
                    expected: tx.amount,
                    got: state.held[id],
                });
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

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::All)
        .from_reader(file);

    let mut state = State::default();

    for input in reader.deserialize() {
        let input = input?;

        match apply(&mut state, input) {
            Ok(_) => {}
            Err(err) => {
                eprintln!(
                    "Got an error when processing input, skipping.\nRow: {input:?}\nError: {err}"
                );
            }
        }
    }

    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(io::stdout());

    for row in state {
        assert_eq!(row.available, row.total - row.held);
        assert_eq!(row.held, row.total - row.available);
        assert_eq!(row.total, row.available + row.held);

        writer.serialize(row)?;
    }

    writer.flush()?;

    Ok(())
}
