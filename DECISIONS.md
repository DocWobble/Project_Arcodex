# DECISIONS.log

> ADR-style, but ruthless. One entry per consequential choice.  
> Append-only; if superseded, add `Superseded-By:` and status `REPLACED`.

## Entry Template

### [YYYY-MM-DD] <short handle>
- **Context:** <why a choice was necessary; constraints at play>
- **Decision:** <the chosen path in one or two sentences>
- **Alternatives:** <bulleted, only those genuinely considered>
- **Trade-offs:** <what we accept to get the outcome>
- **Scope:** <surfaces or modules affected>
- **Impact:** <capabilities enabled/disabled; risks introduced>
- **TTL / Review:** <when to re-evaluate, if applicable>
- **Status:** ACTIVE | REPLACED | REJECTED | BLOCKED
- **Links:** <PRs, scenes, interface entries, goal names>

*(New entries go on top. Keep each under ~20 lines.)*

### [2025-08-17] missing-sandbox-error
- **Context:** `codex-cli` panicked when the `codex-linux-sandbox` binary was missing.
- **Decision:** Return a descriptive error instead of panicking so users understand the failure.
- **Alternatives:** Keep using `.expect` and crash.
- **Trade-offs:** Slightly more code; reliance on error handling.
- **Scope:** `codex-rs/cli` sandbox command path.
- **Impact:** CLI fails gracefully when Landlock sandbox is requested without the binary.
- **TTL / Review:** Revisit if sandbox architecture changes.
- **Status:** ACTIVE
- **Links:** See PR for missing sandbox executable handling.

### [2025-08-19] login-result-handling
- **Context:** Login helpers exited the process directly, making reuse and testing difficult.
- **Decision:** Return `Result`/status enums from login helpers and manage exits in `main.rs`.
- **Alternatives:** Keep process termination inside helper functions.
- **Trade-offs:** More boilerplate in the caller; functions now expose additional types.
- **Scope:** `codex-rs/cli` login module and main entrypoint.
- **Impact:** Enables programmatic control over login flows and clearer testing.
- **TTL / Review:** Revisit when authentication flow changes.
- **Status:** ACTIVE
- **Links:** goal result-based-login

