use rust_decimal::Decimal;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Duplicate transaction: {id} {amount}")]
    DuplicateTransaction { id: u32, amount: Decimal },

    #[error(
        "Insufficient balance for transaction {id} (needed {expected}, got {got})"
    )]
    InsufficientBalance {
        id: u32,
        expected: Decimal,
        got: Decimal,
    },

    #[error("CSV error: {0}")]
    CSV(#[from] csv::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
