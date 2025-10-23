# Ledger Experiment

## Notes

- There's nothing on the documentation standing that zero is a valid `client id`, and the examples themselves start at 1.
- Since we have a hard limit on the account id of `u16`, pre-allocating ALL state should take up to 4.14mb.
- We fail on duplicate transactions, instead of ignoring them.

## Definitions

In the documentation, it says that, when you withdraw more than what you have available, the import `should fail and the total amount of funds should not change`. What does this mean exactly?

1. Does it mean the script itself should fail and stop processing further entries?
2. Does it mean the script should just ignore the entry and continue to process all the other ones?

On the "disputes" part of the spec, it mentions `if the tx specified by the dispute doesn't exist you can ignore it`.

## AI Usage

Keeping with the spirit of the project, no AI code has been used.
