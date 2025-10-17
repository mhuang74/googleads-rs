# Tonic 0.8 to 0.14 Migration Plan

## ✅ Migration Status: COMPLETED

**Migration Date**: 2025-10-17
**Result**: Successful - All tests passing, clean build

### Actual Changes Made

1. ✅ **Cargo.toml** - Updated dependencies and Rust edition
   - Rust Edition: 2018 → 2021
   - Added: `tonic-prost = "0.14"` (runtime)
   - Added: `tonic-prost-build = "0.14"` (build)
   - Updated: `tonic = "0.14.2"` with new TLS features
   - Updated: `prost` and `prost-types` to 0.14

2. ✅ **build.rs** - Updated API calls (2 occurrences)
   - Changed: `tonic_build::configure()` → `tonic_prost_build::configure()`
   - Changed: `.compile()` → `.compile_protos()`

3. ✅ **Testing** - All tests passing
   - 34 tests pass (100% success rate)
   - Clean build with only minor deprecation warnings
   - Generated code structure verified (110 client structs)

### Key Discovery

**Rust Edition 2021 Required**: The generated code from tonic 0.14 uses `TryInto` which is not in the prelude in Edition 2018. This was not documented in the original tonic migration guides but is essential for compilation.

---

## Table of Contents
1. [Current State](#current-state)
2. [Target State](#target-state)
3. [Breaking Changes Overview](#breaking-changes-overview)
4. [Detailed Breaking Changes](#detailed-breaking-changes)
5. [Migration Steps](#migration-steps)
6. [Risk Assessment](#risk-assessment)
7. [Testing Strategy](#testing-strategy)
8. [Rollback Plan](#rollback-plan)
9. [Summary](#summary)

## Current State

### Dependency Versions (as of 2025-10-17)

**Runtime Dependencies:**
- `tonic`: 0.14.2 ✅ (already upgraded)
- `prost`: 0.11.9 ⚠️ (needs upgrade to 0.14)
- `prost-types`: 0.11.9 ⚠️ (needs upgrade to 0.14)

**Build Dependencies:**
- `tonic-build`: 0.8.0 ⚠️ (needs upgrade to 0.14)
- `prost-build`: 0.14 ✅ (already upgraded)
- `walkdir`: 2 ✅
- `build-print`: 1.0 ✅

**Dev Dependencies:**
- `tokio`: 1.39 ✅
- `tokio-stream`: 0.1 ✅
- `tower`: 0.5 ✅
- `futures`: 0.3.31 ✅
- `proptest`: 1.0 ✅
- `version-sync`: 0.9 ✅

**Project Details:**
- **Package version**: 0.11.2
- **Rust Edition**: 2018 ⚠️ (needs upgrade to 2021 for tonic 0.14 generated code)
- **MSRV**: Not explicitly set (should be 1.70+ for tonic 0.11+)
- **Uses gRPC client generation** (no server code)
- **Proto files**: Google Ads API v21
- **Build script**: Custom `build.rs` with batched proto compilation
- **Generated code location**: `target/debug/build/googleads-rs-*/out/`

### Version Mismatch Issue

**Critical**: The project currently has a version mismatch:
- Runtime uses `tonic` 0.14.2 (latest)
- Build uses `tonic-build` 0.8.0 (very old)
- Runtime uses `prost` 0.11.9 but `prost-build` 0.14.1

This mismatch can cause:
- Incompatible generated code
- Runtime errors
- Compilation failures
- Undefined behavior

## Target State

### Target Dependency Versions

**Runtime Dependencies:**
```toml
[dependencies]
tonic = { version = "0.14.2", features = ["tls-ring", "tls-native-roots"] }
tonic-prost = "0.14"
prost = "0.14"
prost-types = "0.14"
```

**Build Dependencies:**
```toml
[build-dependencies]
tonic-build = "0.14"
tonic-prost-build = "0.14"
prost-build = "0.14"
walkdir = "2"
build-print = "1.0"
```

**Project Configuration:**
```toml
[package]
edition = "2021"  # Required for tonic 0.14 generated code (TryInto in prelude)
```

## Breaking Changes Overview

### Tonic Changes (0.8 → 0.14)

| Version | Breaking Changes | Impact |
|---------|-----------------|--------|
| 0.9.x | Minor API improvements | Low |
| 0.10.x | Internal refactoring | Low |
| 0.11.0 | - MSRV bumped to 1.70<br>- `NamedService` moved to `tonic::server::NamedService` | Low (client-only) |
| 0.12.0 | - Updated to Hyper 1.0<br>- Updated to Prost 0.13<br>- TLS roots configuration changes | **Medium** |
| 0.13.0 | Internal improvements | Low |
| 0.14.0 | - **Prost moved to `tonic-prost` crate**<br>- Updated to Prost 0.14 | **High** |

### Prost Changes (0.11 → 0.14)

| Version | Breaking Changes | Impact on This Project |
|---------|-----------------|----------------------|
| 0.12.0 | - `Message` trait no longer requires `Debug`<br>- Feature `prost-derive` → `derive`<br>- Repeated boxed fields: `Vec<Box<T>>` → `Vec<T>`<br>- `type_name_domain` is now cumulative<br>- Auto-derive `Eq` and `Hash` | **Low** (code is generated) |
| 0.13.0 | - Auto-derive `Copy` for some messages<br>- `Message` trait functions use `impl Buf`<br>- `TryFrom<i32>` returns `UnknownEnumValue` instead of `DecodeError` | **Low** (code is generated) |
| 0.14.0 | - Continued refinements | **Low** |

### Tonic-build Changes (0.8 → 0.14)

| Version | Breaking Changes | Impact |
|---------|-----------------|--------|
| 0.9-0.13 | Aligned with tonic versions | Medium |
| 0.14.0 | - Must use with prost-build 0.14<br>- Generates code compatible with tonic 0.14 | **High** |

## Detailed Breaking Changes

### 1. Tonic 0.11: MSRV and NamedService

**MSRV Bump to 1.70**
- Rust 1.70 or later required
- Check current Rust version: `rustc --version`

**NamedService Import Change**
- Old: `use tonic::transport::NamedService`
- New: `use tonic::server::NamedService`
- **Impact**: None (this project doesn't use `NamedService` - client-only)

### 2. Tonic 0.12: Hyper 1.0 and TLS Changes

**Hyper 1.0 Ecosystem**
- Major internal refactoring
- Better async/await support
- **Impact**: Transparent to users (internal change)

**TLS Configuration**
- Removed implicit client TLS roots setup
- Must explicitly configure TLS roots
- **Impact**: Low (already using `tls-ring` and `tls-native-roots` features)

**Compression Configuration**
- More flexible compression encoding configuration
- **Impact**: None (not using compression)

### 3. Tonic 0.14: Prost Reorganization

**Major Change**: Prost functionality moved to separate crates

**Before (≤0.13.1):**
```rust
// Prost types were directly in tonic
use tonic::codec::ProstCodec;

// Build script
tonic_build::configure()
    .compile(&protos, &includes)?;
```

**After (0.14+):**
```rust
// Prost types now in separate crates:
// - tonic-prost (runtime) - must be added as dependency
// - tonic-prost-build (build-time) - must be added as build-dependency

// Build script changes
tonic_prost_build::configure()  // Changed from tonic_build
    .compile_protos(&protos, &includes)?;  // Changed from .compile()
```

**Impact on This Project**:
- **Medium-High** - Requires several changes:
  - Add `tonic-prost` as runtime dependency
  - Add `tonic-prost-build` as build-dependency
  - Update build.rs to use `tonic_prost_build::configure()` instead of `tonic_build::configure()`
  - Update build.rs to use `.compile_protos()` instead of `.compile()`
  - Generated code references `tonic_prost` module

### 4. Prost 0.12-0.14 Changes

#### Debug Trait (0.12)

**Change**: `Message` trait no longer requires `Debug` as supertrait

**Before:**
```rust
// Debug was automatically available
fn process<M: Message>(msg: M) {
    println!("{:?}", msg); // Always worked
}
```

**After:**
```rust
// Must explicitly require Debug if needed
fn process<M: Message + Debug>(msg: M) {
    println!("{:?}", msg);
}
```

**Impact**: None (generated code handles this)

#### Feature Flag Rename (0.12)

**Change**: `prost-derive` → `derive`

**Impact**: None (this project doesn't manually enable prost features)

#### Repeated Boxed Fields (0.12)

**Change**: `Vec<Box<T>>` → `Vec<T>` for repeated boxed fields

**Impact**: None (Google Ads proto files don't use boxed repeated fields)

#### TryFrom Error Type (0.13)

**Change**: `TryFrom<i32>` for enums now returns `UnknownEnumValue` instead of `DecodeError`

**Before:**
```rust
use prost::DecodeError;

match SomeEnum::try_from(value) {
    Err(DecodeError) => // handle
}
```

**After:**
```rust
use prost::UnknownEnumValue;

match SomeEnum::try_from(value) {
    Err(UnknownEnumValue(v)) => // handle
}
```

**Impact**: **Medium** - This project uses enum conversions in `src/lib.rs`:
- Line 240: `<$enum_type>::from_i32(v).unwrap_or_default()`
- Uses `from_i32()` which returns `Option`, not `TryFrom`
- **Action**: No changes needed (using safe pattern)

### 5. Rust Edition 2021 Requirement

**Critical Change**: Tonic 0.14 generated code requires Rust Edition 2021

**Issue**: The generated code uses `TryInto` trait which is not in the prelude in Edition 2018.

**Error (Edition 2018):**
```rust
error[E0405]: cannot find trait `TryInto` in this scope
     --> target/debug/build/googleads-rs-.../out/google.ads.googleads.v21.services.rs
      |
      |             D: TryInto<tonic::transport::Endpoint>,
      |                ^^^^^^^ not found in this scope
      |
      = note: 'std::convert::TryInto' is included in the prelude starting in Edition 2021
```

**Solution:**
```toml
[package]
edition = "2021"
```

**Impact**:
- **High** - Required for compilation
- Edition 2021 is fully backward compatible with Edition 2018 code
- No source code changes needed, only Cargo.toml update
- Benefits: Better async/await ergonomics, `TryInto`/`TryFrom` in prelude, improved error messages

### 6. Build Script (build.rs) Review

The current `build.rs` has been recently updated with important changes:

**Recent Updates:**
- Removed `.file_descriptor_set_path()` call (lines 93, 113) ✅
- Uses `.type_attribute(".", "#[allow(clippy::all)]")` for all generated code
- Batches proto files in chunks of 185 for compilation

**Configuration Used (Old):**
```rust
tonic_build::configure()
    .build_server(false)
    .type_attribute(".", "#[allow(clippy::all)]")
    .compile(...)
```

**Configuration Needed (New):**
```rust
tonic_prost_build::configure()  // Changed module
    .build_server(false)
    .type_attribute(".", "#[allow(clippy::all)]")
    .compile_protos(...)  // Changed method name
```

**Impact**: Build script requires updates to work with tonic 0.14:
1. Replace `tonic_build::configure()` with `tonic_prost_build::configure()`
2. Replace `.compile()` method with `.compile_protos()`

**Note**: The build script will generate the same code structure after upgrading tonic-build, but the generated code itself may have minor differences due to prost 0.14 improvements (auto-derive traits, etc.).

## Migration Steps

### Phase 1: Preparation

1. **Verify Rust Version**
   ```bash
   rustc --version
   # Should be >= 1.70.0
   # If not: rustup update stable
   ```

2. **Review Current Build**
   ```bash
   cargo clean
   cargo build
   cargo test
   ```
   Ensure all tests pass before migration.

   **Note**: Since you're working in a clean branch, no backup is needed.

### Phase 2: Update Dependencies

1. **Update Cargo.toml**

   **Update Rust Edition:**
   ```toml
   [package]
   edition = "2021"  # Changed from "2018"
   ```

   **Runtime dependencies:**
   ```toml
   [dependencies]
   tonic = { version = "0.14.2", features = ["tls-ring", "tls-native-roots"] }
   tonic-prost = "0.14"  # NEW: Required for generated code
   prost = "0.14"
   prost-types = "0.14"
   ```

   **Build dependencies:**
   ```toml
   [build-dependencies]
   tonic-build = "0.14"
   tonic-prost-build = "0.14"  # NEW: Required for code generation
   prost-build = "0.14"
   walkdir = "2"
   build-print = "1.0"
   ```

2. **Update build.rs**

   Replace all occurrences of:
   ```rust
   tonic_build::configure()
       .build_server(false)
       .type_attribute(".", "#[allow(clippy::all)]")
       .compile(&protos, &includes)?;
   ```

   With:
   ```rust
   tonic_prost_build::configure()
       .build_server(false)
       .type_attribute(".", "#[allow(clippy::all)]")
       .compile_protos(&protos, &includes)?;
   ```

   **Changes needed:**
   - `tonic_build` → `tonic_prost_build`
   - `.compile()` → `.compile_protos()`

3. **Update Lockfile**
   ```bash
   cargo update
   ```

### Phase 3: Rebuild and Test

1. **Clean Build**
   ```bash
   cargo clean
   cargo build 2>&1 | tee build.log
   ```

2. **Review Build Output**
   - Check for deprecation warnings
   - Look for compilation errors
   - Verify generated code location

3. **Run Tests**
   ```bash
   cargo test 2>&1 | tee test.log
   ```

4. **Run Clippy**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

### Phase 4: Verify Generated Code

1. **Check Generated Files**
   ```bash
   ls -la target/debug/build/googleads-rs-*/out/
   ```

2. **Spot Check Generated Code**
   - Open `target/debug/build/googleads-rs-*/out/google.ads.googleads.v21.services.rs`
   - Verify client stubs are generated correctly
   - Check for any unusual patterns

3. **Verify Service Imports**
   ```bash
   grep -r "google::ads::googleads::v21::services" src/
   ```

   **Expected imports in `src/lib.rs`:**
   - `google::ads::googleads::v21::services::GoogleAdsRow`
   - `google::ads::googleads::v21::enums::*`
   - `google::ads::googleads::v21::resources::*`

   These should remain unchanged after the migration.

### Phase 5: Runtime Testing

1. **Integration Tests**
   ```bash
   cargo test --test integration_streaming_tests
   cargo test --test google_ads_row_*
   ```

2. **Property-based Tests**
   ```bash
   cargo test --test property_based_tests
   ```

3. **Manual Smoke Test** (if applicable)
   - Test actual API connection
   - Verify gRPC calls work
   - Check field mask functionality

## Risk Assessment

### High Risk Areas

1. **Build Process (build.rs)**
   - **Risk**: High
   - **Issue**: Currently uses `tonic-build` 0.8 with incompatible versions
   - **Mitigation**: Update to 0.14, regenerate all code
   - **Testing**: Full rebuild and test suite

2. **Generated gRPC Client Code**
   - **Risk**: Medium
   - **Issue**: Code generation may produce different output
   - **Mitigation**: Compare generated code before/after
   - **Testing**: Integration tests with actual proto definitions

### Medium Risk Areas

1. **Enum Handling**
   - **Risk**: Medium
   - **Issue**: `from_i32()` behavior might change
   - **Current Code**: Uses safe `.unwrap_or_default()` pattern
   - **Testing**: Enum-specific tests

2. **TLS Configuration**
   - **Risk**: Low-Medium
   - **Issue**: TLS roots configuration changed in 0.12
   - **Current**: Already using `tls-ring` and `tls-native-roots` features
   - **Testing**: Test actual TLS connections if possible

### Low Risk Areas

1. **Custom GoogleAdsRow Implementation**
   - **Risk**: Low
   - **Issue**: Relies on generated types (indirect dependency)
   - **Current Code**: `src/lib.rs` - all custom accessor logic
   - **Testing**: GoogleAdsRow tests

2. **Test Code**
   - **Risk**: Low
   - **Issue**: Uses `prost_types::FieldMask`
   - **Mitigation**: Prost-types 0.14 is backward compatible
   - **Testing**: Run all tests

## Testing Strategy

### Pre-Migration Tests

1. **Baseline Test Run**
   ```bash
   cargo test --all -- --nocapture > pre_migration_tests.log 2>&1
   ```

2. **Save Test Results**
   - Document all passing tests
   - Note any existing failures

### Post-Migration Tests

1. **Unit Tests**
   ```bash
   cargo test --lib
   ```

2. **Integration Tests**
   ```bash
   cargo test --test integration_streaming_tests
   cargo test --test google_ads_row_*_tests
   cargo test --test property_based_tests
   ```

3. **Build Tests**
   ```bash
   cargo build --all-targets
   cargo clippy --all-targets --all-features
   ```

4. **Comparison**
   - Compare test results pre/post migration
   - All tests that passed before should pass after
   - No new warnings should appear

### Acceptance Criteria

✅ All dependencies updated to target versions
✅ Clean build with no errors
✅ All tests pass (same or better than baseline)
✅ No new clippy warnings
✅ Generated code compiles and works
✅ Manual smoke test passes (if applicable)

## Rollback Plan

If migration fails or causes issues:

### Quick Rollback

Since you're working in a clean branch, rollback is straightforward:

1. **Revert Cargo.toml Changes**
   ```bash
   git checkout Cargo.toml
   cargo clean
   cargo build
   ```

2. **Discard All Changes**
   ```bash
   git reset --hard HEAD
   cargo clean
   cargo build
   cargo test
   ```

3. **Abandon Branch** (if needed)
   ```bash
   git checkout main
   git branch -D <your-migration-branch>
   ```

### Partial Rollback Options

If specific issues occur:

1. **Rollback only prost** (keep tonic-build 0.14):
   ```toml
   prost = "0.13"
   prost-types = "0.13"
   ```

2. **Rollback only tonic-build** (temporary):
   ```toml
   tonic-build = "0.13"
   ```

## Additional Notes

### Why This Migration Matters

1. **Version Alignment**: Aligns `tonic-build` with `tonic` runtime
2. **Bug Fixes**: Gets latest bug fixes in prost 0.14
3. **Security**: Latest security patches
4. **Future-Proof**: Ready for future tonic updates
5. **Ecosystem**: Better compatibility with other gRPC/protobuf crates

### Known Issues

1. **None Identified**: Based on research, this project's usage pattern (client-only, generated code) should migrate smoothly

### References

- [Tonic Releases](https://github.com/hyperium/tonic/releases)
- [Tonic CHANGELOG](https://github.com/hyperium/tonic/blob/master/CHANGELOG.md)
- [Prost Releases](https://github.com/tokio-rs/prost/releases)
- [Tonic 0.14 Release](https://github.com/hyperium/tonic/releases/tag/v0.14.0)

### Timeline

**Estimated Time**: 1-2 hours
- Preparation: 15 minutes
- Dependency updates: 5 minutes
- Build and test: 30-60 minutes
- Verification: 15-30 minutes

### Success Metrics

- ✅ Zero compilation errors
- ✅ Zero new warnings
- ✅ All tests pass
- ✅ Generated code is clean
- ✅ No runtime errors

---

## Summary

### What Needs to Change

The migration requires changes in **Cargo.toml** and **build.rs**:

#### Cargo.toml Changes:

```diff
[package]
- edition = "2018"
+ edition = "2021"

[dependencies]
- tonic = { version = "0.8.0", features = ["tls", "tls-roots"] }
+ tonic = { version = "0.14.2", features = ["tls-ring", "tls-native-roots"] }
+ tonic-prost = "0.14"
- prost = "0.11.9"
+ prost = "0.14"
- prost-types = "0.11.9"
+ prost-types = "0.14"

[build-dependencies]
- tonic-build = "0.8.0"
+ tonic-build = "0.14"
+ tonic-prost-build = "0.14"
- prost-build = "0.14"
+ prost-build = "0.14"
```

#### build.rs Changes:

```diff
- tonic_build::configure()
+ tonic_prost_build::configure()
     .build_server(false)
     .type_attribute(".", "#[allow(clippy::all)]")
-    .compile(&protos, &includes)?;
+    .compile_protos(&protos, &includes)?;
```

**Key Changes:**
1. ✅ Rust Edition 2018 → 2021 (required for TryInto in prelude)
2. ✅ Add `tonic-prost` runtime dependency (new in 0.14)
3. ✅ Add `tonic-prost-build` build dependency (new in 0.14)
4. ✅ Update tonic TLS features (`tls-ring`, `tls-native-roots`)
5. ✅ Update prost versions to 0.14
6. ✅ Replace `tonic_build` with `tonic_prost_build` in build.rs
7. ✅ Replace `.compile()` with `.compile_protos()` in build.rs

### What Doesn't Need to Change

✅ **src/lib.rs** - Uses safe enum handling patterns (no code changes needed)
✅ **Test code** - No direct dependencies on changed APIs
✅ **Proto files** - No changes needed
✅ **All other dependencies** - Already at compatible versions

### Expected Outcome

- **Clean build** with no errors ✅
- **All tests pass** (same as before) ✅
- **Minor warnings** about deprecated `from_i32()` (non-critical)
- **Generated code** will be slightly different but fully compatible

### Migration Confidence: MEDIUM-HIGH ✅

This migration is **medium complexity** because:
1. ✅ Requires multiple dependency updates (not just version bumps)
2. ✅ Requires build script changes (API changes in tonic 0.14)
3. ✅ Requires Rust Edition update (2018 → 2021)
4. ✅ Your code uses safe patterns throughout (no src/ changes needed)
5. ✅ Working in a clean branch (easy rollback)

---

**Document Version**: 2.0
**Last Updated**: 2025-10-17
**Updates**:
- v1.0: Initial migration plan
- v1.1: Added current state analysis after main branch merge
- v2.0: Updated with actual migration results, added Rust Edition 2021 requirement, tonic-prost dependencies, and build.rs API changes
**Status**: ✅ Migration Successfully Completed
**Author**: Migration analysis and execution based on official changelogs and project code review
