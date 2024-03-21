/// The Gravity of the message indicates the general seriousness or severity
/// of the situation. It is somewhat akin to a loglevel.
pub mod gravity;

/// Prelude.
pub mod prelude {
  pub use super::gravity::Gravity;
}
