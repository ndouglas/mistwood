#[macro_use]
pub mod macros;

pub mod bool;
pub mod entity;
pub mod r#enum;
pub mod float;
pub mod float_range;
pub mod int;
pub mod int_range;
pub mod string;

pub mod prelude {
  pub use super::bool::*;
  pub use super::entity::*;
  pub use super::float::*;
  pub use super::float_range::*;
  pub use super::int::*;
  pub use super::int_range::*;
  pub use super::macros::*;
  pub use super::r#enum::*;
  pub use super::string::*;
}
