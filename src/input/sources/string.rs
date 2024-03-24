use crate::input::_errors::input::InputError;
use crate::input::_traits::sync_source::SyncSource;

/// An input source that reads from a list of strings.
#[derive(Debug)]
pub struct StringSource {
  inputs: Vec<String>,
}

impl StringSource {
  /// Create a new StringSource with the given inputs.
  pub fn new(inputs: Vec<String>) -> Self {
    let inputs = inputs.into_iter().rev().collect();
    Self { inputs }
  }
}

impl SyncSource<String> for StringSource {
  fn fetch_input(&mut self) -> Result<String, InputError> {
    let input = self.inputs.pop().ok_or(InputError::NoInput)?;
    Ok(input.trim().to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fetch_input() {
    let inputs = vec!["test".to_string(), "test2".to_string()];
    let mut input_source = StringSource::new(inputs);
    let input = input_source.fetch_input().unwrap();
    assert_eq!(input, "test");
    let input = input_source.fetch_input().unwrap();
    assert_eq!(input, "test2");
    let input = input_source.fetch_input();
    assert!(input.is_err());
  }
}
