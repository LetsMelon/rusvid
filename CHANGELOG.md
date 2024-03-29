# Changelog for the crate `rusvid_lib`

For other `CHANGELOGS.md` see in workspace folders.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `EmbeddedRenderer` to create a `mp4`-video which needs ffmpeg installed and findable via path, for more info see the `Dockerfile` ([#64])
- Added `FrameRenderer` to store the individual frames on the disk ([#90])
- Added `RemoteRenderer` to render composition on remote server ([#94])
- Added feature `save_load` to save and load an composition from disk ([#94])
- Implemented animation `ChangeColorAnimation` ([#64])
- Implemented animation `PositionAnimation` ([#64])
- Implemented animation `SetColorAnimation` ([#64])
- Implemented enum `AnimationType` to hold the animations ([#64])
- Implemented features `serde`, `serialize` and `deserialize` to enable `serde` for all structs and enums ([#94])
- Implemented unsafe features `Send` and `Sync` for `Composition` ([#94])

### Fixed

### Changed

- Move examples into `/rusvid_lib`
- Use embedded renderer in `rusvid_lib/examples`
- Move `Layer` struct into separate file ([#64])
- Updated `chrono` to `0.24.0` ([#82]).
- Updated `geo` to `0.24.0` ([#80]).
- Updated `paste` to `1.0.12` ([#77]).
- Updated `rayon` to `1.7.0` ([#78]).
- Updated `thiserror` to `1.0.39` ([#79]).

### Breaking

- Switch from resvg to self made svg holder (`rusvid_core/holder/`) ([#64])
- Remove `resvg` from re-export
- Replaced `utils/rgb_from_hex` and `utils/color_from_hex` with `rusvid_core::pixel::Pixel::from_hex_string` ([#64])

[unreleased]: https://github.com/LetsMelon/rusvid/compare/0.2.1...HEAD

[#64]: https://github.com/LetsMelon/rusvid/pull/64
[#77]: https://github.com/LetsMelon/rusvid/pull/77
[#78]: https://github.com/LetsMelon/rusvid/pull/78
[#79]: https://github.com/LetsMelon/rusvid/pull/79
[#80]: https://github.com/LetsMelon/rusvid/pull/80
[#82]: https://github.com/LetsMelon/rusvid/pull/82
[#90]: https://github.com/LetsMelon/rusvid/pull/90
[#94]: https://github.com/LetsMelon/rusvid/pull/94
