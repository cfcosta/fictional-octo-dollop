mod data;

use std::io;

use csv::{ReaderBuilder, Trim, Writer};
use data::*;

fn apply(state: &mut State, input: Input) {
    let id = input.client as usize;
    state.id[id] = Some(input.client);

    match input.kind {
        InputType::Deposit => {
            state.available[id] += input.amount;
            state.total[id] += input.amount;
        }
        InputType::Withdrawal => {
            if let Some(available) =
                state.available[id].checked_sub(input.amount)
            {
                state.available[id] = available;
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
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CSV error: {0}")]
    CSV(#[from] csv::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), Error> {
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(io::stdin());
    let mut state = State::default();

    for input in reader.deserialize() {
        apply(&mut state, input?);
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
