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
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate strum;
#[allow(unused_imports)]
#[macro_use]
extern crate thiserror;

/// Library-wide type definitions.
pub mod _types;
/// Dependency Injection.
pub mod di;

pub mod prelude {
  pub use crate::_types::prelude::*;
  pub use crate::di::prelude::*;
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
    set_var("RUST_BACKTRACE", "1");
  }
}
