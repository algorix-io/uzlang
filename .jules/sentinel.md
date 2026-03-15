# Sentinel's Security Journal

## 2025-05-23 - Broken/Incomplete SSRF Protection
**Vulnerability:** The SSRF protection logic was partially implemented but broken, preventing compilation and potentially leaving a false sense of security. It contained redundant and syntactically incorrect checks for IPv6 addresses inside a loop.
**Learning:** Broken security code is worse than no security code because it blocks development while offering no protection. Redundant logic (re-implementing checks that a helper function already does) increases the surface area for bugs.
**Prevention:** Centralize security checks in helper functions (like `is_safe_ip`) and rely on them exclusively. Ensure all security features are fully implemented and tested (including compilation) before merging.

## 2026-03-02 - DNS Rebinding TOCTOU Vulnerability Fix
**Vulnerability:** The SSRF protection mechanism had a Time-of-Check Time-of-Use (TOCTOU) vulnerability. The `is_safe_url` function validated the DNS resolution to ensure the IP was safe, but then used a shared `reqwest::blocking::Client` with the original URL, triggering a second DNS resolution. This allowed a DNS rebinding attack where the attacker's DNS server returns a safe IP during validation and an internal IP during the fetch.
**Learning:** Validating a URL and then fetching it natively without pinning the validated IP is inherently vulnerable to DNS Rebinding.
**Prevention:** Create a new HTTP client pinned specifically to the validated IP using `reqwest::blocking::ClientBuilder::new().resolve(...)`. This ensures the actual request uses the exact IP that was validated, mitigating TOCTOU and SSRF bypasses.

## 2026-03-03 - Incomplete Integration of SSRF Protection
**Vulnerability:** Although the `create_safe_client` helper was implemented to prevent SSRF and DNS rebinding, it was not actually used in the `internet_ol` and `internet_yoz` functions. These functions instead referenced a non-existent `is_safe_url` helper and a missing `self.client` field, causing both a compilation error and a security gap.
**Learning:** A security utility is only effective if it is correctly integrated into all relevant code paths. Inconsistent application of security logic can lead to bypasses or broken builds.
**Prevention:** Use automated tests (like `test_ssrf_manual.uz`) to verify that security boundaries are enforced at the functional level, and ensure all networking primitives use the centralized security helpers.
