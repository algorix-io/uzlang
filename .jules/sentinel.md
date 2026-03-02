## 2025-05-23 - DNS Rebinding & SSRF Bypass
**Vulnerability:** The `is_safe_url` function performed string-based validation of hostnames (e.g., blocking "localhost" or "127.*"), which can be bypassed by using a public domain that resolves to a private IP (e.g., `localtest.me` -> `127.0.0.1`).
**Learning:** String matching on URLs is insufficient for security controls. Attackers can control DNS resolution to point innocuous-looking domains to internal resources.
**Prevention:** Always resolve the hostname to an IP address and validate the IP against a blocklist of private/reserved ranges before making the request. Use `std::net::ToSocketAddrs` (in Rust) or similar mechanisms. Be aware of TOCTOU (Time-of-Check Time-of-Use) issues with DNS rebinding if the HTTP client re-resolves the domain; for critical systems, use a custom resolver or pin the IP.

## 2025-05-23 - Broken/Incomplete SSRF Protection
**Vulnerability:** The SSRF protection logic was partially implemented but broken, preventing compilation and potentially leaving a false sense of security. It contained redundant and syntactically incorrect checks for IPv6 addresses inside a loop.
**Learning:** Broken security code is worse than no security code because it blocks development while offering no protection. Redundant logic (re-implementing checks that a helper function already does) increases the surface area for bugs.
**Prevention:** Centralize security checks in helper functions (like `is_safe_ip`) and rely on them exclusively. Ensure all security features are fully implemented and tested (including compilation) before merging.

## 2026-03-02 - DNS Rebinding TOCTOU Vulnerability Fix
**Vulnerability:** The SSRF protection mechanism had a Time-of-Check Time-of-Use (TOCTOU) vulnerability. The `is_safe_url` function validated the DNS resolution to ensure the IP was safe, but then used a shared `reqwest::blocking::Client` with the original URL, triggering a second DNS resolution. This allowed a DNS rebinding attack where the attacker's DNS server returns a safe IP during validation and an internal IP during the fetch.
**Learning:** Validating a URL and then fetching it natively without pinning the validated IP is inherently vulnerable to DNS Rebinding.
**Prevention:** Create a new HTTP client pinned specifically to the validated IP using `reqwest::blocking::ClientBuilder::new().resolve(...)`. This ensures the actual request uses the exact IP that was validated, mitigating TOCTOU and SSRF bypasses.
