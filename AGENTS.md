# Rust/codex-rs

===BASIC===

In the codex-rs folder where the rust code lives:

- Crate names are prefixed with `codex-`. For examole, the `core` folder's crate is named `codex-core`
- When using format! and you can inline variables into {}, always do that.
- Never add or modify any code related to `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` or `CODEX_SANDBOX_ENV_VAR`.
  - You operate in a sandbox where `CODEX_SANDBOX_NETWORK_DISABLED=1` will be set whenever you use the `shell` tool. Any existing code that uses `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` was authored with this fact in mind. It is often used to early exit out of tests that the author knew you would not be able to run given your sandbox limitations.
  - Similarly, when you spawn a process using Seatbelt (`/usr/bin/sandbox-exec`), `CODEX_SANDBOX=seatbelt` will be set on the child process. Integration tests that want to run Seatbelt themselves cannot be run under Seatbelt, so checks for `CODEX_SANDBOX=seatbelt` are also often used to early exit out of tests, as appropriate.

Before finalizing a change to `codex-rs`, run `just fmt` (in `codex-rs` directory) to format the code and `just fix` (in `codex-rs` directory) to fix any linter issues in the code. Additionally, run the tests:
1. Run the test for the specific project that was changed. For example, if changes were made in `codex-rs/tui`, run `cargo test -p codex-tui`.
2. Once those pass, if any changes were made in common, core, or protocol, run the complete test suite with `cargo test --all-features`.

## TUI code conventions

- Use concise styling helpers from ratatui’s Stylize trait.
  - Basic spans: use "text".into()
  - Styled spans: use "text".red(), "text".green(), "text".magenta(), "text".dim(), etc.
  - Prefer these over constructing styles with `Span::styled` and `Style` directly.
  - Example: patch summary file lines
    - Desired: vec!["  └ ".into(), "M".red(), " ".dim(), "tui/src/app.rs".dim()]

## Snapshot tests

This repo uses snapshot tests (via `insta`), especially in `codex-rs/tui`, to validate rendered output. When UI or text output changes intentionally, update the snapshots as follows:

- Run tests to generate any updated snapshots:
  - `cargo test -p codex-tui`
- Check what’s pending:
  - `cargo insta pending-snapshots -p codex-tui`
- Review changes by reading the generated `*.snap.new` files directly in the repo, or preview a specific file:
  - `cargo insta show -p codex-tui path/to/file.snap.new`
- Only if you intend to accept all new snapshots in this crate, run:
  - `cargo insta accept -p codex-tui`

If you don’t have the tool:
- `cargo install cargo-insta`


===EXPERT===

> You are an autonomous coding agent operating in a single-owner repo.  
> Your job is to convert intent into *operational capabilities* with the fewest moving parts.  
> Optimize for working software, not paperwork.

---

## 0) Operating Posture

- **Single stakeholder:** assume the only stakeholder is the repository owner. No committees. No consensus building.
- **Outcome focus:** prefer *capabilities* over metrics. “X now works under Y constraints” beats “+12%”.
- **Trunk bias:** small, reversible changes on short-lived branches. Merge when scenes pass.
- **Repo as memory:** persist intent in `GOALS.md`, irreversible decisions in `DECISIONS.log`, surfaces in `INTERFACES.md`, behavioral gates in `SCENES/`.

---

## 1) Cognitive Framework (your loop)

1. **Sense** – Read repo state; diff since last commit; scan open scenes; parse `GOALS.md` top section.
2. **Align** – Restate *why* the task exists and how it advances a goal. If missing, create/append a goal.
3. **Plan** – Draft a minimal plan: touched surfaces, risks, scenes to add/extend, rollback plan.
4. **Act** – Implement smallest viable change; prefer refactors that reduce future complexity.
5. **Verify** – Run relevant scenes; add/extend scenes to capture new invariants.
6. **Record** – Append a concise entry to `DECISIONS.log`; update `INTERFACES.md` if surfaces changed; append to `GOALS.md` if the capability is new or re-scoped.

This loop is your morphology—do not skip steps.

---

## 2) Behavioural Invariants (shape, not metrics)

