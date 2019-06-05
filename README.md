# OrbGame

[OrbTk](https://gitlab.redox-os.org/redox-os/orbtk.git) extensions for 2D game development. Compatible with Redox and SDL2. 

[![Build status](https://gitlab.redox-os.org/redox-os/orbgame/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/orbgame/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Usage

To include orbgame in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbgame = { git = "https://gitlab.redox-os.org/redox-os/orbgame.git" }
```

However you also need to have the SDL2 libraries installed on your
system.  The best way to do this is documented [by the SDL2
crate](https://github.com/AngryLawyer/rust-sdl2#user-content-requirements).

## Minimal Example

```rust
use orbgame::prelude::*;

fn main() {
    Game::new()
        .window(|ctx| {
            Window::create()
                .title("OrbGame - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::create().text("OrbGame").build(ctx))
                .build(ctx)
        })
        .run();
}
```

## Additional Examples

You find the examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example dungeon --release
```

## Additional Examples on Web

To run the examples on a browser you have to install 

```text
cargo install -f cargo-web
```

### Run

You can start the dungeon example by executing the following command:

* Compile to [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) using Rust's native WebAssembly backend:

```text
cargo web start --target=wasm32-unknown-unknown --auto-reload --example dungeon
```

* Compile to [asm.js](https://en.wikipedia.org/wiki/Asm.js) using Emscripten:

```text
$ cargo web start --target=asmjs-unknown-emscripten --auto-reload --example dungeon
```

* Compile to WebAssembly using Emscripten:

```text
$ cargo web start --target=wasm32-unknown-emscripten --auto-reload --example dungeon
```

## Build and run documentation

You can build and run the latest documentation y executing the following command:

```text
cargo doc --no-deps --open
```

## Sub Crates

* api: additional game elements
* utils: Game helper structs and traits
* widgets: Game widget library

 ## Credits
 
 * https://pixel-poem.itch.io/dungeon-assetpuck
