## Rust Collections — Complete Study Guide

---

### 1. `Vec<T>` — Dynamic Array

_Your most-used collection. Use for product lists, order items, search results._

| Method                   | What it does                  |
| ------------------------ | ----------------------------- |
| `Vec::new()` / `vec![]`  | Create a vector               |
| `.push(val)`             | Add to end                    |
| `.pop()`                 | Remove & return last          |
| `.insert(i, val)`        | Insert at index               |
| `.remove(i)`             | Remove at index               |
| `.len()` / `.is_empty()` | Size checks                   |
| `.contains(&val)`        | Check existence               |
| `.iter()`                | Iterate (borrowing)           |
| `.iter_mut()`            | Iterate (mutable)             |
| `.into_iter()`           | Consume into iterator         |
| `.get(i)`                | Safe index (returns `Option`) |
| `.retain(condition)`     | Keep only matching elements   |
| `.sort()` / `.sort_by()` | Sort in place                 |
| `.dedup()`               | Remove consecutive duplicates |
| `.extend(iter)`          | Append another collection     |
| `.drain(..)`             | Remove & return a range       |
| `.truncate(n)`           | Keep only first n elements    |
| `.windows(n)`            | Sliding window slices         |
| `.chunks(n)`             | Split into fixed-size chunks  |
| `.concat()` / `.join()`  | Flatten nested vecs           |

---

### 2. `HashMap<K, V>` — Key-Value Store

_Use for product lookups by ID, session stores, config maps, caches._

| Method                     | What it does               |
| -------------------------- | -------------------------- |
| `HashMap::new()`           | Create                     |
| `.insert(k, v)`            | Add/overwrite entry        |
| `.get(&k)`                 | Returns `Option<&V>`       |
| `.get_mut(&k)`             | Mutable reference to value |
| `.remove(&k)`              | Delete entry               |
| `.contains_key(&k)`        | Check key existence        |
| `.entry(k).or_insert(v)`   | Insert if missing          |
| `.entry(k).or_default()`   | Insert default if missing  |
| `.entry(k).and_modify(fn)` | Modify existing value      |
| `.keys()`                  | Iterate over keys          |
| `.values()`                | Iterate over values        |
| `.iter()`                  | Iterate `(k, v)` pairs     |
| `.len()` / `.is_empty()`   | Size checks                |
| `.retain(condition)`       | Keep matching entries      |
| `.extend(iter)`            | Merge another map          |

---

### 3. `HashSet<T>` — Unique Values

_Use for tags, permissions, unique SKUs, deduplication._

| Method                   | What it does              |
| ------------------------ | ------------------------- |
| `HashSet::new()`         | Create                    |
| `.insert(val)`           | Add (returns bool if new) |
| `.remove(&val)`          | Delete                    |
| `.contains(&val)`        | Check membership          |
| `.len()` / `.is_empty()` | Size checks               |
| `.iter()`                | Iterate                   |
| `.union(&other)`         | All elements from both    |
| `.intersection(&other)`  | Elements in both          |
| `.difference(&other)`    | In self but not other     |
| `.is_subset(&other)`     | Check subset              |
| `.is_superset(&other)`   | Check superset            |
| `.retain(condition)`     | Keep matching elements    |

---

### 4. `BTreeMap<K, V>` — Sorted Key-Value Store

_Use when you need ordered data — price ranges, sorted product listings, date-ordered events._

| Method                   | What it does             |
| ------------------------ | ------------------------ |
| `BTreeMap::new()`        | Create                   |
| `.insert(k, v)`          | Add entry                |
| `.get(&k)`               | Lookup                   |
| `.remove(&k)`            | Delete                   |
| `.range(a..b)`           | Iterate over a key range |
| `.first_key_value()`     | Lowest key entry         |
| `.last_key_value()`      | Highest key entry        |
| `.entry(k).or_insert(v)` | Insert if missing        |
| `.keys()` / `.values()`  | Ordered iteration        |
| `.contains_key(&k)`      | Check key                |

---

### 5. `BTreeSet<T>` — Sorted Unique Values

_Use for sorted tags, ordered category lists, ranked items._

| Method                         | What it does            |
| ------------------------------ | ----------------------- |
| `BTreeSet::new()`              | Create                  |
| `.insert(val)`                 | Add                     |
| `.remove(&val)`                | Delete                  |
| `.contains(&val)`              | Check membership        |
| `.range(a..b)`                 | Iterate a sorted range  |
| `.first()` / `.last()`         | Min / Max element       |
| `.union()` / `.intersection()` | Set operations (sorted) |

---

### 6. `VecDeque<T>` — Double-Ended Queue

_Use for job queues, order processing pipelines, sliding windows._

