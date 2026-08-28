# googleads-rs — Fully Automated Google Ads API Upgrade Workflow (v3)

> This plan supersedes [plan-2](googleads-rs-automated-upgrade-implementation-plan-2.md), which remains available as the record of the original design. Every deviation from plan-2 was decided in a grilling session (2026-08-28) and is marked **[decided]** with the decision ID (Q-nn) inline. Round-2 grilling decisions (2026-08-28, Q69–Q77) removed the pre-execution human gate, collapsed the label set to three state labels, and deferred the external consumer check.
>
> **Status as of this plan:** Phases 0 and 1 are **implemented** in `grs_agentic_api_upgrade` (crate v25.1.3): `build.rs` derives the API major from `CARGO_PKG_VERSION_MAJOR`; `src/lib.rs` names generated modules only via the `current_gads_version` alias; `utils/update.sh` is hardened (`set -euo pipefail`, required version arg, staged proto swap, pre/post-copy validation, `best_effort` reserved for cosmetics, ruled crate-version arithmetic with minor inference from release notes).
>
> Vocabulary: see `CONTEXT.md` in the repo. **Upgrade** = end-to-end process. **Deterministic migration** = the scripted part (`update.sh`). **AI repair** = bounded agent loop (max 5 attempts). **Upgrade issue** = the automation-created queue item (born `ready-for-agent`). **Validation** = the canonical four-check gate. **External consumer check** = mcc-gaql job, **deferred** [Q73]. plan-2's `safe_run` term is retired (superseded by `best_effort step`).

---

## 1. Goal

Unchanged from plan-2 §1: eliminate the laptop for routine Google Ads API upgrades of `mhuang74/googleads-rs`. The crucial boundary stands:

```text
Release detection ≠ upgrade execution
```

A detected release creates an upgrade issue already labeled `ready-for-agent` and the pipeline runs to a validated draft PR with no human involvement [Q69]. The issue is the human-visible record (status surface, failure report, reopen point for retries); the human enters only at PR approval, or at failure recovery (branch GC + relabel) [Q69, Q72].

## 2. Architecture (revised)

```text
                 Google Ads / googleapis
                         │
                         ▼
             ┌────────────────────────┐
             │ Release detector       │  utils/detect-google-ads-version.sh (bash+curl+jq)   [Q22]
             │ google-ads-upgrade.yml │  weekly cron + workflow_dispatch                     [Q13]
             └───────────┬────────────┘
                         │ GitHub API tree scan, major versions only                 [Q13, Q33]
                         ▼
                Upgrade issue            label: api upgrade bot; body marker
              created ready-for-agent — auto-start, no triage gate           [Q69]
                         │
                 ┌───────┴────────┐
                 │                │
             human review       closed
                 │
                 ▼
       ┌────────────────────────┐
       │ Issue worker           │  google-ads-issue-worker.yml, */15 cron             [Q15→Q25]
       │ claim = label swap     │  ready-for-agent → in-progress + branch check       [Q16, Q56, Q75]
       └───────────┬────────────┘
                   │ gh workflow run (PAT — GITHUB_TOKEN recursion limit)        [Q46, Q49]
                   ▼
       ┌────────────────────────┐
       │ Upgrade execution      │  google-ads-upgrade.yml upgrade job
       │ (stub in M2, real M3)  │                                                           [Q29]
       └───────────┬────────────┘
                   ▼
          update.sh (standalone, unwrapped)                                          [Q54]
                   │
                   ▼
          cargo xtask fmt|check|test|clippy (fail-fast; zero-dep; isolated crate)    [Q1,Q2,Q9,Q64]
                   │
             ┌─────┴─────┐
             │           │
            PASS        FAIL
             │           │
             │      pi (vanilla coding agent), same job, sandboxed env,       [Q6,Q12,Q18,Q65]
             │      assembled /tmp/google-ads-migration.md context             [Q53]
             │      max 5 attempts, attempt log on issue                       [Q38]
             └─────┬─────┘
                   ▼
          draft PR → ready when validated (Validation only; consumer check deferred) [Q50, Q73]
                   │
            human approval (Stage 1; no auto-merge)                               [Q24]
                   ▼
                main
                   ▼
          release workflow (Milestone 5; deferred)                                   [Q34]
```

Workflow files: exactly **two** — `google-ads-upgrade.yml` (detector job + upgrade job) and `google-ads-issue-worker.yml` [Q35].

## 3. Phase 2 — Canonical validation (`xtask`) — next up

### 3.1 Form

