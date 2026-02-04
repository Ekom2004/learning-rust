# 🦀 Rust Systems Programming Bootcamp

> **Goal:** Learn Rust from scratch, then build a crash-resistant "execution memory" prototype

---

## Week 0: Prerequisites (Before You Start)

### Setup & Tooling
- [ ] Install Rust via [rustup](https://rustup.rs/)
- [ ] `cargo new` — Create a new project
- [ ] `cargo build` — Compile your project
- [ ] `cargo run` — Build and run
- [ ] `cargo test` — Run tests
- [ ] `cargo fmt` — Format code
- [ ] `cargo clippy` — Lint and suggestions

---

### Syntax Fundamentals
- [ ] `fn` — Functions and how to define them
- [ ] `let` bindings — Immutable by default
- [ ] `let mut` — Mutable variables
- [ ] Blocks `{}` and scope
- [ ] Expressions vs statements
- [ ] `return` vs implicit return (last expression)

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // implicit return (no semicolon)
}
```

---

### Types
- [ ] Primitives: `i32`, `u64`, `f64`, `bool`, `char`
- [ ] `String` vs `&str` — Owned vs borrowed strings
- [ ] References: `&T` and `&mut T`
- [ ] Tuples: `(i32, String, bool)`
- [ ] Arrays: `[i32; 5]` — Fixed size
- [ ] Slices: `&[T]` — View into array/vec
- [ ] `Vec<T>` — Growable array

```rust
let s: String = String::from("hello");  // owned
let slice: &str = &s;                   // borrowed
let nums: Vec<i32> = vec![1, 2, 3];     // growable
```

---

### Control Flow
- [ ] `if` / `else` / `else if`
- [ ] `loop` — Infinite loop (break to exit)
- [ ] `while` — Conditional loop
- [ ] `for` — Iterate over collections
- [ ] `match` — Pattern matching
- [ ] `break` / `continue`

```rust
for i in 0..5 {
    println!("{}", i);
}

match value {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    _ => println!("other"),
}
```

---

### Structs & Enums (Syntax)
- [ ] `struct` — Define custom types
- [ ] `enum` — Variants with optional data
- [ ] Field access with `.`
- [ ] `impl` blocks for methods
- [ ] Associated functions (`Self::new()`)

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String) -> Self {
        Self { name, age: 0 }
    }
}
```

---

### Modules & Files
- [ ] `mod` — Define modules
- [ ] `use` — Bring items into scope
- [ ] `pub` — Make items public
- [ ] Crate structure: `lib.rs`, `main.rs`

```rust
mod database;           // loads database.rs or database/mod.rs
use database::connect;  // bring function into scope
pub fn public_fn() {}   // visible outside module
```

---

### Standard Library Basics
- [x] `Option<T>` — `Some(value)` or `None`
- [ ] `Result<T, E>` — `Ok(value)` or `Err(e)`
- [ ] `Box<T>` — Heap allocation
- [ ] `Rc<T>` / `Arc<T>` — Reference counting (conceptual)
- [ ] `HashMap<K, V>` — Key-value store
- [ ] `Path` / `PathBuf` — File paths

---

