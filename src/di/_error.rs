use thiserror::Error as ThisError;

/// Errors for the Dependency Injection system.
#[derive(ThisError, Copy, Clone, Debug)]
pub enum DiError {}
