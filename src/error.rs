use thiserror::Error;

#[derive(Debug, Error)]
pub enum SqrtError {
    #[error("Cannot calculate the square root of a negative number: {0}")]
    NegativeNumber(f64),
}
