# AGENTS.md

This file provides guidance for AI coding assistants working with the Nexus repository.

## Project Overview

Nexus is a workflow builder application with:
- **Frontend**: Svelte5 + Sveltekit + TypeScript + Tailwind CSS + Vite (Bun package manager)
- **Backend**: Rust + Axum + Tokio
- **Architecture**: Full-stack with REST API between frontend and backend

## Repository Structure

```
/Users/jaichang/IdeaProjects/nexus/
├── frontend/          # SvelteKit frontend (Bun)
│   ├── src/
│   │   ├── lib/      # Shared components and utilities
│   │   └── routes/   # SvelteKit routes
│   └── package.json
├── nexus-core/        # Rust backend (Axum)
│   └── src/main.rs
└── docker-compose.yml # Docker orchestration
```

## Essential Commands

### Frontend (from `frontend/` directory)

```bash
# Development
bun dev                 # Start dev server (port 5174)
bun run build           # Production build
bun run preview         # Preview production build

# Type checking and linting
bun run check           # Run svelte-check (TypeScript validation)
bun run check:watch     # Watch mode for type checking
bun run lint            # Check Prettier formatting
bun run format          # Format with Prettier

# Testing (when tests are added)
bun test <test-file>    # Run a single test file
```

### Backend (from `nexus-core/` directory)

```bash
# Development
cargo run               # Run the backend (port 3001)
cargo build             # Build the project
cargo build --release   # Release build

# Code quality
cargo fmt               # Format code
cargo clippy            # Run linter
cargo check             # Check compilation without building

# Testing (when tests are added)
cargo test              # Run all tests
cargo test <test-name>  # Run a single test
```

### Docker (from root directory)

```bash
docker-compose up       # Start both frontend and backend
docker-compose up -d    # Detached mode
```

## Code Style Guidelines

### Frontend (TypeScript/Svelte)

#### Formatting (Prettier)
- **Indentation**: Tabs (not spaces)
- **Quote style**: Single quotes
- **Trailing commas**: Never
- **Print width**: 100 characters
- **Plugins**: `prettier-plugin-svelte`, `prettier-plugin-tailwindcss`

#### TypeScript Conventions
- Use strict TypeScript mode (configured in tsconfig.json)
- Use Svelte 5 runes syntax (`$state`, `$derived`, etc.)
- Prefer `type` over `interface` for object shapes
- Use `unknown` instead of `any` when type is uncertain
- Use explicit return types on exported functions
- Use `camelCase` for variables and functions
- Use `PascalCase` for types, interfaces, and component names

#### Svelte Conventions
- Use `<script lang="ts">` for all components
- Use scoped styles within `<style>` blocks (avoid global styles)
- Prefer reactive statements (`$derived`, `$effect`) over manual subscriptions
- Use Svelte's event handling syntax (`on:click`, `on:input`)
- Keep components focused and under 200 lines when possible

#### Import Order
1. Svelte/sveltekit imports
2. Third-party library imports
3. Internal absolute imports (`$lib/...`)
4. Relative imports (siblings, parents)
5. Type-only imports last

Example:
```typescript
import { onMount } from 'svelte';
import { browser } from '$app/environment';
import type { Node } from '@xyflow/svelte';
import WorkflowNode from '$lib/flow/nodes/WorkflowNode.svelte';
import { type NodeKind } from '$lib/flow/nodes/definitions';
```

#### Error Handling
- Use try/catch for async operations
- Provide user-friendly error messages
- Use Svelte's `{#if}` blocks for conditional error display
- Log errors to console in development

### Backend (Rust)

#### Formatting
- Use `cargo fmt` (rustfmt default settings)
- 4 spaces for indentation (Rust default)
- Maximum line width: 100 characters

#### Naming Conventions
- `snake_case` for variables, functions, and modules
- `PascalCase` for types, traits, and structs
- `SCREAMING_SNAKE_CASE` for constants and statics
- `CamelCase` for lifecycle traits (Clone, Debug, etc.)

#### Code Organization
- Group imports: std, third-party, internal
- Use `mod` declarations near the top
- Implement `Clone`, `Debug`, `Serialize`, `Deserialize` for data types
- Use Axum's `extract` and `routing` patterns consistently

#### Error Handling
- Use Axum's `Result<T, StatusCode>` for HTTP handlers
- Use `Result::ok()` and `Result::map_err()` for graceful error handling
- Log errors before returning HTTP responses

Example:
```rust
async fn get_workflow(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Workflow>, StatusCode> {
    let store = state.store.read().await;
    store.workflows
        .iter()
        .find(|w| w.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
```

### CSS/Styling (Tailwind v4)

- Use Tailwind CSS v4 with Vite plugin
- Custom CSS variables in `layout.css` for theming
- Prefer Tailwind utilities for layout, spacing, colors
- Use scoped styles in Svelte components for component-specific styling
- Follow the existing CSS variable naming: `--bg-*`, `--text-*`, `--accent`, etc.

## Key Patterns

### Frontend State Management
- Use Svelte 5 runes for reactive state
- Use `$derived` for computed values
- Keep state close to where it's used (avoid global stores unless necessary)

### API Communication
- Base URL: `import.meta.env.VITE_API_BASE ?? 'http://localhost:3001'`
- Use standard `fetch()` for API calls
- Handle both success and error cases

### Backend Architecture
- Uses Axum's `Router` for routing
- State management through `Arc<RwLock<Store>>` for thread-safe in-memory storage
- CORS enabled for all origins (development)
- RESTful endpoints under `/api/*`

## Before Committing

1. **Run type checks**: `bun run check` (frontend)
2. **Run linter**: `bun run lint` (frontend) or `cargo clippy` (backend)
3. **Format code**: `bun run format` (frontend) or `cargo fmt` (backend)
4. **Test locally**: Ensure both frontend and backend run without errors

## Technology Stack Summary

| Layer | Technology | Version |
|-------|-----------|---------|
| Frontend Framework | SvelteKit | 2.50.1 |
| Frontend Language | TypeScript | 5.9.3 |
| Build Tool | Vite | 7.3.1 |
| Package Manager | Bun | latest |
| Styling | Tailwind CSS | 4.1.18 |
| Flow Diagrams | @xyflow/svelte | 1.5.0 |
| Backend Language | Rust | Edition 2024 |
| Backend Framework | Axum | 0.7 |
| Async Runtime | Tokio | 1.38 |

## References

- [SvelteKit Docs](https://svelte.dev/docs/kit)
- [Svelte 5 Runes](https://svelte.dev/docs/svelte/what-are-runes)
- [Axum Docs](https://docs.rs/axum/)
- [Tailwind CSS v4](https://tailwindcss.com/docs)
