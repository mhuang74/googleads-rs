# googleads-rs — Fully Automated Google Ads API Upgrade Workflow

## 1. Goal

The workflow is deliberately split into **release detection** and **upgrade execution** to preserve manual control.

```text
Google Ads API release
        ↓
Detection workflow
        ↓
Create GitHub Issue
label: api upgrade bot
        ↓
[human can inspect/edit/close/reopen issue]
        ↓
Background issue worker
        ↓
Upgrade workflow
        ↓
deterministic migration
        ↓
validation
        ↓
bounded coding-agent repair
        ↓
PR
        ↓
human review
        ↓
merge
        ↓
release
        ↓
cargo publish
```

This separation also provides a clean way to manually test the upgrade mechanism: a developer can create or relabel an appropriate GitHub Issue without waiting for Google to release a new API version.

Eliminate the need to use a laptop for routine Google Ads API upgrades of `mhuang74/googleads-rs`.

Target workflow:

```text
Google Ads API release
        ↓
Release-detection workflow
        ↓
Create `api upgrade bot` Issue
        ↓
Background issue worker
        ↓
Create isolated upgrade branch
        ↓
Run deterministic proto/API migration
        ↓
cargo fmt / check / test / clippy
        ↓
If failures: bounded coding-agent repair loop
        ↓
Run full validation again
        ↓
Open GitHub PR with upgrade report
        ↓
Human review initially
        ↓
Merge
        ↓
GitHub release workflow
        ↓
cargo publish
```

Longer-term, after several successful upgrades, routine upgrades may be eligible for automatic merge and publish.

---

## 2. Current repository assessment

The current repository is unusually well positioned for this automation.

### Existing upgrade mechanism

`utils/update.sh` already accepts a target Google Ads API version and:

- downloads `googleapis`
- copies Google Ads proto definitions
- copies required Google infrastructure protos
- removes non-proto files
- removes problematic proto comments
- updates version references in `build.rs`, `src/lib.rs`, tests, and README

Therefore, it can become the deterministic core of the upgrade bot.

### Existing code generation

`build.rs` scans the checked-in `proto/` tree and invokes `protoc`/`tonic_prost_build` to generate the Rust client.

### Existing validation

GitHub Actions currently runs:

- `cargo build`
- `cargo test`
- `cargo clippy`
- `cargo fmt`

### Existing downstream compatibility tests

The repository contains consumer-surface tests designed around downstream usage such as `mcc-gaql-rs`.

### Important architectural improvement already made

The April 2026 `prost-reflect` migration removed hundreds of hand-maintained GAQL field match arms and made field access dynamically driven by protobuf descriptors. This substantially reduces the amount of source code that needs manual migration for future Google Ads releases.

---

# 3. Design principles

## 3.1 Deterministic first, AI second

Do not ask an AI agent to perform the whole upgrade.

Instead:

1. deterministic migration
2. compiler
3. tests
4. clippy
5. AI only for residual failures

This minimizes cost, unpredictability, and unnecessary source changes.

## 3.2 The compiler is the primary feedback loop

The coding agent should receive concrete failures from:

- `cargo check`
- `cargo test`
- `cargo clippy`

rather than relying primarily on reasoning about the Google Ads API.

## 3.3 Never weaken tests to make the upgrade pass

The agent must not:

- delete tests
- weaken assertions
- add `#[ignore]` merely to bypass failures
- disable clippy warnings
- change CI to avoid a failure
- remove consumer-surface coverage

## 3.4 Major and minor releases have different policies

Recommended initial policy:

| Upgrade | Automated migration | AI repair | Human approval |
|---|---:|---:|---:|
| Minor API release | Yes | Only if needed | Initially |
| Major API release | Yes | Yes | Yes |
| Cargo dependency update | Dependabot/Renovate | Optional | Existing policy |

After confidence is established, minor releases that pass all gates can be auto-merged.

---

# 4. Phase 0 — Refactor version handling

Before building the bot, remove unnecessary hard-coded version references.

## Current issue

`build.rs` contains a hard-coded Google Ads version path such as:

```rust
path_str.contains("googleads/v23")
```

while `update.sh` separately updates that string.

This means the upgrade process must modify implementation code merely to select the API version.

## Target

Establish one authoritative API-version value.

Preferred options, in order:

1. derive it from `CARGO_PKG_VERSION`
2. or use a small dedicated version file such as `google-ads-version.toml`

