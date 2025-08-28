# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Building and Code Generation
- `deno task build` - Full build (Rust bindings + NPM package)
- `deno task build:rs` - Generate Rust-to-TypeScript bindings only
- `deno task build:npm` - Build NPM package only
- `cargo test --no-default-features` - Generate TypeScript bindings (used internally by build:rs)

### Code Quality
- `deno fmt` - Format TypeScript/Deno code
- `cargo fmt` - Format Rust code
- `deno lint` - Lint TypeScript code
- `deno test` - Run tests for TypeScript code

### Development Setup
After cloning, run: `git config --local include.path ../.gitconfig`

## Architecture Overview

This is a hybrid Rust/TypeScript library for the Seelen UI desktop application. The architecture follows these key principles:

### Hybrid Structure
- **Rust core** (`*.rs` files) - Performance-critical logic and state management
- **TypeScript bindings** (`*.ts` files) - Frontend API and type definitions
- **Mirrored file structure** - Most modules have both `.rs` and `.ts` files

### Key Directories
- `src/state/` - Application state management (settings, themes, widgets, plugins)
- `src/system_state/` - System information access (monitors, network, user data)
- `src/handlers/` - Tauri commands and events for Rust ↔ TypeScript communication
- `src/types/` - Auto-generated TypeScript type definitions
- `gen/` - Generated files (schemas, documentation)

### Type Generation Workflow
1. Rust structs/enums are decorated with `ts-rs` attributes
2. `cargo test` generates TypeScript definitions in `src/types/`
3. Types are automatically synchronized between Rust and TypeScript
4. JSON schemas are generated for validation

### Communication Pattern
- **Commands** - TypeScript → Rust function calls (defined in `handlers/commands.rs`)
- **Events** - Rust → TypeScript notifications (defined in `handlers/events.rs`)
- **Error handling** - Custom `SeelenLibError` enum serialized across boundaries

## Development Workflow

1. When modifying Rust code, run `deno task build:rs` to regenerate TypeScript bindings
2. Always format code with both `cargo fmt` and `deno fmt`
3. The library is published to both NPM and JSR (Deno's registry)
4. Entry points: `./src/lib.ts` (main), `./src/types/mod.ts` (types), `./src/re-exports/tauri.ts` (Tauri re-exports)

## Important Notes

- TypeScript types are generated, not hand-written - modify Rust definitions instead
- The build process uses Cargo tests to trigger type generation (via ts-rs)
- Both Rust and TypeScript code should be formatted before commits
- The library serves as the foundation for Seelen UI widgets, plugins, and themes