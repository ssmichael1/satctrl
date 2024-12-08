use crate::SCResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Matrix is singular")]
    SingularMatrix,
    #[error("Matrix is not square")]
    NotSquareMatrix,
    #[error("Matrix is not invertible")]
    NotInvertibleMatrix,
    #[error("Matrix is not positive definite")]
    NotPositiveDefiniteMatrix,
    #[error("Matrix is not positive semi-definite")]
    NotPositiveSemiDefiniteMatrix,
    #[error("Invalid Index: {0}")]
    InvalidIndex(i32),
}

impl<T> From<MathError> for SCResult<T> {
    fn from(err: MathError) -> Self {
        Err(Box::new(err))
    }
}
