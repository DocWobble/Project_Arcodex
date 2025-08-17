# CONTRIBUTING.md

> Contribution rules for a single-owner, scene-gated repo.

## Branching

- Short-lived feature branches; prefer squash-merge.
- One capability per PR when feasible.

## Code Style

- Keep formatting defaults (e.g., Black/Prettier/gofmt); avoid subjective bikeshedding.
- Name things by purpose, not implementation (`RetryPolicy`, not `DoThing2`).

## Tests & Scenes

- Unit tests for local logic; **scenes** for behaviour across surfaces.
- PRs must add/update scenes when they affect behaviour.

## Dependencies

- New runtime/build dependencies require a `DECISIONS.log` entry.
- Pin versions only where behaviour/ABI matters; otherwise use caret/tilde ranges with lockfiles.

## Security/Privacy

- Secrets never stored in repo; use env or secret manager.
- Redact sensitive fields in artifacts; note this in `INTERFACES.md`.

## Commits

- Small, purposeful commits with clear messages (see `TASKS/README.md`).
- No “drive-by” refactors unless they reduce total complexity and are explained in the PR.

## Review (by agent)

- Self-review with checklist:
  - [ ] WHY/OUTCOME clear
  - [ ] Scenes cover change
  - [ ] Interfaces updated
  - [ ] Decision logged (if any trade-off)
  - [ ] Rollback possible

