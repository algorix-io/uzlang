# Sentinel's Security Journal

## 2025-05-23 - Broken/Incomplete SSRF Protection
**Vulnerability:** The SSRF protection logic was partially implemented but broken, preventing compilation and potentially leaving a false sense of security. It contained redundant and syntactically incorrect checks for IPv6 addresses inside a loop.
**Learning:** Broken security code is worse than no security code because it blocks development while offering no protection. Redundant logic (re-implementing checks that a helper function already does) increases the surface area for bugs.
**Prevention:** Centralize security checks in helper functions (like `is_safe_ip`) and rely on them exclusively. Ensure all security features are fully implemented and tested (including compilation) before merging.

## 2026-03-02 - DNS Rebinding TOCTOU Vulnerability Fix
**Vulnerability:** The SSRF protection mechanism had a Time-of-Check Time-of-Use (TOCTOU) vulnerability. The `is_safe_url` function validated the DNS resolution to ensure the IP was safe, but then used a shared `reqwest::blocking::Client` with the original URL, triggering a second DNS resolution. This allowed a DNS rebinding attack where the attacker's DNS server returns a safe IP during validation and an internal IP during the fetch.
**Learning:** Validating a URL and then fetching it natively without pinning the validated IP is inherently vulnerable to DNS Rebinding.
**Prevention:** Create a new HTTP client pinned specifically to the validated IP using `reqwest::blocking::ClientBuilder::new().resolve(...)`. This ensures the actual request uses the exact IP that was validated, mitigating TOCTOU and SSRF bypasses.

## 2026-03-03 - Fixing Broken SSRF/TOCTOU Implementation
**Vulnerability:** The native functions `internet_ol` and `internet_yoz` were using non-existent helper functions (`is_safe_url`) and non-existent struct fields (`self.client`), effectively breaking networking and potentially bypassing the intended IP-pinning security mechanism.
**Learning:** Security helpers like `create_safe_client` are useless if they are not correctly integrated into the actual points of entry (the native functions). Compilation errors in security-critical paths must be resolved immediately to ensure the protection is active.
**Prevention:** Always verify that security-related refactors actually compile and that the native functions are correctly calling the hardened helpers with the pinned parameters (e.g., using `pinned_url` instead of the user-provided `url_str`).