Because the project's versioning convention maps `major.minor` to Google Ads API `major.minor`, deriving the API version from the crate version is attractive.

## Expected result

Instead of:

```text
update.sh
 ├── modify build.rs
 ├── modify src/lib.rs
 ├── modify tests
 └── modify README
```

aim for:

```text
update version/proto
        ↓
all version-dependent code derives the value
```

## Acceptance criteria

- No Google Ads API version string is hard-coded in `build.rs`.
- A version upgrade does not require an LLM to modify `build.rs`.
- Tests continue to verify version consistency.

---

# 5. Phase 1 — Harden `utils/update.sh`

Keep the existing script initially rather than rewriting everything.

## Required changes

### 5.1 Fail fast

Remove or reconsider the current `safe_run()` behavior for operations where failure should abort the migration.

A migration should never silently continue after an important operation fails.

### 5.2 Make execution reproducible

The script should:

- require a target API version
- verify that the target version exists
- start from a clean/known proto directory
- exit non-zero on failed downloads/extraction/copies
- verify expected proto files exist afterward

### 5.3 Avoid downloading `master` blindly where possible

Prefer a reproducible source/version reference.

The target Google Ads API version should be explicit, while shared Google infrastructure dependencies should be obtained from a known compatible source.

If using the GitHub repository's current state is retained, add validation so an unexpected upstream repository layout fails clearly.

### 5.4 Add a dry validation

After migration:

```bash
test -d proto/google/ads/googleads/v${VERSION}
test -n "$(find proto/google/ads/googleads/v${VERSION} -name '*.proto' -print -quit)"
```

## Acceptance criteria

Running:

```bash
./utils/update.sh vXX
```

from a clean checkout either:

- produces a complete vXX source tree, or
- fails clearly and non-zero.

---

# 6. Phase 2 — Build a local upgrade validation command

Create one canonical command used both locally and in GitHub Actions.

Possible implementation:

```bash
./utils/validate.sh
```

or a Rust `cargo xtask` command.

Recommended validation sequence:

```bash
cargo fmt -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

If coverage is expensive, do not make coverage part of the first upgrade/repair loop. Keep the existing coverage workflow as a separate quality signal.

## Acceptance criteria

A single command returns:

- exit 0 only when all required checks pass
- useful, complete logs when something fails

---

# 7. Phase 3 — Detect the latest Google Ads API version

Use a **dedicated release-detection workflow** whose only responsibility is to discover a new Google Ads API version and create a GitHub Issue.

It should **not** perform the upgrade itself.

Recommended schedule:

```yaml
on:
  schedule:
    - cron: "17 2 * * 1"
  workflow_dispatch:
```

`workflow_dispatch` is useful for testing the detector without waiting for the weekly schedule.

## Version discovery

Prefer machine-readable discovery from the Google APIs repository rather than scraping documentation HTML.

Conceptually:

```text
googleapis repository
       ↓
google/ads/googleads/vXX
       ↓
highest supported version
```

Compare that with the version currently supported by `googleads-rs`.

## No-update case

If:

```text
current == latest
```

exit successfully without creating an Issue.

## New-version case

If:

```text
latest > current
```

create a GitHub Issue with the dedicated label:

```text
api upgrade bot
```

Use a stable title such as:

```text
Upgrade Google Ads API v25 → v26
```

Include structured metadata in the Issue body:

```markdown
## Google Ads API Upgrade

Previous version: v25
Target version: v26

Status: pending

This issue was created automatically by the Google Ads API release detector.

The upgrade has not yet been executed.
```

## Idempotency

The detector must not create duplicate Issues.

Before creating an Issue, search for an existing open or recently closed Issue with:

- label `api upgrade bot`
- target version `v26`

If one exists, do nothing.

A useful machine-readable marker is:

```text
google-ads-api-upgrade: v26
```

in the Issue body.

## Why this separation matters

The Issue becomes a **human-controlled queue item**.

You can:

- inspect the release before running anything
- edit the Issue
- close it to prevent execution
- reopen it
- manually create an Issue for testing
- trigger execution independently from release detection

This makes the system much safer to develop and operate than a single scheduled workflow that immediately modifies the repository.

---

# 8. Phase 4 — Background issue worker

Create a second mechanism responsible for finding Issues labeled:

```text
api upgrade bot
```

and starting the actual upgrade.

There are two good implementation options.

## Preferred initial implementation: scheduled issue worker

Run every few minutes or hourly:

```yaml
on:
  schedule:
    - cron: "*/15 * * * *"
  workflow_dispatch:
