## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-28 - String concatenation performance
**Learning:** `format!("{l}{r}")` was used for string concatenation and implicitly caused multiple re-allocations which were slow in tight loops where a large string is built iteratively (like `a = a + "a"`).
**Action:** Use `String::with_capacity(l.len() + r.len())` followed by `.push_str()` instead of `format!()` macro for predictable string concatenations.

## 2024-06-03 - Optimize argument handling in interpreter
**Learning:** Evaluated arguments in `Expr::Call` were being cloned when passed to both native and user functions. By using `into_iter()` on the argument vector, we can move the values directly into the target scope or native function, eliminating one clone per argument per call.
**Action:** Always prefer consuming owned vectors with `into_iter()` when transferring data between interpreter scopes or into native helpers.

## 2024-06-03 - In-place array updates with Rc::make_mut
**Learning:** The `qosh` (append) function previously cloned the entire array every time. Using `Rc::make_mut` allows O(1) amortized in-place updates if the array is unshared (refcount is 1), which is common for local accumulators. This resulted in a ~45% speedup in the `benchmark.uz` script.
**Action:** Use `Rc::make_mut` for all built-in functions that modify collections to enable Copy-On-Write optimizations.
