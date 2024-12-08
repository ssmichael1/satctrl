mod basemath;
//mod scerror;
mod time;

// All the types
pub type SCResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Matrix base types
pub use basemath::MathError;
pub use basemath::Matrix;
pub use basemath::Quaternion;
pub use basemath::Vector;

/// Common Vector sizes
pub use basemath::Vector1;
pub use basemath::Vector2;
pub use basemath::Vector3;
pub use basemath::Vector4;
pub use basemath::Vector5;
pub use basemath::Vector6;

/// Common square Matrix sizes
pub use basemath::Matrix2;
pub use basemath::Matrix3;
pub use basemath::Matrix4;
pub use basemath::Matrix5;
pub use basemath::Matrix6;

/// Runge-Kutta 4th order method
pub use basemath::rk4_integrate;
pub use basemath::rk4_integrate_inplace;

/// Math utilities
pub use basemath::matrixutils;

/// Filters (Kalman, etc)
pub mod filters;
/// Library utilities
pub mod utils;

// Time utilities
pub use time::Duration;
pub use time::Instant;
pub use time::InstantError;
pub use time::TimeScale;
