# 13-05-2026: Asynchronous Programming (Chapter 17) + Axum

### Technical Concepts

- **I/O-Bound vs. CPU-Bound:** Understanding that async is best for waiting on external data (network/disk) rather than heavy math.
- **Concurrency vs. Parallelism:** Juggling multiple tasks (concurrency) vs. running them at the exact same time on different cores (parallelism).
- **The Future Trait:** A "placeholder" for a value that isn't ready yet.
- **Laziness:** Futures do nothing until they are explicitly polled or `.await`-ed.
- **Postfix Syntax:** Using `.await` at the end of a call to allow clean method chaining.
- **Async Runtime:** The "executor" (like Tokio) that manages the state machine transitions.
- **The `join!` Macro:** Executing multiple futures concurrently and waiting for all to finish.
- **Async Channels:** Using `mpsc` (Multi-Producer, Single-Consumer) to pass data between async tasks.

---

### Searched for

- How to turn a synchronous `main` into an asynchronous entry point.
- Difference between `std::thread::spawn` and `trpl::spawn_task`.
- Why `main` cannot be marked `async` natively in Rust.
- How the `move` keyword works with `async` blocks to transfer ownership.

---

### Reference: API Development with Rust ([rust-api.dev/docs](https://rust-api.dev/docs))

#### Part 1: Introduction

- **Why Rust for Web Services:**
  - Combines the security of high-level languages with the efficiency and performance of low-level languages.
  - Generates highly optimized, small binaries with extremely fast startup times—crucial for minimizing cloud computing costs in serverless environments (AWS Lambda, GCP Cloud, Azure Functions).
  - Strict type safety instantly validates user input during deserialization.
- **RESTful API Concepts:**
  - **Statelessness:** Each request carries all necessary context. Servers do not store session state, enabling seamless horizontal scaling.
  - **Resource-Oriented:** Operations target entities represented as nouns (e.g., `/books`) with unique identifiers.
  - **HATEOAS:** Hypermedia As The Engine Of Application State; responses optionally provide dynamic links instructing clients on available state transitions and actions.
- **HTTP Basics:**
  - **Verbs:** `GET` (read), `POST` (create), `PUT` (full overwrite), `PATCH` (partial update), `DELETE` (remove).
  - **Headers:** Carry request metadata like `Host`, `Accept` (desired response MIME type), and `Content-Type` (request payload format).
  - **Status Codes:** Inform outcomes clearly using ranges: `2xx` (Success), `3xx` (Redirection), `4xx` (Client Error), `5xx` (Server Error).

#### Part 2: Hello World!

- **Workspace Architecture:**
  - Recommended to start projects as multi-package Cargo workspaces to maintain modularity in a single repository.
  - Use `resolver = "2"` in `Cargo.toml` to opt into the modern feature resolver standard.
- **Axum Web Server Setup:**
  - Built on top of `tokio` (for the async runtime) and `hyper`.
  - The `#[tokio::main]` macro transparently sets up an async execution environment.
  - A basic HTTP server binds a TCP listener (`tokio::net::TcpListener::bind`) and serves requests routed via `axum::Router`.
- **Handling JSON:**
  - Leverage `serde` (with `Serialize`/`Deserialize` derives) and `serde_json` for struct serialization.
  - Handlers return tuples of `(StatusCode, axum::Json<T>)` to accurately return HTTP status alongside serializable JSON payloads.

#### Part 3: Useful Things

- **Error Handling Strategies:**
  - **`expect` vs `unwrap`:** Prefer `expect` for unrecoverable errors to attach descriptive panic context.
  - **Application Errors (`anyhow`):** Use `anyhow::Result` in applications for simple context chaining via `.context("...")` and ad-hoc error creation (`anyhow::bail!`).
  - **Handler Errors (`IntoResponse`):** Handlers convert internal errors to API responses by wrapping them in custom types (e.g., a newtype `AppError(anyhow::Error)`) implementing `axum::response::IntoResponse`.
  - **Panic Catching:** Prevent single thread panics from silently dropping connections using `tower_http::catch_panic::CatchPanicLayer`.
- **Data Conversion Traits:**
  - Implement standard library traits `From` and `Into` for infallible model transformations (e.g., converting internal domain structs to API response structs).
  - Use `TryFrom` when conversions can fail, explicitly defining the associated `Error` type.
- **Serde Data Customization:**
  - Re-case struct field serialization using container attributes like `#[serde(rename_all = "camelCase")]` or target individual fields with `#[serde(rename = "...")]`.
  - Control enum representations cleanly via **internally tagged** (`tag = "..."`), **externally tagged** (default), **adjacently tagged** (`tag = "...", content = "..."`), or **untagged** layouts.
- **Async & Threading Architecture:**
  - **The `SimpleFuture` Trait:** Uses a `poll` method returning `Poll::Ready(T)` or `Poll::Pending` along with a `wake` callback to notify the executor when blocked tasks become ready again.
  - **Cooperative Multitasking:** Tasks voluntarily yield execution state back to the runtime when blocked on I/O. Long CPU-bound tasks should avoid blocking cooperative threads.
  - **Ownership (`async move`):** Explicitly moves ownership of captured external references into the resulting `Future` to satisfy thread-lifetime bounds.
  - **Cross-Thread State Boundaries:** Tasks sent across threaded executor threads must be safe to move (`Send` trait). Non-thread-safe reference counts like `Rc` must be replaced by atomic reference counts (`Arc`), paired with `Mutex` for safe interior mutability across threads.
