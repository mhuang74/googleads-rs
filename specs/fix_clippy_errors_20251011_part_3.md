# Plan to Fix Clippy Errors (Part 3) - 2025-10-11

This plan targets resolving the clippy gate: cargo clippy --all-targets --all-features -- -D warnings, while preserving unused code for future phases. It builds on the prior plans:
- [`specs/fix_clippy_errors_20251011.md`](specs/fix_clippy_errors_20251011.md)
- [`specs/fix_clippy_errors_20251011_part_2.md`](specs/fix_clippy_errors_20251011_part_2.md)

Key principle: do not remove unused code. Prefer narrowly-scoped lint allows, no-op preserving patterns (such as aliasing unused imports), or low-risk refactors that do not change behavior.


## Current baseline (from Parts 1 and 2)

Observed categories (representative, not exhaustive):
- Unused imports in integration tests such as:
  - [`tests/google_ads_row_enum_tests.rs`](tests/google_ads_row_enum_tests.rs)
  - [`tests/google_ads_row_phase4567_tests.rs`](tests/google_ads_row_phase4567_tests.rs)
- Dead code in the test helpers module:
  - [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs)
- Redundant field names within struct initialization
  - Primarily within test helper builders and fixtures
- needless_update when all fields are explicitly set but include ..Default::default()
  - In both helpers and tests (e.g., enum/phase4567 tests)
- field_reassign_with_default in tests that intentionally favor readability
  - e.g., initialize with Default then mutate selected fields step-by-step
- excessive_precision in specific test float literals
  - e.g., a long fractional literal in scalar tests


## Root-cause hypotheses (enumerate 5–7)

1) The comprehensive fixture/builders in [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs) purposely include many structs and impl methods to support later phases, which are currently unused and trigger dead_code.

2) Integration tests are split by feature/phase and explicitly import many enums and builders for readability. Some imports are not used in every test, producing unused_imports.

3) Readability-first test setup patterns use Default plus subsequent field assignments, tripping field_reassign_with_default.

4) Some struct initializations specify all fields but still append ..Default::default(), causing needless_update.

5) Occasional redundant field names exist where local variable names match field names in struct literals.

6) Long float literals in tests exceed representable precision, tripping excessive_precision.

7) all-targets and all-features widen clippy’s scope to include dev/test code and code generated for multiple feature sets, surfacing lints not visible under a single-target run.


## Most likely primary sources (distilled)

- Primary: The test helpers module in [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs) (dead_code across multiple builder types and impl methods).
- Secondary: Integration test readability patterns (unused_imports, field_reassign_with_default, needless_update, and an isolated excessive_precision literal).


## Validation steps (run before any changes)

1) Run clippy to capture a fresh baseline across all targets and features:
   - cargo clippy --all-targets --all-features -- -D warnings

2) If needed for triage, collect machine-readable output:
   - cargo clippy --all-targets --all-features --message-format=json -q

3) Tally warnings by file to confirm concentration in:
   - [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs)
   - [`tests/google_ads_row_enum_tests.rs`](tests/google_ads_row_enum_tests.rs)
   - [`tests/google_ads_row_phase4567_tests.rs`](tests/google_ads_row_phase4567_tests.rs)
   - [`tests/google_ads_row_phase1_tests.rs`](tests/google_ads_row_phase1_tests.rs)
   - [`tests/google_ads_row_scalar_tests.rs`](tests/google_ads_row_scalar_tests.rs)

4) Confirm no lints emerge from the crate’s public API in [`src/lib.rs`](src/lib.rs) that would require semantic changes. This keeps the production surface stable.


## Implementation plan (phased, minimal-risk, no removals)

Phase A — Contain dead_code to the helpers module (fastest impact)
- Add a single, file-scoped allow for dead_code at the top of [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs). This keeps all builder types/methods intact for future phases without repeating attributes on each type or impl.
- Rationale: This module is test-only scaffolding by design. File-level containment yields the narrowest scope while eliminating a large class of warnings.

Phase B — Resolve unused imports in integration tests with intent-preserving aliases
- Prefer no-op aliasing over blanket allows to codify intent and avoid future “accidental” unused imports:
  - Example approach: use SomeType as _; to intentionally mark an imported symbol as unused while keeping the reference visible for future use.
- Apply to imports called out in the earlier parts:
  - [`tests/google_ads_row_enum_tests.rs`](tests/google_ads_row_enum_tests.rs) for CustomerBuilder and individual enum imports.
  - [`tests/google_ads_row_phase4567_tests.rs`](tests/google_ads_row_phase4567_tests.rs) for MetricsBuilder and SegmentsBuilder.
- If aliasing is too noisy in a given file, fall back to a narrow, file-top allow for unused_imports in that file only (avoid crate-wide/global settings).

