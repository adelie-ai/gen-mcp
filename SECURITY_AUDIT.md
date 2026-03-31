# Security Audit — genmcp

**Date:** 2026-03-31
**Scope:** Generic MCP server framework/generator

---

## Critical Severity

### 1. Hardcoded Debug Log Path

**File:** `src/main.rs:40`

```rust
.open("/home/dave/projects/genmcp/.cursor/debug.log")
```

Hardcoded absolute path to a specific user's home directory in production code.

**Recommendation:** Remove or replace with standard logging (e.g. `tracing` with env filter). Never hardcode user-specific paths.

---

## Medium Severity

### 2. WebSocket Auth Bypass When Unconfigured

**File:** `src/main.rs:418-420`

When no JWT secret or JWKS verifier is configured, authentication succeeds for any token. This should require explicit opt-in.

**Recommendation:** Default to rejecting connections when auth is not configured. Require an explicit `--no-auth` flag to disable.

---

### 3. No Config File Size Limit

Config files are read without size limits. A multi-GB TOML file causes OOM.

**Recommendation:** Check file size before parsing (e.g. reject > 1 MiB).

---

## Positive Findings

- `Command::new()` used directly (no shell invocation)
- Tool parameters passed via structured arguments, not shell interpolation
- JWT validation when configured