```

The worker searches for eligible Issues:

```text
label:api upgrade bot
state:open
```

and determines whether they are ready to execute.

This is intentionally separate from the release detector.

## Eligibility rules

An Issue should be eligible when:

- it has `api upgrade bot`
- it is open
- it contains a valid target API version
- there is no active upgrade workflow/branch/PR for that target
- it is not explicitly marked as blocked

Optional additional labels:

```text
api upgrade bot
api upgrade running
api upgrade failed
api upgrade ready
```

However, keep the first implementation simple. The single required label should be:

```text
api upgrade bot
```

## Manual testing

To manually test the complete upgrade system, create an Issue such as:

```text
Upgrade Google Ads API v25 → v26
```

with:

```text
api upgrade bot
```

This bypasses the release detector entirely.

That means the developer can test:

```text
Issue
 ↓
issue worker
 ↓
upgrade workflow
```

without having to fake a Google release.

## Avoid duplicate execution

The worker must claim an Issue before starting the upgrade.

Possible mechanisms:

1. add `api upgrade running`
2. record the Issue number in workflow state
3. detect an existing upgrade branch
4. detect an existing upgrade PR

Prefer a combination of a running label plus branch/PR detection.

If multiple GitHub runners start concurrently, the workflow must still be idempotent.

---

# 13. Phase 9 — Agent iteration model

The issue worker should invoke the actual upgrade workflow with:

```text
issue_number
target_version
```

Branch naming:

```text
bot/google-ads-v26
```

or, preferably when multiple attempts/history need to coexist:

```text
bot/google-ads-v26-issue-123
```

Before modifying anything:

```bash
git checkout main
git pull
git checkout -b bot/google-ads-v26
```

The workflow should never modify `main` directly.

The upgrade workflow should update the original Issue with status information as it progresses.

Example:

```markdown
Status: running

Upgrade workflow:
- [x] branch created
- [x] deterministic migration
- [ ] validation
- [ ] AI repair
- [ ] PR
```

# 8. Phase 4 — Create the upgrade branch

Branch naming:

```text
bot/google-ads-v26
```

or:

```text
automation/google-ads-v26
```

Before modifying anything:

```bash
git checkout main
git pull
git checkout -b bot/google-ads-v26
```

The workflow should never modify `main` directly.

---

# 9. Phase 5 — Run deterministic migration

Execute:

```bash
./utils/update.sh v26
```

Then:

```bash
cargo fmt
```

Commit the deterministic migration separately if practical:

```text
chore: upgrade Google Ads API to v26
```

A separate commit is useful because it lets reviewers distinguish generated/proto changes from AI-authored fixes.

---

# 10. Phase 6 — First validation attempt

Run:

```bash
cargo fmt -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Also run the consumer-surface tests.

## If everything passes

Do not invoke an AI agent.

Proceed directly to PR creation.

This is important for both cost and reliability.

---

# 11. Phase 7 — Coding-agent repair loop

If validation fails, invoke a coding agent in the same GitHub Actions workflow.

The agent should receive:

- repository checkout
- target API version
- previous API version
- Google Ads release/migration information
- `git diff`
- compiler output
- test failures
- clippy failures
- project-specific instructions

## Agent task

The agent should be told:

```text
You are upgrading googleads-rs from Google Ads API vOLD to vNEW.

The deterministic migration has already been completed.

Your task is to fix source-code compatibility issues caused by this API upgrade.

Rules:
1. Preserve the public API wherever reasonably possible.
2. Do not modify generated protobuf files manually.
3. Do not modify CI configuration.
4. Do not modify credentials or release configuration.
5. Do not weaken or delete tests.
6. Do not add #[ignore] to bypass failures.
7. Do not remove clippy checks.
8. Prefer fixes that preserve downstream compatibility.
9. Read the Google Ads API migration/release notes.
10. Run the project validation command after making changes.
11. Stop when all validation passes.
12. Stop after a maximum of 5 repair iterations.
```

---

# 12. Agent iteration model

Use a bounded loop.

```text
Attempt 1
  ↓
agent edits code
  ↓
validation
  ↓
failure?

Attempt 2
  ↓
agent edits code
  ↓
validation
  ↓
...

Maximum: 5 attempts
```

Do not allow unlimited autonomous execution.

