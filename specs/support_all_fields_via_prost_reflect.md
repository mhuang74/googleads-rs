# Plan: Replace `get()` match arms with prost-reflect dynamic field access

## Context

The `GoogleAdsRow::get(field_name: &str) -> String` method in `src/lib.rs` currently has **538 hand-maintained match arms** using 10 custom macros to convert GAQL field paths to string values. Each Google Ads API version upgrade requires manually adding match arms for new/changed fields. The crate currently supports ~35 of ~178 resources on `GoogleAdsRow`.

**Goal:** Replace the entire match statement with a generic reflection-based field walker using `prost-reflect`. This eliminates all manual maintenance — every valid GAQL field path works automatically, including new fields added in future API versions.

## Approach

Use `prost-reflect` 0.14 (compatible with the crate's `prost` 0.14) to:
1. Generate a file descriptor set at build time
2. At runtime, encode `GoogleAdsRow` to bytes, decode into a `DynamicMessage`, and walk the dotted GAQL path to format any field as a string

## Files to modify

| File | Change |
|------|--------|
| `Cargo.toml` | Add `prost-reflect` dep + `prost-reflect-build` build-dep |
| `build.rs` | Add `file_descriptor_set_path()` to prost config |
| `utils/update.sh` | Remove the `sed` that strips `optional` keywords from protos |
| `src/lib.rs` | Replace 538 match arms + 10 macros with ~80 lines of reflection walker |
| `tests/test_helpers/mod.rs` | Keep builders as-is (they construct real prost structs) |
| `tests/*.rs` | Existing tests should pass as-is; add new tests for previously unsupported fields |

## Step-by-step implementation

### Step 1: Stop stripping `optional` from protos

In `utils/update.sh`, remove lines 52-54 (the `sed` that strips `optional`). This gives prost-reflect proper field presence semantics. Also provide command line option to force the upgrade even if version numbers are the same.

Then re-run the optional stripping removal on the current proto files:
```bash
# Restore optional keywords in current v23 protos by re-downloading
# OR: since protos are checked in, we can re-fetch just the googleads v23 protos
```

**Note:** Since the current protos already have `optional` stripped, we need to re-download them. We can run `utils/update.sh v23` after removing the sed command, or manually restore the protos from googleapis.

### Step 2: Add dependencies to `Cargo.toml`

```toml
[dependencies]
prost-reflect = "0.14"

[build-dependencies]
prost-reflect-build = "0.14"
```

### Step 3: Modify `build.rs` to generate file descriptor set

Add `file_descriptor_set_path` to each `tonic_prost_build::configure()` call. The key challenge is that the current build compiles protos in chunks — we need all chunks to contribute to a single merged descriptor set.

Approach: Use `prost_build::Config` with `file_descriptor_set_path()` set, and pass it through `tonic_prost_build::configure().compile_protos_with_config()`. Each chunk appends to the same descriptor file. Alternatively, compile the descriptor set in a single separate pass.

**Recommended:** Add a single `prost_build::Config::new().file_descriptor_set_path(out.join("file_descriptor_set.bin")).compile_protos(&all_protos, &[proto_path])` call before the chunked tonic compilation. This generates the descriptor without Rust code; the chunked compilation still handles code generation.

### Step 4: Replace `get()` in `src/lib.rs`

Remove:
- All 10 macro definitions (`attr_str!`, `optional_attr_str!`, `method_str!`, etc.)
- All 538 match arms
- The `use` imports for specific enum variants and oneof variants (lines 30-50)

Add:
- `prost_reflect` imports
- Static `DESCRIPTOR_POOL` (lazy-initialized from embedded descriptor bytes)
- `walk_path(msg: &DynamicMessage, path: &str) -> String` — recursive path walker
- `format_value(value: &Value, field_desc: &FieldDescriptor) -> String` — formats any value as string
- Updated `get(&self, field_name: &str) -> String` that encodes self to bytes, decodes as DynamicMessage, walks path
- New `get_many(&self, field_names: &[&str]) -> Vec<String>` that encodes once, walks multiple paths

#### Key formatting rules for `format_value`:
- **Scalars** (string, i64, f64, bool, i32): `format!("{}", value)`
- **Enums**: Resolve variant name from `EnumDescriptor` (e.g., `"ENABLED"`, `"PAUSED"`)
- **Repeated scalars/strings**: Join with `", "`
- **Repeated enums**: Resolve each variant name, join with `", "`
- **Repeated messages**: Format each with `{:#?}`, join with `"; "`
- **Messages** (terminal, not walked further): `format!("{:#?}", msg)`
- **Unset optional messages**: Return `""`
- **Unknown field path**: Return `"not implemented by googleads-rs"`

#### Custom format overrides:
The only field with custom formatting today is `campaign.bidding_strategy_type` which maps enum variants to camelCase names (`ManualCPC`, `TargetCPA`, etc.). Options:
1. Accept proto-canonical names (`MANUAL_CPC`, `TARGET_CPA`) — simplest, and consistent with all other enum fields
2. Add a small `CUSTOM_FORMATS: HashMap<&str, fn>` for overrides — keeps backward compatibility

**Recommendation:** Accept proto-canonical names. The current custom names are inconsistent with other enum formatting anyway. This is a minor breaking change but simplifies the code.

#### Handling oneof fields:
In proto3, oneof variant fields (e.g., `keyword` inside `oneof criterion`) are accessed directly by field name on the parent message. prost-reflect handles this transparently — `get_field_by_name("keyword")` returns the value if that variant is active, or the default if not. GAQL paths like `ad_group_criterion.keyword.text` naturally decompose into: walk `ad_group_criterion` → walk `keyword` → get `text`. No special oneof handling needed.

#### Handling `change_event.changed_fields`:
Currently formats `FieldMask.paths` with single quotes. With reflection, `changed_fields` is a message with a `paths` repeated string field. The walker would return the debug format of the message. If the current formatting is important, add it as a special case. Otherwise, accept the reflection default (repeated strings joined with `", "`).

### Step 5: Update tests

- **Existing tests should mostly pass** since they test `row.get("field.path")` and the reflection approach returns the same string values for scalars
- **Enum formatting will change**: Current `method_str!` uses `{:#?}` (Rust debug format like `Enabled`) while prost-reflect returns proto names like `ENABLED`. Tests asserting enum values will need updating.
- **Add regression tests** for edge cases: unset optional parents, unknown paths, oneof with inactive variant
- **Add tests for previously unsupported fields** to validate that reflection covers them

### Step 6: Verify compilation and test pass

```bash
cargo build 2>&1
cargo test 2>&1
```

## Enum formatting: breaking change assessment

The biggest behavioral change is enum formatting:
- **Current**: `{:#?}` → Rust debug names like `Enabled`, `Paused`, `Search`, `ResponsiveSearchAd`
- **prost-reflect**: Proto canonical names like `ENABLED`, `PAUSED`, `SEARCH`, `RESPONSIVE_SEARCH_AD`

This affects ~80 match arms using `method_str!` and `optional_method_str!`. All tests asserting enum values will need updating. Downstream users comparing `get()` output against hardcoded strings will break.

**Mitigation options:**
1. Accept the breaking change (cleaner, consistent with API docs)
2. Convert proto names to PascalCase to approximate current format
3. Keep a compatibility layer

## Verification

1. `cargo build` — confirms proto compilation with descriptor set works
2. `cargo test` — all existing tests pass (after updating enum assertions)
3. Manual check: call `get()` with previously unsupported paths (e.g., `"account_link.resource_name"`, `"campaign_simulation.campaign_id"`) and verify they return values instead of `"not implemented by googleads-rs"`
4. Performance: benchmark `get()` vs `get_many()` on a row with 20 fields to confirm acceptable overhead

## Risks

- **prost-reflect 0.14 compatibility**: Need to verify it works with `tonic-prost-build` 0.14's compilation pipeline. If not, may need to generate the descriptor set separately.
- **Proto `optional` restoration**: Re-downloading protos with `update.sh v23` may pull slightly newer versions. Can mitigate by fetching from a pinned googleapis commit.
- **File descriptor set size**: ~2-5MB embedded in binary. Acceptable for this use case.
- **Enum name format change**: Breaking for downstream users. Consider a semver major bump.
