## MUD Parsing

Processing runs through, roughly, this series of steps:

Normalization of command forms, as touched on briefly above, but more exhaustively and boringly.

Checking to see if someone is being directly addressed. This happens by looking to see if any words in the command end with a comma. If any does, the bit of the command before a comma is treated as a description of someone visible, and the parser tries to match that description to someone addressable. Alternate forms of order-giving are not supported, so the parser currently supports RICK, SHAVE YOUR STUPID-LOOKING BEARD but not TELL SHERIFF RICK TO SHAVE HIS BEARD. C'est la vie. The game also implements ASK/TELL in the form ASK RICK ABOUT STUPID BEARD but not RICK, WHAT’S UP WITH YOUR BEARD, because a comma always means that a command is being issued. C'est la vie.

If no one is being addressed, the protagonist is assumed to be the one the order is being given to.

Special processing is used for debugging commands, movement commands, and anything handled by the snowflake.

If we still haven’t figured out what the verb is by this point, which is the case most of the time, we take a look at the first (remaining) word in the command (after dropping anything before a comma, which would be the name of whomever we’re addressing). In English, conveniently, commands begin with the verb, and if the verb is only one word, that’s the verb! Yay!

We check for uses of quotation marks, which indicate a literal string, usually for talking or writing; the game supports SPRAY “YOU SUCK” ON WALL, and SAY “HELLO” TO BARBARA. Quotation-mark handling is awkward and difficult and the parser currently only supports double quotes with no interior single quotes, and no nested quotes, and ABSOLUTELY NO FUNNY BUSINESS WITH QUOTES. If there is a quoted phrase, there need to be both opening and closing quotes. We construct a Phrase, an obscure descendant of Noun that contains a list of words that are treated literally by other code that knows how to handle Phrases. That Phrase is the direct object.

On the other hand, commands are sometimes made of verbs that, practically, have more than one word as part of the verb: LOOK IN as a synonym for search. Irritatingly, English, because it’s a Germanic language, doesn’t always keep the second and subsequent parts of the verb right next to the first parts. Even worse, whether a phrasal verb requires that the parts be kept together or split apart is a regional and/or age and/or class difference for many verbs: some people find TURN BLENDER OFF to be the natural formulation, whereas others prefer TURN OFF BLENDER. (Zach de la Rocha encourages you to “turn on the radio,” whereas Lisa Loeb recounts how, habitually, “I turn the radio on, I turn the radio up.”) We really want to support both. So we check to see if the first part of the verb might possibly be a phrasal verb: if the first word in the verb is LOOK, we check to see if there’s an IN or AT later in the command, and if there is, we rearrange the command so it puts the two parts of the phrasal verb together. This might produce a command that a native speaker wouldn’t think to produce naturally, be we don’t display the intermediate stages of parsing to the user anyway, so it doesn’t matter. (Side note: action-dispatch methods on Noun descendants use underscores to represent the spaces between words in a phrasal verb, and the parsing routines massage this as a late step, so Noun and descendants have a .look_in() method that’s a synonym for .search(), and so forth.)

Once we’ve pulled out any prepositions that are actually part of phrasal verbs, we locate all other prepositions in the parts of the command that we haven’t yet parsed. Then we break the command into the phrases in between the prepositions.

Then, for each of those phrases in between the propositions, we track what the preposition preceding it is, and mach each noun phrase to an object in scope if possible. The noun phrase that’s not preceded by a preposition is the direct object, the other noun phrases are indirect objects, and the preposition used indicates what their role in the sentence is. So for the phrase PUT THE BIRTHDAY CAKE ON THE BED WITH THE SHOVEL, we wind up tracking that the direct object is BiRTHDAY CAKE, the verb is PUT, and the indirect objects are {'using': shovel, 'on': bed}. Identifying the objects in scope is handled by another helper function, get_scope(), which is complex and not shown here.

If any noun phrase can’t be matched to an object in scope, the whole parsing process fails and a message along the lines of YOU CAN’T SEE ANY BIRTHDAY CAKE HERE or THE TINY SHOVEL MADE OF PINK PLASTIC IS TOO SMALL TO SUPPORT THE BIRTHDAY CAKE is printed. Currently, there is no handling of partial correction of erroneous commands, which you get for free with Inform et al.; the player has to retype the whole command on the next turn. (Fixing this is going to require adding a whole other level of abstraction to the parser.)