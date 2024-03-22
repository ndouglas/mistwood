template_list!(WeatherChange, [
  // The air grows colder as night falls.
  // The air grows warmer as day breaks.
  // The air grows more humid as the storm approaches.
  // The air grows drier as the storm passes.
  // The air grows still as the storm arrives.
  "The air grows {{ weather_change }} as {{ event_happens }}.",
], {
  weather_change: String,
  event_happens: String,
});
template_list!(
WindChange,
[
  // The wind picks up as the storm approaches.
  // The wind dies down as the storm passes.
  // The wind howls as the storm arrives.
  // The wind calms as the storm leaves.
  // The wind changes direction as the storm passes.
  "The wind {{ wind_change }} as {{ event_happens }}.",
], {
  wind_change: String,
  event_happens: String,
});

// "EnvironmentalChange: Notification of changes in the environment. E.g., \"The air grows colder as night falls.\"",
// "ItemInteractionSuccess: Positive feedback on interacting with an item. E.g., \"You successfully unlock the chest.\"",
// "ItemInteractionFailure: Negative feedback on interacting with an item. E.g., \"The chest is locked tight.\"",
// "NavigationSuccess: Confirmation of successful movement. E.g., \"You move north.\"",
// "NavigationFailure: Notification of failed movement. E.g., \"The path is blocked by a fallen tree.\"",

// Environmental Interactions
// EnvironmentalChange: Notification of changes in the environment. E.g., "The air grows colder as night falls."
// ItemInteractionSuccess: Positive feedback on interacting with an item. E.g., "You successfully unlock the chest."
// ItemInteractionFailure: Negative feedback on interacting with an item. E.g., "The chest is locked tight."
// NavigationSuccess: Confirmation of successful movement. E.g., "You move north."
// NavigationFailure: Notification of failed movement. E.g., "The path is blocked by a fallen tree."
// NPC Interactions
// NPCDialogue: Template for NPC dialogue. E.g., "{{ npc.name }} says, '{{#if player.knowsSecret}}I knew you would come.{{else}}Who are you?{{/if}}'"
// NPCActionNotice: Notification of NPC actions. E.g., "{{ npc.name }} starts to get agitated."
// FactionStatusChange: Notification of changes in faction relationships. E.g., "Your actions have made you enemies of the {{ faction.name }}."
// Combat and Health
// CombatInitiation: Notification of combat starting. E.g., "{{ enemy.name }} lunges at you!"
// CombatOutcome: Summary of combat results. E.g., "You defeat {{ enemy.name }} but suffer injuries."
// HealthCritical: Warning of critical health levels. E.g., "You are gravely injured and must find aid soon."
// Quests and Objectives
// QuestAccepted: Confirmation of quest acceptance. E.g., "You have accepted the quest: {{ quest.name }}."
// QuestCompleted: Notification of quest completion. E.g., "You have completed the quest: {{ quest.name }}."
// ObjectiveUpdated: Notification of an objective update. E.g., "Objective updated: {{ objective.description }}."
// System Messages and Meta
// SaveGameSuccess: Confirmation of successful game save. E.g., "Game saved successfully."
// SaveGameFailure: Notification of failed game save. E.g., "Failed to save game. Please try again."
// LoadGameSuccess: Confirmation of successful game load. E.g., "Game loaded successfully."
// LoadGameFailure: Notification of failed game load. E.g., "Failed to load game. Please try again."
// Miscellaneous
// InventoryFull: Notification that the player's inventory is full. E.g., "You cannot carry any more items."
// ItemNotFound: Notification that the specified item was not found. E.g., "You search the area but find nothing of interest."
// ActionNotAllowed: Notification of an action that is not allowed. E.g., "You cannot do that here."