- `xtask/` **isolated crate**: own empty `[workspace]` table; root `Cargo.toml` untouched; `cargo package/publish` of googleads-rs byte-identical to today. **[Q9]**
- **Zero external dependencies**: `std::process::Command` shelling to cargo. No clap, no anyhow. **[Q64]**
- Subcommands: `fmt`, `check`, `test`, `clippy` — plus an aggregate `validate` that runs all four. **[Q42]**
- `test` runs `cargo test --all-targets --all-features` (includes `tests/consumer_surface_tests.rs` and version-sync tests). **[Q26, Q42]**
- `clippy`: `cargo clippy --all-targets --all-features -- -D warnings`. `fmt`: `cargo fmt -- --check`.
- No `migrate` wrapper; `update.sh` stays a direct sibling entry point. **[Q54]**

### 3.2 Failure semantics

- Subcommands fail fast (first failing check stops, non-zero exit). **[Q2]**
- The aggregate `validate` is what the AI-repair loop calls; per-check subcommands are what CI and the repair loop's *first* iteration use to gather full context (see §6). **[Q2 follow-up]**

### 3.3 CI adoption

- `rust.yml` jobs call xtask subcommands: build job → `cargo xtask check`; lint job → `cargo xtask fmt` + `cargo xtask clippy`. This **silently tightens CI** from plain `cargo test` to `--all-targets --all-features` — confirmed wanted. **[Q10, Q31]**
- Tarpaulin coverage job stays as-is, outside Validation, per plan-2 §6. **[Q55]**

### 3.4 Verification of the tooling itself

- shellcheck in CI on `utils/*.sh` + manual smoke (broken checkout → non-zero exit) documented in the PR. **[Q5]**

## 4. Phases 3–4 — Detection and worker

### 4.1 Detector (`utils/detect-google-ads-version.sh`, bash + curl + jq) **[Q22]**

- **Mechanism**: GitHub git/trees API (`recursive=1`) against `googleapis/googleapis`; list `google/ads/googleads/v*` directories; highest = latest major. One request, no clone, no HTML scraping. **[Q13]**
- **Granularity**: major versions only. Minor bumps stay `update.sh`'s release-notes-inference job at migration time. **[Q33, Q45]**
- **Cadence**: weekly cron + `workflow_dispatch`. **[Q13]**
- **Current version source**: Cargo.toml major (same authoritative value Phase 0 established).
- **Idempotency**: before creating, search open upgrade issues for body marker `google-ads-api-upgrade: vNN`. Manual and auto issues are interchangeable; detector skips if either exists for the same target. **[Q14, Q51]**
- **Issue creation**: title `Upgrade Google Ads API vOLD → vNEW` **[Q43]**; body carries marker, previous/target versions, and a one-line current-state field (see §5.3); created **already labeled `ready-for-agent`** — execution starts without human intervention. **[Q14, Q51, Q69]**

### 4.2 Issue vocabulary and labels

- Marker label: `api upgrade bot`. State labels — exactly three: `ready-for-agent` (go signal, applied at issue creation), `in-progress` (claim state), `api-upgrade-failed` (terminal failure). **[Q7→Q69, Q37→Q71]**
- `needs-triage`: **dropped** — issues are born `ready-for-agent`; no human gate before execution. **[Q69]**
- `ready-for-human`, `api-upgrade-ready`: **dropped** — on success the issue stays `in-progress` and is auto-closed by the PR's `Closes #N` on merge. **[Q71]**
- 👀 reaction trigger: **dropped** — the label on a fresh issue is the only go signal. **[Q69]**

### 4.3 Worker (`google-ads-issue-worker.yml`)

