## Idea Dump

a dictionary of seen entities that can be parsed.

I feel like I need a bnf grammar…

The game itself literally changes based on where you are.

Lighting system -- include a light rod and allow it to be kept in "utility" or carry slot, award in a newbie quest.

wtf is kimberlite?  don't assume we know what it is unless we're dwarves

actions should probably be handled by separate systems?  IDK.

look at me

make sure "name" allows adjectives, etc.

IsConcealed component (not visible in room description, not caught by "all". Used by the player object so it's not listed in descriptions.)

encaser component (EncasesEntities)

random trait generation (short, fat, speckled, etc) but this should actually be derived from physical reality of the character.  This physical reality can be described with components.  AppearsSpeckled, or HasSkin(..), etc.  A pock-marked, greasy-haired goblin.

Handle objects that can be in multiple locations.

Passages that block entry.

Create an itObject that refers to the last object specified directly.

Extradiegetic verbs:
- Help
- About
- Brief
- Verbose
- Commands
- Credits
- Debug
- Quit
- Transcript
- Pause
- Resume
- Speed (?)

Appraise [entity|object]

Game-of-Life rules for spreading plants through a biome

HasPassages should calculate connections and store them in a variable.  That should be recalculated when a change is made.

Lifelong Planning A* to share segments of paths between multiple agents.  But we should be sure to not share costs between different movement rates, etc.

D* pathfinding for pathfinding across areas with changing sections.

Online Help system

Suggestions to the player (a Suggestion message type)
