# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`jfmt` is a CLI JSON formatter written in Rust. It reads JSON from a file argument or stdin, parses it, and prints it formatted with colored syntax highlighting (object keys in bright magenta).

## Commands

```bash
cargo build --release          # Build optimized binary
cargo install --path .         # Install globally

cargo test                     # Run all tests
cargo test <test_name>         # Run a single test by name (e.g. cargo test test_format_object)
cargo test --lib               # Run only library/unit tests
cargo test --bin jfmt          # Run only binary tests

cargo fmt --check              # Check formatting (used in CI)
cargo fmt                      # Auto-format code
cargo clippy --all-targets --all-features -- -D warnings  # Lint (CI uses -D warnings)
```

## Architecture

Parsing is a two-stage pipeline:

1. **Grammar → Pest pairs** — `src/json.pest` defines the PEG grammar (Apache 2.0). The `pest_derive` macro compiles it into a parser at build time.
2. **Pest pairs → Node trait objects** — `src/parser/parse.rs` converts the flat Pest pair tree into a recursive `Box<dyn Node>` structure.

Formatting is recursive via the `Node` trait (`src/nodes/node.rs`):
- `format_as_root()` — called on the top-level value; produces the final string.
- `format_as_child(tabs: usize)` — called recursively; `tabs` tracks indentation depth.
- Indentation is 2 spaces (`TAB_SPACES = 2`).

Each JSON type has its own struct in `src/nodes/` implementing `Node`. `Object` stores `Vec<(&str, Box<dyn Node>)>` (borrowed key, owned value). `Array` stores `Vec<Box<dyn Node>>`. `String` and `Array` use lifetime parameters (`'a`) to borrow from the original input string.

The formatter entry point is `formatter::format(json: &str) -> String` (`src/formatter.rs`), which ties the two stages together and wraps parse errors as `[Error] …` strings.

`src/main.rs` wires `clap` + `clap-stdin` so the binary accepts an optional file argument; if omitted it reads from stdin.

## Key conventions

- **All tests are inline** (Rust convention, `#[cfg(test)]` blocks at the bottom of each source file). There are ~27 tests spread across `formatter.rs`, `main.rs`, and every file under `nodes/`.
- **Numbers** are stored as `f64`; scientific notation (e.g. `5e3`) is silently normalized to decimal (`5000`) in the output.
- **Color** is applied only to object keys, using raw ANSI escape codes via the `colored` crate (`bright_purple().bold()`).
- **Error handling** uses Pest's built-in error formatting; there are no custom error types.
- CI matrix covers Linux x86\_64/ARM64 and macOS x86\_64/ARM64 via the `cross` tool for releases.
