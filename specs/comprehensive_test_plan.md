# Comprehensive Test Plan for googleads-rs

## Overview

This document outlines a comprehensive testing strategy for googleads-rs, a gRPC client library for Google Ads API v19. The library auto-generates code from protobuf definitions and provides a hand-crafted `GoogleAdsRow::get()` method for field extraction.

## Current State

### Existing Tests
- `tests/version-numbers.rs` - Version consistency validation
  - `test_readme_deps_updated()` - Validates dependencies in README
  - `test_readme_mentions_version()` - Validates version string in README
  - `test_html_root_url()` - Validates doc root URL in lib.rs

### Test Gaps
- No unit tests for `GoogleAdsRow::get()` method (200+ field paths)
- No integration tests with mock gRPC servers
- No validation of generated protobuf code
- No regression tests for API responses
- Limited documentation test coverage (many marked as `ignore`)

## Testing Strategy

### 1. Unit Tests (HIGH PRIORITY)

#### 1.1 GoogleAdsRow::get() Method Tests
**Location:** `tests/google_ads_row_get_tests.rs`

**Objectives:**
- Test all implemented match arms (200+ field paths)
- Verify correct string formatting for different field types
- Test edge cases and error conditions

**Test Categories:**

##### 1.1.1 Scalar Attribute Fields
Test macro: `attr_str!` and `optional_attr_str!`

```rust
#[test]
fn test_campaign_scalar_fields() {
    // Test: campaign.id, campaign.name, campaign.end_date
    // Verify: Correct string formatting of int64, string fields
}

#[test]
fn test_campaign_budget_optional_fields() {
    // Test: campaign_budget.amount_micros (optional parent)
    // Verify: Returns value when present, empty string when absent
}

#[test]
fn test_nested_scalar_fields() {
    // Test: campaign.network_settings.target_search_network
    // Verify: Nested message field access works correctly
}
```

##### 1.1.2 Enum Accessor Method Fields
Test macros: `method_str!` and `optional_method_str!`

```rust
#[test]
fn test_enum_status_fields() {
    // Test: campaign.status, ad_group.status, customer.status
    // Verify: Debug format of enum values (e.g., "Enabled", "Paused")
}

#[test]
fn test_enum_type_fields() {
    // Test: ad_group.type (r#type), campaign.advertising_channel_type
    // Verify: Rust keyword escaping works correctly
}

#[test]
fn test_optional_enum_fields() {
    // Test: campaign_criterion.status (optional parent)
    // Verify: Returns enum debug string or empty string
}
```

##### 1.1.3 Oneof Union Fields
Test macros: `enum_match_str!` and `optional_enum_match_str!`

```rust
#[test]
fn test_keyword_criterion_fields() {
    // Test: ad_group_criterion.keyword.text, ad_group_criterion.keyword.match_type
    // Verify: Correct variant selection and field extraction
}

#[test]
fn test_location_criterion_fields() {
    // Test: campaign_criterion.location.geo_target_constant
    // Verify: Optional oneof handling
}

#[test]
fn test_ad_data_oneof_fields() {
    // Test: ad_group_ad.ad.responsive_search_ad.path1
    // Verify: ResponsiveSearchAd variant selection
}

#[test]
fn test_oneof_variant_mismatch() {
    // Test: Request keyword field when Location variant is active
    // Verify: Returns empty string
}
```

##### 1.1.4 Repeated Field Iteration
Test macro: `enum_match_iterator_str!`

```rust
#[test]
fn test_responsive_search_ad_headlines() {
    // Test: ad_group_ad.ad.responsive_search_ad.headlines
    // Verify: Comma-separated list of headline texts
}

#[test]
fn test_labels_repeated_fields() {
    // Test: campaign.labels, ad_group.labels
    // Verify: Comma-separated string list
}

#[test]
fn test_field_mask_paths() {
    // Test: change_event.changed_fields
    // Verify: Quoted, comma-delimited list
}
```

##### 1.1.5 Custom Enum Mappings

```rust
#[test]
fn test_bidding_strategy_type_custom_mapping() {
    // Test: campaign.bidding_strategy_type
    // Verify: Custom string tokens (ManualCPC, MaximizeConversions, etc.)
}
```

##### 1.1.6 Metrics and Segments Fields

