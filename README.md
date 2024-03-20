![CI](https://github.com/ndouglas/mistwood/actions/workflows/continuous_integration.yml/badge.svg?branch=main)
[![codecov](https://codecov.io/gh/ndouglas/mistwood/graph/badge.svg?token=ZO8NPEDOTB)](https://codecov.io/gh/ndouglas/mistwood)

# Mistwood

[Project](https://github.com/users/ndouglas/projects/3)
[Brainstorming](https://github.com/ndouglas/mistwood/tree/main/READMEs/brainstorming)
[Architecture](https://github.com/ndouglas/mistwood/blob/main/READMEs/architecture.md)
[Component Catalog](https://github.com/ndouglas/mistwood/blob/main/READMEs/component_catalog.md)
[Documentation](https://ndouglas.github.io/mistwood/)

A framework for roguelikes, MUDs, and similar games combining the Actor model (courtesy of [Actix](http://actix.rs)), an Entity-Component-System architecture (courtesy of [Specs](https://specs.amethyst.rs/)), and various other technologies.

I've been using Specs and other ECS systems with roguelike development with some success, but I never managed to deal with some architectural issues come with that approach. The tremendous complexity that comes with roguelike development is, arguably, the challenge and the fun of the genre, but it seemed to spiral out of control for me when I applied an ECS approach.

While working on another project, [Downdelving](https://github.com/downdelving), I started working with an Actor model for the first time. I immediately appreciated how it decoupled a lot of the things that had caused me frustration with an ECS approach, and seemed to alleviate the antipatterns and performance issues.

I had an idea for combining the two approaches into a single architecture.

At the same time, there are a few other concepts that I think are generally useful for developing this sort of hyper-complicated game:
- Rules Engines
- Data-Driven Design
- Data-Oriented and Actor-Oriented Pub-Sub
- Test-Driven Design
- Futures, Streams, and Signals/Functional Reactive Programming
- Embedded Scripting
- Old-Fashioned AI algorithms (Pathfinding, Goal-Oriented Action Programming, Behavior Trees, etc)
- Procedural Generation algorithms (Wave Function Collapse, Drunkard's Walk, basically a Greatest Hits)

My goal is to make the following clean and easy to implement:
- Combat
- Magic
- Quests
- Intelligent NPCs
- Weather
- Stealth mechanics
- Complex NPC group behavior
- _et cetera_

I have several roguelike and MUD projects that I've never managed to get very far with due to architectural concerns, so my hope is that I can come up with a pleasing combination of these ideas and then get a little further 🙂

I'm starting to think I actually like mucking around with architectural concepts more than I actually like making games, so eh. Wish me luck!

![Code Coverage Graph](https://codecov.io/gh/ndouglas/mistwood/graphs/icicle.svg?token=ZO8NPEDOTB)