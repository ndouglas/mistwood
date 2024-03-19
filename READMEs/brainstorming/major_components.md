## Major Components

An **ECS** is very well suited to bulk operations on tabular data; it performs excellently, can operate concurrently with little effort, can scale very well, and decouples operations from data. However, it's not very well suited to iterating through lists of functions that each might rely on accessing large parts of the global state, as this will impact concurrency; each system should be very limited in function and scope.

We might use the ECS for:
- composing complex objects out of components (e.g. mobs)
- flagging objects that need to be processed in some way (e.g. killed)
- decorating objects to add effects (e.g. enchantments, conditions like bleeding or unconsciousness)
- spatially partitioning objects (e.g. with a quadtree or octree) to determine neighbors, e.g. which creatures are in a nearby room and can hear you screaming
- updating components from effect events (e.g. modifying the health of every creature that had a health-modifying effect that tick)
- emitting events from changes to or addition/removal of components
- managing some global data that is relevant to systems (output handles, tick counter, addresses of certain specific actors, event channels, etc)
- components might hold actor addresses and systems can send them messages as appropriate
- data-oriented pub-sub, specifically things where timing and reliability are very important, e.g. taking damage, etc, or non-trivial numbers of events, publishers, or subscribers are involved

We would avoid using the ECS for:
- parsing player input in the context of the world, as the interpreter would need to be informed about essentially everything in the world to bind arguments
- processing commands broadly, as commands might reference anything in the world, and we would have so many commands that dedicating a system to each would be inappropriate overhead
- processing actions broadly, as actions might reference anything in the world, and we would have so many actions that dedicating a system to each would be inappropriate overhead
- processing effects broadly, as effects might reference anything in the world, but effects might be best handled in finely-grained systems
- generating or loading the game map
- AI algorithms, which might take a disproportionate amount of time to execute
- procedural generation algorithms, which might take a disproportionate amount of time to execute
- menus or other user interaction; don't entangle the user interface with the ECS

We might enhance the ECS with:
- Flyweight: Share data across multiple copies of the same component (descriptions, interactions, professions, breeds, races, etc)
- Command: Deferred command execution (build a queue of commands and execute them as a batch); AI algorithms controlled by actors can return commands that would be enqueued and processed by the system; automations, e.g. configuring certain actions to occur when the player loses some amount of health, etc; command queues (an object is provided a list of commands affecting it simultaneously, which is arbitrated objectively); command-based scripting, debugging, and profiling; event-triggered command generation (i.e. monsters react intelligently to threats and conditions); multi-level command processing for GOAP etc to hierarchically nest goals and the corresponding actions (e.g. a "BuildStructure" command could decompose into resource gathering, terrain modification, and construction commands, each processed by specialized systems); use commands to communicate between systems; authorization for commands (i.e. can the actor do the thing?);
- Observer: Dynamic system activation (system turns self on and off in response to receiving events it subscribes to), Rule engine dynamism (enable/disable/modify rules in response to events),
- Prototype: Prototype repository/registry for entities; dynamic prototype modification (add components (e.g. isHostile) to entities of a specific faction if faction standing is low); evolutionary prototyping (prototypes might evolve resistances to things they die to a lot); prototype inheritance/variance/selection => evolution;
- State: Stateful components
- Double Buffer: Maintain read and write copies of component data, using the former to calculate actions and the latter to execute them. Adjustments like damage should be done by calculating a change or delta value and storing that in a separate component. Calculating a future state (e.g. the weather in 4 hours) and interpolating between current state and new state to simulate sunset, sunrise, wind starting to blow, etc.
- Game Loop: Time dilation/simulation control (speed up/slow down time in certain circumstances); adaptive ECS processing (areas further from the player are updated less frequently); event-driven ECS updates (run dispatchers in response to an event, not a mere game loop cycle); predictive AI/proactive systems that look at conjectured future states and use them to inform their decision-making; procedural generation using ECS as a feedback loop (e.g. using lots of fire spells in an area might lead to fire-resistant flora and fauna and damaged scenery); dynamic difficulty adjustment based on player performance;
- Update Method: Disable systems adaptively to reduce e.g. environmental/weather functionality in high-load scenarios (e.g. intense combat).
- Type Object: Components can reference type objects.
- Event Queue: Event queuing and filtering.
- Visitor: Behavioral adjustment (events could visit entities, altering them, e.g. making friendly mobs hostile); traversing local rooms or other entities within range to set them on fire or waterlog them; use this pattern to handle complex event responses in a modular way; for multi-step events that affect various entities differently, use visitors to encapsulate the logic for each step, processing entities as needed; apply game rules dynamically to entities based on their components