## Failure after maximum attempts

Leave the branch/PR in a failed state and report:

```text
Google Ads API vXX upgrade could not be completed automatically.

Validation failures:
- ...
- ...

Agent attempts: 5/5
Human intervention required.
```

The important property is that the system still did useful work and leaves the repository in a reviewable state.

---

# 13. Give the agent Google migration context

The agent should have access to the relevant Google Ads API release notes.

The workflow should construct a small context file:

```text
/tmp/google-ads-migration.md
```

containing:

- old API version
- new API version
- relevant release notes
- known breaking changes
- links to official documentation

This should be provided alongside compiler/test output.

Do not rely on the agent independently discovering everything from the internet.

---

# 14. Add a migration-specific test category

Your existing tests are already strong. Add explicit tests for patterns historically affected by API upgrades.

Recommended categories:

```text
protobuf generation
oneof handling
optional fields
enum names
removed fields
renamed fields
resource names
field masks
GAQL dynamic field access
request/response construction
gRPC service clients
consumer surface
```

These tests should be stable across API versions where possible.

---

# 15. Add an optional real Google Ads smoke test

Create a dedicated integration test that performs a harmless read-only request against a test/customer account.

For example:

```text
SearchGoogleAdsStream
SELECT customer.id, customer.descriptive_name
FROM customer
```

Use GitHub Actions secrets for credentials.

Do not run this test on every ordinary pull request unless necessary.

Recommended:

```text
normal PR:
  compile
  unit tests
  consumer tests
  clippy

upgrade PR:
  all above
  + Google Ads smoke test
```

The smoke test catches cases where generated Rust code compiles but the actual API interaction is incompatible.

---

# 16. PR generation

When migration succeeds, automatically create:

```text
Upgrade Google Ads API v25 → v26
```

## PR body

Generate a structured report:

```markdown
## Google Ads API Upgrade

Previous API: v25
New API: v26

### Automated migration

- [x] Proto definitions updated
- [x] Version references updated
- [x] Generated client builds
- [x] Formatting
- [x] Clippy
- [x] Unit tests
- [x] Consumer surface tests
- [x] API smoke test

### AI migration

Agent required: yes/no
Repair attempts: N

### Changed files

<summary>

### Potential breaking changes

<summary from release notes/diff>

### Validation

<commands and results>
```

---

# 17. Separate generated changes from hand-written fixes

For reviewer clarity, prefer commits like:

```text
1. chore: update Google Ads API proto definitions to v26
2. fix: migrate Rust code for Google Ads API v26 breaking changes
3. test: update v26 compatibility tests
```

If the generated proto diff is enormous, this separation becomes particularly valuable.

---

# 18. Branch protection

Configure `main` so that:

- direct pushes are restricted
- required CI checks must pass
- upgrade bots cannot bypass required checks
- force-pushes are restricted
- release/publish credentials are not available to arbitrary PR workflows

Initially require human approval for upgrade PRs.

---

# 19. GitHub Actions security model

Use separate permissions for different workflows.

## Upgrade workflow

Needs approximately:

```yaml
permissions:
  contents: write
  pull-requests: write
```

Avoid giving unrelated repository permissions.

## Release workflow

Use the minimum permissions required to create/publish releases.

Do not expose crates.io publishing credentials to untrusted pull requests.

---

# 20. crates.io publishing

Move publishing entirely into GitHub Actions.

Recommended release flow:

```text
merge to main
    ↓
create version/tag
    ↓
release workflow
    ↓
cargo package
    ↓
cargo test
    ↓
cargo-semver-checks
    ↓
cargo publish
```

## Version policy

Continue using the existing convention:

```text
Google Ads API 26.1
        ↓
googleads-rs 26.1.x
```

Reserve the patch component for library-only fixes.

---

# 21. Publishing credentials

Prefer modern trusted publishing/OIDC support if available for the crates.io publishing setup.

If a token is required instead, store it as a GitHub Actions secret and make it accessible only to the release workflow.

Never place the token in:

- repository files
- workflow source
- issue comments
- PR environment
- coding-agent environment

The AI repair job should never receive the crates.io credential.

---

# 22. Add `cargo-semver-checks`

Before publishing, run a public API compatibility check.

Purpose:

```text
Google API upgrade
      +
Rust library changes
      ↓
Did we unintentionally break googleads-rs's own public API?
```

This is distinct from Google Ads API compatibility.

