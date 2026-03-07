# Sentinel's Security Journal

## 2024-05-23 - SSRF TOCTOU and DNS Rebinding
**Vulnerability:** The SSRF protection mechanism checked the URL's safety by resolving it once, but then used a fresh `reqwest::Client` to make the actual request. This created a Time-of-Check Time-of-Use (TOCTOU) vulnerability where a malicious DNS server could return a safe IP during the check and a private IP (like 127.0.0.1) during the actual connection (DNS Rebinding).
**Learning:** Checking a URL's safety is not enough if the resolution process is not pinned. Modern HTTP clients often re-resolve DNS for each request.
**Prevention:** Use the `resolve` method (or similar DNS pinning mechanism) in the HTTP client to force it to use the exact IP address that was validated. In Rust's `reqwest`, `ClientBuilder::resolve(host, addr)` allows mapping a hostname to a specific `SocketAddr`.

## 2025-01-24 - SSRF Bypass and DoS via Response Body
**Vulnerability:** The `internet_yoz` (HTTP POST) function was implemented with a logic error that bypassed SSRF protections by using an unrestricted, shared client instead of the safe client created during validation. Additionally, both GET and POST requests lacked response size limits, making the interpreter vulnerable to Denial of Service (DoS) attacks via memory exhaustion if a malicious server returned an extremely large response.
**Learning:** Security checks must be consistently applied across all network-facing functions. Trusting a shared client for sensitive operations like POST requests can lead to bypasses if that client is not properly restricted. Always limit resources consumed from external, untrusted sources.
**Prevention:** Ensure all network functions use the same hardened client creation logic. Implement response body limits using `.take(limit)` when reading from a network stream to prevent unbounded memory allocation.
