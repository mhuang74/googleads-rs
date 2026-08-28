#!/bin/bash

# Deterministic migration script for Google Ads API upgrades.
# Usage: ./utils/update.sh vNN [--force]
#
# Linux-only: relies on GNU sed semantics (no macOS branch).
set -euo pipefail

RELEASE_NOTES_URL="https://developers.google.com/google-ads/api/docs/release-notes"

# Cosmetic cleanup step permitted to fail with a warning only.
# Reserved for non-essential tidying; never use for migration-critical steps.
best_effort() {
  "$@" || echo "Warning: best_effort step failed (continuing): $*" >&2
}

if [ -z "${1:-}" ]; then
  echo "Error: must supply Google Ads API version, e.g., 'v17'"
  exit 1
fi
GOOGLEADS_API_VERSION=$1

# Target major version, e.g. v25 -> 25 (integer, no leading 'v' padding).
TARGET_MAJOR=$((10#${GOOGLEADS_API_VERSION#v}))

# Current crate version from Cargo.toml (authoritative source; its major
# component mirrors the supported Google Ads API major version).
CARGO_VERSION=$(sed -nE 's/^version = "([0-9]+\.[0-9]+\.[0-9]+)"/\1/p' Cargo.toml | head -n1)
if [ -z "$CARGO_VERSION" ]; then
  echo "Error: could not parse package version from Cargo.toml" >&2
  exit 1
fi
CARGO_MAJOR=$((10#${CARGO_VERSION%%.*}))
CURRENT_API_VERSION="v${CARGO_MAJOR}"

if [ "$CURRENT_API_VERSION" == "$GOOGLEADS_API_VERSION" ] && [ "${2:-}" != "--force" ]; then
  echo "Nothing Done. Already at target version $GOOGLEADS_API_VERSION (use --force to update anyway)"
  exit 0
fi

# ---------------------------------------------------------------------------
# Target minor version: inferred from Google's release notes for the target
# major. Version headings carry data-text="vM.N (date)". On fetch/parse
# failure the current minor is kept (non-fatal warning).
# ---------------------------------------------------------------------------
parse_release_notes_minor() {
  local major=$1 html minors best
  if ! html=$(curl -fsSL --max-time 30 "$RELEASE_NOTES_URL"); then
    return 1
  fi
  minors=$(printf '%s' "$html" | grep -oE "data-text=\"v${major}\.[0-9]+" | grep -oE '[0-9]+$' | sort -n)
  if [ -z "$minors" ]; then
    return 1
  fi
  best=$(printf '%s\n' "$minors" | tail -n1)
  printf '%s\n' "$best"
}

TARGET_MINOR=""
if NEW_MINOR=$(parse_release_notes_minor "$TARGET_MAJOR"); then
  TARGET_MINOR=$NEW_MINOR
else
  echo "Warning: could not fetch or parse release notes for v${TARGET_MAJOR}; keeping current minor." >&2
  echo "  Verify the newest minor manually at: $RELEASE_NOTES_URL" >&2
fi

# ---------------------------------------------------------------------------
# New crate version (ruled arithmetic):
#   new major + parsed minor -> M.MIN.0   (release-notes inference)
#   new major + unparsable  -> M.0.0
#   same major, minor > curr -> M.MIN.0
#   same major+minor       -> patch + 1   (--force re-run)
#   unparsable notes       -> keep current minor, warn
# ---------------------------------------------------------------------------
CURR_MINOR=$(awk -F. '{print $2}' <<<"$CARGO_VERSION")
CURR_PATCH=$(awk -F. '{print $3}' <<<"$CARGO_VERSION")

if [ -z "$TARGET_MINOR" ]; then
  if [ "$TARGET_MAJOR" -gt "$CARGO_MAJOR" ]; then
    # New major, unparsable notes: start at .0.0
    NEW_CARGO_VERSION="${TARGET_MAJOR}.0.0"
  else
    NEW_CARGO_VERSION="${TARGET_MAJOR}.${CURR_MINOR}.0"
  fi
elif [ "$TARGET_MAJOR" -gt "$CARGO_MAJOR" ]; then
  # New major: use the release-notes-inferred minor (issue #64 acceptance:
  # 23.2.1 -> v24 must produce 24.2.0, not 24.0.0).
  NEW_CARGO_VERSION="${TARGET_MAJOR}.${TARGET_MINOR}.0"
elif [ "$TARGET_MINOR" -gt "$CURR_MINOR" ]; then
  NEW_CARGO_VERSION="${TARGET_MAJOR}.${TARGET_MINOR}.0"
elif [ "$TARGET_MINOR" -eq "$CURR_MINOR" ]; then
  NEW_CARGO_VERSION="${TARGET_MAJOR}.${TARGET_MINOR}.$((CURR_PATCH + 1))"
else
  # Parsed minor older than current: keep current minor (should not normally happen).
  echo "Warning: release notes report v${TARGET_MAJOR}.${TARGET_MINOR}, older than current minor ${CURR_MINOR}; keeping ${CURR_MINOR}." >&2
  NEW_CARGO_VERSION="${TARGET_MAJOR}.${CURR_MINOR}.0"
fi

echo "Updating googleads-rs to $GOOGLEADS_API_VERSION (crate ${CARGO_VERSION} -> ${NEW_CARGO_VERSION})"

# Download + extract googleapis master, staging the new tree in a temp dir.
# proto/ is swapped in only after validation passes, so a failed
# download/validation leaves the existing tree un-mutated.
STAGE_DIR=$(mktemp -d)/proto
mkdir -p "$STAGE_DIR/google/ads/googleads"

# download latest googleapis
curl https://github.com/googleapis/googleapis/archive/master.zip -o master.zip -L --silent
unzip -q master

# ---------------------------------------------------------------------------
# Pre-copy validation: the master archive carries multiple API majors, so
# "dir exists" alone is not enough — assert the target version directory
# actually arrived before moving anything.
# ---------------------------------------------------------------------------
if [ ! -d "googleapis-master/google/ads/googleads/$GOOGLEADS_API_VERSION" ]; then
  echo "Error: $GOOGLEADS_API_VERSION directory missing from downloaded googleapis master archive" >&2
  exit 1
fi

# infrastructure needed by googleads
mv googleapis-master/google/rpc "$STAGE_DIR/google"
mv googleapis-master/google/longrunning "$STAGE_DIR/google"
mv googleapis-master/google/type "$STAGE_DIR/google"
mv googleapis-master/google/logging "$STAGE_DIR/google"
mv googleapis-master/google/api "$STAGE_DIR/google"

# move latest googleads api
mv googleapis-master/google/ads/googleads/$GOOGLEADS_API_VERSION "$STAGE_DIR/google/ads/googleads"


# ---------------------------------------------------------------------------
# Post-copy validation: infrastructure dirs + version dir must exist and hold
# .proto files. A silent miss here yields a corrupt proto tree.
# ---------------------------------------------------------------------------
check_proto_dir() {
  local dir=$1
  if [ ! -d "$dir" ]; then
    echo "Error: post-copy validation failed: directory missing: $dir" >&2
    exit 1
  fi
  if [ -z "$(find "$dir" -name '*.proto' -print -quit)" ]; then
    echo "Error: post-copy validation failed: no .proto files in: $dir" >&2
    exit 1
  fi
}

for infra_dir in rpc longrunning type logging api; do
  check_proto_dir "$STAGE_DIR/google/$infra_dir"
done
check_proto_dir "$STAGE_DIR/google/ads/googleads/$GOOGLEADS_API_VERSION"

# Validation passed: swap the staged tree into place atomically.
rm -rf proto
mv "$STAGE_DIR" proto

# only keep proto files
find proto -type f -not -name '*.proto' -delete

# Remove comments from 2 proto files to avoid doc test errors
best_effort sed -i -e 's;//.*$;;' -e '/\/\*/,/\*\//d' proto/google/rpc/error_details.proto
best_effort sed -i -e 's;//.*$;;' -e '/\/\*/,/\*\//d' proto/google/rpc/context/attribute_context.proto

# Remove extra proto-e files
best_effort find proto -type f -name '*.proto-e' -delete

# Remove downloaded archive + extracted tree
best_effort rm -rf googleapis-master master.zip

# ---------------------------------------------------------------------------
# Version-reference rewrites.
# The generated versioned module path is only ever named by the single alias
# line in src/lib.rs; no other library or test source carries a version
# literal. build.rs derives everything from Cargo.toml.
# The single hand-edit anchor: repoint the alias at the new generated module.
sed -i "s/googleads::$CURRENT_API_VERSION/googleads::$GOOGLEADS_API_VERSION/g" src/lib.rs

# Crate version bump (patch resets on major/minor change by construction above).
sed -i "s/^version = \"${CARGO_VERSION}\"/version = \"${NEW_CARGO_VERSION}\"/" Cargo.toml

# Doc-root URL carries the full crate version.
sed -i "s|googleads-rs/${CARGO_VERSION}|googleads-rs/${NEW_CARGO_VERSION}|g" src/lib.rs

# README: full crate version, and API major.minor mentions (crate major.minor
# mirrors the API major.minor, so the new API mention is M.MIN from the new
# crate version).
sed -i "s/${CARGO_VERSION}/${NEW_CARGO_VERSION}/g" README.md
NEW_API_MINOR=$(awk -F. '{print $1"."$2}' <<<"$NEW_CARGO_VERSION")
sed -i "s/API v${CARGO_MAJOR}\.${CURR_MINOR}/API v${NEW_API_MINOR}/g" README.md

echo "Migration complete: crate ${CARGO_VERSION} -> ${NEW_CARGO_VERSION}, API $GOOGLEADS_API_VERSION"