/// An error that can occur when processing output.
pub mod error;
/// The output enum.
#[allow(clippy::module_inception)]
pub mod output;
/// Implementations of OutputSink for various output sinks.
pub mod sinks;
/// Traits for output processing.
pub mod traits;

/// The prelude.
pub mod prelude {
  pub use super::error::OutputError;
  pub use super::sinks::generic::GenericSink;
  pub use super::sinks::string::StringSink;
  pub use super::traits::async_sink::AsyncSink;
  pub use super::traits::handler::Handler as OutputHandler;
  pub use super::traits::sync_sink::SyncSink;
}
