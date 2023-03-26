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

## List of all rusvid crates

| Name | Description |
|---|---|
| rusvid_lib | Re-exports all crates and clues them together. 🪡<br><br>![rusvid_lib Crates.io Version](https://img.shields.io/crates/v/rusvid_lib) |
| rusvid_core | Core library for `rusvid_lib` with common structs and types.<br><br>![rusvid_core Crates.io Version](https://img.shields.io/crates/v/rusvid_core) |
| rusvid_effect | Can apply an effect on a `rusvid_core::plane::Plane` and exports some predefined effects.<br><br>![rusvid_effect Crates.io Version](https://img.shields.io/crates/v/rusvid_effect) |
| rusvid_video_encoder | To create a `mp4`-video out of `rusvid_core::plane::Plane`'s.<br><br>![rusvid_video_encoder Crates.io Version](https://img.shields.io/crates/v/rusvid_video_encoder) |
