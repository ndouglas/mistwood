use rand::RngCore;
use std::sync::{Arc, Mutex};

/// A type alias for a shared, mutable object.
///
/// This ensures that we can safely share and mutate services across threads.
pub type SafePrng = Arc<Mutex<dyn RngCore + Send + Sync>>;