| Method                   | What it does          |
| ------------------------ | --------------------- |
| `VecDeque::new()`        | Create                |
| `.push_back(val)`        | Add to back           |
| `.push_front(val)`       | Add to front          |
| `.pop_back()`            | Remove from back      |
| `.pop_front()`           | Remove from front     |
| `.front()` / `.back()`   | Peek without removing |
| `.len()` / `.is_empty()` | Size checks           |
| `.contains(&val)`        | Check existence       |
| `.rotate_left(n)`        | Rotate elements       |

---

### 7. `String` & `&str` — Text

_Use everywhere — product names, descriptions, slugs, emails._

| Method                                      | What it does                 |
| ------------------------------------------- | ---------------------------- |
| `String::new()` / `String::from()`          | Create                       |
| `.push_str(&str)`                           | Append string slice          |
| `.push(char)`                               | Append char                  |
| `.len()` / `.is_empty()`                    | Size checks                  |
| `.contains(&str)`                           | Substring check              |
| `.starts_with()` / `.ends_with()`           | Prefix/suffix check          |
| `.replace(old, new)`                        | Replace substring            |
| `.trim()` / `.trim_start()` / `.trim_end()` | Strip whitespace             |
| `.to_uppercase()` / `.to_lowercase()`       | Case conversion              |
| `.split(pattern)`                           | Split into iterator          |
| `.splitn(n, pat)`                           | Split into max n parts       |
| `.chars()`                                  | Iterate over characters      |
| `.parse::<T>()`                             | Convert to another type      |
| `.as_str()`                                 | Get `&str` reference         |
| `format!()`                                 | Build strings (like sprintf) |

---

### 8. Iterator Adapters

_Works on all collections above — these are what tie everything together._

| Method                               | What it does                 |
| ------------------------------------ | ---------------------------- |
| `.map(fn)`                           | Transform each element       |
| `.filter(fn)`                        | Keep matching elements       |
| `.filter_map(fn)`                    | Filter + transform in one    |
| `.find(fn)`                          | First matching element       |
| `.position(fn)`                      | Index of first match         |
| `.fold(init, fn)`                    | Reduce to single value       |
| `.sum()` / `.product()`              | Aggregate numbers            |
| `.count()`                           | Count elements               |
| `.any(fn)` / `.all(fn)`              | Boolean checks               |
| `.take(n)` / `.skip(n)`              | Limit or offset              |
| `.enumerate()`                       | Add index to each item       |
| `.zip(iter)`                         | Pair two iterators           |
| `.flat_map(fn)`                      | Map then flatten             |
| `.flatten()`                         | Flatten nested iterables     |
| `.collect::<T>()`                    | Consume into collection      |
| `.cloned()` / `.copied()`            | Clone/copy references        |
| `.peekable()`                        | Look ahead without consuming |
| `.chain(iter)`                       | Concatenate two iterators    |
| `.max()` / `.min()`                  | Find extremes                |
| `.max_by_key(fn)` / `min_by_key(fn)` | Find extremes by field       |

---

## Quick Decision Guide

```
Need ordered list?         → Vec
Need key → value lookup?   → HashMap
Need sorted key → value?   → BTreeMap
Need unique values?        → HashSet
Need sorted unique values? → BTreeSet
Need a queue/pipeline?     → VecDeque
Need text?                 → String / &str
```

=============

Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.

- the use of `RUST_BACKTRACE=1` for debugging
- `unwrap_or` and `unwrap_or_else` for handling errors
  - `unwrap_or` computes the default value everytime each time even it was Some, hence it is expensive (use this only when the default value is cheap to compute or a literal value)
  - `unwrap_or_else` computes the default value only when it is None or Err, hence it is efficient (use this only when the default value is expensive to compute or a closure)
- `RUST_BACKTRACE=1` and `cargo run` for debugging
- `dotenvy` crate for loading environment variables from a `.env` file
- `env!` macro for getting environment variables

---

- `panic!` VS `unwrap()` VS `expect()`
  - `panic!` will panic
  - `unwrap()` will panic if the result is `Err` or `None`
  - `expect()` will panic if the result is `Err` or `None` but it will print the message passed to it
- `unwrap_or()` and `unwrap_or_else()`
  - `unwrap_or` computes the default value everytime each time even it was Some, hence it is expensive (use this only when the default value is cheap to compute or a literal value)
  - `unwrap_or_else` computes the default value only when it is None or Err, hence it is efficient (use this only when the default value is expensive to compute or a closure)
- `?` VS `match`
  - `?` operator is used to propagate the error to the caller
  - `match` is used to handle the error in the caller
  - `match` is more verbose and flexible but `?` operator is concise and idiomatic in Rust (use this only when the error is to be propagated)

---

- `map()` VS `map_err()`
  - `map()` transforms the value if `Ok(value)` to `Ok(new_value)`
  - `map_err()` transforms the error if `Err(error)` to `Err(new_error)`

- `and_then()`
  - `and_then()` is used to chain two operations that return `Result`
  - it takes a closure that takes the value from the first `Result` and returns a new `Result`
  - if the first `Result` is `Err`, it returns the error immediately