A Google API breaking change does not automatically justify an accidental break in the Rust crate's public API.

---

# 23. Renovate/Dependabot remains separate

Keep normal dependency automation for:

- `tonic`
- `prost`
- `prost-reflect`
- `tokio`
- `which`
- GitHub Actions
- other Cargo dependencies

Do not try to model Google Ads API releases as ordinary Cargo dependency updates.

Use:

```text
Dependabot/Renovate
        ↓
Cargo dependency upgrades

Google Ads Upgrade Bot
        ↓
Proto/API upgrades
```

---

# 36. Recommended repository additions

Suggested structure:

```text
.github/
  workflows/
    rust.yml
    google-ads-upgrade.yml
    release.yml

utils/
  update.sh
  validate.sh
  detect-google-ads-version.sh

docs/
  automation/
    google-ads-upgrade.md
```

Optional later:

```text
xtask/
```

if the shell scripts become complex enough to justify a Rust task runner.

---

# 30. Suggested `google-ads-upgrade.yml`

High-level structure:

```yaml
name: Google Ads API Upgrade

on:
  schedule:
    - cron: "17 2 * * 1"
  workflow_dispatch:

permissions:
  contents: write
  pull-requests: write

jobs:
  detect:
    runs-on: ubuntu-latest
    outputs:
      upgrade_needed: ${{ steps.detect.outputs.upgrade_needed }}
      target_version: ${{ steps.detect.outputs.target_version }}

    steps:
      - checkout
      - determine-current-version
      - determine-latest-version
      - compare-versions

  upgrade:
    needs: detect
    if: needs.detect.outputs.upgrade_needed == 'true'
    runs-on: ubuntu-latest

    steps:
      - checkout
      - install-protoc
      - install-rust

      - run: ./utils/update.sh v${{ needs.detect.outputs.target_version }}
      - run: cargo fmt

      - validate

      - if: failure()
        run: invoke-coding-agent

      - validate-final

      - create-pull-request
```

The exact coding-agent integration should be selected after evaluating the agent you already use for development.

---

# 31. Suggested validation script

Conceptually:

```bash
#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

If the repository eventually has a dedicated integration-test command, add it here.

The critical requirement is that both humans and automation use the same validation entry point.

---

# 32. Upgrade policy

## Stage 1 — conservative

For every Google Ads release:

```text
detect
→ migrate
→ test
→ AI repair
→ PR
→ human review
→ merge
→ release
→ publish
```

No automatic merge.

## Stage 2 — semi-automatic

After 2–3 successful upgrades:

```text
minor release
→ migrate
→ AI repair if needed
→ all tests pass
→ auto-merge

major release
→ migrate
→ AI repair
→ PR
→ human approval
```

## Stage 3 — highly autonomous

After a longer history of successful upgrades:

```text
minor:
  detect → migrate → test → merge → publish

major:
  detect → migrate → AI repair → test → PR → human approval
```

This preserves human review where risk is highest.

---

# 33. Failure handling

Every failed upgrade should produce a durable GitHub artifact.

Recommended:

- upgrade branch
- GitHub Issue
- PR if enough work was completed
- CI logs
- agent attempt summary
- failing compiler/test output
- Google migration context

Example:

```text
Issue #XXX
Google Ads API v26 upgrade failed

Status:
  Proto migration: PASS
  Compilation: FAIL
  AI repair: 5 attempts
  Tests: FAIL

