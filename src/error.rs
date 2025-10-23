use rust_decimal::Decimal;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Duplicate transaction: {transaction_id} {amount}")]
    DuplicateTransaction {
        transaction_id: u32,
        amount: Decimal,
    },

    #[error(
        "Insufficient balance for transaction {transaction_id} (expected {expected}, got {got})"
    )]
    InsufficientBalance {
        transaction_id: u32,
        expected: Decimal,
        got: Decimal,
    },

    #[error(
        "Client {client_id} is locked, can not progress with transaction {transaction_id}"
    )]
    LockedUser { client_id: u16, transaction_id: u32 },

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
