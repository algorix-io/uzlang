## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-24 - Optimize String Concatenation
**Learning:** In interpreter evaluations (`evaluate_binary`), string concatenation using the `format!()` macro causes unnecessary allocation overhead because it allocates a new `String` object, writes to it, and cannot easily make use of specific string combinations. Pre-allocating `String::with_capacity()` combined with `push_str()`/`write!()` and avoiding empty string allocations provides better performance.
**Action:** When performing dynamic string concatenations inside an interpreter or tight loops, avoid `format!()` if possible. Use `String::with_capacity()` with exact known sizes and write methods, and include fast paths to skip work on empty strings.

## 2026-03-06 - Pre-allocate Collections in Interpreter Loops
**Learning:** During expression evaluation (`Expr::Array`, `Expr::Call`) and user function invocation, using `Vec::new()` or `HashMap::new()` causes reallocation overhead when adding elements, even though the final size is known upfront (`elements.len()`, `args.len()`, `params.len()`). Pre-allocating exact capacities avoids this overhead.
**Action:** Always use `Vec::with_capacity()` or `HashMap::with_capacity()` when the total number of elements to be inserted is known ahead of time, especially in critical paths like expression evaluation or function invocation scopes.
