# Using Rust to compile to Wasm

First [install Rust][install-rust].

If you're new to Rust, it's worth your time to read through the [Rust book][rust-book].  The solutions you create to play this game will generally not be too complex, but it's a good idea to have a base level of familiarity before starting.

Next we need to install the `wasm32-unknown-unknown` target which is what you'll be compiling your solutions to.

```
rustup target add wasm32-unknown-unknown
```

Now we can make our first solution.

Start the game and get the library for level 1. TODO: do this part

```shell
# We create a shell project with Cargo so that we get all the benefits of Cargo
cargo new level1-solution
cd level1-solution

# TODO: add dep and stuff
# Build
cargo build --release --target=wasm32-unknown-unknown

# Get the file
cp target/wasm32-unknown-unknown/release/level1-solution.wasm .
```

And we're done!  We have a Wasm module for our level 1 solution.  Now we can [give it to the game][loading-wasm-modules]

[install-rust]: https://www.rust-lang.org/tools/install
[rust-book]: https://doc.rust-lang.org/book/
[loading-wasm-modules]: ../loading-wasm-modules.md