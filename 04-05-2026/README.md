# Day 04-05-2026:

### Technical Concepts:

- struct shorthand like copy struct: uses `=` operator so it moved not copied values

```rust
let user2 = User {
    username: String::from("anotherusername"),
    email: String::from("anotheremail"),
    ..user1
};
```

- `:?` VS `:#?`
  - `:?` is used for debugging
  - `:#?` is used for pretty printing

- `println!` macros vs `dbg!` macro
  - `println!` is used for printing to the console, it take a reference to a value and print it
  - `dbg!` is used for debugging with taking ownership of the value, and it also prints the file name and line number of where it is called
- Methods VS Associated Functions
  - Methods are functions that are defined in the same scope as the struct and take a reference to the struct as the first parameter
  - Associated functions are functions that are defined in the same scope as the struct but do not take a reference to the struct as the first parameter
- `if let` VS `match` VS `let else`
  - `if let` is used for conditional execution of a block of code if a pattern matches
  - `match` is used for pattern matching
  - `let else` is used for conditional assignment of a value if a pattern matches

---

### Privary Rules:

- everything inside a module is private by default
- `pub` keyword make a module public
- `pub(crate)` keyword make a module public within the crate
- `pub(super)` keyword make a module public within the parent module
- `pub(in <module>)` keyword make a module public within the specified module
- Re-exporting with `pub use`

```Rust
// src/lib.rs

pub mod api;       // Public: The front door for users
mod internal;      // Private: The "basement" of the engine

pub(crate) fn connect_to_db() {
    // Everyone in the engine needs this, but hide it from the customer!
}

mod template_engine {
    pub fn render() {
        // Public to the rest of the app
        parser::parse_logic();
    }

    mod parser {
        // Only visible to 'template_engine'
        // 'api' or 'internal' cannot see this!
        pub(super) fn parse_logic() {
            println!("Working on tags...");
        }
    }
}
```

- Conventions:
  - we bring the `struct` and `enum` with the full path but don't do such with functions or methods
  - To create a module: Create the file, then go to the parent and add mod filename;.
  - To make it usable by others: Use pub mod filename;.
  - For deep folders: Use a folder with a mod.rs (Legacy) or a file with a matching folder name (Modern).
- **Crate**: A unit of compilation, can be a binary or a library.
- **Package**: A collection of crates.
- **Module**: A collection of functions and structs.
