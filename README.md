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

struct GameView;

impl Widget for GameView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("GameView")
            .with_child(
                Container::create()
                    .as_parent_type(ParentType::Single)
                    .with_child(TextBlock::create().with_property(Label::from("OrbGame"))),
            )
    }
}

fn main() {
    let mut game = Game::default();
    game
        .create_window()
        .with_bounds(Bounds::new(0, 0, 420, 730))
        .with_title("OrbGame - minimal example")
        .with_root(GameView::create())
        .with_debug_flag(true)
        .build();
    game.run();
}
```

## Additional Examples

You find the examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example dungeon --release
```

## Build and run documenation

You can build and run the latest documentation y executing the following command:

```text
cargo doc --no-deps --open
```

 ## Credits
 
 * https://pixel-poem.itch.io/dungeon-assetpuck
