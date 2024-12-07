use thiserror::Error;

#[derive(Error, Debug)]
pub enum SCErr {
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Not found")]
    NotFound,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Invalid state")]
    InvalidState,
    #[error("Invalid output")]
    InvalidOutput,
    #[error("Invalid matrix index")]
    InvalidMatrixIndex,
    #[error("Matrix is singular")]
    MatrixIsSingular,
    #[error("Vector norm is zero")]
    VectorNormIsZero,
    #[error("Invalid time string")]
    InvalidTimeString,
    #[error("Non-positive definite matrix")]
    NonPositiveDefiniteMatrix,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Format error: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Time error: {0}")]
    Instant(crate::InstantError),
    #[error("Matrix error: {0}")]
    Matrix(crate::MathError),
}

pub type SCResult<T> = std::result::Result<T, SCErr>;

impl<T> From<crate::InstantError> for SCResult<T> {
    fn from(err: crate::InstantError) -> Self {
        Err(SCErr::Instant(err))
    }
}

impl<T> From<crate::MathError> for SCResult<T> {
    fn from(err: crate::MathError) -> Self {
        Err(SCErr::Matrix(err))
    }
}

impl<T> From<SCErr> for SCResult<T> {
    fn from(err: SCErr) -> Self {
        Err(err)
    }
}
