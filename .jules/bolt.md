## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-28 - String concatenation performance
**Learning:** `format!("{l}{r}")` was used for string concatenation and implicitly caused multiple re-allocations which were slow in tight loops where a large string is built iteratively (like `a = a + "a"`).
**Action:** Use `String::with_capacity(l.len() + r.len())` followed by `.push_str()` instead of `format!()` macro for predictable string concatenations.

## 2024-06-05 - Optimized array appending and function calls
**Learning:** Evaluated function arguments were previously being cloned into the function's scope, causing unnecessary  increments. Using  when populating the scope avoids this. Similarly, the  (append) function was cloning the entire array before modification. Using  allows O(1) in-place updates for unshared arrays, speeding up array construction by ~18%.
**Action:** Use  to transfer ownership of evaluated values into scopes or collections. Use  for efficient copy-on-write modifications.


## 2024-06-05 - Optimized array appending and function calls
**Learning:** Evaluated function arguments were previously being cloned into the function's scope, causing unnecessary Rc increments. Using into_iter() when populating the scope avoids this. Similarly, the qosh (append) function was cloning the entire array before modification. Using Rc::make_mut allows O(1) in-place updates for unshared arrays, speeding up array construction by ~18%.
**Action:** Use into_iter() to transfer ownership of evaluated values into scopes or collections. Use Rc::make_mut for efficient copy-on-write modifications.
