//! A library for roguelikes, MUDs, and other text adventures.
//!
//! I don't really anticipate anyone using this library, as it's very specific
//! and intensely opinionated. Nevertheless, I'm making it public in case
//! someone finds it useful or interesting.
//!
//! This crate provides a set of tools for building text-based games:
//!
//! - **Dependency Injection**: A system for managing game state and providing
//!   services to game objects.
//! - **Messaging**: A highly extensible system for creating messages that can
//!   be displayed to the player, based on template processing.
//! - **Pseudorandom Number Generation**: A system for managing pseudorandom
//!   number generators, including a registry for sharing them across the game.
//! - **Input**: A system for managing player input, whether by keypresses or
//!   by text commands, and parsing it into commands.
//! - **Commands**: A system for managing game commands, which are any actions
//!   that the player can take, in or out of character.

// Linting.
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(ambiguous_glob_imports)]
#![deny(ambiguous_glob_reexports)]
#![deny(bare_trait_objects)]
#![deny(const_item_mutation)]
#![deny(explicit_outlives_requirements)]
#![deny(let_underscore_drop)]
#![deny(meta_variable_misuse)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(non_ascii_idents)]
#![deny(single_use_lifetimes)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
// #![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
#![deny(variant_size_differences)]
#![allow(unused_macros)]
#![allow(async_fn_in_trait)]

// Crate-wide configuration.
#[allow(unused_imports)]
#[macro_use]
extern crate all_asserts;
#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;
#[allow(unused_imports)]
#[macro_use]
extern crate derivative;
#[allow(unused_imports)]
#[macro_use]
extern crate derive_builder;
#[allow(unused_imports)]
#[macro_use]
extern crate derive_more;
#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[allow(unused_imports)]
#[macro_use]
extern crate paste;
#[allow(unused_imports)]
#[macro_use]
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate strum;
#[allow(unused_imports)]
#[macro_use]
extern crate thiserror;

/// Library-wide type definitions.
pub mod _types;
/// Commands.
pub mod commands;
/// Dependency Injection.
pub mod di;
/// Input processing.
pub mod input;
/// Message processing.
pub mod messaging;
/// Pseudorandom number generation and related utilities.
pub mod prng;

/// Prelude for the library.
pub mod prelude {
  pub use crate::_types::prelude::TypeMap;
}

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  #[allow(unused_imports)]
  use super::*;

  /// Call this function at the beginning of each test.
  pub fn init() {
    // Enable logging for tests.
    let _ = builder().is_test(true).try_init();
    // Enable backtraces.
    set_var("RUST_BACKTRACE", "full");
  }
}
