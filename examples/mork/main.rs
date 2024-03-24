//! Mork is a clone of Zork using the Mistwood library.
//!
//! ░▒▓██████████████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░
//! ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
//! ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
//! ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░░▒▓███████▓▒░
//! ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
//! ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
//! ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
//!
//! The goal is to reimplement a classic text adventure game, building out the
//! necessary functionality in the Mistwood library as we go.
#[macro_use]
extern crate mistwood;
use mistwood::messaging::_traits::template_processor::TemplateProcessor as TemplateProcessorTrait;
use mistwood::messaging::prelude::TemplateProcessor;
use mistwood::messaging::prelude::TemplateProvider;
use std::io::{self, Write};

pub mod game_state;
use game_state::GameState;
pub mod message_templates;
use message_templates::*;

fn main() {
  let mut game_state = GameState::new();
  let mut processor = TemplateProcessor::new();
  processor.set_strict_mode(true);
  let mut provider = TemplateProvider::new();
  // As a player, I want to see the game's title and version number when I start
  // the game so that I know what game I'm playing.
  // See #77.
  let first_lines = info_message!(provider, FirstLines, {
    revision: 1,
    serial_number: 12345,
  });
  println!("{}", processor.process_message(&first_lines).unwrap());
  // As a player, I want a game loop so that the game doesn't end after one
  // command.
  // See #78.
  loop {
    // As a player, I want to see a description of my current location so that I
    // know where I am.
    // See #79.
    println!("{}", game_state.describe_current_location());
    // As a player, I want to see a prompt so that I know when to provide input.
    // See #80.
    print!("> ");
    // Ensure the prompt is displayed immediately.
    io::stdout().flush().unwrap();
    let mut input = String::new();
    // As a player, I want to be able to provide input to the game.
    // See #81.
    match io::stdin().read_line(&mut input) {
      Ok(_) => {
        input = input.trim().to_string();
        // As a player, I want to be able to quit the game.
        // See #75.
        if input.eq_ignore_ascii_case("quit") {
          // As a player, I want to be prompted for confirmation before
          // quitting the game so that I don't accidentally lose progress.
          // See #76. This ties in with #75 as well. And how do we handle
          // prompting for player confirmation in an asynchronous game?
          let quit_confirmation = info_message!(provider, QuitConfirmation);
          println!("{}", processor.process_message(&quit_confirmation).unwrap());
          let mut response = String::new();
          io::stdin().read_line(&mut response).unwrap();
          if response.trim().eq_ignore_ascii_case("y") {
            break;
          } else {
            println!("Ok.");
            continue;
          }
        }
        // As a player, I want to see the result of my input so that I know what
        // happened.
        // See #83.
        match game_state.process_input(&input) {
          Ok(output) => println!("{}", output),
          Err(e) => println!("Error: {}", e),
        }
      },
      // As a player, I want to see an error message if my input is invalid
      // so that I know what went wrong.
      // See #82.
      Err(error) => println!("Failed to read input: {}", error),
    }
  }
}
