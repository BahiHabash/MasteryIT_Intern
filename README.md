> This repository serves as a log of my daily progress, code exercises, and questions.

## Repository Architecture

The workspace is organized chronologically. Each day has its own dedicated directory containing isolated Cargo projects for specific tasks or exercises:

```text
📦 repository-root
├── 📂 day-01/
│   ├── 📂 project-1/
│   └── 📂 project-2/
└── 📂 day-02/
    └── 📂 project-1/
```
---

## How to Run a Specific Project

To execute a program locally, navigate into its specific project directory and run it via Cargo:

```bash
# 1. Navigate to the target project directory
cd ./day-01/project-1

# 2. Execute the Rust binary
cargo run
```
---

## Naming Conventions

* **Day Directories:** Prefixed with `day-` like (`day-01`, `day-15`).