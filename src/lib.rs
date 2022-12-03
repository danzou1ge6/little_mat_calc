/// Implements a [`Rational`] in the form of fraction
pub mod rational;

/// Implements a [`Polynomial`]
pub mod polynomial;

/// Defines the trait that elements in a matrix must satisfy
///
/// In short, these elements must be *linear*
pub mod element;

/// Defination of trait [`Mat`] and other solid matrix types
pub mod matrix;

/// Errors related to matrix operations
pub mod error;

/// A macro to concat blocks of matrixs
pub use mat_macro::concated_mat;
/// A macro to create [`DataMatrix`] with known dimension
pub use mat_macro::mat;

// Exports
pub use matrix::alg;
pub use matrix::MatBlock;
pub use matrix::{ConcatedMatrix, DataMatrix, EliminatedMatrix, Mat, SliceMatrix};
pub use polynomial::Polynomial;
pub use rational::Rational;
