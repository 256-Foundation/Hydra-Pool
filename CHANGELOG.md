# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.14] - 2025-10-31

### Added

- Cosign docker images
- Add docker compose and config example to release
- Add docker compose instruction as primary way to run pool

### Deprecated

- Ansible templates are not being maintained. We might bring them back
  in the future.
- Enable running workflows from github tags page.
- Sign docker images with cosign

## [1.1.13] - 2025-10-30

### Added

- Add docker files for hydrapool, grafana and prometheus
- Add a docker compose file for ease of use for end users
- Add docker build work flow to build docker images on github actions

## [1.1.12] - 2025-10-29

### Fixed

- Fix write permission for writing debian package from workflow

## [1.1.11] - 2025-10-29

### Fixed

- Book keeping fix for tag

## [1.1.10] - 2025-10-29

### Fixed

- Fix debian package failure to get release tag

## [1.1.9] - 2025-10-29

### Added

- Add debian package workflow using cargo-deb

[unreleased]: https://github.com/256-foundation/Hydra-Pool/compare/v1.1.14...HEAD
[1.1.14]: https://github.com/256-foundation/Hydra-Pool/compare/v1.1.13...v1.1.4
[1.1.13]: https://github.com/256-foundation/Hydra-Pool/compare/v1.1.12...v1.1.13
[1.1.12]: https://github.com/256-foundation/Hydra-Pool/compare/v1.1.11...v1.1.12
[1.1.11]: https://github.com/256-foundation/Hydra-Pool/compare/v1.1.10...v1.1.11
[1.1.10]: https://github.com/256-foundation/Hydra-Pool/compare/v1.1.9...v1.1.10
[1.1.9]: https://github.com/256-foundation/Hydra-Pool/compare/v1.1.8...v1.1.9

