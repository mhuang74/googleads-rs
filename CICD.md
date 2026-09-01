# CI/CD

Google Ads API crate versions mirror the API major version (crate `24.x.y` supports Google Ads API v24; minor/patch come from Google's release notes). The upgrade pipeline tracks upstream and opens PRs automatically; PRs to `main` require all four quality checks to pass.

## Workflows overview

| Workflow | File | Trigger | Runs |
|---|---|---|---|
| build googleads-rs | [rust.yml](.github/workflows/rust.yml) | push/PR to `main` | build, test-with-coverage, lint, shellcheck |
| google-ads-detect | [google-ads-detect.yml](.github/workflows/google-ads-detect.yml) | cron Thu 04:30 UTC, manual | detect |
| google-ads-issue-worker | [google-ads-issue-worker.yml](.github/workflows/google-ads-issue-worker.yml) | cron Thu 05:00 UTC, manual | poll |
| google-ads-upgrade | [google-ads-upgrade.yml](.github/workflows/google-ads-upgrade.yml) | `workflow_dispatch` only | upgrade |

Both crons run on Thursdays; the worker fires 30 minutes after detect so a fresh issue is normally claimed the same day. All jobs run on `ubuntu-latest` with `CARGO_TERM_COLOR=always`.

## CI gates (rust.yml)

Required checks for PRs targeting `main` and pushes to `main`. Shared toolchain: `./.github/actions/install-protoc` (protoc pinned for prost builds) + `dtolnay/rust-toolchain@stable`.

- **build** — `cargo xtask check` (cargo check via the xtask wrapper).
- **test-with-coverage** — `cargo tarpaulin --all-features --workspace --out Xml --out xml` producing `cobertura.xml` + `junit.xml` in `target/tarpaulin/`, uploaded to Codecov (`codecov-action@v5`, non-fatal on error, `CODECOV_TOKEN`) and `codecov/test-results-action@v1` (runs even when cancelled).
- **lint** — `cargo xtask fmt` then `cargo xtask clippy` (toolchain with `clippy, rustfmt` components; xtask wrapper enforces `-D warnings`).
- **shellcheck** — `ludeeus/action-shellcheck@2.0.0` over `utils/*.sh`, severity `warning`.

## Automated Google Ads API upgrades

Three-workflow pipeline (ADR 0002 split: detect / issue-worker / upgrade). Issue labels are the coordination mechanism; the issue body's `Current state:` line is tracked throughout. Branch naming: `bot/google-ads-v<major>-issue-<n>`.

```
weekly Thu 04:30     weekly Thu 05:00              dispatched by worker
┌─────────┐  issue   ┌──────────────┐  dispatch  ┌─────────────────┐
│ detect  │ ──────►  │ issue-worker │ ─────────► │ upgrade         │
│         │ ready-   │ poll         │ (labels→   │ migrate+validate│
│         │ for-     │ claim+serial │ in-progress│ (+pi AI repair) │
│         │ agent    │ +reclaim     │  +comment) │ → draft PR      │
└─────────┘          └──────────────┘            └─────────────────┘
                                                       │
                    merge (human/bot) ◄── PR to main ◄──┘
```

### 1. google-ads-detect — release detector

Weekly Thursday 04:30 UTC; `workflow_dispatch` for detector-only test runs. Job **detect** runs `./utils/detect-google-ads-version.sh` with jq installed:

1. Reads the crate major from `Cargo.toml` (the authoritative current API version).
2. Single GitHub `git/trees` API request (recursive, no clone, no scraping) against `googleapis/googleapis` `master`; lists `google/ads/googleads/v*` dirs and takes the highest major — major versions only (minor bumps are inferred later by `utils/update.sh` from release notes).
3. If latest ≤ current: exit "no new version". Otherwise create an issue titled `Upgrade Google Ads API v<cur> → v<new>` with body marker `google-ads-api-upgrade: vNN`, labeled `ready-for-agent` + `api upgrade bot` (the worker's sole go signal). Idempotent: skips creation if an open issue already targets that version.

### 2. google-ads-issue-worker — claim & dispatch

Job **poll** (5-minute timeout, `GH_TOKEN` = `ISSUE_WORKER_PAT` at job level):

1. **Find ready-for-agent upgrade issues** — lists open issues labeled `ready-for-agent` + `api upgrade bot`. Global serialization: if *any* `bot/google-ads-v*` branch or open `head:bot/google-ads-v*` PR exists, skip (one upgrade in flight at a time). Else take the oldest issue, extract the target version from its `google-ads-api-upgrade: vNN` body marker, and set `found/issue_number/target_version` outputs. Any extraction failure → `found=false`, skip cleanly.
2. **Reclaim stuck in-progress issues** — safety net for wedged `in-progress` + `api upgrade bot` issues: for each, if there is *no* queued/running `google-ads-upgrade.yml` run for that issue *and* *no* open PR whose body says `Closes #<issue>` (checked per issue so a busy pipeline can't mask a stuck one), comment "♻️ Reclaiming..." and atomically flip `in-progress` → `ready-for-agent` for re-claim next poll.
3. **Claim issue (label swap + branch check)** (only if `found=true`) — if branch `bot/google-ads-<ver>-issue-<n>` already exists, skip. Otherwise single-API-request atomic label swap `ready-for-agent` → `in-progress` (gh applies add+remove in one call; no intermediate state), then post a claim comment and `gh workflow run google-ads-upgrade.yml -f issue=<n> -f target_version=<ver>`. Belt-and-suspenders with step 1: if two manual dispatches race past the label swap, only one can win the branch check. On comment/dispatch failure: comment "⚠️ rolling back", restore `ready-for-agent`, exit 1 — the next weekly poll re-claims.

Retry contract: after a failed run the human deletes the failed branch/PR, removes `api-upgrade-failed`, re-adds `ready-for-agent`; the worker re-claims on the next weekly poll. The upgrade job always starts fresh from `main`, never on a failed branch. The branch check prevents duplicate branch creation if the old branch wasn't GC'd.

### 3. google-ads-upgrade — migration + AI repair

`workflow_dispatch` only — normally dispatched exclusively by google-ads-issue-worker (direct dispatch of a non-eligible issue aborts in re-validate). Manual dispatch is intended solely for **resume mode** (`resume=true`): re-validate an existing branch and retry validation/PR creation without deleting the branch or flipping labels. Inputs: `issue` (number), `target_version` (e.g. `v25`), `resume` (boolean, default false). Job **upgrade** (30-minute timeout; GH tokens scoped per-step, never job-level — keeps the PAT out of the pi agent's environment):

1. **Re-validate issue inputs** (`ISSUE_WORKER_PAT` + `GH_REPO`) — verifies the issue exists and flips to `api-upgrade-failed` on ineligibility via `flip_to_failed` (3 attempts, 5s backoff, exits non-zero if the flip itself fails). Label reads retry 3× so a transient GitHub API failure aborts *without* touching labels rather than masquerading as a failed check. Resume mode accepts `in-progress` OR `api-upgrade-failed` (flipping `api-upgrade-failed` → `in-progress`) and skips branch/PR checks — reusing the existing branch is the point. Fresh mode requires `in-progress` and aborts (→ `api-upgrade-failed`) if the branch or a PR for it already exists.
2. **Update issue state** — body `Current state:` → `starting migration` / `resuming (re-validating)` + progress comment with target/branch/run URL.
3. **Environment** — `actions/checkout@v4` (`fetch-depth: 0`), install-protoc, stable Rust + rustfmt/clippy.
4. **Create upgrade branch** — resume: fetch and check out the existing branch. Fresh: always branch fresh off `main` (never build on a failed branch).
5. **Migration** (skipped in resume mode) — `cargo fmt`, then the deterministic migration `./utils/update.sh <target_version>`, then commit `"feat: upgrade to Google Ads API <target_version>"` and push.

    `utils/update.sh vNN` (Linux-only, GNU sed): reads the current crate version from `Cargo.toml`; infers the target minor from Google's release-notes headings (`data-text="vM.N (date)"`, current minor kept + warning on parse failure); computes the new crate version (`M.MIN.0`; patch bump on same-major re-runs with `--force`); downloads and extracts the `googleapis` `master` archive; pre-validates the target version dir actually arrived in the multi-major archive; stages `google/rpc`, `google/longrunning`, `google/type`, `google/logging`, `google/api` + the target `googleads/vNN` tree in a temp dir; validates staged dirs contain `.proto` files before the atomic swap into `proto/`; strips non-proto files; scrubs comments from two rpc protos to avoid doctest errors; then rewrites `Cargo.toml` version, the `current_gads_version` alias line in `src/lib.rs` (the only version-literal anchor — `build.rs` derives the rest), the doc-root URL, and README version/API mentions.
6. **Assemble migration context** — writes `/tmp/google-ads-migration.md` (versions, release-notes URL, repo structure, plan-2 §11 hard rules: no test deletion/weakening/`#[ignore]`, no clippy disables, no CI/`proto/`/`build.rs`/`utils/` edits, no removing the external consumer check, max 5 repair attempts).
7. **First validation** — runs `cargo xtask fmt|check|test|clippy` individually (precise failure context), collecting output in `/tmp/first-validation-failures.txt`; outcome exposed as `failed=(true|false)`.
8. **On success → PR** — idempotent: if a PR already exists for the branch, mark it ready and comment; otherwise `gh pr create --draft` (title `Feature: Upgrade to Google Ads API v<NN>`, body `Closes #<issue>`, label `api upgrade bot`), then `gh pr ready` and issue body → `PR ready for review`.
9. **On failure → AI repair (pi)** — installs `@earendil-works/pi-coding-agent` and runs up to `PI_MAX_ATTEMPTS=5` iterations of the pi repair agent. Each invocation gets `--provider`/`--model` (repo *variables* `PI_MODEL`/`PI_PROVIDER`, defaults `ollama-cloud`/`glm-5.3-flash`), `--api-key` from the `PI_API_KEY` secret (injected into this step's env only — `GH_TOKEN` never reaches pi), `--print --no-session --approve`; the prompt inlines `/tmp/google-ads-migration.md` + the validation failure log (pi has no context/failure flags), plus a strict system prompt enforcing the plan-2 §11 rules and common upgrade failure patterns. Network restricted to the crates.io index via runner firewall. Per attempt: pi output + `git diff HEAD` captured to an attempt log, then `cargo xtask validate` — on pass, commit `"fix: repair validation failures ... (attempt N)"` + push + issue comment; on failure, post the log tail ("🤖 AI repair attempt N/5 failed") and commit+push partial fixes for inspection, continue.
10. **Post-repair** — success mirrors step 8 (issue state → `PR ready for review (after AI repair)`). Failure: issue state → `repair failed (5/5)`, comment with workflow-run link, branch left in place, labels flip `in-progress` → `api-upgrade-failed`, exit 1. Retry hint differs by mode: resume mode → re-dispatch with `resume=true`; fresh mode → delete branch/PR + relabel.

### Issue label state machines

- Normal loop: `ready-for-agent` → `in-progress` → (PR ready for review; body state `PR ready for review`) → merge `Closes` the issue.
- Failed migration: `in-progress` → `api-upgrade-failed` (+ `api upgrade bot` label throughout) → human cleanup (`ready-for-agent`) re-enters the loop; or resume mode re-dispatch with `resume=true`.
- Stuck issue: `in-progress` with no running `google-ads-upgrade` run and no open PR → worker reclaims to `ready-for-agent` next weekly poll.
- Issue body `Current state:` lifecycle: `pending migration` → `starting migration` / `resuming (re-validating)` → `validating` → `AI repair in progress` → `PR ready for review` (or `repair failed (5/5)`).

### Secrets & variables

| Name | Type | Used by |
|---|---|---|
| `GITHUB_TOKEN` | default | detect (issue listing/creation) |
| `ISSUE_WORKER_PAT` | secret | worker poll; upgrade label/PR/issue mutation (per-step scoped) |
| `PI_API_KEY` | secret | pi AI repair auth (`--api-key`), upgrade step-scoped only |
| `CODECOV_TOKEN` | secret | rust.yml coverage + test-result uploads |
| `PI_MODEL` / `PI_PROVIDER` | repo variables | pi repair model config (defaults `glm-5.3-flash` / `ollama-cloud`), changeable without workflow commits |

### Design notes

- Single-upgrade serialization: worker refuses to claim while any `bot/google-ads-v*` branch/open PR exists; upgrade re-validates labels plus branch/PR non-existence, so two racing dispatches can't double-create work.
- Human checkpoints: the pipeline never merges the PR or publishes to crates.io; human review decides, and failure recovery relies on explicit human relabeling.
- The `schedule` trigger on google-ads-upgrade was retired to detect (ADR 0002); input-param `if:` routing also retired — dispatch paths are worker or human-resume only.