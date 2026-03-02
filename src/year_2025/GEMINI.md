# Advent of Code: Solving Standards

Standardizes the approach for multi-year AoC solutions to ensure scalability and speed.

## 1. Architecture
- **Yearly Isolation:** Each year resides in `src/year_XXXX/`.
- **Standard Signature:** Every `solve()` function should return `Result<(u64, u64)>` to keep the runner generic.
- **Single-Part Days:** For days with only one part (like Day 25 or Day 12 of 2025), return `0` for the second value.
- **Common Utilities:** Leverage `src/utils/` for shared parsing, grid handling, and math logic to avoid code duplication.

## 2. Performance Mandate
- **Release Target:** Solutions must run in <100ms on modern hardware in `--release` mode. If a solution takes >1s, it requires an algorithmic refactor (e.g., memoization, pruning, or a better heuristic).
- **Efficiency:** Favor stack-allocated arrays and fixed-size grids over `Vec<Vec<T>>`.

## 3. The AoC Testing Cycle
- **Example First:** Copy the puzzle example into the local `tests` block. The solution is not ready until it passes the example.
- **Regression:** Once the real answer is found, add it as a test case to prevent future refactors from breaking the solution.

## 4. Input Handling
- **Pathing:** Use centralized utility functions to resolve paths (`inputs/year_XXXX/day_XX/input.txt`).
- **Streaming:** For very large inputs, consider streaming the file rather than reading the whole string.
