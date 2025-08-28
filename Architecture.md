# Architecture

This document outlines the architecture of the **Seelen UI Library** (`slu-lib`), providing a high-level overview of its design and core concepts.

## Core Concepts

### Hybrid Architecture

The library is built on a hybrid architecture that combines a **Rust core** with **TypeScript/Deno bindings**. This approach leverages Rust's performance and safety for core logic, while providing a flexible and easy-to-use API for frontend development with TypeScript.

### Mirrored File Structure

The project follows a **mirrored file structure**. For many modules, there is a Rust file (`.rs`) that implements the core logic and a corresponding TypeScript file (`.ts`) that provides the bindings and frontend-facing API. For example, `src/state/theme.rs` contains the Rust implementation for themes, while `src/state/theme.ts` contains the TypeScript bindings.

This approach keeps the codebase organized and makes it easy to navigate between the backend and frontend implementations of a specific feature.

## Directory Structure

Here is a high-level overview of the key directories:

-   `src/`: Contains the core source code of the library.
    -   `handlers/`: Manages Tauri commands and events, facilitating communication between the Rust core and the TypeScript frontend.
    -   `state/`: Contains the definitions and logic for managing the application's state, such as settings, themes, widgets, and plugins.
    -   `system_state/`: Provides access to system-level information, like monitor details, network status, and user information.
    -   `types/`: Contains the generated TypeScript types.
    -   `utils/`: Holds utility functions and helpers used across the library.
-   `gen/`: Stores auto-generated files, including JSON schemas and TypeScript type definitions.
-   `scripts/`: Contains build and utility scripts for tasks like generating bindings and packaging the library.
-   `mocks/`: Includes mock data for testing and development.

## State Management

The application state is primarily managed by the Rust core. The state is defined using Rust structs and enums, which are then serialized and sent to the TypeScript frontend when requested. The frontend can modify the state by invoking commands, and it can subscribe to events to receive updates when the state changes.

## Communication: Tauri Commands and Events

The communication between the TypeScript frontend and the Rust backend is handled through Tauri's asynchronous IPC mechanism.

-   **Commands:** The frontend can invoke commands to request data from or send data to the Rust core. Commands are defined in `src/handlers/commands.rs` and exposed to the frontend via `src/handlers/commands.ts`.
-   **Events:** The Rust core can emit events to notify the frontend of state changes or other significant occurrences. The frontend can subscribe to these events to react accordingly. Events are defined in `src/handlers/events.rs` and exposed in `src/handlers/events.ts`.

## Type Generation

To ensure type safety and a seamless developer experience, TypeScript types are automatically generated from the Rust code. The `ts-rs` crate is used to generate TypeScript definitions from Rust structs and enums. These generated types are stored in the `src/types/` directory and are used throughout the TypeScript codebase.

This process guarantees that the frontend and backend data models are always in sync.

## Error Handling

Errors in the Rust core are handled using a custom `SeelenLibError` enum. When a command results in an error, the error is serialized and sent to the frontend, where it can be caught and handled appropriately. This ensures that errors are not lost and can be properly managed in the UI.
