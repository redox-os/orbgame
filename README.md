# orbgame
The Orbital 2D Game Engine. Compatible with Redox and SDL2. 

[![Travis Build Status](https://travis-ci.org/redox-os/orbgame.svg?branch=master)](https://travis-ci.org/redox-os/orbgame)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Screenshot

![orbgame](https://github.com/FloVanGH/assets/blob/master/orbgame-screenshot.png)

## Usage

To include orbgame in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbgame = { git = "https://github.com/redox-os/orbgame.git" }
```

However you also need to have the SDL2 libraries installed on your
system.  The best way to do this is documented [by the SDL2
crate](https://github.com/AngryLawyer/rust-sdl2#user-content-requirements).

## Examples

You find the examples in the `examples/` directory.

You can start the adventure example by executing the following command:

```text
cargo run --example adventure
```

## The game file

A game and its parts like scene, map and entities are described in [ron (rust object notation)](https://github.com/ron-rs/ron).

**Example**
```text
Game (
    title: "MyGame",
    width: 800,
    height: 600,
    target_fps: 60,
    theme: "theme.css",
    ...
)
```

## Scripting

orbgame provides the [rhai](https://github.com/jonathandturner/rhai) scripting language. You could use it to write the game logic and the behavior of entities.

**Example**
```text
player.mov(1, -1);
```

## How to create and run a game

Create a game file e.g. 'mygame.ron' and run it with:

```text
orbgame ../path/to/game/file/mygame.ron
```
 
 ## Credits
 
 * https://opengameart.org/content/zelda-like-tilesets-and-sprites