```rust
#[test]
fn test_metrics_scalar_fields() {
    // Test: metrics.impressions, metrics.clicks, metrics.ctr
    // Verify: Correct double/int64 formatting
}

#[test]
fn test_segments_fields() {
    // Test: segments.date, segments.device, segments.ad_network_type
    // Verify: Mix of scalar and enum accessor fields
}
```

##### 1.1.7 Edge Cases and Error Conditions

```rust
#[test]
fn test_unimplemented_field_path() {
    // Test: "unknown.field.path"
    // Verify: Returns "not implemented by googleads-rs"
}

#[test]
fn test_missing_optional_parent() {
    // Test: campaign_criterion.* when campaign_criterion is None
    // Verify: Returns empty string without panic
}

#[test]
fn test_empty_repeated_fields() {
    // Test: campaign.labels when empty
    // Verify: Returns empty string
}
```

#### 1.2 Test Data Builders
**Location:** `tests/test_helpers/mod.rs`

Create builder pattern for constructing test GoogleAdsRow instances:

```rust
pub struct GoogleAdsRowBuilder {
    campaign: Option<Campaign>,
    ad_group: Option<AdGroup>,
    metrics: Option<Metrics>,
    // ... other resources
}

impl GoogleAdsRowBuilder {
    pub fn with_campaign(mut self, campaign: Campaign) -> Self { ... }
    pub fn with_metrics(mut self, metrics: Metrics) -> Self { ... }
    pub fn build(self) -> GoogleAdsRow { ... }
}

// Helper builders for nested messages
pub struct CampaignBuilder { ... }
pub struct AdGroupBuilder { ... }
pub struct MetricsBuilder { ... }
```

### 2. Integration Tests (MEDIUM PRIORITY)

#### 2.1 Mock gRPC Server Tests
**Location:** `tests/integration/grpc_client_tests.rs`

**Dependencies:**
Add to `Cargo.toml`:
```toml
[dev-dependencies]
tonic = { version = "0.8.0", features = ["testing"] }
tokio = { version = "1.39", features = ["full", "test-util"] }
```

**Test Scenarios:**

```rust
#[tokio::test]
async fn test_search_stream_request_success() {
    // Create mock GoogleAdsServiceClient
    // Send SearchGoogleAdsStreamRequest
    // Verify response streaming works
    // Test field_mask extraction
}

#[tokio::test]
async fn test_search_stream_with_field_mask() {
    // Test that field_mask.paths are correctly populated
    // Verify get() works with all paths from field_mask
}

#[tokio::test]
async fn test_search_stream_error_handling() {
    // Mock Status error responses
    // Verify error messages and details are captured
}

#[tokio::test]
async fn test_multiple_stream_batches() {
    // Mock multiple SearchGoogleAdsStreamResponse items
    // Verify all results are processed
}

#[tokio::test]
async fn test_interceptor_integration() {
    // Test GoogleAdsAPIAccess interceptor
    // Verify authentication headers
}
```

#### 2.2 End-to-End Query Tests
**Location:** `tests/integration/query_tests.rs`

```rust
#[tokio::test]
#[ignore] // Requires actual API credentials
async fn test_real_api_campaign_query() {
    // Connect to real Google Ads API
    // Execute simple campaign query
    // Verify response structure
}
```

### 3. Build System Tests (MEDIUM PRIORITY)

#### 3.1 Proto Compilation Tests
**Location:** `tests/build_tests.rs`

```rust
#[test]
fn test_proto_file_discovery() {
    // Verify proto files are found in expected locations
    // Test: googleads/v19, google/rpc, google/longrunning
}

#[test]
fn test_package_extraction() {
    // Read sample proto files
    // Verify package name extraction logic
}

#[test]
fn test_proto_filtering() {
    // Verify only v19 protos are included
    // Test path filtering logic
}

#[test]
fn test_rust_keyword_mapping() {
    // Test map_keyword() function
    // Verify: "type" -> "r#type", "self" -> "self_", etc.
}

#[test]
fn test_protos_rs_generation() {
    // Mock package set
    // Verify write_protos_rs() output format
    // Check module nesting and include_proto! calls
}
```

### 4. Property-Based Tests (LOW PRIORITY)

#### 4.1 Fuzzing GoogleAdsRow::get()
**Location:** `tests/property_tests.rs`

**Dependencies:**
```toml
[dev-dependencies]
proptest = "1.0"
```

