# Migration to Google Ads API v22

This document outlines the necessary changes and implementation details for migrating the library to Google Ads API v22.

## Upgrade Guide Review

The official Google Ads API v22 Upgrade Guide can be found here:
https://developers.google.com/google-ads/api/docs/upgrade

## Overview

The migration to Google Ads API v22 involves several key steps:
1. Update proto files to v22
2. Update version references across codebase
3. Address breaking changes and removed resources
4. Update tests and documentation
5. Verify GoogleAdsRow.get() implementation

## Implementation Steps

### Step 1: Download and Update Proto Files

Use the existing `utils/update.sh` script:

```bash
./utils/update.sh v22
```

This script will:
- Download latest googleapis proto files
- Extract v22 API definitions
- Update version references in:
  - `build.rs` - Update path filter from `googleads{}v21` to `googleads{}v22`
  - `src/lib.rs` - Update module imports from `v21` to `v22`
  - `tests/*.rs` - Update test imports
  - `tests/test_helpers/*.rs` - Update helper imports
  - `README.md` - Update version documentation

### Step 2: Update Build Configuration

The `build.rs` file needs to be updated to reference v22:

```rust
// In build.rs, line ~28
path_str.contains(&format!("googleads{}v22", std::path::MAIN_SEPARATOR))
    || path_str.contains(&format!("google{}rpc", std::path::MAIN_SEPARATOR))
    || path_str.contains(&format!("google{}longrunning", std::path::MAIN_SEPARATOR))
```

### Step 3: Update Library Module Path

In `src/lib.rs`, update all imports:

```rust
// Change from:
use crate::google::ads::googleads::v21::enums::...
use crate::google::ads::googleads::v21::resources::...

// To:
use crate::google::ads::googleads::v22::enums::...
use crate::google::ads::googleads::v22::resources::...
```

Update the GoogleAdsRow implementation:

```rust
impl google::ads::googleads::v22::services::GoogleAdsRow {
    pub fn get(&self, field_name: &str) -> String {
        // ... implementation
    }
}
```

### Step 4: Update Tests

Update all test files to use v22 paths:

```rust
// In tests/*.rs files
use googleads_rs::google::ads::googleads::v22::services::{
    GoogleAdsServiceClient,
    SearchGoogleAdsStreamRequest,
    SearchGoogleAdsStreamResponse,
    GoogleAdsRow,
};
```

## Breaking Changes and Migration Paths

### Removed Customizer Feeds and FeedItemService

**Status**: Likely already removed in v21 (not found in current v21 proto files)

- The `Feed` resource and `FeedService` have been removed
- `FeedItem` and `FeedItemService` have been removed
- All related fields and operations no longer available

**Migration Path**:
- Use `CustomizerAttribute` for customizer functionality
- Use `Asset` for dynamic content insertion
- For location extensions, use `LocationAsset`
- For call extensions, use `CallAsset`

**Code Impact**:
- No changes needed in `src/lib.rs` as these fields are not currently implemented in GoogleAdsRow.get()
- If any customer code references feeds, update to use Assets API

### Removed Label Service Resources

**Status**: To be verified in v22

- Some label-related resources may be consolidated or removed
- Any code referencing these services or resources will need to be updated.

**Current Label Resources in v21** (to verify in v22):
- `label.proto`
- `ad_group_label.proto`
- `campaign_label.proto`
- `customer_label.proto`
- `ad_group_criterion_label.proto`
- `ad_group_ad_label.proto`

**Migration Path**:
- Verify which label services remain in v22
- Update any label-related queries to use supported resources

### Field Removals and Replacements

#### Campaign Status Fields

**Removed**:
- Legacy status fields may be deprecated

**Use Instead**:
- `campaign.primary_status` (already in v21)
- `campaign.primary_status_reasons` (already in v21)

**Code Impact**:
- Verify GoogleAdsRow.get() implementation includes primary_status fields
- Current v21 implementation should already support these fields

#### Ad Group Status Fields

**Use**:
- `ad_group.primary_status`
- `ad_group.primary_status_reasons`

#### Ad Group Criterion Status Fields

