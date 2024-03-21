## Data Design Requirements

Here is a list of data design requirements. 1,2,3,4 are universal
to all roguelikes. 5 is required if monsters can use passive or
reactive items. 6 is required if a game has containers. 7 is
required for game designs where outside influences can alter the
timing of a creature's next turn. 8,9 are required if your game has
proactive items. 10 is required if there is a way that the timing
of proactive items can be interfered with. 11 is required for
a game with both proactive items and containers.

1) You must be able to find the monster when it is that monster's
turn to act. (turn->monster mapping)

2) You must be able to find the monster when you know only that
monster's location. (location->monster mapping)

3) You must be able to find an item when you know only that item's
location (location->item mapping).

4) Monsters cause effects that are local to the monster, which
implies that it must be possible to find and affect a location when
only the monster's ID is known. (monster->location mapping)

5) For reactive or passive items which are common in roguelikes,
Given a monster, you need to be able to find the things it's
carrying. (monster->item mapping, carrier->carried)

6) If the game has containers, it must be possible to find the
contained items given the container (item->item mapping).

7) If the time of a monster's next turn can be changed by another
monster or the player, then it must be possible to access the
monster's turn given the monster (monster->turn mapping).

8) For proactive items, you must be able to access items when
it's the turn of those items to act. (turn->item mapping)

9) If proactive items do things to or for the monster that's
carrying them you need to be able to find the monster carrying
the item given the item (Item->monster mapping).

10) If items are proactive, and outside influences can change
the time of their next turn, then it must be possible to access
the item's turn given the item (item->turn mapping).

11) If your game has proactive items and containers it must be
able to find the container given the contained item (item->item
mapping).

Degenerate (player-only) solutions to 5 or 5,7,8 are still
required if the player is the only creature that can use items.

Good games can be made on the basis of 1,2,3,4, degenerate 5.
I think Angband has 1,2,3,4, degenerate 5, 7. A full
implementation of 5 allows a quantum leap in game design.
Nethack has 1,2,3,4,5,6,7 at least, and may have 8,9,10,11:
I'd have to source-dive to know for sure.

The design problems of 8,9,10 are simplified dramatically if
you start with an "actor" type that can be either a monster or
an item. I recommend this.

So you have to pick your design requirements based on the game
you want to write, then find a good way to arrange data so that
all of these searches work efficiently and you can still deallocate
something without leaving dangling pointers to it lying around
somewhere. I don't think it's possible without having some
redundant information stored somewhere. But you get to pick
what redundant information you store and can choose to only store
things that don't cause danger of wild pointer references.

I have used hashtables several times in my game to store direct
pointers to things, and then have those things referred to
elsewhere using the hash table key rather than the pointer. The
hash table abstracts the ability to handle dangling references
without crashing, by returning an error code when a deallocated
item is searched for.