Phase C — Address field_reassign_with_default in tests
- Tests often prioritize clarity over minimal assignments. Options, in order of preference:
  1) Keep the readable pattern and add a narrow, file-top allow for the field_reassign_with_default lint in the specific test files that rely on it most:
     - [`tests/google_ads_row_enum_tests.rs`](tests/google_ads_row_enum_tests.rs)
     - [`tests/google_ads_row_phase1_tests.rs`](tests/google_ads_row_phase1_tests.rs)
  2) If a test has very short initialization, directly construct the struct with all fields set in the literal, but only when this doesn’t hurt readability.
- Do not refactor to reduce test clarity in complex fixtures.

Phase D — Remove needless_update where safe, else allow narrowly
- For struct literals where all fields are set, drop ..Default::default() in:
  - [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs)
  - [`tests/google_ads_row_phase4567_tests.rs`](tests/google_ads_row_phase4567_tests.rs)
- If a case is ambiguous or would reduce clarity, apply a narrow, file-top allow for needless_update in that file.

Phase E — Fix redundant field names in struct literals
- Replace patterns like field_name: field_name with just field_name in:
  - [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs)
- This is a mechanical readability improvement with no behavioral risk.

Phase F — Handle the single excessive_precision literal
- In [`tests/google_ads_row_scalar_tests.rs`](tests/google_ads_row_scalar_tests.rs):
  - Prefer keeping the literal as-is and adding a narrow, line- or file-level allow for excessive_precision because it’s test data meant to assert parsing/rounding behavior.
  - Only truncate precision if the test’s intent remains identical.

Phase G — Optional containment for future-proofing (tests only)
- If more test-only lints emerge, consider adding a test-only lint shim per file (at the top of each integration test) that centralizes the few allows we want to tolerate in tests (field_reassign_with_default, needless_update, excessive_precision, and occasionally unused_imports). Avoid workspace- or crate-wide suppression to keep scope tight.


## Execution order

1) Apply file-level containment for dead_code in [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs).
2) Update unused imports in:
   - [`tests/google_ads_row_enum_tests.rs`](tests/google_ads_row_enum_tests.rs)
   - [`tests/google_ads_row_phase4567_tests.rs`](tests/google_ads_row_phase4567_tests.rs)
3) Fix redundant field names in [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs).
4) Remove obvious, safe needless_update patterns; allow narrowly where warranted:
   - [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs)
   - [`tests/google_ads_row_phase4567_tests.rs`](tests/google_ads_row_phase4567_tests.rs)
5) For field_reassign_with_default, prefer narrow file-level allows in:
   - [`tests/google_ads_row_enum_tests.rs`](tests/google_ads_row_enum_tests.rs)
   - [`tests/google_ads_row_phase1_tests.rs`](tests/google_ads_row_phase1_tests.rs)
6) Add a targeted allow for excessive_precision or adjust the literal in:
   - [`tests/google_ads_row_scalar_tests.rs`](tests/google_ads_row_scalar_tests.rs)
7) Re-run clippy across all targets and features, then run tests.


## Validation (after changes)

- cargo clippy --all-targets --all-features -- -D warnings returns exit code 0.
- cargo test passes.
- Confirm no new warnings were introduced, and that the large majority of prior warnings have been eliminated or contained to the smallest possible scope.


## Safeguards and non-removal guarantees

- No API/behavior changes in production code (for example, no functional change in [`src/lib.rs`](src/lib.rs)).
- No removal of any builders, types, or imports intended for future phases.
- Prefer localized, file-scoped lint management in tests and helpers over global workarounds.


## Risk assessment and rollback

- Risk: Over-broad allows could hide useful signal.
  - Mitigation: keep allows file-local and limited to known test-only patterns.
- Rollback: All edits are mechanical and isolated to test files and helpers. Revert individual files if any regression is suspected.


## Acceptance criteria

- Clippy passes with -D warnings for all targets and features.
- All tests pass.
- Unused code remains present for future phases.
- Lint allows are narrowly scoped (primarily file-local in tests/helpers).
- Readability of tests is preserved or improved.


## Request for confirmation (diagnosis checkpoint before implementation)

Please confirm this diagnosis and containment strategy:
- The majority of actionable lints are test-only (helpers and integration tests), with production code remaining largely unaffected.
- We will:
  - Contain dead_code to [`tests/test_helpers/mod.rs`](tests/test_helpers/mod.rs) via a file-level allow.
  - Replace redundant field names and prune needless_update only when safe, else add narrow, file-level allows.
  - Prefer unused import aliasing (as _) in test files, falling back to file-level allows if needed.
  - Handle field_reassign_with_default and excessive_precision with narrow, test-file-level allows to preserve readability.

Upon confirmation, proceed with Phase A through Phase G in the Execution order above, validating via clippy and test runs after implementation.