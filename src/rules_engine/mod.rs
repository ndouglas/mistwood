pub mod arguments;
pub mod conclusions;
pub mod conditions;
pub mod operations;
pub mod traits;

pub mod prelude {
  pub use super::arguments::prelude::*;
  // pub use super::conclusions::prelude::*;
  // pub use super::conditions::prelude::*;
  // pub use super::operations::prelude::*;
  pub use super::traits::prelude::*;
}
