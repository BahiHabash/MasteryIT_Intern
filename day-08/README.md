# Day 08:

### Technical Concepts
* `prelude`.
* `inline modules` and its usecases.
* exporting modules, structs and fields.
* module visability looking inward: A child can see a parent's private items, but a parent cannot see a child's private items..
* `#[cfg(test)]` macro with inline modules
* `crate` (root) VS `super` (parent) VS `self` (current)
* Everything in Rust is private by default
* `pub` VS `pub(crate)` VS `pub(super)`
* In Rust, the filesystem and the module tree are decoupled.

---

### Searched for
* How import/export modules works in rust?
* Node.js Filesystem VS Rust Logical module Tree
* Does `prelude` is like `barrel exporting` technique?
* Why do I need a lib if I already have modules?

---

### Courses & Challenges
- Completed the `9th chapter` of [tour of rust](https://tourofrust.com/)

---

### Articles: 
[Structuring a Rust Backend](https://medium.com/@rivelbab/rust-actix-web-structuring-and-organizing-an-api-like-a-pro-790657e61ba5)
[Mastering Large Project Organization in Rust](https://dev.to/leapcell/mastering-large-project-organization-in-rust-n11)

---

### Market standards:

* strictly enforce absolute paths for importing distant modules.
 ```Rust
// Even if this file is located at `src/billing/invoices/generators/pdf.rs`,
// we don't navigate upwards. We navigate from the absolute root.
use crate::auth::jwt::validate_token;
use crate::database::connection::get_pool;
```

* Avoid nested mod.rs files.
```Rust
src/
├── main.rs            // The front door. ONLY contains `mod` declarations and a tiny `main()` function.
├── users.rs           // The `users` module. Declares `mod handlers;` and `mod models;`.
├── users/             // Directory containing the children of `users.rs`
│   ├── handlers.rs    // e.g., HTTP logic
│   └── models.rs      // e.g., Database structs
├── billing.rs         // The `billing` module. Declares `mod invoices;`.
└── billing/
    └── invoices.rs
```