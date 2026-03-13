## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-28 - String concatenation performance
**Learning:** `format!("{l}{r}")` was used for string concatenation and implicitly caused multiple re-allocations which were slow in tight loops where a large string is built iteratively (like `a = a + "a"`).
**Action:** Use `String::with_capacity(l.len() + r.len())` followed by `.push_str()` instead of `format!()` macro for predictable string concatenations.

## 2024-06-05 - Optimized array appending and ownership-based argument handling
**Learning:** Using `Rc::make_mut` for array updates allows O(1) amortized in-place modification when the `Rc` is unshared. Additionally, consuming arguments via `into_iter()` in function calls avoids redundant `Value` and `Rc` clones.
**Action:** Always prefer `Rc::make_mut` for collection updates in the interpreter. Use `into_iter()` on evaluated arguments to transfer ownership to native or user functions.
