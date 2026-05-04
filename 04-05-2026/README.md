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
