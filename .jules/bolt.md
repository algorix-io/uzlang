## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-23 - Rc::make_mut cloning overhead
**Learning:** When using `Rc::make_mut` on unshared data (like array buffers in `Interpreter`), first calling a method like `get_variable` that returns an `Rc` clone increments the reference count. This causes `Rc::make_mut` to defensively deep clone the entire buffer, turning an expected O(1) in-place update into a hidden O(N) operation.
**Action:** Always fetch the target value by mutable reference `&mut` (e.g., from the environment map directly via `get_mut`) rather than cloning the `Rc`, ensuring the reference count remains 1 so `Rc::make_mut` works in O(1) time.
