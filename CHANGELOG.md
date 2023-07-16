<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- `sheets_to_db` script incorrectly parsing data.
- Client dates incorrectly showing day # instead of date #.
- Client dates not showing a 0 if the minutes were less than 10.

### Added

- Relevant HTML title name.
- Relevant HTML favicon.
- State input is now a dropdown menu.
- Adminer and migration to developer docker-compose.
- Makefile for easier building.

## [0.3.1] - 07.13.2023

### Fixed

- Unable to run docker-compose due to invalid syntax and environment variables.

## [0.3.0] - 07.13.2023

### Added

- Successful migration message.
- Deployment secrets.
- Tailwind dependency.
- Optimized environment variable parsing and error messaging.
- Version endpoint that shows current cargo version.

## [0.2.0] - 07.13.2023

### Fixed

- Server hanging due to mutex not being released.

### Added

- Statistics View and Endpoint.

## [0.1.0] - 07.13.2023

- Initial Release.
