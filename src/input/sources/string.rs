use crate::input::_errors::input::InputError;
use crate::input::_traits::input_source::InputSource;
use std::io;

/// An input source that reads from standard input.
struct StringSource {
  inputs: Vec<String>,
}

impl StringSource {
  pub fn new(inputs: Vec<String>) -> Self {
    Self { inputs }
  }
}

impl InputSource for StringSource {
  fn fetch_input(&mut self) -> Result<String, InputError> {
    let input = self.inputs.pop().unwrap_or_default();
    Ok(input.trim().to_string())
  }
}
