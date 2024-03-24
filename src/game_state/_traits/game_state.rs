/// A trait for the game state.
pub trait GameState {
  /// Determine if the game loop should exit.
  fn get_quit_flag(&self) -> bool;
  /// Set the game loop to exit.
  fn set_quit_flag(&mut self, quit_flag: bool);
}
