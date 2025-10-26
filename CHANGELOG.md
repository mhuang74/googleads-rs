# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.12.1] - 2025-10-26

### Added
- Support for campaign.asset_automation_settings field
- Support for asset_group.final_urls field
- Support for asset_group.final_mobile_urls field
- Support for campaign.start_date field
- Add CHANGELOG file based on git tags

## [0.12.0] - 2025-10-18

### Changed
- Migration to Google Ads API v22

## [0.11.2] - 2025-10-17

### Changed
- Update tonic requirement from 0.8.0 to 0.14.2 (#42)

### Added
- Add CodeCov badge

## [0.11.1] - 2025-10-17

### Changed
- Remove path for descriptor since it's unused and breaks downstream build
- Move to futures 0.3.31 to resolve warning
- Update build-print requirement from 0.1 to 1.0
- Update prost-build requirement from 0.11 to 0.14
- Update tower requirement from 0.4 to 0.5

### Added
- Add integration tests for gRPC client and protobuf generation

### Fixed
- Code formatting improvements (cargo fmt)

## [0.11.0] - 2025-10-13

### Changed
- Migrate to Google Ads API V21 (#39)

### Added
- Implement get() match arms for all metrics and most attributes in v19 (#37)

### Fixed
- Fix missing gRPC client generation by switching back to tonic_build

## [0.10.0] - 2025-03-01

### Changed
- Upgrade to Google Ads API V19
- Update README

## [0.9.2] - 2025-02-09

### Fixed
- Fix build issues on Windows (#33)

## [0.9.1] - 2025-02-07

### Added
- Support additional ad_group_ad and change_event columns (#32)

## [0.9.0] - 2024-11-21

### Changed
- Migrate to Google Ads API v18 (#29)

## [0.8.0] - 2024-08-10

### Changed
- Upgrade to Google Ads API v17.1 (#27)
- Update README with correct Google Ads version

## [0.7.0] - 2024-04-14

### Changed
- Migrate to Google Ads API v16
- Fix proto errors

## [0.6.0] - 2023-11-25

### Changed
- Migrate to Google Ads API v15

### Added
- Add Google Ads API version information

## [0.5.1] - 2023-09-12

### Added
- Support Google Labels (#17)
- Add credits to README

### Fixed
- Fix GitHub action badge

### Changed
- Remove old obsolete GitHub workflows
- Update README

## [0.5.0] - 2023-08-13

### Added
- Initial tagged release with Google Ads API support

[0.12.1]: https://github.com/mhuang74/googleads-rs/compare/v0.12.0...v0.12.1
[0.12.0]: https://github.com/mhuang74/googleads-rs/compare/v0.11.2...v0.12.0
[0.11.2]: https://github.com/mhuang74/googleads-rs/compare/v0.11.1...v0.11.2
[0.11.1]: https://github.com/mhuang74/googleads-rs/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/mhuang74/googleads-rs/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/mhuang74/googleads-rs/compare/v0.9.2...v0.10.0
[0.9.2]: https://github.com/mhuang74/googleads-rs/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/mhuang74/googleads-rs/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/mhuang74/googleads-rs/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/mhuang74/googleads-rs/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/mhuang74/googleads-rs/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/mhuang74/googleads-rs/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/mhuang74/googleads-rs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/mhuang74/googleads-rs/releases/tag/v0.5.0
