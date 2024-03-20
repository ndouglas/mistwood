## Messages

As this is a text-based game, **Messages** are how information is conveyed to the player. This document is concerned with establishing a basic concept of a Message and its components and the systems with which it interacts so that I can implement it quickly and efficiently.

As with everything else, my goals are:

- power
- flexibility
- performance

### Content

The raw content of the Message is a string. Ideally, this is generic over static strings (&'static str), strings with a specific lifetime (&'a str), and owned String objects.

This string is presumably going to be interpreted as a template string for a system like Mustache or Handlebars. Consequently, we will need to pass some JSON-compatible object along with each string.

### Data

This is the data passed along with the raw content, which will be substituted into the string by some system like Mustache or Handlebars (I'm leaning Handlebars). This should be completely JSON-friendly.

### Gravity

The Gravity of the message is a reference to an enum variant that indicates the general seriousness or severity of the situation, which is somewhat akin to a loglevel.

- `Info`: Normal information. Room descriptions, results of successful operations ("you pick up the rock"), etc.
- `Notice`: Basic updates or status notifications with no immediate negative consequences, or something isn't quite right. "You're getting hungry." "It's getting dark." "You don't see any rocks."
- `Warning`: Indications of potential negative outcomes or the need for caution. These are not immediately dangerous but could escalate if ignored. E.g., "You are very hungry," "The path ahead looks treacherous."
- `Alert`: Situations that require immediate attention but aren't yet life-threatening. These alerts signify that the player should take action to avoid harm. E.g., "You are starving," "You are bleeding."
- `Critical`: Severe situations that pose an immediate threat to the player's progress, health, or survival. These messages indicate conditions that need urgent action to prevent catastrophic outcomes. E.g., "You are bleeding badly," "You are on the brink of starvation."
- `Fatal`: Conditions that are irreversible or have led to a game-ending scenario. These messages might be used sparingly, such as in permadeath situations or critical mission failures. E.g., "You have bled out," "You have succumbed to starvation." The jig is up.

It is expected that these conditions will imply some formatting of their own (for instance, Fatal might be bold maroon red, critical bright red, alert red, warning yellow, notice and info white). In all cases, formatting is handled as a stack, i.e. any formatting that occurs within the string is added, and when no longer needed the previous state is restored. Formatting is not at any time "reset" to the global default.

### Metadata

Some additional information should be packaged with each message (possibly switched off for release?):
- timestamp of creation
- (subject to expansion)
