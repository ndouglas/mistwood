use thiserror::Error as ThisError;

/// An error that can occur when parsing the content of input.
///
/// This error type is used to represent parsing errors, such as invalid syntax
/// or unexpected input. This is distinct from errors that occur due to the
/// mechanical process of reading input, such as file errors or network
/// connection errors.
#[derive(ThisError, Debug, Clone, Copy)]
pub enum ParseError {}
