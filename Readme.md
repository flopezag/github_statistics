# GitHub Statistics Collector

A high-performance, asynchronous Rust application that collects engagement statistics from FIWARE GitHub repositories — including **stargazers**, **contributors**, **forks**, **watchers**, and **issue authors**.  

This project parallelizes API calls across repositories and forks using **Tokio** and **FuturesUnordered**, ensuring efficient data retrieval even for large organizations such as FIWARE.

---

## 🚀 Features

- ✅ Collects GitHub repository data:
  - Stargazers (users who starred)
  - Contributors (including forked repos)
  - Watchers (subscribers)
  - Issue authors
- ⚡ Fully asynchronous using `tokio` and `reqwest`
- 🔀 Parallel processing across multiple repositories and forks
- 🧱 Built with the Axum web framework (includes a simple HTTP status route)
- 🕒 Handles GitHub rate limits gracefully (auto-sleep on 403)
- 🔐 Secure GitHub API access via personal token

---

## 🧩 Tech Stack

| Component | Purpose |
|------------|----------|
| **Rust** | Core programming language |
| **Axum** | Lightweight async web server |
| **Reqwest** | HTTP client for GitHub API calls |
| **Tokio** | Async runtime |
| **Futures** | Parallel async task management |
| **Serde / Serde JSON** | JSON parsing and serialization |
| **Dotenv** | Environment variable management |

---

## 🧰 Installation & Setup

### 1. Prerequisites
- Rust (v1.70+)
- A GitHub Personal Access Token (with `read:public_repo` permissions)
- `cargo` build tool

### 2. Clone the repository
```bash
git clone https://github.com/yourusername/github-stats-collector.git
cd github-stats-collector
```

### 3. Create your .env file

```bash
GITHUB_TOKEN=your_github_token_here
```

The program only reads public repository data, so it does not need write 
or admin permissions. There are two options to generate the token, either
Personal Access Token (classic) or Fine-grained Tokens (recommended by 
GitHub)

When creating a Personal Access Token (classic):

- Go to → GitHub Settings → Developer Settings → Personal Access Tokens 
→ Tokens (classic)
- Click “Generate new token (classic)”. Set:
  - Expiration: reasonable (e.g., 90 days or 1 year).
  - Scopes: check only, read:public_repo (this grants access to read 
  public repositories’ metadata)

Copy the generated token and save it securely. If you prefer Fine-grained 
Tokens (recommended by GitHub):

- Choose “Fine-grained personal access token”
- Under Repository access, select “All public repositories”
- Under Permissions → Repository Permissions, set:
  - Metadata → Read-only
  - Contents → Read-only
  - Issues → Read-only (for issue authors)
  - Pull requests → Read-only (optional)
  - No other permissions are required.

### 4. Define the repositories to analyze

Create a repos.json file in the project root:
```bash
[
  "FIWARE/context.Orion-LD",
  "FIWARE/tutorials.Step-by-Step"
]
```

### 5. Run the collector
```bash
cargo run
```

## 📊 Example Output

```bash
Fetching stats for FIWARE/context.Orion-LD...
[FIWARE/context.Orion-LD] Stargazers: 20, Developers: 10, Total Users: 45
Fetching stats for FIWARE/tutorials.Step-by-Step...
[FIWARE/tutorials.Step-by-Step] Stargazers: 35, Developers: 15, Total Users: 60

Total FIWARE users: 90
Total FIWARE developers: 22
```

## 🌐 Web Endpoint

The project includes a minimal Axum server that exposes a health-check route:

http://localhost:8080/


Response:

GitHub Stats Collector Running 🚀

## 🧩 Directory Structure

github-stats-collector/
├── Cargo.toml
├── src/
│   └── main.rs
├── repos.json
├── .env
├── README.md
└── ROADMAP.md

## Roadmap

To take an overview of the Roadmap defined for this component, please
take a look to the [Roadmap.md](./Roadmap.md) document.

## 🤝 Contributions

Pull requests, feature suggestions, and improvements are welcome!
Please open an issue before submitting major changes.

## 📧 Contact

Maintained by [Your Name or Organization]
If you have questions, reach out via GitHub Issues or email.

## ⚖️ License

This project is licensed under the [Apache 2.0 License](./LICENSE).
