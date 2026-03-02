## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.
## 2024-03-02 - Optimize Array Element Updates
**Learning:** Updating array elements with `arr[idx] = val` previously triggered a clone of the array in `get_variable` which increased the `Rc` count, preventing `Rc::make_mut` from performing O(1) in-place updates and leading to O(N) reallocation on every assignment. Accessing the value directly in the environment stack via mutable reference avoids incrementing the `Rc` count.
**Action:** When updating elements in Rc-wrapped structures in the AST evaluator, locate the variable directly in the scope stack to borrow it mutably rather than fetching an owned clone first.
