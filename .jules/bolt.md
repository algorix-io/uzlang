## 2024-05-23 - Reuse HashMap Scopes
**Learning:** In interpreter loops (`Stmt::For`), allocating a new `HashMap` for scope in every iteration causes significant allocator churn. Reusing a single `HashMap` and clearing it provides ~10% performance improvement in loop-heavy code.
**Action:** When implementing block scopes in loops, prefer clearing and reusing the scope container over re-allocating it.

## 2025-01-28 - String concatenation overhead with `format!` macro
**Learning:** Using the `format!("{}{}”, a, b)` macro inside tight execution loops introduces substantial performance overhead due to runtime formatting string parsing and multiple internal allocations. In an AST interpreter evaluating strings, this significantly penalizes tight loops.
**Action:** Use manual `String::with_capacity(a.len() + b.len())` paired with `.push_str()` for string concatenation. Additionally, adding fast-path checks for empty strings (`"" + str` or `str + ""`) avoids memory allocation entirely by just cloning the `Rc<str>` of the non-empty operand.
