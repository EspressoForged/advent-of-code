# Implementation & Safety Standards (Expert Level)

This configuration refines the global standards for the `src` directory, focusing on implementation rigor and low-level safety.

## 1. Safety & The "Unwrap" Ban
- **Strict Prohibition:** `unwrap()` and `expect()` are forbidden in `src/` outside of `#[cfg(test)]` blocks. 
- **Alternative:** Use the `?` operator for propagation, or `Result`/`Option` combinators (`unwrap_or_else`, `map_err`) for recovery.
- **Invariant Documentation:** If a panic is technically impossible due to a proven invariant (e.g., bounds checked by a previous loop), use `expect("INVARIANT: <reason>")` and provide a `// SAFETY:` or `// INVARIANT:` comment explaining why.

## 2. Parsing & External Data
- **Nom-First:** For all structured text, use `nom`. It is faster, safer, and more maintainable than regex or manual slicing.
- **Validation at Boundary:** Data should be validated and transformed into domain-specific types (using the Newtype pattern) as soon as it enters the system.

## 3. Concurrency & Parallelism
- **Rayon:** Use `rayon` for data-parallel operations on iterators.
- **Send/Sync:** Ensure all shared data structures correctly implement `Send` and `Sync`.
- **Lock Contention:** Prefer atomic types over `Mutex` for simple shared counters or flags to avoid lock overhead.

## 4. Performance Goals
- **Allocations:** Minimize heap allocations in hot loops. Use `SmallVec` or `ArrayVec` if the size is bounded and small.
- **Complexity:** Document the Big-O complexity of critical algorithms in doc comments.
- **Profile-Driven Optimization (PDO):** Use tools like `cargo flamegraph` to identify bottlenecks before optimizing.
