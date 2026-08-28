# 0002 — Detector split into its own workflow file

Date: 2026-08-29 (grilling round 3, Q78 of plan-3)

## Status

Accepted (supersedes the Q35 decision in plan-3 — "exactly two workflow files"; Q35 kept as historical record)

## Context

plan-3 Q35 co-located the release detector and the upgrade execution as two jobs of one workflow file, `google-ads-upgrade.yml`, keeping the repo at "exactly two" workflow files. Both jobs ran from disjoint triggers inside one `on:` block: the detector on the weekly cron, the upgrade job on `workflow_dispatch` with `issue`/`target_version` inputs. Because both triggers shared the file, each job needed an `if:` guard that discriminated runs by input parameter — `inputs.issue == 0` meant "this dispatch is really a detector run," which also forced `issue` to be nominally `required` yet spoofable to `0`, a footgun for manual detector testing.

## Decision

1. **Split the detector into its own file** — `google-ads-detect.yml` (weekly cron + bare `workflow_dispatch`, no inputs). `google-ads-upgrade.yml` keeps only the `workflow_dispatch` trigger and is dispatched by `google-ads-issue-worker.yml` via `gh workflow run`; the worker's dispatch contract is unchanged.
2. **Retire input-param trigger routing.** All per-job `if:` guards that existed only to route one shared trigger set between two concerns are removed. A workflow file's trigger set is its routing mechanism; discriminating runs by input parameters inside a shared file is an anti-pattern.
3. **Manual detector testing** is a direct dispatch of `google-ads-detect.yml` — the `issue=0` contortion is retired.
4. Direct manual dispatch of `google-ads-upgrade.yml` remains technically possible (it is the same `workflow_dispatch` trigger the worker uses, and GitHub has no API-only trigger) but is **not the intended path**; re-validation requires the referenced issue to be `in-progress` and flips garbage dispatches to `api-upgrade-failed`.

## Consequences

- Three workflow files; plan-3 §2 "exactly two" is superseded by Q78.
- Each workflow's purpose is visible from its trigger set alone — no cross-reading of `if:` guards to know what a run does.
- Runtime behavior is unchanged: same weekly cadence, same worker dispatch, same issue lifecycle. This is a structural/readability change, not a behavioral one.