/// Encapsulate all the possible errors that can occur in the library
///
pub enum SCError {
    /// Error message
    Message(String),
    NotFound,
    InvalidInput,
    InvalidState,
    InvalidOutput,
    InvalidMatrixIndex,
    MatrixIsSingular,
    VectorNormIsZero,
    NonPositiveDefiniteMatrix,
}

pub type SCResult<T> = Result<T, SCError>;
