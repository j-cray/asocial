# Asocial: Project Roadmap & Integration Plan

## 1. Project Overview
**Asocial** is a bespoke, unified social media management tool designed for a single-person team to stream content scheduling, cross-posting, and analytics. It prioritizes user privacy, data ownership, and a unified workflow for platforms like Mastodon, Bluesky, Twitter/X, and LinkedIn.

### Core Objectives
*   **Unified Posting:** Write once, publish everywhere (Mastodon, Bluesky, etc.).
*   **Scheduling:** Automate posting times.
*   **Privacy-First Analytics:** Track engagement without invasive surveillance.
*   **Single-User Focus:** Optimized for a "1-Man Team" (low maintenance, high efficiency).

## 2. Technical Stack Strategy (Consolidated)
*Based on "1-Man Team" constraint, we recommend a robust, low-complexity monolithic stack.*

*   **Backend:** **Node.js** (TypeScript) or **Python** (FastAPI/Django). *Recommendation: Node.js (TS) for shared types with frontend.*
*   **Frontend:** **React** or **Vue**. *Recommendation: React (Vite) for ecosystem speed.*
*   **Database:** **PostgreSQL** (Robust, standard) or **SQLite** (if strictly local/single-user). *Recommendation: PostgreSQL for reliable scheduling/queueing.*
*   **Queue/Job Runner:** **Redis/BullMQ** (Node) or **Celery** (Python) for scheduling posts.

## 3. OpenProject Import Data
*Copy and paste the table below into Excel/Sheets to export as CSV for OpenProject, or use the "Import" feature if available.*

| Type | Subject | Description | Priority | Est. Hours | Phase |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Phase** | **Phase 0: Foundation & Setup** | Initial repo setup and decisions | High | 10 | 0 |
| Task | Initialize Git Repository | Git init, .gitignore, main branch setup | High | 1 | 0 |
| Task | Select Tech Stack | Finalize decision: Node vs Python, React vs Vue | High | 2 | 0 |
| Task | License Creation | Create/Review License (MIT/AGPL?) | Low | 1 | 0 |
| Task | Design Database Schema | Users, Posts, Platforms, Schedule tables | High | 4 | 0 |
| Task | CI/CD Pipeline Setup | Basic linting, build verification | Normal | 2 | 0 |
| **Phase** | **Phase 1: Core Functionality (MVP)** | Essential API and Dashboard | High | 40 | 1 |
| Task | API: Authentication | User login/session management | High | 5 | 1 |
| Task | API: Platform Connection | Securely storing API keys/tokens | High | 5 | 1 |
| Task | Frontend: Dashboard Skeleton | Main layout, navigation, responsive container | High | 5 | 1 |
| Task | Core: Post Composer | UI for writing text and attaching media | High | 8 | 1 |
| Task | Feature: Drafts | Save posts as drafts (Collaboration support later) | Normal | 4 | 1 |
| Task | Backend: Integration - Mastodon | Implement posting to Mastodon API | High | 6 | 1 |
| Task | Backend: Integration - Bluesky | Implement posting to Bluesky (AT Protocol) | High | 7 | 1 |
| **Phase** | **Phase 2: Advanced Scheduling** | Automation engine | High | 25 | 2 |
| Task | Backend: Scheduler Engine | Job queue setup (Redis/Cron) | High | 10 | 2 |
| Task | Frontend: Calendar View | Visual scheduler for posts | Normal | 8 | 2 |
| Task | Timezone Handling | Robust user timezone support | Normal | 3 | 2 |
| Task | Verification: Schedule Tests | Unit tests for time-sensitive queueing | High | 4 | 2 |
| **Phase** | **Phase 3: Additional Integrations** | Expanding reach | Normal | 20 | 3 |
| Task | Research: API Pricing | Twitter/X & LinkedIn API limits/costs | High | 4 | 3 |
| Task | Backend: Integration - Twitter/X | (If viable) Implement posting | Normal | 8 | 3 |
| Task | Backend: Integration - LinkedIn | Implement posting | Low | 8 | 3 |
| **Phase** | **Phase 4: Analytics & Polish** | Insights and UX | Low | 30 | 4 |
| Task | Analytics: Data Model | Schema for tracking likes/reposts/replies | Normal | 5 | 4 |
| Task | Analytics: Dashboard Widget | Visualize basic engagement stats | Low | 8 | 4 |
| Task | Feature: ActivityPub | Research & Integration for native federation | Low | 10 | 4 |
| Task | UX: Optimization Pass | Spinner states, error handling, mobile check | Normal | 5 | 4 |
| Task | Asset: App Icon/Artwork | Branding | Low | 2 | 4 |
| **Phase** | **Phase 5: Documentation & Release** | Handover and Maintenance | Normal | 10 | 5 |
| Task | Docs: API Documentation | Swagger/OpenAPI spec | Normal | 4 | 5 |
| Task | Docs: User Guide | How to add accounts, schedule posts | Low | 4 | 5 |
| Task | Maintenance: Dead Code Scan | Cleanup before v1.0 | Low | 2 | 5 |

## 4. Detailed Roadmap Narrative

### Phase 0: Foundation (Weeks 1-2)
*Focus: laying the groundwork to avoid refactoring later.*
*   **Action:** Decide on the Tech Stack immediately. If you are comfortable with JS/TS, the **T3 Stack (Next.js, tRPC, Tailwind)** or a simple **Express + React** SPA is ideal for a one-man team.
*   **Deliverable:** A clean repo, connected to CI, with a clear Database Schema diagram (ERD).

### Phase 1: MVP - "The Connector" (Weeks 3-6)
*Focus: successfully posting "Hello World" to at least two platforms.*
*   **Action:** Build the Post Composer first. It needs to handle character limits for different platforms (e.g., Mastodon's 500 vs. others).
*   **Deliverable:** A working app where you can write a post, select "Mastodon" and "Bluesky", hit "Send", and see it appear live.

### Phase 2: The Time Machine (Weeks 7-9)
*Focus: async jobs and reliability.*
*   **Action:** Implement the Scheduler. This requires a reliable background worker (e.g., BullMQ for Node).
*   **Deliverable:** Being able to schedule a post for 3:00 AM and having it actually publish while you sleep.

### Phase 3: Expansion (Weeks 10-12)
*Focus: broad reach.*
*   **Action:** Tackle the harder APIs (LinkedIn, X). These often have strict rate limits or paid tiers.
*   **Deliverable:** More checkboxes in the "Post to" menu.

### Phase 4: Feedback Loop (Weeks 13-15)
*Focus: measuring success.*
*   **Action:** Build the Analytics scraper. It needs to be polite (don't spam APIs) and privacy-focused (store aggregates, not user lists if possible).
*   **Deliverable:** A dashboard showing "Total Reach" across platforms.

## 5. Next Steps
1.  **Import** the table above into OpenProject.
2.  **Select** your Tech Stack (Phase 0).
3.  **Execute** "Initialize Git Repository" task.
