# Day 04:

### Technical Concepts
* `&str` VS `String`
* `Mutable` VS `IMMutable` reference.
* `Ownership` VS `Borrowing`.
* `Borrowing` VS `Moving` (Transfer ownership)
* `Least Previlege Principle` in Rust.
* `Dereference`
* `Life time tagging`, and `'static` life time

### Searched for
* why String in Rust doesn't end with `\0`? 
  * String pointer is a `fat pointer` that holds (Acctual pointer to the data on the heap, length of the string, capacity of the string)
  * slice string is a pointer that holds (Acctual pointer to the data on the heap, length of the string)
  * this ensure catching out-of-boundry access in compilation time.


### Articles
- [Rust Ownership and Borrowing Explained](https://dev.to/leapcell/rust-ownership-and-borrowing-explained-22l6).

### Courses & Challenges
- Completed the `5th chapter` of [tour of rust](https://tourofrust.com/)
