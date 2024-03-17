use crate::prelude::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NullContext;

#[typetag::serde]
impl Context for NullContext {}
