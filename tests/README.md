# googleads-rs Test Suite

This directory contains the test suite for the googleads-rs library.

## Test Structure

```
tests/
├── README.md                          # This file
├── test_helpers/                      # Test helper modules
│   └── mod.rs                        # Builder patterns for test data
├── google_ads_row_scalar_tests.rs    # Tests for scalar field extraction
├── google_ads_row_enum_tests.rs      # Tests for enum field extraction
└── version-numbers.rs                # Version consistency tests
```

## Running Tests

### Run all tests
```bash
cargo test
```

### Run specific test suite
```bash
cargo test --test google_ads_row_scalar_tests
cargo test --test google_ads_row_enum_tests
```

### Run a specific test
```bash
cargo test test_campaign_id
```

### Run tests with output
```bash
cargo test -- --show-output
```

### Run tests with coverage
```bash
cargo install cargo-llvm-cov
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

## Test Helpers

The `test_helpers` module provides builder patterns for constructing test data.

### GoogleAdsRowBuilder

Build complete GoogleAdsRow instances with selected resources:

```rust
use test_helpers::{GoogleAdsRowBuilder, CampaignBuilder, MetricsBuilder};

let row = GoogleAdsRowBuilder::new()
    .with_campaign(
        CampaignBuilder::new()
            .id(123456)
            .name("Test Campaign")
            .build()
    )
    .with_metrics(
        MetricsBuilder::new()
            .impressions(10000)
            .clicks(500)
            .build()
    )
    .build();

assert_eq!(row.get("campaign.id"), "123456");
assert_eq!(row.get("metrics.impressions"), "10000");
```

### Available Builders

- **GoogleAdsRowBuilder** - Main row builder
- **CampaignBuilder** - Campaign resource
- **AdGroupBuilder** - AdGroup resource
- **CampaignBudgetBuilder** - CampaignBudget resource
- **CustomerBuilder** - Customer resource
- **MetricsBuilder** - Metrics data
- **SegmentsBuilder** - Segments data

### Builder Examples

#### Campaign with Status and Type
```rust
use googleads_rs::google::ads::googleads::v19::enums::{
    campaign_status_enum::CampaignStatus,
    advertising_channel_type_enum::AdvertisingChannelType,
};

let campaign = CampaignBuilder::new()
    .id(123456)
    .name("Search Campaign")
    .status(CampaignStatus::Enabled)
    .advertising_channel_type(AdvertisingChannelType::Search)
    .build();
```

#### AdGroup with Bids
```rust
let ad_group = AdGroupBuilder::new()
    .id(789012)
    .name("Brand Keywords")
    .cpc_bid_micros(5000000)  // $5.00
    .target_cpa_micros(25000000)  // $25.00
    .build();
```

#### Metrics with Performance Data
```rust
let metrics = MetricsBuilder::new()
    .impressions(10000)
    .clicks(500)
    .ctr(0.05)
    .cost_micros(1500000)  // $1.50
    .conversions(25.5)
    .conversions_value(1234.56)
    .build();
```

#### Segments with Dimensions
```rust
use googleads_rs::google::ads::googleads::v19::enums::device_enum::Device;

let segments = SegmentsBuilder::new()
    .date("2024-10-10")
    .device(Device::Mobile)
    .hour(14)
    .build();
