use crate::game_state::_traits::game_state::GameState;

/// A trait defining a game loop.
///
/// This trait defines a template for a synchronous game loop, the thought
/// being that a synchronous one is easier to define, develop, and debug.
pub trait GameLoop {
  /// The actual game loop.
  fn run(&self, state: &mut dyn GameState) {
    // Initialize game world, load assets, etc.
    self.setup(state);
    // The inner core of the game loop.
    // Until we have been told to quit...
    while !self.is_finished(state) {
      // Handle player commands or AI decisions.
      self.process_input(state);
      // Update game state, NPC behaviors, environment changes, etc.
      self.update(state);
      // Send updates to players or render the game state in some form.
      self.render(state);
    }
    // Perform any necessary cleanup before the game loop exits.
    self.teardown(state);
  }
  /// Initialize game world, load assets, etc.
  fn setup(&self, state: &mut dyn GameState);
  /// Determine if the game loop should exit.
  fn is_finished(&self, state: &mut dyn GameState) -> bool {
    state.get_quit_flag()
  }
  /// Handle player commands or AI decisions.
  fn process_input(&self, state: &mut dyn GameState);
  /// Update game state, NPC behaviors, environment changes, etc.
  fn update(&self, state: &mut dyn GameState);
  /// Send updates to players or render the game state in some form.
  fn render(&self, state: &mut dyn GameState);
  /// Perform any necessary cleanup before the game loop exits.
  fn teardown(&self, state: &mut dyn GameState);
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestGameState {
    quit_flag: bool,
  }

  impl GameState for TestGameState {
    fn get_quit_flag(&self) -> bool {
      self.quit_flag
    }
    fn set_quit_flag(&mut self, quit_flag: bool) {
      self.quit_flag = quit_flag;
    }
  }

  struct TestGameLoop {}

  impl GameLoop for TestGameLoop {
    fn setup(&self, _state: &mut dyn GameState) {
      println!("Setting up game loop...");
    }
    fn process_input(&self, _state: &mut dyn GameState) {
      println!("Processing input...");
    }
    fn update(&self, _state: &mut dyn GameState) {
      println!("Updating game state...");
    }
    fn render(&self, state: &mut dyn GameState) {
      state.set_quit_flag(true);
      println!("Rendering game state...");
    }
    fn teardown(&self, _state: &mut dyn GameState) {
      println!("Tearing down game loop...");
    }
  }

  #[test]
  fn test_game_loop() {
    let game_loop = TestGameLoop {};
    let mut game_state = TestGameState { quit_flag: false };
    game_loop.run(&mut game_state);
  }
}