- **Trigger**: `*/15` cron poll + `workflow_dispatch`. Executes an issue when it sees the `ready-for-agent` label — that label is the sole go signal (fresh issues carry it at creation; retries re-add it). **[Q15→Q69]**
- **Claim (idempotent)**: label swap `ready-for-agent` → `in-progress`, **plus** branch/PR existence check for the target version (belt and suspenders; scheduled runs of one workflow don't overlap, but manual dispatches can). On claim, the worker posts a comment: timestamp, target version, dispatched run link. **[Q16, Q56, Q75]**
- **Validation**: worker validates issue eligibility; the upgrade workflow **re-validates** on its own start (defense in depth against garbage manual dispatch inputs). **[Q68]**
- **Hand-off**: `gh workflow run google-ads-upgrade.yml -f issue=N -f target_version=vNN` using a **fine-grained PAT** (default GITHUB_TOKEN's events don't trigger other workflows). PAT is a repo secret, injected only into the worker job, never the agent environment. **[Q46, Q49]**
- **Serialization**: one upgrade at a time globally; worker skips if any upgrade branch/PR exists. **[Q39]**
- **Dependabot**: fully separate; no worker deferral. Existing dependabot PR → CI → human merge flow is sufficient isolation. **[Q61]**

## 5. Upgrade execution (M2 stub, M3 real) **[Q27, Q29]**

### 5.1 Branch and commits

- Branch `bot/google-ads-vNN-issue-<N>`. Never touches `main`. **[Q17]**
- Commit separation: `chore: update Google Ads API proto definitions to vNN` first; then `fix:`/`test:` commits from AI repair. Review boundary between generated and hand-written changes. **[Q17, plan-2 §17]**

### 5.2 Flow

1. Re-validate issue inputs **[Q68]**; re-check no existing branch/PR for target.
2. `git checkout main && git pull && git checkout -b bot/google-ads-vNN-issue-N`.
3. `cargo fmt` (auto-format after migration; xtask `fmt` then gates as `--check`) **[Q21]**.
4. `./utils/update.sh vNN` (standalone invocation) **[Q54]**.
5. Validation via xtask; on failure → AI repair loop (same job, inline; see §6) **[Q65]**.
6. Open **draft PR**; mark ready when Validation (fmt/check/test/clippy) passes. The external consumer check is **deferred** [Q73] and is not part of PR-ready criteria. PR body contains `Closes #N` so merging auto-closes the issue. **[Q50→Q71, Q57, Q73]**

### 5.3 Issue as status surface

- Body: marker + versions + a **one-line current-state field** the worker updates atomically at stage boundaries. All narrative progress (per-attempt logs, stage transitions) lands as **comments** — comment timeline. plan-2 §Phase-9's full body checklist: dropped (edit-conflict risk). **[Q36, Q38, Q58, Q59]**
- **Close semantics**: issue closes automatically when the PR merges (via `Closes #N` in the PR body), or by human close after failure resolution. Worker never closes on failure — it flips to `api-upgrade-failed` and posts the failure report (failing output, attempts N/5). Branch/PR left in failed state, never deleted by the worker; garbage collection is human's call. **[Q19→Q71, Q40, Q60]**
- **Retry after failure**: human deletes the failed branch/PR (GC), removes `api-upgrade-failed`, re-adds `ready-for-agent` → the worker re-claims and the upgrade restarts **fresh from `main`** with a full new 5-attempt budget; the agent never resumes on the failed branch. **[Q72, Q76]**

## 6. AI repair (pi)

- Agent: **pi, vanilla bare-bones coding agent** (not Oh My Pi), headless on the Actions runner. Detailed plumbing (auth, output capture) decided at Milestone 4 start. **[Q6, Q12]**
- **Same job** as validation, sandboxed environment: no secrets env (PAT, crates.io token, smoke-test creds all excluded), network limited to crates.io index; guardrails via strict system prompt + post-hoc diff review. plan-2 §11 rules apply verbatim (no test weakening, no CI edits, no `#[ignore]`, max 5 attempts). **[Q18, Q65]**
- **Context**: workflow pre-assembles `/tmp/google-ads-migration.md` (old/new versions, release-notes excerpt, known breaking changes, links). Agent never self-serves from the web. **[Q53]**
- Loop: validate → repair → validate, inline in the upgrade job. On persistent failure: issue → `api-upgrade-failed`, per §5.3. **[Q65→Q71, Q19]**

## 7. External consumer check (mcc-gaql) — DEFERRED **[Q73]**

- **Status**: deferred, not part of the current plan's active scope. Not a PR-ready criterion (amends Q50/Q57); not in M3 (amends §8).
- **Why deferred**: mcc-gaql's source imports `google::ads::googleads::vNN` paths directly, so an ephemeral `[patch]` to a new major won't compile until mcc-gaql's imports are migrated — an out-of-repo migration, not a check. The mechanical update cost across mcc-gaql's source tree outweighs the check's value right now.
- **Revisit trigger**: the first upgrade merge that breaks mcc-gaql's CI on `main` forces the mcc-gaql import migration; at that point the check below is (re)introduced.
- **Deferred mechanism** (for when it returns): job checks out `mcc-gaql` (external repo, git dep on `main`), writes an **ephemeral `[patch]` section** into its workspace Cargo.toml pointing `googleads-rs` at the bot branch, builds and tests, discards the change. No mcc-gaql repo changes. **[Q66]**
- In-repo `consumer_surface_tests.rs` is a normal integration target — already inside `cargo xtask test`. The term "consumer test" in prose refers to that; mcc-gaql is the "external consumer check". (CONTEXT.md updated accordingly.)

## 8. Milestones (revised)

- **M2 — Phase 2 + stub pipeline**: xtask (isolated, zero-dep, four subcommands + validate) **[Q1, Q9, Q64]**; rust.yml adopts xtask subcommands **[Q10]**; pinned protoc (exact version, documented for laptop parity) **[Q32, Q67]**; detector script + detector job **[Q13, Q22]**; issue creation with marker, born `ready-for-agent` **[Q7, Q37, Q69]**; **worker with stub execution** (dispatches an upgrade workflow that runs migration + validation + draft PR, no AI repair yet) **[Q27, Q29]**. Branch protection on `main` enabled **before** first bot PR **[Q47]**.
- **M3 — real execution**: AI repair via pi inline (sandbox, context file, 5 attempts, issue attempt log) **[Q12, Q18, Q53, Q65, Q38]**; hardening pass (claim idempotency under manual dispatch, failure-state transitions, re-validation, retry-via-relabel) **[Q68, Q76]**. External consumer check: **deferred** [Q73].
- **M4 — repair-loop hardening**: pi plumbing details (auth mode, output capture), prompt tuning from first real failures.
- **M5 — release automation** (deferred; out of this plan's active scope) **[Q34]**: tag-driven release workflow, cargo-semver-checks, crates.io trusted publishing, publish creds never in agent env (plan-2 §19–21 carried forward).
- **M6 — autonomy + smoke test**: real-API smoke test (read-only `SearchGoogleAdsStream` against a test account; read-only scoped creds in Actions secrets, injected only into the smoke-test job) **[Q20, Q44]**; auto-merge minors after track record; Stage 2/3 progression per plan-2 §32 with human merge policy starting at Stage 1 **[Q24]**.

## 9. Decisions that override plan-2 (index)

| plan-2 said | plan-3 says | Ref |
|---|---|---|
| Optional `api upgrade running/failed/ready` label family | Exactly three state labels: `ready-for-agent`, `in-progress`, `api-upgrade-failed`; no triage roles | Q7, Q37, Q69, Q71 |
| */15 issue worker, poll only | */15 poll; `ready-for-agent` label is the sole go signal (issues born labeled; 👀 dropped) | Q15, Q23, Q69 |
| Full body checklist updates (§Phase 9) | One-line body status + comment timeline | Q36, Q58, Q59 |
| One `google-ads-upgrade.yml` with detect+upgrade jobs | Two workflows: upgrade (detect+upgrade jobs) + issue-worker | Q35 |
| §30 detect+upgrade in one workflow, §7 detector separate | Detector = job in upgrade workflow, logic in utils/ script | Q13, Q22, Q35 |
| Milestone 2 = detector only | M2 = detector + worker with stub execution | Q27, Q29 |
| Agent unspecified | pi (vanilla), same-job sandbox, decided details at M4 | Q6, Q12, Q18 |
| §23 Dependabot separate | Confirmed fully separate, no serialization | Q61 |
| Coverage in CI, role unclear | Coverage stays outside Validation, tarpaulin job unchanged | Q55 |
| §18 branch protection "configure" | Enable before first bot PR | Q47 |
| §34 history.md for observability | Issue is the record; no separate history file | Q38, Q48 |
| §8 claim mechanisms list | Label swap + branch/PR existence check | Q16, Q56 |
| PRs regular or draft unspecified | Draft until Validation passes (consumer check deferred) | Q50, Q73 |
| §12 failure → report | Issue → `api-upgrade-failed`; branches kept, human GCs, retry = relabel `ready-for-agent` → fresh from main | Q19, Q40, Q60, Q71, Q72, Q76 |
| Human triage gate before execution | No gate: issues born `ready-for-agent`, worker auto-dispatches; human enters at PR approval or failure recovery | Q69, Q74 |
| Consumer check in PR-ready criteria + M3 | Deferred with revisit trigger (mcc-gaql CI breakage on main) | Q73 |
| Success: `ready-for-human`/`api-upgrade-ready` labels | Success keeps `in-progress`; PR `Closes #N` auto-closes issue on merge | Q71 |

## 10. Definition of done

Unchanged from plan-2 §37: a new Google Ads API version can be released while the laptop is offline, and GitHub independently detects, migrates, repairs, validates, PRs, and (after approval policy) publishes. Human intervention only for: PR approval (Stage 1 policy), failure recovery (branch GC + relabel for retry), ambiguous semantic changes, behavior changes invisible to compile/tests, and security/policy gates. **[Q69, Q72]**