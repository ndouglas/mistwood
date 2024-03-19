## Architecture

This document proposes an architecture based on the brainstorming I did in [major_components](./brainstorming/major_components.md).

### Core Ideas

I have determined that I have three core ideas around which this library will be based:

- Entity-Component-System
- Actors
- Rule Engine

Most of the other ideas I have are modifications of, connections between, or implementations of these core ideas.

#### Entity-Component-System

The game loop is built around an Entity-Component-System model, or **ECS**. An ECS is very well suited to bulk operations on tabular data; it performs excellently, can operate concurrently with little effort, can scale very well, and decouples operations from data. However, it's not very well suited to iterating through lists of functions that each might rely on accessing large parts of the global state, as this will impact concurrency; each system should be very limited in function and scope.

#### Actors

Complex, asynchronous, and long-running processes are primarily implemented through **Actors**, as used in the Actor Model. The Actor Model is very well suited to concurrent and distributed systems by treating actors as the fundamental unit of computation, each with its own state and behavior; actors communicate through asynchronous message passing, reducing dependencies between components and enhancing modularity. However, debugging and reasoning about asynchronous systems can be challenging, and there is some amount of overhead.

#### Rule Engine

A **Rule Engine** allows us to define rules in a declarative manner, making it easier to add, modify, or remove rules without affecting the core game logic. When an event occurs, the engine iterates over its rules, checks which ones apply, and executes them in a specified order. In [a recent experiment](https://github.com/ndouglas/pebbleton/), I played with serializing and deserializing rules from YAML, but I found that this is likely to be of limited value; I'm the only person playing with this system, I'm the only person likely to use these rules, and there's no way I'm going to be doing it in YAML. So I'll just implement these in code, or in Rust macros.

### Enhancements

Of course, there are infinite ways to extend the behavior of the core systems, but I've identified a few that I believe will be of particular importance:

- Streams and Signals
- Data-Oriented Pub-Sub
- Actor-Oriented Pub-Sub
- Good Old-Fashioned AI
- Procedural Generation
- Embedded Scripting

#### Streams and Signals

These facilitate asynchronous programming, making it easier to write non-blocking code that efficiently handles I/O operations and long-running tasks, to manage processes in a declarative, readable, and maintainable fashion, and to develop highly responsive systems that react to changes in data efficiently and in real-time. However, debugging them can be painful.

**Streams** allow unspooling a series of values, such as a list, over time. I think this is the best way to handle certain buffering operations, for instance for handling input from the user that includes a sequence of commands.

**Signals** allow efficient monitoring of changes in state over time. I think this is a clean and elegant way of bubbling up changes in state through layers of indirection and allowing efficient real-time reactions to changes in data.

#### Data-Oriented Pub-Sub

**Data-Oriented Pub-Sub** focuses on the efficient handling of data events through a channel-based system. It emphasizes the performance aspects of event handling, especially suitable for applications where the primary concern is the rapid processing of a high volume of events with minimal latency. The "data-oriented" aspect refers to the focus on the events (data) themselves and how they are stored, accessed, and processed, often optimizing for cache coherency and minimizing processing overhead.

#### Actor-Oriented Pub-Sub

**Actor-Oriented Pub-Sub** centers on actors as the primary entities for both producing and consuming events. Each actor manages its own state and handles messages (events) asynchronously, making this approach inherently suited for distributed systems, emphasizing concurrency and fault tolerance and offering sophisticated message delivery and handling.

#### Good Old-Fashioned AI

**Good Old-Fashioned AI** (or GOFAI) is a term I've adopted to refer to, basically, rule-based artificial intelligence, such as A* search algorithms, alpha-beta pruning, Dijkstra's algorithm, expert systems, etc, as opposed to more "modern" techniques and models such as neural networks, LLMs, etc. There's some messiness with the definition, admittedly. I want to use AI techniques broadly, as a way to humanize characters, to make monsters more interesting, to simulate economic markets and political theatre, etc. The applications are almost limitless, and I intend to use as many different approaches in as many different ways as possible.

#### Procedural Generation

**Procedural Generation** is the art and science of generating content with algorithms. It is the lifeblood of any roguelike. It increases playability by providing a self-renewing source of novelty. It can add unpredictability and challenge, even a sense of mystery. It is also very challenging to program, as it tends to be technically demanding and our brains are very good at perceiving and identifying patterns.

#### Embedded Scripting

**Embedded Scripting** refers to implanting a programming language into a program in such a way that the program can be controlled not just through normal interactive means (buttons, cursor movements, etc) but programmatically. This can ease debugging and development massively, but it can also expand content by making quests and storylines scriptable and able to be modified and expanded without messing with core game logic.

### Conclusions

Although this may be expanded at any point, I think the above list of concepts will be the scaffolding around which this library is built.
