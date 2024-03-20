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

#### "Special" Files and Directories in Modules

I view everything in alphabetical order, so I've found that it's useful to prefix certain things with an underscore (`_`) to ensure that they are sorted at the top of the list. This tends to be things that are useful to all of the members of a particular module. These are:

- `_constants`: I use constants mostly for conversion factors and things like that. I prefer to keep them together and in a separate context from the code that references them, even if they are only referenced in a single location. Some constants are used in multiple places, and are IMHO best situated in a single location independent of any individual use, and so I err on the side of conformity across constants.
- `_data`: Sometimes functionality may utilize data that is imported into the binary during the compilation process. This is distinct from data that might be used at runtime, and is generally pretty uncommon. This tends to be a directory rather than a file.
- `_error`: I've recently been mostly using `anyhow::Error` for error handling, especially because of the development and maintenance cost of allowing conversion between different error kinds at module boundaries (even with an excellent crate like `thiserror`). That said, I expect to use this occasionally.
- `_macros`: Rust macros, since they need to be declared in a particular way (`#[export_macro]`) and imported in a specific order to be useful.
- `_test_data`: Tests may read or write raw data, which I try to keep local to the files containing the tests. This tends to be a `.gitignore`d directory rather than a file.
- `_traits`: I keep traits with the constructs that target them, rather than the constructs that implement them. I think this makes interface design and segregation cleaner and easier. These will generally be separate files in a directory.
- `_types`: This is for type aliases, newtypes, unit structs, etc. Especially in scientific concepts, where I might use 138,000 of something, I tend to use newtypes so that I get some degree of compile-time checking to ensure that my units are being handled and converted correctly.

One of the things that these have in common is that they don't end up having unit tests (although in some cases I might write unit tests for a trait's default implementation of some function).

#### Example Directory Structure

Here's an example directory. Note that I'm no longer using the `mod.rs` file convention, as that's apparently no longer recommended.

```yaml
widget: Some directory.
  _traits/: Trait definitions.
    flappable.rs: The "Flappable" trait.
    slappable.rs: The "Slappable" trait.
  _constants.rs: Constant definitions.
  _macros.rs: Macro definitions.
  _traits.rs: Import traits.
  _types.rs: Type definitions.
  bar.rs: Definition of the Bar enum which uses some or all of the above resources.
  baz.rs: Definition of the `baz::write()`, `baz::format()`, and `baz::read()` functions.
  foo.rs: Definition of the Foo struct.
```

#### Macros

I really like macros, and will probably use them a tremendous amount because of how much they're able to compress detail vertically. I understand that they also consequently obfuscate and conceal. I'm hoping that the difficulty caused by this will be reduced by scoping them as narrowly as possible and using them (mostly) in a few select spaces.

### Conclusion

Other than the departures noted above, I try to write idiomatic Rust code.

If I do something weird:
- It might be because I'm an idiot; please feel free to open a ticket and point it out.
- I should comment it if I'm doing it knowingly, and if it's not commented, see above.
