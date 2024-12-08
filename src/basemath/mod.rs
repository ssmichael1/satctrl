mod matherr;
mod matrix;
mod matrixops;
mod quaternion;
mod rk4;

#[cfg(test)]
mod tests;

pub use matherr::MathError;
pub use matrix::Matrix;
pub use matrix::Vector;
pub use quaternion::Quaternion;

pub use rk4::rk4_integrate;
pub use rk4::rk4_integrate_inplace;

/// Some common vector types
pub type Vector6 = Vector<6>;
pub type Vector5 = Vector<5>;
pub type Vector4 = Vector<4>;
pub type Vector3 = Vector<3>;
pub type Vector2 = Vector<2>;
pub type Vector1 = Vector<1>;

/// Some common matrix types
pub type Matrix2 = Matrix<2, 2>;
pub type Matrix3 = Matrix<3, 3>;
pub type Matrix4 = Matrix<4, 4>;
pub type Matrix5 = Matrix<5, 5>;
pub type Matrix6 = Matrix<6, 6>;

pub mod matrixutils;
