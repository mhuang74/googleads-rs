# Google Ads API v23 Upgrade Plan

## Objective
Upgrade the googleads-rs library from Google Ads API v22 to v23, ensuring all functionality continues to work while adopting new features and handling breaking changes.

## Phase 1: Preparation and Proto File Updates

### 1.1 Download v23 Proto Files
- Run `./utils/update.sh v23` to download and extract v23 API definitions
- Verify proto files are downloaded to `proto/google/ads/googleads/v23/` directory

### 1.2 Update Build Configuration
- **File**: `build.rs`
- **Change**: Update path filter from `v22` to `v23` on line 29:
  ```rust
  path_str.contains(&format!("googleads{}v23", std::path::MAIN_SEPARATOR))
  ```
- Verify build compiles after proto update

## Phase 2: Codebase Version Updates

### 2.1 Update Library Imports
- **File**: `src/lib.rs`
- **Changes**:
  - Update all imports from `v22` to `v23`
  - Line 30: Update BiddingStrategyType import path
  - Line 36: Update primary status enum imports
  - Line 45: Update resource imports (AdData, Criterion variants)
  - Line 51: Update GoogleAdsRow impl block type to `v23`

### 2.2 Update Test Files
- **Files**: `tests/*.rs`, `tests/test_helpers/*.rs`
- **Changes**: Update all imports from `googleads_rs::google::ads::googleads::v22` to `v23`

### 2.3 Update Documentation
- **File**: `README.md`
- **Changes**: Update version references to v23
- **File**: `src/lib.rs` (line 20)
- **Changes**: Update doc root URL if needed
- **File**: `Cargo.toml`
- **Changes**: Update version number (e.g., 0.12.1 → 0.13.0)

## Phase 3: GoogleAdsRow.get() Breaking Changes

### 3.1 Handle Renamed Video Metrics
- **File**: `src/lib.rs`
- **Changes**: Update metric names (currently commented out in v22):

| Line (v22) | Old Field | New Field | Action |
|-------------|-----------|-----------|--------|
| 558 | `metrics.average_cpv` | `metrics.trueview_average_cpv` | Uncomment and rename |
| 653 | `metrics.video_view_rate` | `metrics.trueview_video_view_rate` | Uncomment and rename |
| 654 | `metrics.video_views` | `metrics.trueview_video_views` | Uncomment and rename |
| 859 | `metrics.video_view_rate_in_feed` | `metrics.trueview_video_view_rate_in_feed` | Uncomment and rename |
| 860 | `metrics.video_view_rate_in_stream` | `metrics.trueview_video_view_rate_in_stream` | Uncomment and rename |
| 861 | `metrics.video_view_rate_shorts` | `metrics.trueview_video_view_rate_shorts` | Uncomment and rename |

### 3.2 Add New Campaign DateTime Fields
- **File**: `src/lib.rs`
- **Changes**: Add match arms after line 432 (`campaign.start_date`):
  ```rust
  "campaign.start_date_time" => attr_str!([campaign], start_date_time),
  "campaign.end_date_time" => attr_str!([campaign], end_date_time),
  ```
- Note: Keep `campaign.start_date` and `campaign.end_date` for backward compatibility if proto still supports them

### 3.3 Add New Asset Field
- **File**: `src/lib.rs`
- **Changes**: Add match arm after line 362 (`asset.source`):
  ```rust
  "asset.orientation" => method_str!([asset], orientation),
  ```

### 3.4 Remove Already-Commented AssetPerformanceLabel
- **File**: `src/lib.rs`
- **Changes**: Line 778 (`asset_group_asset.performance_label`) is already commented out
- Action: Remove the commented line entirely for cleaner code

### 3.5 Verify No CallAd References
- Search entire codebase for `CallAd` or `CallAdInfo` references
- Remove any match arms for these removed resources

## Phase 4: Build and Compile Verification

### 4.1 Clean Build
```bash
cargo clean
cargo build
```

### 4.2 Fix Compilation Errors
- Address any missing imports
- Fix any type mismatches from renamed fields
- Ensure all proto types are accessible

## Phase 5: Test Updates and Execution

### 5.1 Update Test Files for v23
- Update all GAQL queries that reference renamed metrics
- Update any hardcoded field names in test data
- Update mock data structures to use v23 types

### 5.2 Run Unit Tests
```bash
cargo test google_ads_row
cargo test
```

### 5.3 Run Integration Tests
```bash
cargo test --test build_verification_tests
cargo test --test mock_integration_tests
```

### 5.4 Fix Test Failures
- Update tests that use removed fields
- Update tests that expect old metric names
- Add new tests for campaign datetime fields and asset orientation

## Phase 6: Validation

### 6.1 Verify GoogleAdsRow.get() Functionality
- Test that all existing field paths still work
- Test new fields (campaign.start_date_time, asset.orientation)
- Test renamed video metrics return correct data
- Verify backward compatibility for deprecated fields

### 6.2 Test Common Query Patterns
```sql
-- Campaign query with new datetime fields
SELECT campaign.id, campaign.name, campaign.status,
       campaign.start_date_time, campaign.end_date_time,
       metrics.impressions, metrics.clicks
FROM campaign

-- Video metrics with new names
SELECT campaign.id, metrics.trueview_average_cpv,
       metrics.trueview_video_views
FROM campaign

-- Asset with orientation
SELECT asset.id, asset.type, asset.orientation
FROM asset
```

### 6.3 Generate Documentation
```bash
cargo doc --no-deps --open
```

## Phase 7: Finalization

### 7.1 Update Changelog
- Document v23 upgrade
- List breaking changes
- List new features

### 7.2 Version Bump
- Update `Cargo.toml` version (e.g., 0.13.0)
- Tag release in git

### 7.3 Commit Changes
```bash
git add .
git commit -m "Upgrade to Google Ads API v23

- Update proto files to v23
- Rename video metrics (trueview_* prefix)
- Add campaign.start_date_time and campaign.end_date_time
- Add asset.orientation field
- Update all imports from v22 to v23
- Update version to 0.13.0"
```

## Summary of Files to Modify

| File | Changes |
|------|---------|
| `build.rs` | v22 → v23 in path filter |
| `src/lib.rs` | v22 → v23 imports, renamed metrics, new fields |
| `Cargo.toml` | Version bump (0.13.0) |
| `README.md` | Version documentation update |
| `tests/*.rs` | v22 → v23 imports |
| `specs/migrate_to_gads_v23.md` | New migration document |

## Breaking Changes to Document

1. **Video metrics renamed**: All video-related metrics now have `trueview_` prefix
2. **Campaign dates**: New `start_date_time`/`end_date_time` fields added
3. **CallAd removed**: No longer supported in v23
4. **AssetPerformanceLabel**: No longer returned for Search/Display campaigns

## New Features to Document

1. Campaign datetime fields with time component support
2. Asset orientation field for image/video assets
3. New services available (IncentiveService, BenchmarksService, AssetGenerationService)
