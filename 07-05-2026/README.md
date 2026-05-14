# Rust Async - Thread Pool

## Intro

- Rust's async doesn't run code by itself. It needs an "executor" or "runtime" to poll futures.
- **Tokio** is the most popular runtime.
- **Key Concept**: Async tasks are _lightweight_ (stackless coroutines), not OS threads.

## 📝 Day 1: Executors & Tasks (14th May 2026)

### 1. The Problem

Without an executor, `async fn` just defines a function. It doesn't _run_.

```rust
async fn my_task() {
    println!("Hello");
}

fn main() {
    // my_task(); // WRONG - returns a Future, but never runs it!
}
```

### 2. The Solution: Tokio Runtime

Tokio provides the "muscle" to run async code.

```rust
use tokio::runtime::Runtime;

async fn say_hello() {
    println!("Hello");
}

fn main() {
    // 1. Create Runtime
    let rt = Runtime::new().unwrap();

    // 2. "Block On" - Run a future to completion
    //    This is like .await on the top-level future
    rt.block_on(say_hello());
}
```

**Output:**

```
Hello
```

### 3. Spawning Tasks

`block_on` waits for one task. What if we want to do multiple things at once?

```rust
use tokio::runtime::Runtime;

async fn task(id: u32) {
    println!("Task {} started", id);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Task {} finished", id);
}

fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        // Spawn doesn't wait! It returns a Handle.
        let handle1 = tokio::spawn(task(1));
        let handle2 = tokio::spawn(task(2));

        println!("Tasks spawned!");

        // We can do other work here while tasks run in background
        // ...

        // Wait for them to finish
        let _ = handle1.await;
        let _ = handle2.await;
    });
}
```

### 4. Tokio Main Macro (The Easy Way)

Typing `Runtime::new().unwrap().block_on(...)` is tedious.
Tokio provides a macro to do this automatically.

**`Cargo.toml`**:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

**`main.rs`**:

```rust
async fn task(id: u32) {
    println!("Task {} started", id);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Task {} finished", id);
}

#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(task(1));
    let handle2 = tokio::spawn(task(2));

    let _ = tokio::join!(handle1, handle2);
}
```

**Output (Concurrent!):**

```
Task 1 started
Task 2 started
(1 second later)
Task 1 finished
Task 2 finished
```

---

## 🎯 Key Takeaways

1. **Async is "Do Tomorrow"**: Defining an `async fn` doesn't run it.
2. **Executor Needed**: Tokio (runtime) polls futures to make progress.
3. **`tokio::spawn`**: Starts a task in the background.
4. **`tokio::join!`**: Waits for multiple tasks concurrently.
5. **`#[tokio::main]`**: The magic macro for simple apps.
