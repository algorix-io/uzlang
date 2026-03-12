## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-28 - String concatenation performance
**Learning:** `format!("{l}{r}")` was used for string concatenation and implicitly caused multiple re-allocations which were slow in tight loops where a large string is built iteratively (like `a = a + "a"`).
**Action:** Use `String::with_capacity(l.len() + r.len())` followed by `.push_str()` instead of `format!()` macro for predictable string concatenations.

## 2024-06-15 - Optimized Array Appending (qosh)
**Learning:** Cloning the entire array on every `qosh` (append) operation caused O(n²) time complexity for incremental array construction.
**Action:** Use `into_iter()` on evaluated arguments and `Rc::make_mut` on the array to allow O(1) amortized in-place updates when the array is unshared. This yielded a ~20% improvement in benchmarks.
