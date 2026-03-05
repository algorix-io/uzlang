# Sentinel's Security Journal

## 2024-05-23 - SSRF TOCTOU and DNS Rebinding
**Vulnerability:** The SSRF protection mechanism checked the URL's safety by resolving it once, but then used a fresh `reqwest::Client` to make the actual request. This created a Time-of-Check Time-of-Use (TOCTOU) vulnerability where a malicious DNS server could return a safe IP during the check and a private IP (like 127.0.0.1) during the actual connection (DNS Rebinding).
**Learning:** Checking a URL's safety is not enough if the resolution process is not pinned. Modern HTTP clients often re-resolve DNS for each request.
**Prevention:** Use the `resolve` method (or similar DNS pinning mechanism) in the HTTP client to force it to use the exact IP address that was validated. In Rust's `reqwest`, `ClientBuilder::resolve(host, addr)` allows mapping a hostname to a specific `SocketAddr`.
