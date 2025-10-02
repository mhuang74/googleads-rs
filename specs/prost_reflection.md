# Prost-reflect dynamic field access for GoogleAdsRow.get

This spec introduces a prost-reflect based dynamic accessor to retrieve values from GoogleAdsRow by GAQL style dot paths, avoiding ongoing manual maintenance of the large match in [src/lib.rs](src/lib.rs).


## Goals

- Add a feature-flagged reflective implementation of get(path) for GoogleAdsRow.
- Resolve GAQL-style paths across nested messages, optionals, oneofs, repeated fields, and maps.
- Output enums by symbolic names and provide consistent string formatting for well-known types.
- Maintain current behavior as fallback for bespoke formatting cases.
- Optimize for common usage pattern of iterating FieldMask.paths per row.


## Non-goals

- Full GAQL semantic validation.
- Replacing all custom formatting in the existing manual path completely at once.
- Generating compile-time code for path lookup (kept as a future option).


## High-level design

flowchart TD
  Client[Caller iterates field mask paths] --> API[GoogleAdsRow::get]
  API --> Cfg[If feature reflective_get enabled]
  Cfg --> Cache[Build or reuse per row DynamicMessage]
  Cache --> Resolver[Path resolver over DynamicMessage]
  Resolver --> Format[Value to string formatting policy]
  Format --> Out[String value]
  Cfg -. disabled .-> Legacy[Fallback to manual matcher]
  Legacy --> Out

Rationale:
- DynamicMessage and descriptors let us navigate fields by name without hand-coding arms.
- A per-row DynamicMessage avoids repeated encode decode per path when iterating masks.
- Fallback retains specialized, human-friendly rendering for a subset of fields while we transition.


## Build-time changes

1) Dependencies and feature flag in [Cargo.toml](Cargo.toml)

```toml
[features]
reflective_get = ["prost-reflect"]

[dependencies]
prost-reflect = { version = "0.14", optional = true, features = ["derive"] }
once_cell = { version = "1", optional = true }

[features]
# ensure once_cell available when enabling reflective_get
reflective_get = ["prost-reflect", "once_cell"]
```

Notes:
- prost-reflect feature gated under reflective_get to keep default binary size minimal.
- once_cell used for lazy static initialization of DescriptorPool.

2) Emit and embed FileDescriptorSet in [build.rs](build.rs)

We need a self-contained descriptor set for all compiled protos.

Implementation outline:
- Generate the same protos via tonic-build as today.
- Additionally, run prost-build with file_descriptor_set_path, include_imports, include_source_info.
- Write the produced descriptor set to OUT_DIR and embed it with include_bytes!.

Sketch:

```rust
// in build.rs
let descriptor_path = Path::new(&out_dir).join("googleads_descriptor.pb");
prost_build::Config::new()
    .file_descriptor_set_path(&descriptor_path)
    .include_file("ignored") // no include file generation
    .compile_protos(&all_protos, &[proto_path.clone()])?;
println!("cargo:rerun-if-changed={}", descriptor_path.display());
```

Key settings:
- include_imports true by default for prost_build::Config descriptors with prost-build 0.11+, but verify. If needed, call .include_file and then load full deps from compile_protos; alternatively, use prost_reflect_build for convenience.
- Ensure we feed the same set of files used by tonic_build to avoid mismatches.

If prost_build::Config cannot be reconciled with tonic_build input batching cleanly, switch to a single pass using prost_reflect_build which can export descriptors while generating prost types for tonic. Keep current tonic-build generation as is, but prefer a unified list all_protos to produce a single descriptor set. The descriptor set must include every file needed by GoogleAdsRow and its reachable graph.


## Runtime initialization

Create a global DescriptorPool from embedded descriptor bytes:

```rust
// somewhere in lib.rs or a new module behind the feature gate
#[cfg(feature = "reflective_get")]
mod reflection {
    use once_cell::sync::Lazy;
    use prost_reflect::DescriptorPool;

    pub static DESCRIPTORS: Lazy<DescriptorPool> = Lazy::new(|| {
        // File is produced by build.rs into OUT_DIR, then embedded
        static BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/googleads_descriptor.pb"));
        DescriptorPool::decode(BYTES).expect("decode descriptor set")
    });
}
```

