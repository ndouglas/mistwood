use serde::{Deserialize, Serialize};

/// The Gravity of the message indicates the general seriousness or severity
/// of the situation. It is somewhat akin to a loglevel.
///
/// It is expected that these conditions will imply some formatting of their
/// own (for instance, Fatal might be bold maroon red, critical bright red,
/// alert red, warning yellow, notice and info white).
///
/// In all cases, formatting is handled as a stack, i.e. any formatting that
/// occurs within the string is added, and when no longer needed the previous
/// state is restored. Formatting is not at any time "reset" to the global
/// default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Gravity {
  /// Normal information. Room descriptions, results of successful operations
  /// ("you pick up the rock"), etc.
  Info,
  /// Basic updates or status notifications with no immediate negative
  /// consequences, or something isn't quite right. "You're getting hungry."
  /// "It's getting dark." "You don't see any rocks."
  Notice,
  /// Indications of potential negative outcomes or the need for caution. These
  /// are not immediately dangerous but could escalate if ignored. E.g., "You
  /// are very hungry," "The path ahead looks treacherous."
  Warning,
  /// Situations that require immediate attention but aren't yet
  /// life-threatening. These alerts signify that the player should take action
  /// to avoid harm. E.g., "You are starving," "You are bleeding."
  Alert,
  /// Severe situations that pose an immediate threat to the player's progress,
  /// health, or survival. These messages indicate conditions that need urgent
  /// action to prevent catastrophic outcomes. E.g., "You are bleeding badly,"
  /// "You are on the brink of starvation."
  Critical,
  /// Conditions that are irreversible or have led to a game-ending scenario.
  /// These messages might be used sparingly, such as in permadeath situations
  /// or critical mission failures. E.g., "You have bled out," "You have
  /// succumbed to starvation." The jig is up.
  Fatal,
}
