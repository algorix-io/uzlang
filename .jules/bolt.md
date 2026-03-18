## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-28 - String concatenation performance
**Learning:** `format!("{l}{r}")` was used for string concatenation and implicitly caused multiple re-allocations which were slow in tight loops where a large string is built iteratively (like `a = a + "a"`).
**Action:** Use `String::with_capacity(l.len() + r.len())` followed by `.push_str()` instead of `format!()` macro for predictable string concatenations.

## 2026-03-18 - Argument Ownership and In-place Array Updates
**Learning:** Passing arguments to functions by cloning 'Value' (which contains Rc) adds significant overhead in tight loops. Evaluated arguments can be consumed as an iterator to transfer ownership. Furthermore, 'qosh' (array append) can be optimized from O(N) to amortized O(1) by using 'Rc::make_mut' to perform in-place updates when the array is not shared.
**Action:** Use 'into_iter()' for argument handling in interpreters and prefer 'Rc::make_mut' for COW data structures to allow in-place mutations when possible.