The **Actor Model** is very well suited to concurrent and distributed systems by treating actors as the fundamental unit of computation, each with its own state and behavior; actors communicate through message passing, reducing dependencies between components and enhancing modularity. However, debugging and reasoning about asynchronous systems can be challenging, and there is some amount of overhead.

We might use Actors for:
- managing internal state for characters in AI
- managing internal logic for characters and other complex objects
- handling user input and parsing it into commands with appropriate references to the game world context
- action execution
- actor-oriented pub-sub, especially propagation of sensory perception, etc where slight degrees of difference in timing and reliability might actually have a positive effect
- reading/writing files or other input/output
- generating/loading the game map
- executing AI algorithms asynchronously
- executing procedural generation algorithms asynchronously
- executing background systems with sparse processing, e.g. random weather events, economic and political simulations, etc
- actors can write to event channels to communicate with the ECS

We would avoid using Actors for:
- processes involving non-trivial amounts of tabular data/components
- processes involving non-trivial numbers of objects
- processes where performance/predictability/ordering/consistency/testability is important
- reacting to changes in or addition/removal of components, generally

We might enhance Actors with:
- State: allow the Actor to alter its behavior when its internal state changes; NPCs are described differently; NPCs use different AI; chunks might be more or less dangerous; quests would observe different things, be listed differently, etc; game difficulty can vary; weapons and magical items can be usable, impaired, unusable, valueless, cursed, etc
- Proxy: decorate the Actor and intercept incoming messages to alter or nullify them before they reach the target
- Flyweight: much of Actors' functionality can/should/must be used with a flyweight approach; environmental effects like weather, appearance of the sky, etc; AI/event handling behavior such as dialogue trees, etc
- Observer: make Actors observe/subscribe to global state such as weather, appearance of a player, etc; implement quests as actors that observe events, rather than as objects that need to be called when a player does something; dynamic procedural generation informed by world events; trigger market changes based on resource availability; observe player statistics/achievements/deeds and alter other actors' behavior in response, etc, dynamic component composition (add/remove components in response to events), etc (think alert mechanic in stealth games where perceiving an event leads to a state change); location/scenery observation (e.g. areas in a torrential downpour might flood; snow might melt if you cast "fireball"); actors aware of observers (e.g. a goblin might not flee if he is being observed by a number of his clansmen);
- Prototype: make Actor prototypes for creature species/tribe/etc; Biome prototypes that can be cloned into a chunk and modified appropriately; quest prototypes for low-effort quests (e.g. postal worker quest in After the Plague II); prototypes for dialogue trees and event responses, etc; prototypes respond to curses;
- Singleton: game state manager; map generator manager; weather manager; resource manager;
- Double Buffer: NPC AI/intentions/reactions to events/etc are written to a buffer while reading from current world state and applied simultaneously; procedurally generate content offscreen and bring it onscreen; multi-threaded AI decision making with votes written to buffers and combined; dynamic world updates like weather/economics/politics/etc to apply aggregate changes rather than individual shifts; "what if I" allowing speculation/forecasting of events to occur if a certain action is taken; AI gets theory of mind
- Type Object: Breeds/races/etc can be implemented as type objects and consulted by NPC actors
- Service Locator: Allow mocking algorithms/services for tests; allow actors to query world state or game state effectively and efficiently (e.g. "what is the weather? are there weapons on the ground?"); decouple actors from their AI implementations; quest generators that do fetch quests for locally rare items; market simulations that proportion expense to scarcity; write performance monitoring data, etc
- Dirty Flag: lazy evaluation of game state changes; don't mark monsters dirty if no player is within x nodes; mark quests dirty when activated; mark AI pathfinding algorithms dirty only when path/obstacles change to avoid re-querying spatial data;
- Spatial Partitioning: load/unload/deactivate actors by chunk; effective pathfinding over large distances; event triggered when player enters chunk; localized rules/laws;
- Visitor: Implement the visitor as an actor in such a way that it sends messages to actors requesting that they execute a method on the caller and then report back. Parallelize visitor traversal by using the actor pattern. Allow visitors to use the Command pattern to reify actions, for e.g. scheduling command execution at a later date. After a magical event, visit everyone in the town, ask them for their magical resistance and hit points, calculate damage accordingly, and request that they update their health. When a dragon attacks, have a visitor ask each actor if they can see the dragon and their morale/other characteristics, and make them afraid/flee/fight the dragon as appropriate. In a battle, have a visitor assess the townspeople and assign them to posts according to their strengths. Have a visitor periodically update the dress and appearance of characters to reflect their financial resources and recent events.

