# Set Cover Solver TUI in Rust

A terminal-based application written in **Rust** for solving the **weighted set cover problem** using both a **greedy heuristic** and an **exact algorithm**.

This project is meant to showcase:
- algorithm design,
- combinatorial optimization,
- file parsing and input validation,
- terminal UI development in Rust.

## Features

- terminal user interface,
- loading problem instances from text files,
- greedy algorithm for fast approximate solutions,
- exact algorithm for optimal solutions,
- execution time measurement,
- result presentation directly in the TUI.

## Tech stack

- Rust
- Cargo
- ratatui
- crossterm

## Repository structure

```text
.
├── .github/
│   └── workflows/
│       └── ci.yml
├── examples/
│   ├── sample_medium.txt
│   └── sample_small.txt
├── src/
│   ├── algorithms.rs
│   ├── data.rs
│   ├── main.rs
│   └── ui.rs
├── .gitignore
├── Cargo.toml
├── LICENSE
└── README.md
```

## Input format

The first line defines the universe:

```text
[1 2 3 4 5]
```

Each next line defines a subset and its cost:

```text
[[1 2] 3]
[[2 3 4] 4]
[[4 5] 2]
```

Meaning:
- `[1 2 3 4 5]` is the universe,
- `[[1 2] 3]` means subset `{1, 2}` with cost `3`.

Ready-to-use example instances are available in the `examples/` directory.

## How to run

Clone the repository and enter the project directory:

```bash
git clone <your-repo-url>
cd set-cover-tui-rust
```

Run the application:

```bash
cargo run
```

Build release binary:

```bash
cargo build --release
```

## How to use

1. Start the application with `cargo run`.
2. Choose `Load data` in the TUI.
3. When prompted in the terminal, provide a file path, for example:

```text
examples/sample_small.txt
```

4. Run either:
   - `Optimal algorithm`
   - `Greedy algorithm`

The app will display selected subsets, total cost, and execution time.

## Quality checks

Format check:

```bash
cargo fmt --check
```

Linting:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Tests:

```bash
cargo test --all-targets --all-features
```

## License

MIT
