## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-28 - String concatenation performance
**Learning:** `format!("{l}{r}")` was used for string concatenation and implicitly caused multiple re-allocations which were slow in tight loops where a large string is built iteratively (like `a = a + "a"`).
**Action:** Use `String::with_capacity(l.len() + r.len())` followed by `.push_str()` instead of `format!()` macro for predictable string concatenations.

## 2026-03-02 - Function Call Optimization
**Learning:** Using `into_iter()` on evaluated function arguments allows transferring ownership to the function scope or native function handlers. This avoids cloning the `Value` enum (and its internal `Rc`s) for every argument in every function call.
**Action:** When evaluating a list of expressions into a vector (like function arguments), consume the vector via `into_iter()` if the values are only needed once.

## 2026-03-02 - Array Append Optimization
**Learning:** In the `qosh` (append) native function, using `Rc::make_mut` on an owned `Rc<Vec<Value>>` (possible after the `into_iter()` optimization) allows O(1) in-place updates when the array is unshared.
**Action:** Use COW (Copy-On-Write) semantics with `Rc::make_mut` to optimize collection updates when the interpreter has unique ownership of the collection.