**Test Cases:**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_get_never_panics(field_path in ".*") {
        // Create minimal GoogleAdsRow
        // Call get() with random field path
        // Verify: No panics, always returns String
    }

    #[test]
    fn test_get_with_random_valid_paths(
        field_path in prop::sample::select(vec![
            "campaign.id",
            "metrics.clicks",
            // ... all valid paths
        ])
    ) {
        // Create fully populated GoogleAdsRow
        // Verify get() returns non-empty string
    }
}
```

### 5. Documentation Tests (HIGH PRIORITY)

#### 5.1 Fix Existing Doc Tests
**Location:** `src/lib.rs`

**Current Issues:**
- Proto comment blocks cause compilation errors
- Main example marked as `ignore`

**Actions:**
1. Review all ````ignore` blocks
2. Create minimal working examples
3. Add `# ` hidden setup code for complex examples

```rust
/// # Example
///
/// ```
/// # use googleads_rs::google::ads::googleads::v19::services::GoogleAdsRow;
/// # use googleads_rs::google::ads::googleads::v19::resources::Campaign;
/// let mut row = GoogleAdsRow::default();
/// let mut campaign = Campaign::default();
/// campaign.id = 12345;
/// campaign.name = "Test Campaign".to_string();
/// row.campaign = Some(campaign);
///
/// assert_eq!(row.get("campaign.id"), "12345");
/// assert_eq!(row.get("campaign.name"), "Test Campaign");
/// ```
```

#### 5.2 README Example Tests
**Location:** `tests/readme_examples.rs`

Extract code examples from README and verify they compile/work.

### 6. Regression Tests (MEDIUM PRIORITY)

#### 6.1 Golden File Tests
**Location:** `tests/golden/`

**Structure:**
```
tests/golden/
  ├── responses/
  │   ├── campaign_response_v19.json
  │   ├── ad_group_response_v19.json
  │   └── metrics_response_v19.json
  └── expected_output/
      ├── campaign_get_output.txt
      └── ad_group_get_output.txt
```

**Test Implementation:**

```rust
#[test]
fn test_campaign_get_output_unchanged() {
    // Load golden response JSON
    // Deserialize to GoogleAdsRow
    // Call get() on all known paths
    // Compare with expected output file
    // Fail if output changes (indicates regression)
}
```

### 7. Performance Tests (LOW PRIORITY)

#### 7.1 Benchmarks
**Location:** `benches/get_benchmarks.rs`

**Dependencies:**
```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "get_benchmarks"
harness = false
```

**Benchmark Scenarios:**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_get_scalar_field(c: &mut Criterion) {
    let row = create_populated_row();
    c.bench_function("get campaign.id", |b| {
        b.iter(|| row.get(black_box("campaign.id")))
    });
}

fn bench_get_enum_field(c: &mut Criterion) {
    let row = create_populated_row();
    c.bench_function("get campaign.status", |b| {
        b.iter(|| row.get(black_box("campaign.status")))
    });
}

fn bench_get_oneof_field(c: &mut Criterion) {
    let row = create_populated_row();
    c.bench_function("get ad_group_criterion.keyword.text", |b| {
        b.iter(|| row.get(black_box("ad_group_criterion.keyword.text")))
    });
}

criterion_group!(benches, bench_get_scalar_field, bench_get_enum_field, bench_get_oneof_field);
criterion_main!(benches);
```

### 8. CI/CD Enhancements

#### 8.1 GitHub Actions Workflow Updates
**Location:** `.github/workflows/rust.yml`

**Additions:**

```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install protobuf
        run: sudo apt-get -y install protobuf-compiler

      - name: Run tests with coverage
        run: |
          cargo install cargo-llvm-cov
          cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install protobuf
        run: sudo apt-get -y install protobuf-compiler

      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check

  doc-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install protobuf
        run: sudo apt-get -y install protobuf-compiler

      - name: Test documentation
        run: cargo test --doc

  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install protobuf
        run: sudo apt-get -y install protobuf-compiler

      - name: Run benchmarks
        run: cargo bench --no-fail-fast
```

## Implementation Roadmap

### Phase 1: Foundation (Week 1-2)
- [ ] Create test helper builders for GoogleAdsRow
- [ ] Implement unit tests for scalar fields (attr_str, optional_attr_str)
- [ ] Implement unit tests for enum fields (method_str, optional_method_str)
- [ ] Add test coverage reporting to CI

