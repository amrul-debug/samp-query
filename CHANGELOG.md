# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of the SAMP Query protocol
- Core library with support for all query types (information, rules, clients, detailed player data)
- Comprehensive error handling
- Async API using Tokio
- Timeout and retry mechanisms
- Documentation

## [0.2.0] - 2025-05-13

### Added
- Command-line interface (CLI) for querying SA-MP servers
- RESTful API service for querying SA-MP servers
- Example applications demonstrating library usage
- Benchmarks for performance testing

### Fixed
- Fixed module visibility issues for tests and benchmarks
- Added proper annotations for unused but important code
- Improved error handling in API component
- Fixed port conflict issues in API service
- Optimized packet handling for better performance

### Changed
- Made ClientConfig accessible for tests and external usage
- Improved documentation with more detailed examples
- Enhanced error messages for better debugging
- Restructured project as a workspace with multiple components

## [0.1.0] - 2025-05-9

### Added
- Initial release of the SAMP Query library
- Support for querying server information
- Support for querying server rules
- Support for querying player lists
- Support for querying detailed player information
- Support for measuring server ping
- Support for executing RCON commands
- Comprehensive error handling
- Async API using Tokio
- Timeout and retry mechanisms
- Documentation

## Migration Guides

### Upgrading from 0.x to 1.0

When version 1.0 is released, this section will contain information about breaking changes and how to migrate from 0.x versions.

## Credits

- Original SA-MP Query Mechanism specification by the SA-MP team
- Implementation by me (Amrul Hadi)