**Futures, Streams, and Signals** facilitate asynchronous programming, making it easier to write non-blocking code that efficiently handles I/O operations and long-running tasks, to manage processes in a declarative, readable, and maintainable fashion, and to develop highly responsive systems that react to changes in data efficiently and in real-time. However, debugging them can be painful.

We might use Futures to:
- make Actors _appear_ synchronous by running lengthier operations in the background

We might use Streams to:
- buffer player input to enable handling command sequences efficiently while ensuring the game processes each action in order and respects the dependencies between them

We might use Signals to:
- allow subscription to actor state, e.g. updating health bars, making a mob grin when your health drops below 20%, etc
- operations on state changes, e.g. debouncing, filtering, etc

**Event-Driven Design**, specifically Publisher-Subscriber architectures (or "Pub-Sub"), decouple components from each other by allowing them to communicate via Events, but can make development and debugging more challenging and raise the risk of "event storms" where a cascade or cycle of events causes performance degradation. There are two variations that we're interested in, Data-Oriented and Actor-Oriented.

**Data-Oriented Pub-Sub** focuses on the efficient handling of data events through a channel-based system. It emphasizes the performance aspects of event handling, especially suitable for applications where the primary concern is the rapid processing of a high volume of events with minimal latency. The "data-oriented" aspect refers to the focus on the events (data) themselves and how they are stored, accessed, and processed, often optimizing for cache coherency and minimizing processing overhead.

We might use Data-Oriented Pub-Sub to:
  - create publishers and subscribers for a variety of simple, unambiguous events
  - centrally manage simple operations (respond to decrement-health event by decrementing health kept in component; emit death event if appropriate)
  - manage operations that affect large swaths of the game world (updating adjacency graphs to improve path-finding when a door is unlocked, updating quadtrees/octrees when an actor moves, notifying everything in an area to go dormant because no players are present, etc)

We would avoid using Data-Oriented Pub-Sub for:
- any long-running operations
- sparse, infrequent events
- events few would care about

**Actor-Oriented Pub-Sub** centers on actors as the primary entities for both producing and consuming events. Each actor manages its own state and handles messages (events) asynchronously, making this approach inherently suited for distributed systems, emphasizing concurrency and fault tolerance and offering sophisticated message delivery and handling.

We might use Actor-Oriented Pub-Sub to:
- load a chunk when a player approaches a new area (as this is a potentially lengthy operation)
- trigger a counterattack when a player attacks a creature
- advise an actor that they might need to revise their current goal if operating an goal-oriented action planning

We would avoid using Actor-Oriented Pub-Sub for:
- high-traffic events
- broad-scope events
- events many would care about
- trivial events
- events trivial to process

A **Rule Engine** allows us to define rules in a declarative manner, making it easier to add, modify, or remove rules without affecting the core game logic. When an event occurs, the engine iterates over its rules, checks which ones apply, and executes them in a specified order.

We might use the Rules Engine to:
- centralize creation and reference of game rules
- suggest intelligent behaviors to patrolling actors
- enforce realistic behavior of objects (e.g. when "fireball" is cast, ignite flammable objects on the floor)
- provide feature flags (e.g. bools of arbitrary specificity to enable/disable functionality)

