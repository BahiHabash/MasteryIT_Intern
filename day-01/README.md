# Day 01: Rust Fundamentals & Control Flow

### Technical Concepts
* **Primitive dataTypes**
* **IF Conditions**
* **Loops**
* **Functions**
* **Fixed-Size Arrays:** `[char; 9]` and `[[usize; 3]; 8]`.
* **External Crates:** `cargo add package-name`.

---
### Searched for
- naming convetion for variables, functions, and files
- how to colorize output string in console
- how to install a package
- how to read input from the console


---
### Debugging & Challenges Resolved
* **used a string method with char**
  ```
  no method named cyan found for type char in the current scope
  ```
  * `solution`: used the method `to_string()` to convert chart to string.

* **Macro vs. Method Signatures:** 
  ```
  .expect("{}", "Please enter a valid number".red());
  unexpected argument #2 of type ColoredString
  note: method defined here
  pub fn expect(self, msg: &str) -> T
  ```
  * `soluttion`: used `Gemini` to solve it - built the string.