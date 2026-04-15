# aSocial Project Roadmap

This document outlines the detailed roadmap, broken into phases and modular subtasks, for building **aSocial** - a feature-rich, high-performance, and secure self-hostable Rust application for social media management.

## Project Core Goals
*   **Architecture**: High-performance Rust backend (Axum) with a WASM frontend (Leptos) and secure PostgreSQL storage.
*   **Aesthetics**: Slick modern UI featuring "Premium Dark/Glassmorphism", micro-animations, and responsive layouts.
*   **Integrations**: Support for Bluesky, Pixelfed, Instagram, Facebook, LinkedIn, X, and TikTok.
*   **Capabilities**: Unified post composition, cross-platform scheduling, and deep analytics tools.
*   **Self-Hostable**: Easy Docker/Compose deployment and rigorous data privacy boundaries.

---

## Phase 1: Foundation & Architecture Setup

**Goal**: Establish the base codebase structure, frontend/backend communication, and the database schema.

- [ ] **Task 1.1: Cargo Workspace Initialization**
  - Create a workspace encompassing `server` (Axum), `web` (Leptos), and `shared` (Types) crates.
  - Setup pre-commit linting (Clippy, Rustfmt).
- [ ] **Task 1.2: Database & State Management**
  - Initialize PostgreSQL via SQLx.
  - Design schemas for Users, OAuth Credentials, social accounts, and posts.
- [ ] **Task 1.3: Authentication System**
  - Implement JWT and Server-Side Session token logic.
  - Setup basic Identity provider (login/registration).
- [ ] **Task 1.4: Base Frontend Design System**
  - Configure TailwindCSS for the "Premium Dark/Glassmorphism" aesthetic.
  - Build core UI components: Sidebar, Topnav, Modal, Cards, Buttons (with hover effects).
- [ ] **Task 1.5: Containerization**
  - Create `Dockerfile` and `docker-compose.yml` for local development setup in Nix/NixOS friendly environments.

## Phase 2: Platform API Integrations & Core Systems

**Goal**: Establish secure communication with target social platforms.

- [ ] **Task 2.1: Secure Credential Management**
  - Implement AES-256 encryption at rest for all 3rd-party OAuth Tokens and Application Secrets.
- [ ] **Task 2.2: ActivityPub & AT Protocol Integration**
  - Implement Bluesky SDK (XRPC) for auth/posting.
  - Implement Pixelfed API (Mastodon-compatible endpoints).
- [ ] **Task 2.3: Meta Family API Integration**
  - Integrate Facebook Graph API (Facebook Pages & Groups).
  - Integrate Instagram Graph API (Publishing & Media capabilities).
- [ ] **Task 2.4: Professional & Microblogging Integrations**
  - Implement LinkedIn API (Share & Profile posting).
  - Implement X (Twitter) API v2 (Tweets, Media Uploads).
- [ ] **Task 2.5: Video Platform Integrations**
  - Implement TikTok Content Posting API.

## Phase 3: The Unified Composer & Background Worker

**Goal**: Allow users to craft and schedule multi-platform posts seamlessly.

- [ ] **Task 3.1: Unified Composer UI**
  - Build a rich-text editor with per-platform validation (character limits, media size/ratio compliance).
  - Add drag-and-drop media integration.
- [ ] **Task 3.2: Universal Background Task Queue**
  - Implement a durable background queue in Rust (e.g., Tokio-based runner backed by PostgreSQL) to process scheduled tasks without a separate Redis instance.
- [ ] **Task 3.3: Content Scheduling Logic**
  - Allow chronological scheduling, timezone-aware queuing, and recurring content rules.
- [ ] **Task 3.4: Content Calendar UI**
  - Design and build a drag-and-drop calendar view mapping out scheduled posts across all platforms.

## Phase 4: Analytics Tools & Engagement Tracking

**Goal**: Provide users with actionable insights.

- [ ] **Task 4.1: Engagement Ingestion**
  - Build workers to poll or receive webhooks from connected platforms (likes, retweets, comments, views).
- [ ] **Task 4.2: Analytics Processing Engine**
  - Create database views/aggregations measuring follower growth rates and engagement metrics over time.
- [ ] **Task 4.3: Realtime Dashboard UI**
  - Build interactive charts using a WASM-compatible charting library.
  - Implement UI panes corresponding to optimal posting times and audience demography (where supported).
- [ ] **Task 4.4: Export Capabilities**
  - CSV/PDF report generation for clients/branding.

## Phase 5: Security, Performance, and Release

**Goal**: Finalize for public distribution and self-hosting capabilities.

- [ ] **Task 5.1: Performance Optimization**
  - Optimize Leptos payload size using `wasm-opt`.
  - Minimize binary footprint of Axum with efficient multi-stage Docker builds.
- [ ] **Task 5.2: Security Hardening**
  - Verify strict CORS, CSP, and implement strict Rate Limiting modules on all APIs.
- [ ] **Task 5.3: Deployment Documentation**
  - Create extensive README and Setup Guides tailored for Docker Compose, NixOS, and bare-metal environments.