We might enhance the Rules Engine with:
- Command: allow the Rules Engine to execute Actions directly; generate AI goals from conditions (e.g. if poisoned, seek antidote); defer/queue/cancel commands (e.g. "sleep; eat;"); adaptive commands (e.g. "flee");
- Flyweight: shared rulesets; integrate within ECS systems to keep systems light; integrate within Actor systems to allow actors to access expert system-ish functionality
- Observer: dynamically enable/disable rules and rulesets in response to events; dynamically enable/disable systems; call Rules Engine from systems as a global event observer/handler; filter events with rules (e.g. deaf creatures can't hear sounds)
- Prototype: modify clones of prototype on creation to reflect current game state (political situation, astrological signs, phase of the moon; scarcity of certain resources); modify prototypes in response to game events (e.g. give trait "fearful" after an orc attack); perhaps existing models will gain nostalgia for the state prior to the change; rule-based templating for prototypes (e.g. "creature spawned in biome gains biomeEffect => orcs living at Fire Mountain gain fire resistance");
- Singleton: rules engine as singleton
- State: make various states more accessible to rules via context (e.g. weather, factionMood, etc); change ECS system behavior based on context (e.g. WeatherSystem acts differently based on Weather state); give bonuses/penalties to rule probabilities based on state; give traits or labels ("Notorious Outlaw" or "Wearing Scary Mask") to add modifiers to base state; switch behaviors based on state (e.g. aggressive -> neutral -> defensive -> flee strategies based on health);
- Game Loop: first system on each loop evaluates game state with a ruleset to set shared resources (e.g. is the player dead? is the player currently acting?)
- Type Object: rules can inherit behavior from a "super-rule" or type rule that allows a form of inheritance or a sort of library of shared functionality; context could also be provided through a similar mechanism, so that all rules instantly have access to updated state; also useful for a double-buffer approach
- Component: map components to rules so that e.g. a FireResistant component automatically applies a corresponding rule; allow attaching arbitrary rules to a RulesComponent attached to an entity to confer traits; record events in actor history so that actors can "learn" and defend against or even preempt moves, etc; use rules to guide procedural content generation by including/excluding specific components
- Event Queue: hybrid event management (use Rules to determine which entities should be updated by ECS and which via Actor; can handle events that require both immediate, stateful responses (suitable for actors) and bulk, stateless processing (ideal for ECS)); communication bridge between different architectural components, ensuring that the ECS and Actor Model remain decoupled; prioritized event processing (a prioritization mechanism within the Event Queue to manage the execution order of events); dynamic rule application (dynamically apply rules based on the current game state or specific events, e.g. have traits or effects that alter the order in which events are processed); event driven rule activation (use events not just as triggers for actions but as conditions for rule activation within the Rules Engine, e.g. if player runs from a rat, make rats more common); a feedback loop where the outcomes of rules processing can generate new events, which in turn may trigger additional rules or actions within both the ECS and Actor Model; rules that specifically govern interactions between ECS-managed entities and actors. This can help in creating intricate gameplay mechanics where the environment (ECS) and NPCs or player characters (Actors) interact in complex ways;
- Service Locator: expose Rules Engine via Service Locator
- Dirty Flag: have rulesets be contingent upon a relevant dirty flag to prevent unnecessary reevaluations of existing data; have the dirty flag be observed and state changes trigger rule sets that evaluate the entity (e.g. isPoisoned might be flagged whenever a bug bites the entity or a flask of poison is thrown or the entity eats rotten meat; actual rule evaluation may take longer);
- Chain of Responsibility: determine priorities based on rulesets (am I dying? am I being killed? am I being attacked? am I starving? am I being threatened? am I very hungry? do I see a suspicious person? am I very tired? do I see something weird? am I tired? am I hungry? etc)
- Mediator: facilitate communication between ECS, Actors, Rules Engine, etc; embed rules engine within mediator object;
- Template Method: create a delegate with simplified implementations for most entity lifecycle events, but also implement a complex implementation that uses the Rules Engine that can be opted into (e.g. by declaring components).
- Proxy: place Proxy objects between actors and their callers and trigger the Rules Engine from the Proxy; these can be particular actors (e.g. creatures) or services or systems e.g. disabled by feature flags or by current context (player is asleep and dreaming, so ignore their in-character commands); use proxies as middlewares that alter rule inputs or outputs or introduce side effects (logging, etc); queue or ignore low-priority events for later dispatch (e.g. I'm hungry but I'm currently fighting a dragon; I hear a sound but the bed is on fire);
- Decorator: Add/remove/alter components on an entity in response to conditions or events ("poisoned" can lead to damage being done; "wet" can reduce the effect of fire attacks but also a "fire shield" spell)
