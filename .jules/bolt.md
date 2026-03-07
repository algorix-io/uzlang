## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2024-05-24 - String Concatenation Bottlenecks
**Learning:** Using the `format!("{}{}", a, b)` macro for simple string concatenation in the core interpreter loop (like `evaluate_binary` `+` operator) is noticeably slower and causes more memory allocator overhead than using `String::with_capacity(a.len() + b.len())` and `.push_str()`. This macro overhead is magnified when running tight loops in UzLang.
**Action:** Replace `format!` usage with direct `String::with_capacity` and `push_str` methods for string concatenation, especially in hot paths like the interpreter's evaluation logic.
