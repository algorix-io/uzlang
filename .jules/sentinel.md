# Sentinel's Security Journal

## 2025-05-23 - Broken/Incomplete SSRF Protection
**Vulnerability:** The SSRF protection logic was partially implemented but broken, preventing compilation and potentially leaving a false sense of security. It contained redundant and syntactically incorrect checks for IPv6 addresses inside a loop.
**Learning:** Broken security code is worse than no security code because it blocks development while offering no protection. Redundant logic (re-implementing checks that a helper function already does) increases the surface area for bugs.
**Prevention:** Centralize security checks in helper functions (like `is_safe_ip`) and rely on them exclusively. Ensure all security features are fully implemented and tested (including compilation) before merging.

## 2026-03-02 - DNS Rebinding TOCTOU Vulnerability Fix
**Vulnerability:** The SSRF protection mechanism had a Time-of-Check Time-of-Use (TOCTOU) vulnerability. The `is_safe_url` function validated the DNS resolution to ensure the IP was safe, but then used a shared `reqwest::blocking::Client` with the original URL, triggering a second DNS resolution. This allowed a DNS rebinding attack where the attacker's DNS server returns a safe IP during validation and an internal IP during the fetch.
**Learning:** Validating a URL and then fetching it natively without pinning the validated IP is inherently vulnerable to DNS Rebinding.
**Prevention:** Create a new HTTP client pinned specifically to the validated IP using `reqwest::blocking::ClientBuilder::new().resolve(...)`. This ensures the actual request uses the exact IP that was validated, mitigating TOCTOU and SSRF bypasses.

## 2025-05-24 - Robust IPv6 SSRF Protection
**Vulnerability:** Manual string formatting like `format!("{}:{}", host, port)` for `to_socket_addrs()` can fail or be bypassed with certain IPv6 literal formats (e.g., those already containing colons but missing brackets).
**Learning:** Using `(host, port).to_socket_addrs()` is the standard and most robust way in Rust to handle DNS resolution, as it correctly parses various host formats including bracketed IPv6 literals.
**Prevention:** Avoid manual string concatenation for address resolution. Use the `ToSocketAddrs` implementation for tuples of `(str, u16)` to ensure consistent and secure parsing of all host types.