Considerations:
- The DescriptorPool is immutable and shareable.
- Verify the message full name for GoogleAdsRow to locate its descriptor at runtime.


## Data flow

1) Transcoding GoogleAdsRow to DynamicMessage
- Encode self into bytes with prost encode_to_vec.
- Find MessageDescriptor for google.ads.googleads.v19.services.GoogleAdsRow.
- Decode a DynamicMessage using that descriptor from the bytes.

2) Resolving dot paths
- Split incoming path by dot into segments.
- Walk the message tree; at each step, resolve:
  - regular field name: follow into field value.
  - oneof variant: if segment does not match a field but matches a oneof case name, check active variant and proceed if it matches.
  - repeated fields: if the current value is a list and next segment is numeric selector [N] allow indexed access; otherwise map over the list and aggregate sub-field results.
  - maps: treat as repeated of entries with key and value; allow .key and .value or index selection policy.
- Stop when last segment is reached; convert the primitive or enum to string.

3) Formatting
- Enum: return symbolic name, not integer.
- Timestamps: RFC3339 UTC using prost_types::Timestamp semantics.
- Durations: render seconds.nanos with s suffix or ISO 8601 duration if preferred and consistent.
- Bytes: base64 standard.
- Numerics and strings: to_string as-is.
- Unset optionals: empty string.
- Not-found segments: empty string to maintain today’s user experience.


## Path resolution semantics

Segment resolution order at each step:
1) Try a normal field with that exact proto field name.
2) If not found, check oneof cases:
   - For each oneof on the message, see if the segment matches the case name.
   - If the active oneof matches this case, descend into that variant sub-message or scalar wrapper.
3) If still not found, return empty string.

Repeated values:
- If path uses an explicit numeric selector like segment [N], pick element N if in range, else empty.
- If no selector, but next segment exists, map element wise and join using delimiter.
- If the repeated element is scalar and terminal, join using delimiter.

Maps:
- Treat as repeated entries with .key and .value fields.
- If the next segment is a known key literal enclosed in backticks in GAQL, optional enhancement is key based access; default behavior is to join values.

Delimiter policy:
- Default delimiter: ", ".
- Configurable via internal constant if we need to align with current outputs per field type.

Empty values:
- Any missing leg returns empty string immediately.

Compatibility notes:
- Existing manual matcher has bespoke formatting for some enum aggregations, e.g. campaign.bidding_strategy_type mapping to friendly labels. Keep the manual matcher as fallback for exact known paths while reflective path covers the general case.


## API surface

We retain the existing method in [src/lib.rs](src/lib.rs):

- If feature reflective_get is enabled, try reflective path first; on error or unsupported cases, fall back to the manual matcher.
- If disabled, behave exactly as today.

Proposed helpers behind feature flag:

```rust
#[cfg(feature = "reflective_get")]
impl google::ads::googleads::v19::services::GoogleAdsRow {
    pub fn get_reflective(&self, path: &str) -> String {
        // Fast path: reuse cached DynamicMessage if available, else create
        // Then resolve and format
    }
}
```

Per-row cache type:

```rust
#[cfg(feature = "reflective_get")]
pub struct RowView {
    dyn_row: prost_reflect::DynamicMessage,
}

#[cfg(feature = "reflective_get")]
impl RowView {
    pub fn from_row(row: &google::ads::googleads::v19::services::GoogleAdsRow) -> Self { /* ... */ }
    pub fn get(&self, path: &str) -> String { /* ... */ }
}
```

Usage:
- For loops over field_mask.paths, build RowView once per GoogleAdsRow and call get for each path.


## Implementation plan

1) Add feature flag and dependencies in [Cargo.toml](Cargo.toml).
2) Update [build.rs](build.rs) to produce an embedded descriptor set for all compiled protos.
3) Add reflection module with global DescriptorPool.
4) Implement DynamicMessage transcoder for GoogleAdsRow:
   - Locate MessageDescriptor by full name.
   - Encode self with prost, decode to DynamicMessage with descriptor.
5) Implement path resolver:
   - parse dot path, support optional [N] array access per segment,
   - message field lookup by name,
   - oneof variant resolution when segment matches case name,
   - repeated handling: index or join mapping,
   - map handling via key and value.
