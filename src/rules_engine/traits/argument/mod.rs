pub mod bool;
pub mod entity;
pub mod r#enum;
pub mod float;
pub mod int;
pub mod script;
pub mod string;

pub mod prelude {
  pub use super::bool::*;
  pub use super::entity::*;
  pub use super::float::*;
  pub use super::int::*;
  pub use super::r#enum::*;
  pub use super::script::*;
  pub use super::string::*;
}
