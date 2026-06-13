# 13-06-2026: Cornucopia

A proof-of-concept Rust application demonstrating the power of **Cornucopia**, a SQL-first code generation utility that generates type-safe Rust interfaces directly from your PostgreSQL schema and queries.

---

## What is Cornucopia?

[Cornucopia](https://github.com/cornucopia-rs/cornucopia) is an alternative to traditional ORMs (like Diesel or SeaORM) and compile-time macro libraries (like SQLx). Instead of using complex macros that bloat compile times and degrade IDE auto-completion, Cornucopia compiles raw SQL files into standard, plain Rust modules.

### Key Advantages

- **SQL-First Approach:** Plain SQL files act as the single source of truth for your database logic.
- **Strict Type Safety:** Queries are validated against a PostgreSQL instance during code generation, ensuring type-safe inputs and outputs.
- **Fast Compile Times:** Code generation happens out-of-band via a CLI, meaning no heavy compile-time macros during your main build process.
- **First-Class IDE Integration:** Since the generated code is plain Rust stored in a subdirectory crate, it is fully indexable and supported by `rust-analyzer`.
- **Advanced Types Support:** Native support for PostgreSQL features like arrays, custom domains, enums, and composite types.

---

## Project Structure

The project is structured under the [cornucopia](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia) subfolder:

- **[schema/001_app.sql](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/schema/001_app.sql)**: Defines the database schema, including users, projects, and tasks.
- **[queries/app.sql](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/queries/app.sql)**: Parameterized SQL queries annotated with Cornucopia metadata comments (e.g., `--! upsert_user`).
- **[db/](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/db)**: The generated helper crate containing Cornucopia-generated Rust code. Do not edit files here directly.
- **[src/main.rs](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/src/main.rs)**: The application entry point demonstrating connection setup and executing type-safe queries.
- **[Cargo.toml](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/Cargo.toml)**: The package manifest containing the local path dependency pointing to the generated `db` module.

---

## Database Schema & Queries

### 1. Schema ([schema/001_app.sql](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/schema/001_app.sql))

Defines three tables with relationship constraints:

- `app_users`: Stores user emails and display names.
- `projects`: Tracks projects owned by users.
- `tasks`: Manages tasks under projects, with assigning, statuses (`todo`, `done`), and priorities.

### 2. Queries ([queries/app.sql](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/queries/app.sql))

Each query starts with a Cornucopia directive annotation:

- `--! upsert_user`: Inserts or updates user display name by email.
- `--! upsert_project`: Creates or updates a project.
- `--! delete_project_by_key`: Deletes a project and cascaded references.
- `--! create_task`: Adds a new task with a given assignee, title, and priority.
- `--! complete_task`: Marks a task as `done`.
- `--! task_board`: Lists tasks for a specific project sorted by status, priority, and ID.
- `--! project_summary`: Returns high-level statistics of open/closed tasks.

---

## How Code Generation Works

To generate the code, Cornucopia validates schemas and queries against a PostgreSQL database. It supports three generation models:

### Option A: Schema Generation (Recommended for Containerized Environments)

If Docker/Podman is running, Cornucopia will automatically spin up an ephemeral container, load the schema, validate the queries, and output the generated Rust code.

```bash
cornucopia schema -q queries -d db schema/001_app.sql
```

_Add `-p true` or `--podman true` to specify Podman instead of Docker._

### Option B: Live Mode (Recommended for Local Development)

If you have a local PostgreSQL instance running and configured in your `.env` file, you can run generation against it:

```bash
cornucopia live -q queries -d db postgres://postgres@localhost:5432/Cornucopia-learn
```

---

## Running the Application

### 1. Setup Database

Ensure PostgreSQL is running and update the `DATABASE_URL` in [.env](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/.env):

```env
DATABASE_URL=postgres://postgres@localhost:5432/Cornucopia-learn
```

### 2. Run the Main Application

Start the application via Cargo. The code will automatically apply the migration schema from `001_app.sql`, then execute the queries using the generated client:

```bash
cargo run --bin cornucopia_poc
```

### Execution Showcase

In [main.rs](file:///e:/2-Projects/MasteryIT_Intern/13-06-2026/cornucopia/src/main.rs), we bind parameters using method chains and load results using `.one()` or `.all()`:

```rust
// Create or update a user and get a strongly typed model back
let user = db::queries::app::upsert_user()
    .bind(&client, &"alex@example.com", &"Alex Morgan")
    .one()
    .await?;

// Retrieve multiple rows as a vector of structured models
let tasks = db::queries::app::task_board()
    .bind(&client, &project.id)
    .all()
    .await?;
```
