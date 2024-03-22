use crate::messaging::gravity::Gravity;
use serde_json::Value as JsonValue;

/// Message encapsulates a message to be sent to the user.
#[derive(Debug, Derivative, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derivative(PartialOrd)]
pub struct Message {
  /// The message template.
  pub template: String,
  /// The gravity of the message.
  #[derivative(PartialOrd = "ignore")]
  pub gravity: Gravity,
  /// Data to be inserted into the template.
  #[derivative(PartialOrd = "ignore")]
  pub data: JsonValue,
  /// Metadata about the message.
  #[derivative(PartialOrd = "ignore")]
  pub metadata: Option<JsonValue>,
}

impl Message {
  /// Create a new message.
  pub fn new(template: String, gravity: Gravity, data: JsonValue, metadata: Option<JsonValue>) -> Self {
    Self {
      template,
      gravity,
      data,
      metadata,
    }
  }
}
