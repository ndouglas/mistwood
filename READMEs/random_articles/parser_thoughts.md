Parser Thoughts

My parser has three stages to it.

The first is the lexical parsing stage, where I extract the words and
look them up in a dictionary to make sure they're known to the game.
I also discard useless words at this time, such as articles (i.e.
"the", "a", "an").

Next is the classify stage, where I try to assign a grammatical type
to each word (verb, noun, adjective, etc.). To do this, I use a
position-dependent algorithm, which I'll describe below.

Last is the "resolve" stage, where I try to resolve the words to
internal game objects (the verb resolves to a verb handler class and
each object to a game object class).

If everything works correctly I end-up with a Command object that
contains references to the verb and object handlers (as well as the
original sentence in case it's needed later).

The rules for the classify stage go something like this:

- the first word is always a verb

- in a simple (verb/noun) sentence, the direct object is always the
last word

- an indirect object can only exist in a prepositional phrase, which
must begin with a preposition; I have a list of prepositions and I
search words #3 through #n-1 for a match; if a prep is found, the word
directly to the left of it is the direct object and the last word in
the sentence is the indirect object

- if a preposition is found in the second position I assume that the
sentence uses the "implied self" form; e.g. in "look at picture" the
direct object is the implied "you" -- i.e. "look you at picture", or,
more properly, "you look at picture" (remember, from English class:
who is doing the looking? "you" :)

- I can also recognize an adverb in the last position; e.g. "turn
light on"

- any noun/object can be preceded by an adjective

After assigning a grammatical type to each word I check to see if I
missed any words; if I did, I assume that the sentence uses a
structure that I don't understand and issue an error message.

This logic allows the following types of sentences:

look
examine the red key
open the wooden door with the gold key
turn the oil lamp on
look at the faded picture
look behind the oil painting

I also provide special support for "and" and "then" (I admit it's been
a while since I played an Infocom game, but I think this is how they
did it):

- AND is used to join multiple direct objects together to all receive
the same action. e.g. take gold key and rock and red ball

- THEN is used to join together multiple verb clauses (sentences).
e.g. "get the green robe THEN put it on" [;)]

AND can be abbreviated with a comma and THEN can be abbreviated with a
period. e.g. "take gold key, rock. open door with key. throw rock"

I handle AND and THEN by generating multiple sentences, internally,
during the classify stage. If THEN is used, the sentence is split at
each THEN. If AND is used, the affected sentence is copied multiple
times and each copy gets one of the objects. Thus, the command
execution stage (which follows the parsing stage) is greatly
simplified: it only has to deal with one verb, one direct object, and
one indirect object.

I also support the special word "it" (as demonstrated above in the
"green robe" example). IT always refers to the direct object in the
previous verb clause. If there was no D.O. in the previous V.C., IT
cannot be resolved and an error results.

I also support the special word "all", which can be used with verbs
like take, drop, put, etc., to manipulate all objects within reach.
(However, ALL isn't resolved in the parser like everything else
mentioned above. I have a dummy object in the game called ALL which
tricks the parser into thinking a normal object has been referenced.)

After writing my earlier post today, I did some thinking and had an
epiphany, of sorts. ;) Briefly, I had been writing bunches of ugly ad
hoc code to validate prepositions and adverbs and I knew deep inside
of me that it was wrong to do it this way. After some thought, I
realized that prepositions and adverbs, although they are technically
different parts of speech, really serve the same purpose in a verb
clause: they modify the verb. Thus, I realized that I could simplify
my logic by calling them "verb modifiers" and treat them very much
like adjectives (which are essentially noun modifiers)!

Adjectives and nouns are paired for processing; i.e. if the user types
"gold key" it can only match an object that has the same noun and
adjective. (However, the game allows the adjective to be omitted if
there's only one "key" available.)

Likewise, verbs and their modifiers should be paired together as well!
Instead of matching just the verb and then writing ugly code to
verify the correctness of the modifier, a verb/modifier pair actually
constitute a unique verb!

For example, "look at picture" and "look behind picture" are both uses
of LOOK. BUT, "look at" and "look behind" are really two very
different things. Therefore, the game should have two separate verbs:
"look at" and "look behind". If verb matching is performed in this
manner, there is no further need to validate the verb modifier.

Another example: "put key in box" and "put key under rock" both use
PUT, but are really two different verbs: "put in" and "put under".

Another example (different sentence form): "turn lamp on" and "turn
lamp off" are two separate verbs: "turn on" and "turn off".

I'm sure that I'm not the first to think of this and some folks who
read this will laugh and say "No shit Sherlock! We knew that all
along!!" Well, good for you. And good for me too because now I know
it too. ;)

Further thinking through of this concept led me to convince myself
that I don't need specific verb handling classes (as I currently
have), but rather, verbs should be implemented within the object/noun
classes (with default processing supplied in the base class).
Furthermore, if the verb/modifier matching is done in the object
classes in this manner, virtually all of my vocabulary will be defined
in a distributed manner within the object classes and my dictionary
can be generated at runtime when the objects are created and
registered. This will allow me to do away with having to predefine a
vocabulary and hand-generate word IDs, etc. When I want to add a new
object to the game, I simply create a new class and specify the verbs
(and verb modifiers) that it will use right there in the new class.
This is much, much simpler than my current program architecture and it
means I have a bunch of new work to do. :)

(IIRC, TADS defines verbs as members of objects and I knew this so I
should have thought of this sooner. But, I'm writing Java code not a
3GL, so it's slightly different. I don't remember how TADS handles
verb modifiers, or if it even does.)

When I wrote earlier to Ed I mentioned that my code uses hard-coded
IDs, etc. But I now have a vision for a new architecture that will
not need anything of the sort and which can be neatly organized into a
generic reusable Java class library. And, believe it or not, the
changes required to my existing code will not be very extensive.

As I said, I have some work to do... ;)
