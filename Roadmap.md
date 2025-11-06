# Project Roadmap

This document outlines the planned improvements and future goals for the **GitHub Statistics Collector**.

---

## ✅ Current Status

- Parallelized GitHub API calls for:
  - Stargazers
  - Contributors (including forks)
  - Watchers
  - Issue creators
- Automatic rate-limit handling (1-hour cooldown)
- Minimal Axum HTTP service
- CLI-based data aggregation for multiple repositories
- JSON configuration support (`repos.json`)

---

## 🧭 Short-Term Goals (Q4 2025)

| Goal | Description | Status |
|------|--------------|--------|
| **1. REST API for results** | Expose results as `/stats` JSON endpoint with repository-level breakdowns. | 🔜 Planned |
| **2. Caching layer** | Store fetched results to minimize redundant API calls and respect rate limits. | 🔜 Planned |
| **3. Configurable concurrency** | Allow tuning of async task parallelism (e.g., via `TOKIO_MAX_CONCURRENCY`). | 🔜 Planned |
| **4. Improved error handling** | Replace panics with structured error responses using `anyhow` or `thiserror`. | ⚙️ In progress |

---

## 🧱 Medium-Term Goals (2026)

| Goal | Description | Status |
|------|--------------|--------|
| **1. Web Dashboard** | Build a small frontend (React + Axum API) to visualize collected statistics. | 🔜 Planned |
| **2. Data persistence** | Store collected data in SQLite or Postgres using `sqlx`. | 🔜 Planned |
| **3. GitHub Actions Integration** | Automate daily stats collection and store results in repository artifacts. | 🔜 Planned |
| **4. CSV/JSON Export** | Add endpoints or CLI flags to export aggregated results. | 🔜 Planned |
| **5. Parallel rate-limit management** | Implement intelligent queueing with per-token rate tracking. | 🔜 Planned |

---

## 🌍 Long-Term Vision

- Build a **“GitHub Insights Service”** that aggregates ecosystem-wide engagement metrics for open-source projects (especially FIWARE-related).
- Offer both **CLI** and **API** modes for automation.
- Integrate authentication and user dashboards.
- Support other platforms (GitLab, Bitbucket) through modular API adapters.

---

## 🧠 Potential Enhancements

- [ ] Add tracing/logging with `tracing` crate and `flexi_logger`
- [ ] Include unit tests and integration tests with mocked GitHub API
- [ ] Provide Docker container for easy deployment
- [ ] Generate periodic reports (PDF/HTML) using a scheduler
- [ ] Integrate with FIWARE’s internal analytics system

---

## 💬 Feedback & Collaboration

Suggestions and contributions are highly appreciated.  
Open a GitHub issue or reach out if you want to collaborate on:

- API design
- Dashboard UI
- Data persistence
- Performance optimizations

---