```

## Test Coverage

### Current Coverage (Phase 1)

| Test Suite | Tests | Status |
|------------|-------|--------|
| google_ads_row_scalar_tests | 40 | ✅ All passing |
| google_ads_row_enum_tests | 40 | ✅ All passing |
| version-numbers | 3 | ✅ All passing |
| **Total** | **83** | **✅ 100% passing** |

### Coverage by Macro Type

| Macro | Coverage | Tests |
|-------|----------|-------|
| attr_str! | ✅ Complete | 30+ tests |
| optional_attr_str! | ✅ Complete | 5+ tests |
| method_str! | ✅ Complete | 35+ tests |
| optional_method_str! | ✅ Complete | 5+ tests |
| enum_match_str! | ⏳ Phase 2 | 0 tests |
| enum_match_iterator_str! | ⏳ Phase 2 | 0 tests |
| optional_enum_match_str! | ⏳ Phase 2 | 0 tests |

### Coverage by Resource Type

| Resource | Scalar Fields | Enum Fields | Oneof Fields | Status |
|----------|---------------|-------------|--------------|--------|
| Campaign | ✅ Complete | ✅ Complete | ⏳ Phase 2 | Partial |
| AdGroup | ✅ Complete | ✅ Complete | ⏳ Phase 2 | Partial |
| AdGroupAd | ⏳ Phase 2 | ⏳ Phase 2 | ⏳ Phase 2 | Not started |
| AdGroupCriterion | ⏳ Phase 2 | ⏳ Phase 2 | ⏳ Phase 2 | Not started |
| CampaignCriterion | ✅ Partial | ✅ Partial | ⏳ Phase 2 | Partial |
| CampaignBudget | ✅ Complete | N/A | N/A | Complete |
| Customer | ✅ Complete | ✅ Complete | N/A | Complete |
| Metrics | ✅ Complete | N/A | N/A | Complete |
| Segments | ✅ Complete | ✅ Complete | N/A | Complete |

## Writing New Tests

### Test Template

```rust
#[test]
fn test_resource_field_name() {
    // Arrange: Build test data
    let resource = ResourceBuilder::new()
        .field_name(value)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_resource(resource)
        .build();

    // Act: Call get() method
    let result = row.get("resource.field_name");

    // Assert: Verify expected output
    assert_eq!(result, "expected_value");
}
```

### Testing Edge Cases

```rust
#[test]
fn test_optional_field_absent() {
    // Test when optional parent is not present
    let row = GoogleAdsRowBuilder::new().build();
    assert_eq!(row.get("optional_resource.field"), "");
}

#[test]
fn test_empty_repeated_field() {
    // Test when repeated field is empty
    let resource = ResourceBuilder::new()
        .labels(vec![])
        .build();
    let row = GoogleAdsRowBuilder::new()
        .with_resource(resource)
        .build();
    assert_eq!(row.get("resource.labels"), "");
}
```

### Testing Enums

```rust
#[test]
fn test_enum_field() {
    use googleads_rs::google::ads::googleads::v19::enums::status_enum::Status;

    let resource = ResourceBuilder::new()
        .status(Status::Enabled)
        .build();

    let row = GoogleAdsRowBuilder::new()
        .with_resource(resource)
        .build();

    assert_eq!(row.get("resource.status"), "Enabled");
}
```

## Continuous Integration

The test suite runs automatically on every push and pull request via GitHub Actions.

### CI Jobs

1. **build** - Linux build and test
2. **test-with-coverage** - Coverage reporting with cargo-llvm-cov
3. **lint** - Clippy and rustfmt checks
4. **build-windows** - Windows build and test

### Coverage Reporting

Coverage reports are automatically uploaded to Codecov on every CI run. View coverage at:
- Badge in README.md (when configured)
- Codecov dashboard (when configured)

## Best Practices

### Do's ✅
- Use builder patterns for test data construction
- Test both positive and negative cases
- Test edge cases (empty, zero, max values)
- Use descriptive test names
- Keep tests independent and isolated
- Test one behavior per test

### Don'ts ❌
- Don't rely on test execution order
- Don't use hardcoded timestamps
- Don't skip edge case testing
- Don't create overly complex test setups
- Don't test multiple behaviors in one test

## Troubleshooting

### Tests fail to compile
```bash
# Clean and rebuild
cargo clean
cargo build --tests
```

### Proto compilation errors
```bash
# Ensure protobuf compiler is installed
# On macOS:
brew install protobuf

# On Ubuntu:
sudo apt-get install protobuf-compiler

# On Windows:
choco install protoc
```

### Slow test execution
```bash
# Run tests in parallel (default)
cargo test

# Run tests with specific thread count
cargo test -- --test-threads=4
```

## Contributing

When adding new fields to `GoogleAdsRow::get()`:

1. Add corresponding test(s) in appropriate test file
2. Use existing builders or extend them as needed
3. Test both presence and absence of optional fields
4. Test edge cases (empty, zero, boundaries)
5. Ensure all tests pass locally before pushing
6. Check coverage report to ensure new code is tested

## Resources

- [Comprehensive Test Plan](../specs/comprehensive_test_plan.md)
- [Phase 1 Implementation Summary](../specs/phase1_implementation_summary.md)
- [Implementation Guide](../specs/how_to_implement_get_matcharms.md)
- [Google Ads API Documentation](https://developers.google.com/google-ads/api/docs/start)
