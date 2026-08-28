# 0001 — No human gate before upgrade execution; three-label issue lifecycle

Date: 2026-08-28 (grilling round 2, Q69–Q74 of plan-3)

## Status

Accepted

## Context

plan-2/plan-3 originally routed every detected Google Ads API release through a
human triage step: the detector created an upgrade issue at `needs-triage`, and a
human flipped it to `ready-for-agent` (or added a 👀 reaction) before the issue
worker would dispatch the upgrade workflow. The stated goal of the automation is
to eliminate the laptop from routine upgrades, yet this gate reintroduced a
human round-trip before every run.

A separate constraint surfaced during review: the planned external consumer
check (building mcc-gaql against the upgrade branch) cannot compile against a
new major, because mcc-gaql's source imports `google::ads::googleads::vNN`
paths directly — running the check would require migrating mcc-gaql's import
tree first, which is an out-of-repo migration, not a validation step.

## Decision

1. **No pre-execution human gate.** The detector creates the upgrade issue
   already labeled `ready-for-agent`; the `*/15` worker poll is the sole
   dispatcher (fresh issues and retries alike). `needs-triage` and the 👀
   trigger are dropped. The worker remains the sole dispatcher even though the
   detector could dispatch directly — a single dispatch path keeps retry
   automation uniform.
2. **Label set collapsed to three state labels**: `ready-for-agent`,
   `in-progress`, `api-upgrade-failed` (plus the `api upgrade bot` marker
   label). Success keeps the issue `in-progress`; the PR body carries
   `Closes #N` so merging auto-closes the issue. `ready-for-human` and
   `api-upgrade-ready` are dropped.
3. **Failure retry contract**: on repair-budget exhaustion the worker flips the
   issue to `api-upgrade-failed` and leaves the branch/PR in place (worker
   never deletes). Retry is a human three-step — delete the failed branch/PR,
   remove `api-upgrade-failed`, re-add `ready-for-agent` — after which the
   upgrade restarts fresh from `main` with a full new 5-attempt budget. The
   agent never resumes on a failed branch.
4. **External consumer check deferred** with a revisit trigger: the first
   upgrade merge that breaks mcc-gaql's CI on `main` forces the mcc-gaql import
   migration, at which point the check is reintroduced. PR-readiness is
   Validation (fmt/check/test/clippy) only.

## Consequences

- A detector bug or anomalous googleapis tree state can auto-run a migration
  and open a PR without human review. Accepted: Validation gates the PR, the PR
  is a draft until validated, global serialization caps blast radius at one
  branch, and discarding a bad PR is cheap. Any residual gate would restore the
  laptop dependency the workflow exists to remove.
- Retry requires the human to garbage-collect the failed branch before
  relabeling; relabeling onto an existing branch is not a supported path.
- mcc-gaql can silently break on `main` after an upgrade merge; its CI is the
  tripwire, and the repair is a one-time import migration in that repo.