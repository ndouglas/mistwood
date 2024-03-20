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

pub mod actors;
pub mod ai;
pub mod data;
pub mod ecs;
pub mod events;
pub mod frp;
pub mod procgen;
pub mod script;

pub mod prelude {
  // pub use crate::actors::prelude::*;
  // pub use crate::ai::prelude::*;
  // pub use crate::data::prelude::*;
  // pub use crate::ecs::prelude::*;
  // pub use crate::events::prelude::*;
  // pub use crate::frp::prelude::*;
  // pub use crate::procgen::prelude::*;
  // pub use crate::script::prelude::*;
}

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  #[allow(unused_imports)]
  use super::*;

  /// Call this function at the beginning of each test module.
  pub fn init() {
    // Enable logging for tests.
    let _ = builder().is_test(true).try_init();
    // Enable backtraces.
    set_var("RUST_BACKTRACE", "1");
  }
}
