# rusvid

![Crates.io Version](https://img.shields.io/crates/v/rusvid_lib)
![docs.rs](https://img.shields.io/docsrs/rusvid_lib)
![Lines of code](https://img.shields.io/tokei/lines/github/LetsMelon/rusvid)
![Crates.io Downloads](https://img.shields.io/crates/d/rusvid_lib)

Write and render svg-animations with Rust ✨

(no gui or cli, under active development)

## Dependencies

- Ffmpeg in path
- rustc 1.63.0-nightly

## Examples

Look into `/examples` folder an run with `cargo run --package $NAME` (e.g `cargo run --package simple_animation` to run the simple animation example)

It is recommended to always run the program in release-mode.

## List of all rusvid crates

| Name | Description |
|---|---|
| rusvid_lib | Re-exports all crates and clue them together.<br><br>![rusvid_lib Crates.io Version](https://img.shields.io/crates/v/rusvid_lib) |
| rusvid_core | Core library for `rusvid_lib` with common structs and types.<br><br>![rusvid_core Crates.io Version](https://img.shields.io/crates/v/rusvid_core) |
| rusvid_effect | Can apply an effect on a `rusvid_core::plane::Plane` and exports some predefined effects.<br><br>![rusvid_effect Crates.io Version](https://img.shields.io/crates/v/rusvid_effect) |
| rusvid_video_encoder | To create a `mp4`-video out of `rusvid_core::plane::Plane`'s.<br><br>![rusvid_video_encoder Crates.io Version](https://img.shields.io/crates/v/rusvid_video_encoder) |
