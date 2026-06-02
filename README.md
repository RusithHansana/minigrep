# minigrep

A minimal command-line grep-like tool built in Rust, following **Chapter 12 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)** book.

This project demonstrates core Rust concepts through building a real-world CLI tool: reading files, handling command-line arguments, error handling, environment variables, and writing tests.

---

## 📸 Demo

<!-- TODO: Replace the path below with your actual screenshot -->
<img width="2766" height="1536" alt="Image" src="https://github.com/user-attachments/assets/f2a143c5-5610-4744-a7fc-ea43fe6584b2" />

---

## ✨ Features

- 🔍 **Search** for a query string across lines of a file
- 🔠 **Case-sensitive** search by default
- 🔡 **Case-insensitive** search via an environment variable (`IGNORE_CASE`)
- ⚠️ **Helpful error messages** printed to `stderr`
- 🧪 **Unit tests** for both search modes

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

### Clone & Build

```bash
git clone https://github.com/your-username/minigrep.git
cd minigrep
cargo build
```

---

## 🛠️ Usage

```bash
cargo run -- <query> <file_path>
```

### Case-Sensitive Search (default)

```bash
cargo run -- nobody poem.txt
```

**Output:**
```
I'm nobody! Who are you?
Are you nobody, too?
```

### Case-Insensitive Search

Set the `IGNORE_CASE` environment variable to enable case-insensitive search:

```bash
IGNORE_CASE=1 cargo run -- nobody poem.txt
```

**Output:**
```
I'm nobody! Who are you?
Are you nobody, too?
```

---

## 📁 Project Structure

```
minigrep/
├── src/
│   ├── main.rs      # Entry point — argument parsing, Config, run()
│   └── lib.rs       # Core logic — search() and search_case_insensitive()
├── poem.txt         # Sample input file (Emily Dickinson poem)
├── Cargo.toml
└── README.md
```

---

## 🧪 Running Tests

```bash
cargo test
```

The library includes unit tests for both case-sensitive and case-insensitive search:

```rust
#[test]
fn case_sensitive() { ... }

#[test]
fn case_insensitive() { ... }
```

---

## 📖 Concepts Covered (Chapter 12)

| Concept | Where Applied |
|---|---|
| Accepting CLI arguments | `std::env::args()` in `main.rs` |
| Reading files | `std::fs::read_to_string()` in `run()` |
| Error handling with `Result` | `Config::build()` returns `Result` |
| Separation of concerns | Logic split between `main.rs` and `lib.rs` |
| Environment variables | `IGNORE_CASE` via `std::env::var()` |
| Writing to `stderr` | `eprintln!()` for error output |
| TDD with unit tests | Tests in `lib.rs` using `#[cfg(test)]` |
| Lifetimes | `search<'a>` ties output lifetime to `contents` |

---

## 📄 License

This project is for learning purposes, following the examples in [The Rust Programming Language](https://doc.rust-lang.org/book/) book.
