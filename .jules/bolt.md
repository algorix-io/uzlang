## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-28 - String concatenation performance
**Learning:** `format!("{l}{r}")` was used for string concatenation and implicitly caused multiple re-allocations which were slow in tight loops where a large string is built iteratively (like `a = a + "a"`).
**Action:** Use `String::with_capacity(l.len() + r.len())` followed by `.push_str()` instead of `format!()` macro for predictable string concatenations.

## 2026-03-11 - Efficient Array and String Operations
**Learning:** Using `into_iter()` to transfer ownership of evaluated arguments into function scopes or native functions like `qosh` avoids unnecessary clones of `Value` and `Rc` containers. Combined with `Rc::make_mut`, this allows for O(1) amortized in-place updates for unshared arrays.
**Action:** Always prefer moving values using `into_iter()` when they are no longer needed in the caller, and use `Rc::make_mut` for efficient collection updates.
