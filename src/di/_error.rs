use thiserror::Error as ThisError;

/// Errors for the Dependency Injection system.
#[derive(ThisError, Debug)]
pub enum DiError {
  /// An error occurred while building an object.
  #[error("Could not build object: {0}")]
  BuildError(String),
  /// An error occurred while getting an input.
  #[error("Could not get input: {0}")]
  NotFound(String),
}
