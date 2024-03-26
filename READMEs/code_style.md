## Code Style

This document is intended to serve as an explanation of my own style, both to others (who might come across this project and wonder WTF is this guy doing), and myself, to remind myself WTF I'm doing and WTF I should be doing.

Generally, this project follows the [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/index.html). Where this project differs from defaults, this is enforced by `cargo fmt` and [`rustfmt.toml`](../rustfmt.toml).

That shouldn't be controversial, but there are a few explanatory notes I'd like to add about other conventions I've adopted.

### Weird Stuff

#### Global `macro_use`

If a crate comes with macros that I expect to use throughout my project, I'll use the `#[macro_use] extern crate <crate>` pattern to make macro globally accessible.

#### Short, specific `.rs` files

I hate scrolling in a file and trying to find something. Consequently, I tend to break things up more than I see in most Rust code. A struct or enum and its impl will generally be in a single file along with its tests, but no more than that. A trait, even a very small one, will get its own file. I'll normally list constants together in a single file in an appropriate location. When in doubt, I err on the side of putting things in separate files.

#### Deep Directory Hierarchies

I've heard that implementers prefer nested hierarchies while users prefer flat hierarchies. I _definitely_ prefer _deeply_ nested hierarchies. That said, I also try to name (or at least use-as) things in such a way that they can be easily merged into a flatter namespace.

#### Example Directory Structure

Here's an example directory. Note that I'm no longer using the `mod.rs` file convention, as that's apparently no longer recommended.

```yaml
widget: Some directory.
  traits/: Trait definitions.
    flappable.rs: The "Flappable" trait.
    slappable.rs: The "Slappable" trait.
  bar.rs: Definition of the Bar enum which uses some or all of the above resources.
  baz.rs: Definition of the `baz::write()`, `baz::format()`, and `baz::read()` functions.
  constants.rs: Constant definitions.
  foo.rs: Definition of the Foo struct.
  macros.rs: Macro definitions.
  traits.rs: Import traits.
  types.rs: Type definitions.
```

#### Macros

I really like macros, and will probably use them a tremendous amount because of how much they're able to compress detail vertically. I understand that they also consequently obfuscate and conceal. I'm hoping that the difficulty caused by this will be reduced by scoping them as narrowly as possible and using them (mostly) in a few select spaces.

### Conclusion

Other than the departures noted above, I try to write idiomatic Rust code.

If I do something weird:
- It might be because I'm an idiot; please feel free to open a ticket and point it out.
- I should comment it if I'm doing it knowingly, and if it's not commented, see above.
