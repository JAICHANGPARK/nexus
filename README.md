# Nexus

Nexus is a modern workflow builder application designed for agentic automation. It features a full-stack architecture with a reactive frontend for designing complex workflows and a robust Rust-based backend for high-performance execution and state management.

## Overview

The project aims to provide a seamless interface for building, managing, and executing agent-based workflows. By leveraging Svelte 5's fine-grained reactivity and Rust's safety and speed, Nexus offers a powerful platform for orchestrating LLM-driven tasks and system integrations.

## Key Technologies

### Frontend
- **Svelte 5 (SvelteKit):** Utilizes the latest Svelte features, including Runes for state management.
- **TypeScript:** Ensures type safety across the UI layer.
- **Tailwind CSS v4:** Provides a modern, utility-first styling framework.
- **Vite:** High-speed development server and bundler.
- **Bun:** Used as the JavaScript runtime and package manager.

### Backend
- **Rust (Edition 2024):** The core engine built for performance and reliability.
- **Axum:** A modular web framework for building asynchronous APIs.
- **Tokio:** The industry-standard async runtime for Rust.
- **Serde:** Framework for serializing and deserializing Rust data structures.

### Infrastructure
- **Docker Compose:** Orchestrates the full-stack environment for development and deployment.

## Project Structure

- `frontend/`: The SvelteKit application for the workflow designer interface.
- `nexus-core/`: The Rust backend service responsible for execution logic and API handling.
- `ref/`: Reference implementations and documentation from industry-standard workflow engines (Dify, n8n) used for architectural alignment.
- `docker-compose.yml`: Configuration for running the entire stack in containers.

## Getting Started

### Prerequisites

To develop or run Nexus locally, ensure you have the following installed:
- **Rust:** Latest stable toolchain.
- **Bun:** For frontend dependency management and execution.
- **Docker & Docker Compose:** Required for containerized execution.

### Installation and Running

#### Using Docker (Recommended)
The easiest way to start the full stack is via Docker Compose:
```bash
docker-compose up
```
- Frontend: http://localhost:5173
- Backend: http://localhost:3001

#### Manual Local Development

**Backend (Rust):**
```bash
cd nexus-core
cargo run
```

**Frontend (SvelteKit):**
```bash
cd frontend
bun install
bun dev
```

## Development Commands

### Frontend
- **Start Dev Server:** `bun dev`
- **Type Checking:** `bun run check`
- **Linting:** `bun run lint`
- **Formatting:** `bun run format`
- **Build:** `bun run build`

### Backend
- **Run:** `cargo run`
- **Check:** `cargo check`
- **Format:** `cargo fmt`
- **Lint:** `cargo clippy`
- **Test:** `cargo test`

## Coding Conventions

- **Frontend:** Adhere to Svelte 5 best practices using Runes (`$state`, `$derived`, `$effect`).
- **Backend:** Follow standard Rust idiomatic patterns. Use `axum` for routing and `tokio` for async operations.
- **Formatting:** Use Prettier for the frontend and `rustfmt` for the backend. Always format code before contributing.