use crate::input::_errors::input::InputError;
use crate::input::_traits::input_source::InputSource;
use std::io;

/// An input source that reads from standard input.
struct StdinSource;

impl InputSource for StdinSource {
  fn fetch_input(&mut self) -> Result<String, InputError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
  }
}
