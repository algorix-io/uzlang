## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-28 - String concatenation performance
**Learning:** `format!("{l}{r}")` was used for string concatenation and implicitly caused multiple re-allocations which were slow in tight loops where a large string is built iteratively (like `a = a + "a"`).
**Action:** Use `String::with_capacity(l.len() + r.len())` followed by `.push_str()` instead of `format!()` macro for predictable string concatenations.

## 2024-06-05 - native function optimizations
**Learning:** Optimizing `qosh` with `Rc::make_mut` and `into_iter()` provides a significant speedup (up to 25% on `benchmark.uz`) by enabling in-place updates for unshared arrays. Fast-pathing `matn` for strings and consuming arguments via iterators in `Expr::Call` further reduces allocation churn.
**Action:** Always check if `Rc`-wrapped collections can be modified in-place using `Rc::make_mut` and prefer consuming iterators (`into_iter`) over indexing/cloning when evaluating function arguments.
