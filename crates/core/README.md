# Rusvid_core sub-crate of [rusvid_lib](https://crates.io/crates/rusvid_lib)

![Crates.io Version](https://img.shields.io/crates/v/rusvid_core)
![docs.rs](https://img.shields.io/docsrs/rusvid_core)
![Lines of code](https://img.shields.io/tokei/lines/github/LetsMelon/rusvid)
![Crates.io Downloads](https://img.shields.io/crates/d/rusvid_core)

Core library for [rusvid_lib](https://crates.io/crates/rusvid_lib) to write and render svg-animations with Rust ✨

## Functionality

- `Plane` to store pixel data
- `Point` to store 2d coordinates, implements mathematical traits
- `Pixel` to represent a pixel ('Color')
- `holder/` to represent paths and render them as a `Plane`, implements various transformations

## Dependencies

- rustc 1.63.0-nightly
