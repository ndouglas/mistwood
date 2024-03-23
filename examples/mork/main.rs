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

pub mod message_templates;
use message_templates::FirstLines;

fn main() {
  let mut processor = TemplateProcessor::new();
  processor.set_strict_mode(true);
  let mut provider = TemplateProvider::new();
  let first_lines = info_message!(provider, FirstLines, {
    revision: 1,
    serial_number: 12345,
  });
  println!("{}", processor.process_message(&first_lines).unwrap());
}