Human action required.
```

Do not silently retry forever.

---

# 34. Observability

Track each upgrade:

```text
API version
date detected
date PR created
agent required?
agent iterations
final outcome
human fixes required
time to merge
```

After several releases, this will tell you whether the automation is actually reliable.

A simple `docs/automation/history.md` is sufficient initially.

---

# 35. Rollback strategy

Never publish directly from an unreviewed upgrade branch.

If an automated release causes a problem:

1. yank the problematic crate version if appropriate
2. fix main
3. publish patch version
4. investigate the upgrade bot

Keep generated proto changes and migration changes traceable through Git commits.

---

# 31. Implementation sequence

## Milestone 1 — Make migration deterministic

- [ ] Refactor version handling
- [ ] Harden `update.sh`
- [ ] Add `utils/validate.sh`
- [ ] Ensure clean-checkout upgrade works on Ubuntu
- [ ] Ensure validation command reproduces existing CI checks

**Deliverable:** one command can perform and validate a complete API upgrade without a laptop-specific environment.

## Milestone 2 — Automatic Issue creation

- [ ] Implement version detector
- [ ] Add weekly scheduled detector workflow
- [ ] Add `api upgrade bot` label
- [ ] Create idempotent upgrade Issue
- [ ] Add `workflow_dispatch` for detector testing

**Deliverable:** Google releases a new version and a reviewable `api upgrade bot` Issue appears automatically.

## Milestone 3 — Issue-driven upgrade execution

- [ ] Implement background issue worker
- [ ] Detect open `api upgrade bot` Issues
- [ ] Add claim/running state
- [ ] Dispatch upgrade execution workflow
- [ ] Support manual Issue-driven testing
- [ ] Create upgrade branch
- [ ] Run `update.sh`
- [ ] Run validation
- [ ] Automatically open PR
- [ ] Update the Issue with execution status

**Deliverable:** an `api upgrade bot` Issue can independently start an upgrade, without the release detector running.

## Milestone 4 — AI repair

- [ ] Select coding-agent execution method
- [ ] Create strict agent instructions
- [ ] Pass compiler/test output
- [ ] Pass Google migration context
- [ ] Implement maximum 5 attempts
- [ ] Re-run validation after every attempt
- [ ] Push agent commits to upgrade branch

**Deliverable:** routine breaking Rust changes are fixed without laptop intervention.

## Milestone 5 — Release automation

- [ ] Configure GitHub release workflow
- [ ] Add `cargo-semver-checks`
- [ ] Configure crates.io trusted publishing or secret
- [ ] Add release/tag validation
- [ ] Run `cargo package`
- [ ] Run `cargo publish`

**Deliverable:** after merge, crates.io publishing requires no laptop.

## Milestone 6 — Increase autonomy

After observing several successful releases:

- [ ] Auto-merge low-risk upgrades
- [ ] Add merge queue
- [ ] Add real Google Ads smoke test
- [ ] Auto-publish low-risk upgrades
- [ ] Retain human approval for major/high-risk upgrades

---

# 37. Definition of done

The project is considered fully automated when the following is true:

> A new Google Ads API version can be released while the developer's laptop is offline, and GitHub independently detects the release, migrates the repository, repairs ordinary breaking changes, validates the result, creates a reviewable PR, and—after configured approval/merge policy—publishes the resulting crate to crates.io.

The developer should only need to intervene when:

- the migration exceeds the agent's repair budget
- tests reveal an ambiguous semantic change
- Google changed behavior that cannot be inferred from compilation/tests
- a security/release policy requires human approval

---

# 38. Recommended final architecture

```text
                 Google Ads / googleapis
                         │
                         ▼
             ┌────────────────────────┐
             │ Release detector       │
             │ weekly + manual        │
             └───────────┬────────────┘
                         │
                    new version
                         │
                         ▼
                GitHub Issue
              label: api upgrade bot
                         │
                 ┌───────┴────────┐
                 │                │
             human review       closed
                 │
                 ▼
       ┌────────────────────────┐
       │ Background issue       │
       │ worker                 │
       └───────────┬────────────┘
                   │
              claim Issue
                   │
                   ▼
       ┌────────────────────────┐
       │ Upgrade execution      │
       │ workflow               │
       └───────────┬────────────┘
                   │
                   ▼
          deterministic update.sh
                   │
                   ▼
               validation
                   │
             ┌─────┴─────┐
             │           │
            PASS        FAIL
             │           │
             │      coding agent
             │           │
             │      max 5 attempts
             │           │
             └─────┬─────┘
                   │
                   ▼
            compatibility tests
                   │
                   ▼
                 PR
                   │
            human approval
                   │
                   ▼
                main
                   │
                   ▼
          release workflow
                   │
                   ▼
             cargo publish
```

The crucial operational boundary is:

```text
Release detection ≠ upgrade execution
```

A detected release creates a queued Issue; it does not automatically mutate the repository.

The Issue itself becomes the human-visible control point.

# 39. Priority recommendation

If implementation time is limited, prioritize in this exact order:

1. **Refactor version handling**
2. **Harden `update.sh`**
3. **Create one canonical validation command**
4. **Automate version detection + PR creation**
5. **Add bounded coding-agent repair**
6. **Automate crates.io publishing**
7. **Add real Google Ads smoke test**
8. **Enable automatic merging for low-risk releases**

The first four steps already remove most manual work. The AI repair loop then addresses the remaining unpredictable portion.

