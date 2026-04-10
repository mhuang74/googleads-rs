# Code Review Report: support_all_fields Via prost-reflect

**Review Date:** 2026-04-10  
**Spec Reference:** specs/support_all_fields_via_prost_reflect.md  
**Files Reviewed:**
- src/lib.rs (prost-reflect implementation)
- build.rs (descriptor set generation)
- Cargo.toml (dependencies)
- utils/update.sh (proto handling)
- tests/*.rs (18 test files)

---

## Executive Summary

The implementation successfully replaces 538 hand-maintained match arms with a ~222-line prost-reflect-based solution. The core reflection logic is well-designed, but there are **significant test inconsistencies** and **outdated documentation** that must be addressed before this code is production-ready.

---

## Issues Found

### Issue 1: CRITICAL - Enum Formatting Inconsistency Across Tests

**Status:** Will cause test failures  
**Files:** tests/google_ads_row_primary_status_tests.rs, tests/integration_streaming_tests.rs, tests/mock_integration_tests.rs

The spec correctly identified that enum formatting would change from PascalCase (`Enabled`) to SCREAMING_SNAKE_CASE (`ENABLED`). However, the test files have **inconsistent expectations**:

**File: tests/google_ads_row_enum_tests.rs (CORRECT)**
```rust
assert_eq!(row.get("campaign.status"), "ENABLED");   // ✓ Matches prost-reflect output
assert_eq!(row.get("campaign.status"), "PAUSED");    // ✓ Matches prost-reflect output
```

**File: tests/google_ads_row_primary_status_tests.rs (INCORRECT)**
```rust
assert_eq!(result, "Eligible");   // ✗ Will fail - prost-reflect returns "ELIGIBLE"
assert_eq!(result, "Paused");    // ✗ Will fail - prost-reflect returns "PAUSED"
assert_eq!(result, "Removed");   // ✗ Will fail - prost-reflect returns "REMOVED"
```

**File: tests/integration_streaming_tests.rs (INCORRECT)**
```rust
assert_eq!(row.get("campaign.status"), "Enabled");   // ✗ Will fail - prost-reflect returns "ENABLED"
let expected_status = if i % 2 == 0 { "Paused" } else { "Enabled" };  // ✗ Wrong case
```

**File: tests/mock_integration_tests.rs (INCORRECT)**
```rust
assert_eq!(output[2].1, "Enabled");  // ✗ Same issue
```

**Recommendation:** Update all test expectations to use SCREAMING_SNAKE_CASE to match the proto canonical names returned by prost-reflect.

---

### Issue 2: HIGH - Outdated Test Documentation Comments

**Status:** Documentation/technical debt  
**Files:** Multiple test files

Test files contain comments referencing macros that no longer exist:

**File: tests/google_ads_row_enum_tests.rs:1-5**
```rust
// Unit tests for GoogleAdsRow::get() method - Enum Fields
//
// This module tests the method_str! and optional_method_str! macros  // <-- DELETED CODE
// which are used for extracting enum field values via accessor methods  // <-- OUTDATED
```

**File: tests/google_ads_row_scalar_tests.rs:1-5**
```rust
// This module tests the attr_str! and optional_attr_str! macros  // <-- DELETED CODE
```

**File: tests/google_ads_row_oneof_tests.rs:1-5**
```rust
// This module tests the enum_match_str! and optional_enum_match_str! macros  // <-- DELETED CODE
```

**File: tests/google_ads_row_repeated_tests.rs:1-5**
```rust
// This module tests the enum_match_iterator_str! macro  // <-- DELETED CODE
```

**Recommendation:** Update all test file headers to reflect the prost-reflect implementation.

---

### Issue 3: MEDIUM - Primary Status Test Implementation Bypasses Enum Type

**Status:** Code smell / brittle test  
**File:** tests/google_ads_row_primary_status_tests.rs

These tests set raw i32 values instead of using enum variants:

```rust
let campaign = Campaign {
    primary_status: 2,  // Magic number instead of enum variant
    ..Default::default()
};
```

While this technically works (prost-reflect will resolve `2` to "ELIGIBLE"), it makes the tests harder to understand and maintain.

**Recommendation:** Import the enum and use variant names:
```rust
use googleads_rs::google::ads::googleads::v23::enums::campaign_primary_status_enum::CampaignPrimaryStatus;

let campaign = Campaign {
    primary_status: CampaignPrimaryStatus::Eligible as i32,
    ..Default::default()
};
```

---

### Issue 4: MEDIUM - Primary Status Reasons Tests Expect PascalCase for Enums

**Status:** Will fail with prost-reflect  
**File:** tests/google_ads_row_primary_status_tests.rs:104-146

```rust
assert_eq!(result, "CampaignPaused");      // ✗ Will get "CAMPAIGN_PAUSED"
assert_eq!(result, "CampaignRemoved");     // ✗ Will get "CAMPAIGN_REMOVED"
assert_eq!(result, "Unspecified");         // ✗ Will get "UNSPECIFIED"
```

These are repeated enum fields that get joined with ", ". The current tests expect PascalCase but prost-reflect returns SCREAMING_SNAKE_CASE.

---

### Issue 5: LOW - Oneof Match Type Test Expects Numeric String

**Status:** Design decision / potentially incorrect  
**File:** tests/google_ads_row_oneof_tests.rs:31-44

```rust
// Match type is returned as numeric string
assert_eq!(row.get("ad_group_criterion.keyword.match_type"), "3");
```

With prost-reflect, this will now return "PHRASE" (the enum variant name) instead of "3". This is actually the **correct behavior** per the spec, but the test expects the old numeric string.

**Recommendation:** Update test to expect "PHRASE", "EXACT", "BROAD" instead of "2", "3", "4".

---

### Issue 6: LOW - Missing Error Handling for Descriptor Pool Initialization

**Status:** Code quality  
**File:** src/lib.rs:40-45

```rust
static DESCRIPTOR_POOL: LazyLock<prost_reflect::DescriptorPool> = LazyLock::new(|| {
    prost_reflect::DescriptorPool::decode(FILE_DESCRIPTOR_SET_BYTES).expect(
        "Failed to decode file descriptor set. This should not happen if the build succeeded.",
    )
});
```

While the `.expect()` is justified (this is a build-time invariant), there's no validation that the file descriptor set actually contains the expected message types. If the proto compilation changes or the descriptor path is wrong, runtime errors will occur.

**Recommendation:** Consider adding a debug assertion or validation that `google.ads.googleads.v23.services.GoogleAdsRow` can be found in the descriptor pool.

---

### Issue 7: LOW - change_event.changed_fields Formatting Not Customized

**Status:** Spec deviation  
**File:** src/lib.rs

The spec mentions (line 93-94):
> Handling `change_event.changed_fields`: Currently formats `FieldMask.paths` with single quotes. With reflection, `changed_fields` is a message with a `paths` repeated string field. The walker would return the debug format of the message.

The current implementation does not have special handling for `change_event.changed_fields`. It will return the debug format of the FieldMask message rather than the formatted paths.

**Recommendation:** Either accept this behavioral change or add a special case in `format_value` for FieldMask messages.

---

### Issue 8: LOW - Unused Import in src/lib.rs

**Status:** Code cleanup  
**File:** src/lib.rs:33

```rust
use prost_reflect::{DynamicMessage, ReflectMessage, Value};
```

`ReflectMessage` is imported but never used. Only `DynamicMessage::decode`, `Message::encode`, and `Value` are used.

**Recommendation:** Remove unused import:
```rust
use prost_reflect::{DynamicMessage, Value};
```

---

### Issue 9: LOW - get_many Returns Error String for All Fields on Encode Failure

**Status:** Design choice  
**File:** src/lib.rs:91-112

```rust
pub fn get_many(&self, field_names: &[&str]) -> Vec<String> {
    let mut buf = Vec::new();
    if self.encode(&mut buf).is_err() {
        return field_names.iter().map(|_| "not implemented by googleads-rs".to_string()).collect();
    }
    // ...
}
```

If encoding fails, all fields get the same "not implemented by googleads-rs" error. This conflates "encoding failed" with "field not implemented".

**Recommendation:** Consider different error messages for encode failure vs unknown field:
```rust
if self.encode(&mut buf).is_err() {
    return field_names.iter().map(|_| "error: failed to encode row".to_string()).collect();
}
```

---

### Issue 10: LOW - Test Helpers Comment References Old Resource Count

**Status:** Documentation  
**File:** tests/test_helpers/mod.rs:27-71

The GoogleAdsRowBuilder has 47+ Option<T> fields (expanded from the spec's original estimate of 35). The comment at the top should reflect this.

---

## Positive Findings

### 1. Excellent Build Configuration

**File:** build.rs

The build.rs correctly:
- Generates the file descriptor set before tonic compilation (lines 63-71)
- Uses `--experimental_allow_proto3_optional` flag for proto3 optional support
- Compiles in chunks to avoid command-line length limits
- Properly maps Rust keywords in module names

### 2. Clean Implementation of get_many

**File:** src/lib.rs:91-112

The `get_many` method efficiently encodes once and walks multiple paths, exactly as specified. This is an excellent performance optimization over calling `get()` multiple times.

### 3. Correct Enum Resolution

**File:** src/lib.rs:163-178

The enum resolution logic correctly uses the field descriptor to look up variant names:

```rust
Value::EnumNumber(num) => {
    if let Some(field) = field_desc {
        if let prost_reflect::Kind::Enum(enum_desc) = field.kind() {
            for value in enum_desc.values() {
                if value.number() == *num {
                    return value.name().to_string();  // Returns "ENABLED"
                }
            }
        }
    }
    num.to_string()
}
```

### 4. Proper Handling of Repeated Fields

**File:** src/lib.rs:183-212

The `format_value` function correctly differentiates between scalar lists (joined with ", ") and message lists (joined with "; ").

### 5. Oneof Fields Work Transparently

As noted in the spec (line 91-92), prost-reflect handles oneof fields naturally - no special code is needed. The test files confirm this works correctly.

### 6. utils/update.sh Correctly Preserves `optional` Keywords

The sed command that stripped `optional` keywords has been removed (the spec's Step 1), allowing proto3 optional fields to work correctly with prost-reflect.

---

## Summary Table

| Issue | Severity | Status | Effort to Fix |
|-------|----------|--------|---------------|
| Enum formatting inconsistency (primary_status tests) | CRITICAL | Must fix before merge | Medium |
| Outdated test documentation comments | HIGH | Should fix | Low |
| Primary status uses magic numbers | MEDIUM | Should fix | Low |
| Primary status reasons expects PascalCase | MEDIUM | Must fix before merge | Low |
| Oneof match_type expects numeric string | LOW | Must fix before merge | Low |
| Missing descriptor validation | LOW | Nice to have | Low |
| change_event.changed_fields formatting | LOW | Design decision | N/A |
| Unused ReflectMessage import | LOW | Code cleanup | Trivial |
| get_many error message conflation | LOW | Design decision | N/A |
| Outdated resource count comment | LOW | Documentation | Trivial |

---

## Recommendations

1. **Before merging:** Update all enum value assertions in tests to use SCREAMING_SNAKE_CASE
2. **Before merging:** Update all test file headers to remove references to deleted macros
3. **Before release:** Consider running a broader integration test to verify field coverage
4. **Future enhancement:** Consider caching DynamicMessage in get_many for even better performance
5. **Future enhancement:** Add benchmark comparing old match-arm approach vs prost-reflect

---

## Appendix: Test Files Requiring Updates

| File | Lines to Review | Reason |
|------|-----------------|--------|
| google_ads_row_primary_status_tests.rs | All enum assertions | PascalCase → SCREAMING_SNAKE_CASE |
| google_ads_row_primary_status_details_tests.rs | Enum assertions | Same issue |
| integration_streaming_tests.rs | Lines 59, 231, 279, etc. | "Enabled" → "ENABLED" |
| mock_integration_tests.rs | Lines 97, 252, etc. | "Enabled" → "ENABLED" |
| google_ads_row_oneof_tests.rs | Lines 43, 57 | "3" → "PHRASE", etc. |
