// NOTE: This is all very beta and subject to change.
//
// Some of this is crap.
//
// Don't pay too much attention to it.

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
template_list!(EnvironmentNarrative, [
  // Leaves rustle softly as a gentle breeze passes through the forest.
  // Shadows lengthen as the sun sets behind the mountains.
  // The forest falls silent as a predator's call echoes in the distance.
  // A sense of foreboding fills the air as dark clouds gather overhead.
  // The path becomes clearer as the fog begins to lift.
  "The {{ object }} {{ action }} as {{ trigger_event }}.",
], {
  object: String,
  action: String,
  trigger_event: String,
});

template_list!(WildernessEncounter, [
  // A bear lumbers out of the underbrush as you walk through the forest.
  // A deer darts across your path as you walk through the forest.
  // A wolf howls in the distance as you pass through the forest.
  // A squirrel chatters angrily as you trudge through the undergrowth.
  // A rabbit freezes in place as you approach through the underbrush.
  "A {{ creature }} {{ behavior }} as you {{ player_action }} through the {{ location }}.",
  // A bear lumbers out of the underbrush, then disappears into the forest.
  // A deer darts across your path, then vanishes into the undergrowth.
  // A wolf howls in the distance, then falls silent.
  // A squirrel chatters angrily, then scampers away.
  "A {{ creature }} {{ behavior }}, then {{ goes_away }}.",
  // As you walk through the forest, a bear lumbers out of the underbrush.
  // As you walk through the forest, a deer darts across your path.
  // As you pass through the forest, a wolf howls in the distance.
  // As you trudge through the undergrowth, a squirrel chatters angrily.
  // As you approach through the underbrush, a rabbit freezes in place.
  "As you {{ player_action }}, a {{ creature }} {{ behavior }} through the {{ location }}.",
], {
  creature: String,
  behavior: String,
  player_action: String,
  goes_away: String,
  location: String,
});
template_list!(MysticalEvent, [
  // The fireball explodes as the wizard casts the spell.
  // The portal shimmers as the key is turned.
  // The potion bubbles as the ingredients are mixed.
  // The crystal glows as the incantation is spoken.
  // The amulet shatters as the curse is broken.
  "The {{ mystical_element }} {{ reaction }} as the {{ event_trigger }} {{ event_action }}.",
  // As the wizard casts the spell, the fireball explodes.
  // As the key is turned, the portal shimmers.
  // As the ingredients are mixed, the potion bubbles.
  // As the incantation is spoken, the crystal glows.
  "As the {{ event_trigger }} {{ event_action }}, the {{ mystical_element }} {{ reaction }}.",
], {
  mystical_element: String,
  reaction: String,
  event_trigger: String,
  event_action: String,
});
template_list!(VillageDynamics, [
  // The village comes alive as the market opens.
  // The village grows quiet as the sun sets.
  // The village bustles with activity as the festival begins.
  // The village falls silent as the storm approaches.
  "The village {{ atmosphere }} as {{ significant_event }} {{ event_outcome }}.",
  // With the arrival of spring, the village comes alive.
  // With the arrival of autumn, the village grows quiet.
  // With the arrival of summer, the village bustles with activity.
  // With the arrival of winter, the village falls silent.
  "With the arrival of {{ season }}, the village {{ atmosphere }}.",
], {
  atmosphere: String,
  significant_event: String,
  event_outcome: String,
});
template_list!(SeasonalChange, [
  // The flowers bloom as spring arrives.
  // The leaves change color as autumn arrives.
  "The {{ plant }} {{ plant_action }} as {{ season }} arrives.",
  // As winter arrives, the snow begins to fall.
  "As {{ season }} arrives, the {{ environmental_effect }} becomes {{ observable_change }}.",
  // The cold becomes more biting as winter arrives.
  // The heat becomes more oppressive as summer arrives.
  // The rain becomes more frequent as the monsoon season arrives.
  "The {{ environmental_effect }} becomes {{ observable_change }} as {{ season }} arrives.",
], {
  season: String,
  environmental_effect: String,
  observable_change: String,
});

template_list!(DayNightCycle, [
  // With the sun rising, the birds begin to sing.
  // With the sun setting, the shadows grow long.
  // With the moon rising, the night creatures awaken.
  // With the moon setting, the world grows still.
  "With the {{ time_of_day }}, the {{ event }}.",
], {
  time_of_day: String,
  event: String,
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
