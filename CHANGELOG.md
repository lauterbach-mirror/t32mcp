# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-06-30
### Added
- Initial release of the TRACE32 MCP server.
- Rust implementation that connects to TRACE32 via the remote API.
- Includes basic skills for standard debugging operations, with each operation provided by a PRACTICE script.
- Prebuilt executables for x86-64 Linux (`t32mcp-linux-x86_64`) and Windows
  (`t32mcp-windows-x86_64.exe`), plus the TRACE32 skill bundled as
  `skill-trace32.zip`, built in CI and attached to the release, so building
  from source is optional.
