# Asocial: Project Roadmap & Integration Plan

## 1. Project Overview
**Asocial** is a bespoke, unified social media management tool designed for a single-person team to stream content scheduling, cross-posting, and analytics. It prioritizes user privacy, data ownership, and a unified workflow for platforms like Mastodon, Bluesky, Twitter/X, and LinkedIn.

### Core Objectives
*   **Unified Posting:** Write once, publish everywhere (Mastodon, Bluesky, etc.).
*   **Scheduling:** Automate posting times.
*   **Privacy-First Analytics:** Track engagement without invasive surveillance.
*   **Single-User Focus:** Optimized for a "1-Man Team" (low maintenance, high efficiency).

## 2. Technical Stack Strategy (Pure Rust)
*Aim: High performance, type safety, and a single cohesive language ecosystem for a 1-man team.*

*   **Language:** **Rust** (Edition 2021+).
*   **Frontend/UI:** **Iced** (Native).
    *   *Why:* Cross-platform (Linux/macOS/Windows) with strict type safety (Elm architecture). No webview overhead. Future mobile support.
*   **Backend/Logic:** **Tokio** (Async Runtime).
*   **Database:** **PostgreSQL** (via `sqlx`).
    *   *Why:* Robust, standard, and handles complex job queueing natively.
*   **Job Queue:** **PostgreSQL-native** (SKIP LOCKED).
    *   *Why:* Eliminates need for Redis. Keeps infrastructure simple (App + DB).

## 3. OpenProject Import Data
*Copy and paste the table below into Excel/Sheets to export as CSV for OpenProject, or use the "Import" feature if available.*

| Type | Subject | Description | Priority | Est. Hours | Phase |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Phase** | **Phase 0: Foundation & Setup** | Initial repo setup and decisions | High | 10 | 0 |
| Task | Initialize Git Repository | Git init, .gitignore, main branch setup | High | 1 | 0 |
| Task | Setup: Rust Toolchain | Install rustup, cargo, and Iced dependencies | High | 2 | 0 |
| Task | Setup: PostgreSQL | Local Docker Compose or Native Postgres setup | High | 2 | 0 |
| Task | License Creation | Create/Review License (MIT/AGPL?) | Low | 1 | 0 |
| Task | Design Database Schema | Users, Posts, Platforms, Schedule tables (SQL) | High | 4 | 0 |
| Task | CI/CD Pipeline Setup | `cargo check`, `cargo fmt`, build verification | Normal | 2 | 0 |
| **Phase** | **Phase 1: MVP (Desktop)** | Essential Posting from Desktop App | High | 40 | 1 |
| Task | UI: App Shell | Main Window, Navigation Sidebar (Iced) | High | 5 | 1 |
| Task | Core: Database Layer | `sqlx` migrations and connection pool | High | 4 | 1 |
| Task | UI: Post Composer | Text input, Image picker, Platform toggle | High | 10 | 1 |
| Task | Backend: Integration - Mastodon | Rust client for Mastodon API | High | 6 | 1 |
| Task | Backend: Integration - Bluesky | Rust client for Bluesky (AT Proto) | High | 7 | 1 |
| Task | Feature: Drafts | Save drafts to local DB | Normal | 3 | 1 |
| **Phase** | **Phase 2: Async Scheduling** | Automation engine | High | 25 | 2 |
| Task | Backend: Job Queue | Implement generic job runner (pg-backed) | High | 10 | 2 |
| Task | UI: Calendar View | Custom widget for visual scheduling | Normal | 12 | 2 |
| Task | Verification: Schedule Tests | Async tests for job execution | High | 4 | 2 |
| **Phase** | **Phase 3: Expansion** | Expanding reach & polish | Normal | 20 | 3 |
| Task | Backend: Integration - LinkedIn | OAuth2 flow and posting | Normal | 8 | 3 |
| Task | Backend: Integration - Twitter/X | (If viable) API implementation | Normal | 8 | 3 |
| **Phase** | **Phase 4: Analytics & Polish** | Insights and UX | Low | 30 | 4 |
| Task | Analytics: Data Model | Aggregating stats in Postgres | Normal | 5 | 4 |
| Task | UI: Dashboard Widgets | Charts/Graphs in Iced | Normal | 10 | 4 |
| Task | Feature: Deep Linking | Handle system links | Low | 4 | 4 |
| Task | Asset: App Icon/Artwork | Branding | Low | 2 | 4 |
| **Phase** | **Phase 5: Documentation & Release** | Handover and Maintenance | Normal | 10 | 5 |
| Task | Docs: Developer Guide | How to build and run locally | Normal | 4 | 5 |
| Task | Docs: User Guide | Usage instructions | Low | 4 | 5 |
| Task | Maintenance: Cargo Audit | Security scan and dependency update | Low | 2 | 5 |

## 4. Detailed Roadmap Narrative

### Phase 0: Foundation (Weeks 1-2)
*Focus: laying the groundwork to avoid refactoring later.*
*   **Action:** Set up the Rust environment and PostgreSQL. Ensure `cargo run` launches a basic Iced window.
*   **Deliverable:** A compiles-clean repo with CI/CD and a running database container.

### Phase 1: MVP - "The Connector" (Weeks 3-6)
*Focus: successfully posting "Hello World" from a native app.*
*   **Action:** Build the Post Composer in Iced. Handle state management for multiple accounts.
*   **Deliverable:** A desktop app where you can write a post, select "Mastodon", and hit Send.

### Phase 2: The Time Machine (Weeks 7-9)
*Focus: async jobs and reliability.*
*   **Action:** Implement the Scheduler using `sqlx` and PostgreSQL's atomic features (SKIP LOCKED) to create a robust job queue without Redis.
*   **Deliverable:** "Set and forget" scheduling that runs efficiently in the background.

### Phase 3: Expansion (Weeks 10-12)
*Focus: broad reach.*
*   **Action:** Add LinkedIn/X support. Refine the UI for complex inputs (threads, alt text).
*   **Deliverable:** A power-user tool for multiple networks.

### Phase 4: Feedback & Mobile Prep (Weeks 13-15)
*Focus: polish and future-proofing.*
*   **Action:** Implement Analytics. Review Iced's mobile support boundaries and prepare layout for responsiveness.
*   **Deliverable:** A polished desktop app ready for daily use, with a path to mobile.

## 5. Next Steps
1.  **Import** the table above into OpenProject.
2.  **Confirm** Rust and Docker installation.
3.  **Execute** "Initialize Git Repository" task.

