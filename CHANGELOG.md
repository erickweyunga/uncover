# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2025-11-06

### Added
- **Path and query parameter support**: Routes can now extract path parameters (e.g., `/users/:id`) and query parameters (e.g., `?page=1&limit=10`)
  - New `PathParams` and `QueryParams` types with type-safe conversion methods (`get_u64()`, `get_string()`, etc.)
  - Added `path` and `query` fields to `Context` struct
  - All HTTP method handlers (GET, POST, PUT, PATCH, DELETE) now extract and provide access to parameters
- **Parameter declaration API**: Endpoints can declare query and path parameters for documentation
  - New `ParamInfo` struct for parameter metadata
  - Chainable API: `.query("name").desc("description").required()`
  - `.path_param()` method for declaring path parameters
- **OpenAPI integration for query parameters**: Query parameters now appear in OpenAPI specification
  - Parameters show in `/openapi.json` with correct schema and descriptions
  - Interactive documentation in Scalar UI displays all query parameters
  - Helper function converts parameter metadata to OpenAPI format

### Fixed
- **Port configuration synchronization**: `api_servers` now automatically updates when `bind()` is called
  - OpenAPI documentation correctly shows the actual server port instead of always showing :3000
  - Bind address and API server URLs stay in sync

### Changed
- Documentation website structure simplified following FastHTML pattern

## [0.1.1] - 2025-11-05

### Fixed
- Fixed inline documentation to use 'Uncovr' instead of 'Uncover'
- Updated all module documentation headers
- Updated AppConfig default values to use 'Uncovr'
- Fixed inline code examples in documentation

## [0.1.0] - 2025-11-04

### Initial Release
- First working version of Uncovr framework
