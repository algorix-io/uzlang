## 2025-05-18 - Interpreter Loop Allocation
**Learning:** Allocating a new `HashMap` for every iteration of a loop in an interpreter significantly impacts performance due to frequent heap allocations.
**Action:** Use object pooling or reuse a single mutable structure (clearing it between iterations) for loop scopes, especially in `Stmt::For` or similar constructs.

## 2025-05-18 - Persistent Resources in Interpreter
**Learning:** Re-initializing heavy resources like `reqwest::Client` in every function call (or forgetting to store them) is a major anti-pattern.
**Action:** Store shared resources in the `Interpreter` struct and initialize them once in `new()`.
