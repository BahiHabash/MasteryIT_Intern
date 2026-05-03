# 03/05/2026: Revisit Chapters 1-5 of The Book

### Technical Concepts

- **Ownership & Memory**:
  - The ownership movement is actually the pointers being copied and then invalidating the older owner.
  - Types that implement the `Copy` trait copy data rather than moving it.
  - If the type size is known at compilation time, it can implement the `Copy` trait.
  - Usecases of `.expect()`: when to avoid and when to use it for better error handling.
- **Strings & Slices**:
  - `&str` (string slice) VS `String` (owned string).
  - String literals are of type `&'static str`.
  - Difference between `utf-8` and `unicode`.
  - Raw string literals: `r#""#`.
  - `.len()` method returns the length in bytes, not characters.
  - `char` is always 4 bytes long.
  - Converting string to char vector: `"".chars().collect::<Vec<char>>()`.
  - String concatenation using `+` and `.push_str()`.
  - Using the `format!` macro for complex string building.

### Searched for

- Why to use `.get()` instead of `[..]` when dealing with string slicing (safety against panic on invalid UTF-8 boundaries).

### Courses & Challenges

- Completed **the first 5 chapters** of [The Rust Book](https://doc.rust-lang.org/book/).

---

## Chapter Summary (Revision)

- **Chapter 1**: Getting Started & Cargo.
- **Chapter 2**: Programming a Guessing Game (I/O, Match).
- **Chapter 3**: Common Programming Concepts (Types, Functions, Control Flow).
- **Chapter 4**: Understanding Ownership (The Core of Rust).
- **Chapter 5**: Structs & Method Syntax.
