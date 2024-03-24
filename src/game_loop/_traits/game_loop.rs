/// A trait defining a game loop.
///
/// This trait defines a template for a synchronous game loop, the thought
/// being that a synchronous one is easier to define, develop, and debug.
pub trait GameLoop {
  /// The actual game loop.
  fn run(&self) {
    // Initialize game world, load assets, etc.
    self.setup();
    // The inner core of the game loop.
    // Until we have been told to quit...
    loop {
      // Handle player commands or AI decisions.
      self.process_input();
      // Update game state, NPC behaviors, environment changes, etc.
      self.update();
      // Send updates to players or render the game state in some form.
      self.render();
      // Check if we should exit the game loop.
      if self.is_finished() {
        break;
      }
    }
    // Perform any necessary cleanup before the game loop exits.
    self.teardown();
  }
  /// Initialize game world, load assets, etc.
  fn setup(&self);
  /// Determine if the game loop should exit.
  fn is_finished(&self) -> bool;
  /// Handle player commands or AI decisions.
  fn process_input(&self);
  /// Update game state, NPC behaviors, environment changes, etc.
  fn update(&self);
  /// Send updates to players or render the game state in some form.
  fn render(&self);
  /// Perform any necessary cleanup before the game loop exits.
  fn teardown(&self);
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestGameLoop {
    finished: bool,
  }

  impl GameLoop for TestGameLoop {
    fn setup(&self) {
      println!("Setting up game loop...");
    }
    fn is_finished(&self) -> bool {
      self.finished
    }
    fn process_input(&self) {
      println!("Processing input...");
    }
    fn update(&self) {
      println!("Updating game state...");
    }
    fn render(&self) {
      println!("Rendering game state...");
    }
    fn teardown(&self) {
      println!("Tearing down game loop...");
    }
  }

  #[test]
  fn test_game_loop() {
    let game_loop = TestGameLoop { finished: true };
    game_loop.run();
  }
}
