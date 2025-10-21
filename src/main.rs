mod data;

use data::*;

fn apply(state: &mut State, input: Input) {
    let id = input.client as usize;
    state.id[id] = input.client;

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

fn main() {
    let file = vec![];
    let mut state = State::default();

    for input in file {
        apply(&mut state, input);
    }

    for ((((id, available), held), total), locked) in state
        .id
        .into_iter()
        .zip(state.available.into_iter())
        .zip(state.held.into_iter())
        .zip(state.total.into_iter())
        .zip(state.locked)
    {
        assert_eq!(available, total - held);
        assert_eq!(held, total - available);
        assert_eq!(total, available + held);

        println!("{id},{available},{held},{locked}");
    }
}