**Use**:
- `ad_group_criterion.primary_status`
- `ad_group_criterion.primary_status_reasons`

### Other Breaking Changes

To be documented after reviewing official v22 upgrade guide:
- Check for removed enum values
- Check for renamed fields
- Check for changed field types
- Check for removed metrics or segments

## Testing Strategy

### Build Verification

1. **Compile Check**:
```bash
cargo clean
cargo build
```

2. **Run Integration Tests**:
```bash
cargo test --test build_verification_tests
cargo test --test mock_integration_tests
```

3. **Verify Service Clients**:
- GoogleAdsServiceClient
- CustomerServiceClient
- CampaignServiceClient
- AdGroupServiceClient

### GoogleAdsRow.get() Verification

1. **Test all implemented field paths** in src/lib.rs
2. **Check for removed fields** and update match arms
3. **Run unit tests**:
```bash
cargo test google_ads_row
```

4. **Fields to verify exist in v22**:
- All metrics fields
- All segments fields
- Resource fields (campaign, ad_group, ad, etc.)
- Primary status fields

### Documentation Update

1. Update `README.md` version badge
2. Update `Cargo.toml` version
3. Update inline documentation
4. Generate and review docs:
```bash
cargo doc --no-deps --open
```

## Deprecations and Enhancements

### New Fields and Resources

To be identified from v22 release notes:
- New resource types
- New metrics
- New segments
- New enum values

### Performance Improvements

v22 may include:
- Query performance optimizations
- New reporting capabilities
- Enhanced targeting options

## Potential Issues and Solutions

### Issue 1: Proto Compilation Errors

**Symptoms**: Build fails with protoc errors

**Solutions**:
- Ensure protobuf-compiler is installed
- Check proto file syntax (comments in examples)
- Verify all required proto dependencies are included
- Add `ignore` blocks to code examples in proto comments if needed

### Issue 2: Missing Field in GoogleAdsRow.get()

**Symptoms**: Field path returns "not implemented by googleads-rs"

**Solutions**:
- Add field to match statement in src/lib.rs
- Use appropriate macro: `attr_str!`, `method_str!`, or `optional_attr_str!`
- Add test case for the field

### Issue 3: Removed Resource Referenced in Code

**Symptoms**: Compilation error for missing resource type

**Solutions**:
- Check v22 upgrade guide for replacement
- Update to use new resource or API
- Remove references to deprecated resources

## Checklist for Migration

- [ ] Run `./utils/update.sh v22`
- [ ] Verify proto files downloaded correctly
- [ ] Update `build.rs` path filters
- [ ] Update `src/lib.rs` module imports
- [ ] Update all test files
- [ ] Update `README.md`
- [ ] Run `cargo clean && cargo build`
- [ ] Fix any compilation errors
- [ ] Run all tests
- [ ] Verify GoogleAdsRow.get() functionality
- [ ] Update documentation
- [ ] Update `Cargo.toml` version
- [ ] Test with real API calls (if available)
- [ ] Review release notes for any additional changes

## Resources

- [Google Ads API v22 Release Notes](https://developers.google.com/google-ads/api/docs/release-notes)
- [Google Ads API v22 Upgrade Guide](https://developers.google.com/google-ads/api/docs/upgrade)
- [Google Ads API Reference](https://developers.google.com/google-ads/api/reference/rpc/v22/overview)
- [googleapis GitHub Repository](https://github.com/googleapis/googleapis)

## Post-Migration Validation

1. **Verify common query patterns**:
```sql
SELECT campaign.id, campaign.name, campaign.status, 
       campaign.primary_status, metrics.impressions, metrics.clicks
FROM campaign
WHERE campaign.status = 'ENABLED'
```

2. **Test streaming responses**:
- Verify SearchGoogleAdsStreamRequest works
- Verify field_mask is properly handled
- Verify GoogleAdsRow.get() returns correct values

3. **Check error handling**:
- Verify gRPC status codes
- Verify error messages are clear
- Verify retry logic if implemented

## Notes

- The `update.sh` script automates most version updates
- Manual verification is required for breaking changes
- Consider running migration in a feature branch
- Test thoroughly before releasing to production
- Check if any custom code depends on removed resources
