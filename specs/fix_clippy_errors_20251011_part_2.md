# Plan to Fix Clippy Errors (Part 2) - 2025-10-11

## Overview

After fixing the compilation error (duplicate impl block in `tests/test_helpers/mod.rs`), running `cargo clippy --all-targets --all-features -- -D warnings` revealed multiple clippy warnings that need to be addressed. The instruction is to **not remove unused code** as it may be needed in the future, so the strategy is to suppress the warnings using appropriate `#[allow(...)]` attributes.

## Error Categories and Analysis

### 1. Unused Imports
**Files affected:**
- `tests/google_ads_row_enum_tests.rs`
- `tests/google_ads_row_phase4567_tests.rs`

**Specific imports:**
- `CustomerBuilder` (enum_tests)
- Multiple enum imports: `AdNetworkType`, `AdvertisingChannelSubType`, `CampaignServingStatus`, `ClickType`, `Slot` (enum_tests)
- `MetricsBuilder`, `SegmentsBuilder` (phase4567_tests)

**Fix Strategy:** Add `#[allow(unused_imports)]` to the import statements.

### 2. Dead Code
**File affected:** `tests/test_helpers/mod.rs`

**Unused structs (never constructed):**
- `AdGroupCriterionBuilder`
- `CampaignCriterionBuilder`
- `AdBuilder`
- `AdGroupAdBuilder`
- `AccountBudgetBuilder`
- `AssetGroupBuilder`
- `AudienceBuilder`
- `BiddingStrategyBuilder`
- `LabelBuilder`
- `CustomerClientBuilder`
- `SearchTermViewBuilder`
- `SmartCampaignSearchTermViewBuilder`
- `ChangeEventBuilder`
- `AdGroupAdAssetViewBuilder`
- `AssetFieldTypeViewBuilder`

**Unused methods:** All methods in the above builders' impl blocks.

**Fix Strategy:** Add `#[allow(dead_code)]` to each unused struct and impl block.

### 3. Needless Update (clippy::needless_update)
**Files affected:**
- `tests/test_helpers/mod.rs` (lines 893, 983)
- `tests/google_ads_row_phase4567_tests.rs` (lines 45, 434)

**Issue:** Using `..Default::default()` when all struct fields have already been explicitly specified.

**Fix Strategy:** Refactor to remove unnecessary `..Default::default()`

### 4. Excessive Precision (clippy::excessive_precision)
**File affected:** `tests/google_ads_row_scalar_tests.rs` (line 541)

**Issue:** Float literal `0.12345678901234567890` has excessive precision.

**Fix Strategy:** Add `#[allow(clippy::excessive_precision)]` to the line, or consider truncating the literal if appropriate, but since it's test data, suppression is preferred.

### 5. Field Reassign with Default (clippy::field_reassign_with_default)
**File affected:** `tests/google_ads_row_enum_tests.rs` (multiple lines: 337, 352, 367, 469, 492)

**Issue:** Assigning fields to structs created with `Default::default()`.

**Fix Strategy:** Refactor to set values directly

## Implementation Plan

### Phase 1: Suppress Unused Imports
1. In `tests/google_ads_row_enum_tests.rs`:
   - Add `#[allow(unused_imports)]` above the import of `CustomerBuilder`
   - Add `#[allow(unused_imports)]` above the enum imports block

2. In `tests/google_ads_row_phase4567_tests.rs`:
   - Add `#[allow(unused_imports)]` above the import of `MetricsBuilder` and `SegmentsBuilder`

### Phase 2: Suppress Dead Code in test_helpers/mod.rs
For each unused builder struct and its impl:
1. Add `#[allow(dead_code)]` above the struct definition
2. Add `#[allow(dead_code)]` above the impl block

Affected builders (in order):
- `AdGroupCriterionBuilder`
- `CampaignCriterionBuilder`
- `AdBuilder`
- `AdGroupAdBuilder`
- `AccountBudgetBuilder`
- `AssetGroupBuilder`
- `AudienceBuilder`
- `BiddingStrategyBuilder`
- `LabelBuilder`
- `CustomerClientBuilder`
- `SearchTermViewBuilder`
- `SmartCampaignSearchTermViewBuilder`
- `ChangeEventBuilder`
- `AdGroupAdAssetViewBuilder`
- `AssetFieldTypeViewBuilder`

### Phase 3: Suppress Clippy Lints
1. **needless_update:**
   - `tests/test_helpers/mod.rs` lines 893, 983
   - `tests/google_ads_row_phase4567_tests.rs` lines 45, 434

2. **excessive_precision:**
   - `tests/google_ads_row_scalar_tests.rs` line 541

3. **field_reassign_with_default:**
   - `tests/google_ads_row_enum_tests.rs` lines 337, 352, 367, 469, 492

Add appropriate `#[allow(clippy::lint_name)]` attributes to each location.

## Verification
After implementing all suppressions:
1. Run `cargo clippy --all-targets --all-features -- -D warnings` again
2. Confirm no warnings remain
3. Run tests to ensure functionality is preserved

## Notes
- All unused code is preserved as per requirements
- Suppressions are targeted to minimize scope
- Test functionality should remain unchanged
- This plan addresses all clippy warnings identified in the current codebase