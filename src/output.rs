/// An error that can occur when processing output.
pub mod _error;
/// Traits for output processing.
pub mod _traits;
/// The output enum.
#[allow(clippy::module_inception)]
pub mod output;
/// Implementations of OutputSink for various output sinks.
pub mod sinks;

/// The prelude.
pub mod prelude {
  pub use super::_error::OutputError;
  pub use super::_traits::async_sink::AsyncSink;
  pub use super::_traits::handler::Handler as OutputHandler;
  pub use super::_traits::sync_sink::SyncSink;
}
