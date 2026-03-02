# Global Rust Engineering Standards

This document establishes the non-negotiable standards for Rust development. Prioritize safety, performance, and idiomatic clarity.

## 1. Tooling & Automation
- **Formatting:** `cargo fmt` must be executed on every change. 
- **Linting:** `cargo clippy -- -D warnings` is the baseline. All warnings are treated as errors. Address lints by improving code, not by silencing them with `#[allow(...)]` unless strictly necessary and documented.
- **Dependency Management:** Regularly audit dependencies with `cargo audit`. Prefer established, high-quality crates (e.g., `serde`, `tokio`, `nom`).
- **Advanced Monitoring:** Use `cargo-deny` to manage crate licenses and security vulnerabilities, and `cargo-bloat` to monitor binary size regressions.

## 2. Error Handling Philosophy
- **Contextual Propagation:** Use `anyhow` for high-level error propagation. Use `.context()` or `.with_context()` at every architectural boundary (I/O, parsing, logic jumps) to provide a clear trace.
- **Library Context:** Use `thiserror` to define exhaustive, domain-specific error enums.
- **Panic Policy:** `unwrap()` and `expect()` are **forbidden** in production logic. They are only permitted in:
    1. Tests (`#[cfg(test)]`).
    2. Documented "impossible" states (preceded by `// SAFETY: <explanation>`).
    3. Prototypes (must be refactored before finalization).

## 3. Advanced Architectural Mandates
- **Type-Driven Design:** 
    - **Newtype Pattern:** Strictly avoid "primitive obsession." Wrap raw types (`usize`, `String`, `u64`) in domain-specific `struct` wrappers (e.g., `struct Coordinate(usize);`).
    - **Typestate Pattern:** Use the Typestate pattern to enforce valid state transitions at compile time (e.g., `Puzzle<Unsolved>` to `Puzzle<Solved>`).
- **Encapsulation:** 
    - Default to `pub(crate)` for all internal modules and types. Only use `pub` for items that are strictly part of the public API.
    - Use "Sealed Traits" to prevent downstream crates from implementing traits intended only for internal use.
- **Explicit Intent:** Apply `#[must_use]` to all functions where ignoring the return value is likely a logic error.

## 4. Testing & Validation
- **Unit Testing:** Every module must contain a `#[cfg(test)] mod tests` block.
- **Integration Testing:** Public APIs and multi-module flows must be tested in `tests/`.
- **Benchmark:** Performance-critical paths (e.g., AoC search algorithms) should be benchmarked using `criterion` or `divan`.
- **Validation:** Every feature is incomplete until `cargo test` passes in its entirety.

## 5. Documentation Standards
- **Exhaustive Documentation:** All `pub` items must have `///` doc comments.
- **Sections:** Non-trivial functions must include:
    - `# Errors`: When and why it fails.
    - `# Panics`: If there are any edge cases that trigger a panic.
    - `# Examples`: A runnable code block showing intended usage.
- **Module Level:** Every `mod.rs` must start with `//!` explaining the architectural role of the module.

## 6. Performance & Idiomatic Patterns
- **Zero-Cost Abstractions:** Prefer traits and generics over `dyn` unless polymorphism is dynamic by nature.
- **Functional Style:** Use iterators (`map`, `filter`, `fold`) for clarity and to assist the compiler with bounds-check elimination.
- **SIMD Policy:** For performance-critical hot loops, prioritize `std::simd` (Portable SIMD) over raw assembly or architecture-specific intrinsics.