### Debugging & Testing
- [ ] `println!("value: {}", x)` — Print output
- [ ] `dbg!(x)` — Debug print with location
- [ ] `#[test]` — Mark test functions
- [ ] `assert!(condition)` — Panic if false
- [ ] `assert_eq!(a, b)` — Panic if not equal

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
}
```

---

## Week 1: Core Language Mechanics

### Days 1–2: Ownership, Borrowing, Moves
**Focus:** Argue with the compiler until you understand why

- [ ] Understand the 3 ownership rules
- [ ] Write examples that fail to compile (intentionally!)
- [ ] Practice borrowing (`&T`) and mutable borrowing (`&mut T`)
- [ ] Understand move semantics and when copies happen

**Exercise Ideas:**
```rust
// Try to use a value after moving it
// Try to have two mutable references
// Try to return a reference to a local variable
```

---

### Days 3–4: Enums + Pattern Matching
**Focus:** Model states where illegal transitions are impossible

- [ ] Create enums with data variants
- [ ] Use `match` for exhaustive handling
- [ ] Build a simple state machine (e.g., `Draft → Submitted → Approved`)
- [ ] Make the compiler reject invalid state transitions

**Mini-Project:** State machine where you can't go from `Approved` back to `Draft`

---

### Days 5–6: Structs + Immutability
**Focus:** Data that can't be corrupted after creation

- [ ] Define structs with private fields
- [ ] Implement constructors (`new()`) that validate on creation
- [ ] Build an **append-only decision log**
- [ ] No mutation after a decision is "committed"

**Mini-Project:** `DecisionLog` where entries can only be added, never modified

---

### Days 7–8: Result / Option Everywhere
**Focus:** Make failure explicit and recoverable

- [ ] Replace all panics with `Result<T, E>`
- [ ] Use `Option<T>` for values that might not exist
- [ ] Practice `?` operator for error propagation
- [ ] Simulate crashes and test recovery paths

**Exercise:** Write a function that can fail 3 different ways, handle all of them

---

## Week 2: Building the Prototype

### Days 9–10: Traits for Persistence
**Focus:** Define behaviors, not implementations

- [ ] Create `Persistable` trait (serialize to disk)
- [ ] Create `Replayable` trait (reconstruct from log)
- [ ] Implement traits for your decision types
- [ ] Test: serialize → kill process → deserialize → same state

**Traits to implement:**
```rust
trait Persistable {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> where Self: Sized;
}

trait Replayable {
    fn replay(decisions: &[Decision]) -> Self;
}
```

---

### Days 11–12: Execution Memory Prototype
**Focus:** Build something that survives crashes

- [ ] Create the main "execution memory" struct
- [ ] Log every decision to disk before executing
- [ ] Implement crash detection on startup
- [ ] Replay decisions to restore state
- [ ] **Test:** Kill the process mid-execution, restart, verify it resumes

**The Core Loop:**
```
1. Load existing decisions from disk
2. Replay to reconstruct state
3. Accept new decision
4. Persist decision to disk (fsync!)
5. Execute decision
6. Goto 3
```

---

### Days 13–14: Refactor & Harden
**Focus:** Let the compiler be your enforcer

- [ ] Remove all `unwrap()` and `expect()` calls
- [ ] Add proper error types with `thiserror`
- [ ] Make invalid states unrepresentable
- [ ] Add compile-time guarantees where possible
- [ ] Write documentation comments

**Checklist:**
- [ ] Can construct an invalid state? ❌ Should be impossible
- [ ] Can skip persisting before executing? ❌ Should be impossible
- [ ] Can corrupt the log? ❌ Should be impossible

---

## 📚 Resources

| Resource | Link |
|----------|------|
| The Rust Book | https://doc.rust-lang.org/book/ |
| Rustlings Exercises | https://github.com/rust-lang/rustlings |
| Rust by Example | https://doc.rust-lang.org/rust-by-example/ |
| Error Handling | https://doc.rust-lang.org/book/ch09-00-error-handling.html |

---

## 🎯 Success Criteria

By Day 14, you should be able to:

1. **Explain** ownership/borrowing without looking anything up
2. **Design** state machines that reject invalid transitions at compile time
3. **Handle** all errors explicitly with `Result`
4. **Build** a system that survives kills and restarts cleanly
5. **Read** the compiler's errors and know exactly what to fix

---

## 💡 Tips

- **Day 1 mindset:** The compiler is not your enemy—it's your pair programmer
- **When stuck:** Read the error message fully, it usually tells you what to do
- **Commit often:** Track your learning progress with git
- **Break things:** The best way to learn Rust is to make it fail

---

*Started: ___________*  
*Completed: ___________*