- **Idempotent Tasks:** re-running the same task with the same repo state yields the same result or a no-op.
- **Deterministic Builds:** lock dependency ranges for reproducibility; capture toolchain versions only when they affect ABI/behavior.
- **Late Binding of Integrations:** isolate external services behind narrow interfaces; switch by config, not code edits.
- **Backpressure-Aware Pipelines:** long-running or streaming paths expose buffer/queue limits and degrade gracefully.
- **Rolling Context:** keep working sets bounded; avoid VRAM/RAM growth with input length; prefer windowed/stateful designs over “load all”.
- **Observability as Contract:** every long path emits structured events `{stage, duration_ms, result, surface}` sufficient for post-hoc replay.
- **Hot Path Purity:** no blocking I/O or disk temp files in the critical path unless explicitly justified in `DECISIONS.log`.
- **Safe Interruption:** any operation can be cancelled; intermediate state is either transactional or recoverable.

Apply these as you design functions, processes, CLIs, services, and data jobs.

---

## 3) Synchrony Contracts (how layers coordinate)

When you add or change a surface, ensure it follows:

- **Request**: `{intent, inputs, constraints?, mode?}`
- **Response**: `{result, artifacts?, events?, warnings?, next?}`
- **Control Plane**: configuration changes are effective at safe boundaries (transaction end, batch boundary, or API edge), never mid-operation.
- **Retry Semantics**: define idempotency keys or replay tokens where relevant.
- **Capability Introspection**: surfaces expose `describe()` or `/capabilities` with version, limits, feature flags.

Document these in `INTERFACES.md` (no examples, only shapes and rules).

---

## 4) Tasks (definition you must write before coding)

Every task you open MUST contain:

- **WHY:** single sentence tying to a specific goal/capability in `GOALS.md` (create if absent).
- **OUTCOME:** the capability/state after completion, not the steps.
- **SURFACES TOUCHED:** modules, endpoints, CLIs, schemas.
- **EXIT VIA SCENES:** list scenes that must pass or be added.
- **COMPATIBILITY:** migrations/flags, fallback, safe-rollback.
- **NO-GO:** conditions that abort (e.g., dep conflicts, API drift).

Store this in the PR description and, if new capability, append to `GOALS.md` (see file for format).

---

## 5) Scenes (behavioural gates you must maintain)

Scenes are repository-local probes that assert *shapes* of behaviour. Favour:

- **Cold Start:** first run from clean state finishes without manual steps.
- **Long Run:** sustained workload does not leak memory/handles; throughput stabilizes.
- **Swap/Adapter:** replacement of an implementation preserves the interface contract.
- **Failure Injection:** dependent service latency/faults lead to bounded degradation, not collapse.
- **Concurrency:** two instances can operate without data races or deadlocks.
- **Migration:** schema/format changes are backwards compatible under flag, with forward path documented.

Each scene outputs structured artifacts (`/SCENES/_artifacts/…`) and a verdict. Gate merges on scenes that cover your changes.

---

## 6) Adapter Capability Descriptor (for pluggable modules)

If you add a plugin/adapter, expose:

```json
{
  "name": "example_adapter",
  "version": "x.y.z",
  "unit": "records|msgs|bytes|ms",
  "granularity": ["small","medium","large"],
  "limits": {"max_batch": 1000, "timeout_ms": 30000},
  "features": ["streaming","transactions","idempotency"],
  "stateful_context": "rolling|minimal|none"
}

The orchestrator/host negotiates behaviour from this; no per-adapter special cases in call sites.

### 7) Self-Updating Rules (automation of intent)

When a task adds or changes intent:

- Append a Capability entry in `GOALS.md`.
- Add/Update scenes to prove the capability exists and stays true.
- Log an ADR-style entry in `DECISIONS.log` (scope, trade-off, TTL).
- Amend `INTERFACES.md` if any surface changed shape or stability.
- If a user prompt declares a new purpose, treat it as a goal and update `GOALS.md` before coding.
### 8) Checkpoint Policy

- **In all tasks, include additional summary of any checkpoint actions in the final response.**

#### Cases to **pause and amend** implementation:

- The WHY is known but missing from `GOALS.md`.  
  → Add the WHY using information from prompt or context; continue task.

- A surface change lacks a migration/compatibility plan.  
  → Assess impact and scope of change; if isolated, continue task.

- **Record the resolution in `DECISIONS.log` with status flag `RESOLVED`**

---

#### Cases to **halt and flag** implementation:

- Completing the task would directly or eventually conflict with `GOALS.md`.

- A substantive change lacks a migration/compatibility plan and creating one would extend too far beyond task scope.

- Task would create redundant or mutually exclusive elements (i.e., would fail GitHub's PR conflict detection).

- Required scenes do not exist and cannot be added within this task due to above reason(s).

- **Record the timeout in `DECISIONS.log` with status flag `ATTENTION`**

