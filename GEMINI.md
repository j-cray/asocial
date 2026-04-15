# asocial Project Guidelines

## Project Description

**asocial** is a feature-rich, high-performance, and secure self-hostable application for social media management, built purely in Rust. It utilizes a workspace-based architecture with an **Axum** backend, a **Leptos** WASM frontend, and **PostgreSQL** for durable, secure storage. It is designed to provide a unified composer and deep analytics across multiple platforms, including Bluesky, Pixelfed, Instagram, Facebook, LinkedIn, X, and TikTok. A core tenet is its "Premium Dark/Glassmorphism" aesthetic and an uncompromising stance on data privacy through self-hosting.

## Development Workflow Instructions

As an AI coding assistant working on this repository, you must strictly adhere to the following workflow rules at all times:

1. **Test-Driven Development (TDD)**: Create tests *first* where possible. Ensure that you create a test for *everything possible* before considering a feature complete.
2. **Mandatory Checks**: Run `cargo nextest`, `cargo clippy`, and `cargo fmt` after completing every task. Do not push or conclude tasks if these checks fail.
3. **Roadmap Tracking**: Tick off items on the `docs/plans/roadmap.md` file synchronously as you complete them. Maintain this as a living source of truth for project progress.
4. **README Maintenance**: Maintain the `README.md` file so that it accurately reflects the *current state* of the project (features, setup instructions, architecture) at all times. Update it as new components are wired up.