### Phase 2: Core Functionality (Week 3-4)
- [ ] Implement unit tests for oneof fields (enum_match_str, optional_enum_match_str)
- [ ] Implement unit tests for repeated fields (enum_match_iterator_str)
- [ ] Add edge case and error condition tests
- [ ] Achieve >80% coverage of get() method

### Phase 3: Integration & Build (Week 5-6)
- [ ] Implement mock gRPC server integration tests
- [ ] Add build system tests for proto compilation
- [ ] Create golden file regression tests
- [ ] Add clippy and rustfmt to CI

### Phase 4: Advanced Testing (Week 7-8)
- [ ] Implement property-based tests with proptest
- [ ] Add performance benchmarks with criterion
- [ ] Fix and expand documentation tests
- [ ] Create comprehensive test coverage report

### Phase 5: Maintenance (Ongoing)
- [ ] Add tests for new match arms as they're implemented
- [ ] Update golden files when API version changes
- [ ] Monitor test performance and optimize slow tests
- [ ] Review and update test documentation

## Test Metrics & Goals

### Coverage Targets
- **Overall code coverage:** >75%
- **`GoogleAdsRow::get()` coverage:** >90%
- **Critical path coverage:** 100%

### Quality Metrics
- **All tests pass on:** Linux, macOS, Windows
- **Test execution time:** <2 minutes for full suite
- **Flaky test rate:** <1%
- **Documentation coverage:** All public APIs have examples

### Continuous Monitoring
- Track coverage trends over time
- Monitor test execution time
- Review failed test patterns
- Update tests when API versions change (v19 -> v20, etc.)

## Testing Best Practices

### General Guidelines
1. **Test Isolation:** Each test should be independent
2. **Descriptive Names:** Use clear, behavior-focused test names
3. **Arrange-Act-Assert:** Structure tests clearly
4. **DRY Principle:** Use helper builders to reduce duplication
5. **Fast Tests:** Keep unit tests fast (<100ms each)
6. **Deterministic:** No random failures or timing dependencies

### GoogleAdsRow-Specific Guidelines
1. **Test All Macros:** Each macro variation should have dedicated tests
2. **Test Edge Cases:** Missing parents, empty oneofs, empty repeated fields
3. **Test Proto Changes:** Regression tests catch breaking proto updates
4. **Document Test Data:** Use realistic example values from Google Ads API
5. **Version Awareness:** Tag tests with API version (v19) for future migration

### Maintenance Guidelines
1. **Update Tests First:** When adding new match arms, add tests first (TDD)
2. **Review Coverage:** Check coverage reports before merging PRs
3. **Keep Golden Files Fresh:** Update when API responses change
4. **Document Failures:** Add comments explaining complex test scenarios
5. **Clean Up:** Remove obsolete tests when removing features

## References

- [Google Ads API Documentation](https://developers.google.com/google-ads/api/docs/start)
- [Tonic Testing Guide](https://github.com/hyperium/tonic/blob/master/examples/README.md)
- [Rust Testing Best Practices](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Property-Based Testing with Proptest](https://github.com/proptest-rs/proptest)
- [Criterion Benchmarking](https://github.com/bheisler/criterion.rs)

## Appendix A: Complete Field Path Test Matrix

### Campaign Fields (22 paths)
- [x] campaign.id (scalar)
- [x] campaign.name (scalar)
- [x] campaign.status (enum)
- [x] campaign.advertising_channel_type (enum)
- [x] campaign.bidding_strategy_type (custom mapping)
- [x] campaign.network_settings.target_search_network (nested scalar)
- [ ] ... (add all campaign paths)

### AdGroup Fields (14 paths)
- [x] ad_group.id (scalar)
- [x] ad_group.name (scalar)
- [x] ad_group.status (enum)
- [x] ad_group.type (enum with keyword escaping)
- [ ] ... (add all ad_group paths)

### Metrics Fields (100+ paths)
- [x] metrics.impressions (scalar)
- [x] metrics.clicks (scalar)
- [x] metrics.ctr (scalar)
- [ ] ... (add all metrics paths)

### Segments Fields (20+ paths)
- [x] segments.date (scalar)
- [x] segments.device (enum)
- [x] segments.ad_network_type (enum)
- [ ] ... (add all segments paths)

### Total: 200+ field paths to test

## Appendix B: Sample Test Code

See test implementation examples inline throughout this document.
