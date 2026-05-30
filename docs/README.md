# Last.fm CLI Documentation

This directory contains comprehensive documentation for the Last.fm CLI tool.

## Documentation Overview

### User Documentation

- **[CLI_SHOWCASE.md](CLI_SHOWCASE.md)** - Complete showcase of all 32 CLI commands with real examples and outputs
- **[CLI_USER_MANUAL.md](CLI_USER_MANUAL.md)** - Detailed user manual with installation, configuration, and usage instructions

### Technical Documentation

- **[CLI_ARCHITECTURE.md](CLI_ARCHITECTURE.md)** - Technical architecture documentation following trait-based design patterns
- **[REFACTORING_SUMMARY.md](REFACTORING_SUMMARY.md)** - Summary of the major refactoring from class-based to trait-based architecture
- **[REFACTORING_PLAN.md](REFACTORING_PLAN.md)** - Original refactoring plan and implementation strategy

### Implementation Details

- **[AUTHENTICATION_PLAN.md](AUTHENTICATION_PLAN.md)** - Authentication system design and implementation details
- **[TESTING_SUMMARY.md](TESTING_SUMMARY.md)** - Testing strategy and coverage information
- **[DEPLOYMENT_STATUS.md](DEPLOYMENT_STATUS.md)** - Deployment configuration and status

## Quick Links

- [Main README](../README.md) - Project overview and getting started
- [Self-Hosting Guide](../README.md#self-hosting) - Deploy your own Cloudflare Worker
- [CLI Tool Binary](../target/release/lastfm-cli) - Compiled CLI executable (after building)

## Command Categories

The CLI provides 32 commands across 10 categories:

1. **Authentication** (3 commands) - Login, logout, and status management
2. **Personal/My** (5 commands) - Quick access to your own Last.fm data
3. **Artist** (5 commands) - Artist information, similar artists, top tracks/albums
4. **Album** (2 commands) - Album information and search
5. **Track** (3 commands) - Track information, similar tracks, search
6. **Chart** (3 commands) - Global charts for artists, tracks, and tags
7. **Geographic** (2 commands) - Country-specific top artists and tracks
8. **Tag** (5 commands) - Genre/tag information and top content
9. **User** (3 commands) - User profiles and listening history
10. **Library** (1 command) - User library exploration

For detailed examples of each command, see [CLI_SHOWCASE.md](CLI_SHOWCASE.md).