6) Implement value formatting policy:
   - enums by name,
   - timestamps to RFC3339,
   - durations readable,
   - bytes base64,
   - default to to_string else empty.
7) Integrate into existing get in [src/lib.rs](src/lib.rs) behind cfg(feature = "reflective_get"):
   - try reflective path,
   - if empty and known bespoke field, use manual mapping,
   - otherwise return reflective result or existing default.
8) Performance:
   - RowView providing cached DynamicMessage,
   - reuse Vec buffers for encode,
   - micro-benchmark in tests to validate overhead is acceptable.
9) Testing:
   - Unit tests for resolver on synthetic DynamicMessage trees,
   - Integration tests against real GoogleAdsRow samples for attributes, metrics, segments,
   - Oneof example: ad_group_criterion.keyword.text,
   - Repeated example: ad_group_ad.ad.responsive_search_ad.headlines.text join,
   - Optional unset fields returning empty string,
   - Enum formatting matches symbolic names.
10) Documentation:
   - Update crate docs in [src/lib.rs](src/lib.rs) example to mention reflective_get,
   - Add feature docs to README.


## Detailed resolver behavior

Parsing:
- Split on ., but allow future extension for bracketed indices [N].
- Maintain a cursor of the current Value (prost_reflect::Value) and an optional descriptor context.

At a message Value:
- Try field by name with message_descriptor.get_field_by_name(segment).
- If found:
  - If scalar or enum, proceed to formatting if terminal.
  - If message, set cursor to nested message.
  - If list, set cursor to list Value and continue.
  - If map, set cursor to map Value and continue.
- Else, check oneofs:
  - For each OneofDescriptor, see if any case.field.name == active case and segment equals the case.field.name or a short alias like keyword:
    - If active case name equals segment, set cursor to that case value and continue.
- If neither matches, return empty.

At a list Value:
- If next token is [N], take N; else map across:
  - If terminal, format each element and join.
  - If non-terminal, descend per element and join results.

At a map Value:
- Treat as list of entries of message with fields key and value.
- Allow .key and .value traversal; default when terminal is to join value strings.

Terminal formatting:
- Value::EnumNumber: use descriptor to resolve to EnumValueDescriptor.name() if available.
- Value::Message for well-known types:
  - Timestamp: to RFC3339,
  - Duration: seconds.nanos with s suffix,
  - Any: type_url and value length or attempt unpack if descriptor present,
  - Wrapper types: unwrap inner primitive.
- Other messages with no policy: return empty if terminal was requested without subfield.


## Compatibility and fallback

- When reflective_get is enabled, the new path is authoritative for most fields.
- For paths with custom friendliness, e.g., campaign.bidding_strategy_type with custom labels, route through existing manual mapping to preserve output, or introduce a small mapping table on top of enum names.
- For unknown or unresolvable paths, return the same default as today: empty string or not implemented by googleads-rs if we choose to keep that exact sentinel for parity.


## Risks and mitigations

- Descriptor size: Embedding full Google Ads API descriptors increases binary size.
  - Mitigation: feature gate, optionally split a slim descriptor set with only reachable packages for v19.
- Performance: Encode decode cost and reflection overhead.
  - Mitigation: RowView cache per row, buffer reuse, and benchmark.
- GAQL naming differences: In some cases GAQL surface names may not match proto fields.
  - Mitigation: allow a small alias map for known divergent segments, e.g., map headlines to headlines, but if any mismatches arise, maintain an alias table in code with minimal overhead.


## Rollout plan

- Phase 1: Land behind feature reflective_get off by default. Ship as minor version bump with experimental tag in docs.
- Phase 2: Gather feedback on coverage and performance, close gaps with small aliasing and bespoke formats.
- Phase 3: Consider enabling by default, keeping manual matcher as fallback for special cases.


## Open items

- Confirm exact MessageDescriptor full name for GoogleAdsRow loaded from [build.rs](build.rs) generated protos.
- Decide final delimiter for repeated joins, currently ", ".
- Decide precise formatting for Duration and Any if encountered.
- Evaluate whether to expose RowView in public API or keep internal.


## References

- Current manual accessor: [src/lib.rs](src/lib.rs)
- Build script: [build.rs](build.rs)
- Manifest: [Cargo.toml](Cargo.toml)