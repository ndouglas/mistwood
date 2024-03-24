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
    while !self.is_finished() {
      // Handle player commands or AI decisions.
      self.process_input();
      // Update game state, NPC behaviors, environment changes, etc.
      self.update();
      // Send updates to players or render the game state in some form.
      self.render();
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
