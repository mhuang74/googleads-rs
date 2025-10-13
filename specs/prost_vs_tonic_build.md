# prost_build vs tonic_build: Missing gRPC Client Issue

## Problem Statement

After switching from `tonic_build` to `prost_build`, the `GoogleAdsClient` gRPC client is no longer generated. The switch was made to add `.type_attribute(".", "#[allow(clippy::all)]")` to exclude all generated Rust code from clippy checks, which was blocking build jobs.

## Root Cause Analysis

### Key Difference Between prost_build and tonic_build

1. **prost_build**: Only generates protobuf message structs (data structures)
   - Does NOT generate gRPC service clients or servers
   - Outputs: `.rs` files with message definitions

2. **tonic_build**: Generates both protobuf messages AND gRPC service clients/servers
   - Wrapper around prost_build that adds gRPC code generation
   - Outputs: `.rs` files with messages + gRPC service clients (trait implementations using `tonic::client::Grpc`)

### What Changed in build.rs

**Before (with tonic_build)**:
```rust
tonic_build::configure()
    .build_server(false)  // Only generate clients, not servers
    .compile(chunk, &[proto_path.clone()])?;
```

**After (with prost_build)**:
```rust
prost_build::Config::new()
    .file_descriptor_set_path("target/descriptors.bin")
    .type_attribute(".", "#[allow(clippy::all)]")
    .compile_protos(chunk, std::slice::from_ref(&proto_path))?;
```

### Why GoogleAdsClient is Missing

The `GoogleAdsClient` is a gRPC service client defined in the Google Ads API proto files. With `prost_build`, only the message types are generated, but the service client code (which implements the RPC methods) is NOT generated.

## Solution Options

### Option 1: Use tonic_build with type_attribute (RECOMMENDED)

`tonic_build` supports the same configuration methods as `prost_build` since it wraps it internally.

**Implementation**:
```rust
tonic_build::configure()
    .build_server(false)
    .file_descriptor_set_path("target/descriptors.bin")
    .type_attribute(".", "#[allow(clippy::all)]")
    .compile(chunk, &[proto_path.clone()])?;
```

**Pros**:
- Simplest solution - one line change per compile call
- Generates both messages and gRPC clients
- Maintains existing functionality
- Both prost_build and tonic_build dependencies already in Cargo.toml

**Cons**:
- None

### Option 2: Use prost_build for messages + tonic_build for services

Split the generation: use `prost_build` for messages (to apply clippy attribute) and `tonic_build` for services.

**Implementation**:
- Complex: requires identifying which proto files define services vs messages
- May cause conflicts if both try to generate the same message types
- Requires careful configuration to avoid duplicate type definitions

**Pros**:
- Theoretical separation of concerns

**Cons**:
- Overly complex
- Risk of type conflicts
- No real benefit over Option 1

### Option 3: Use tonic-build crate with build_client() method

Explicitly enable client generation using tonic_build's fluent API.

**Implementation**:
```rust
tonic_build::configure()
    .build_client(true)   // Explicitly enable client generation
    .build_server(false)  // Disable server generation
    .type_attribute(".", "#[allow(clippy::all)]")
    .compile(&protos, &[proto_path])?;
```

**Pros**:
- Makes intent explicit
- Same as Option 1 with explicit flag

**Cons**:
- `build_client(true)` is already the default, so it's redundant

## Recommended Plan

**Choose Option 1**: Switch back to `tonic_build::configure()` and add the `.type_attribute()` call.

### Implementation Steps

1. Replace all `prost_build::Config::new()` calls with `tonic_build::configure()`
2. Keep `.type_attribute(".", "#[allow(clippy::all)]")` to exclude clippy
3. Keep `.build_server(false)` to avoid generating server code
4. Update the file_descriptor_set_path if needed
5. Test build to ensure:
   - gRPC clients are generated
   - Clippy doesn't block the build
   - All existing functionality works

### Code Changes Required

**File**: `build.rs` (lines 88-95, 104-114)

**From**:
```rust
prost_build::Config::new()
    .file_descriptor_set_path("target/descriptors.bin")
    .type_attribute(".", "#[allow(clippy::all)]")
    .compile_protos(&misc_protos, std::slice::from_ref(&proto_path))?;
```

**To**:
```rust
tonic_build::configure()
    .build_server(false)
    .file_descriptor_set_path("target/descriptors.bin")
    .type_attribute(".", "#[allow(clippy::all)]")
    .compile(&misc_protos, &[proto_path.clone()])?;
```

Apply the same pattern to all other compilation blocks (lines 104-114).

### Verification Steps

1. Run `cargo clean` to remove all generated code
2. Run `cargo build` to regenerate with tonic_build
3. Verify `GoogleAdsClient` exists in generated code
4. Run `cargo clippy` to ensure no clippy errors from generated code
5. Run existing tests to ensure functionality is preserved

## Additional Notes

- Both `prost-build = "0.11"` and `tonic-build = "0.8.0"` are already in `Cargo.toml` build-dependencies
- No dependency changes needed
- The `.type_attribute(".", "#[allow(clippy::all)]")` method is available in both libraries
- tonic-build 0.8.0 uses prost-build internally, so the generated message types will be identical
