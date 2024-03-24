pub struct GameState {}

impl GameState {
  /// Create a new game state.
  pub fn new() -> Self {
    GameState {}
  }

  /// Describe the current location.
  pub fn describe_current_location(&self) -> String {
    "You are in a dark, damp cave.".to_string()
  }

  /// Process the player's input.
  pub fn process_input(&mut self, _input: &str) -> Result<String, &'static str> {
    Ok("You move forward.".to_string())
  }
}
