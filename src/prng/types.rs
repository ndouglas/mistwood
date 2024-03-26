use rand::RngCore;
use std::sync::{Arc, Mutex};

/// A type alias for a shared, mutable pseudorandom number generator.
///
/// This ensures that we can safely share and mutate services across threads.
pub type SafePrng = Arc<Mutex<dyn RngCore + Send + Sync>>;

/// A type alias for a boxed pseudorandom number generator.
///
/// This ensures that we can safely share and mutate services across threads.
pub type BoxedPrng = Box<dyn RngCore + Send + Sync>;
