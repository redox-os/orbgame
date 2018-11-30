# OrbGame

[OrbTk](https://gitlab.redox-os.org/redox-os/orbtk.git) extensions for 2D game development. Compatible with Redox and SDL2. 

[![Build status](https://gitlab.redox-os.org/redox-os/orbgame/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/orbgame/pipelines)
[![Travis Build Status](https://travis-ci.org/redox-os/orbgame.svg?branch=master)](https://travis-ci.org/redox-os/orbgame)
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

## Examples

You find the examples in the `examples/` directory.

You can start the adventure example by executing the following command:

```text
cargo run --example adventure
```

 ## Credits
 
 * https://opengameart.org/content/zelda-like-tilesets-and-sprites
