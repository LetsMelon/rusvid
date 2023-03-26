# Changelog for the crate `rusvid_core`

For other `CHANGELOGS.md` see in workspace folders.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Improved documentation for `plane::Plane`
- Default for `holder::likes::types_like::TypesLike` ([#64])
- Implemented prelude `rusvid_core::prelude` ([#64])
- Implemented method `from_hex_string` in `pixel::Pixel` ([#64])

### Fixed

### Changed

- `Plane::save_with_format` returns the path when successful ([#90])

### Breaking

[unreleased]: https://github.com/LetsMelon/rusvid/compare/0.2.1...HEAD

[#64]: https://github.com/LetsMelon/rusvid/pull/64
[#90]: https://github.com/LetsMelon/rusvid/pull/90
