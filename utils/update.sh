#!/bin/bash

# Strict error mode: abort on any unhandled failure, unset variable, or pipe failure.
set -euo pipefail

# Cosmetic cleanup step permitted to fail with a warning only.
# Reserved for non-essential tidying; never use for migration-critical steps.
best_effort() {
  "$@" || echo "Warning: best_effort step failed (continuing): $*" >&2
}

# Cross-platform in-place sed
# Usage: sed_inplace 'pattern' file [file2 ...]
sed_inplace() {
  if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "$@"
  else
    sed -i "$@"
  fi
}

if [ -z "${1:-}" ]; then
  echo "Error: must supply Google Ads API version, e.g., 'v17'"
  exit 1
fi
GOOGLEADS_API_VERSION=$1

# Determine the current Google Ads API version from build.rs
current_version=$(grep -oE 'googleads\{\}v[0-9]+' build.rs | grep -oE 'v[0-9]+')

if [ "$current_version" == "$GOOGLEADS_API_VERSION" ] && [ "${2:-}" != "--force" ]; then
  echo "Nothing Done. Already at target version $GOOGLEADS_API_VERSION (use --force to update anyway)"
  exit 0
fi

echo "Updating googleads-rs to $GOOGLEADS_API_VERSION"

rm -rf proto
mkdir proto
# download latest googleapis
curl https://github.com/googleapis/googleapis/archive/master.zip -o master.zip -L --silent
unzip -q master

# infrastructure needed by googleads
mkdir -p proto/google
mv googleapis-master/google/rpc proto/google
mv googleapis-master/google/longrunning proto/google
mv googleapis-master/google/type proto/google
mv googleapis-master/google/logging proto/google
mv googleapis-master/google/api proto/google

# move latest googleads api
mkdir -p proto/google/ads/googleads

############################### GOOGLE ADS API VERSION ###############################
mv googleapis-master/google/ads/googleads/$GOOGLEADS_API_VERSION proto/google/ads/googleads
######################################################################################

# only keep proto files
find proto -type f -not -name '*.proto' -delete

# Remove comments from 2 proto files to avoid doc test errors
best_effort sed_inplace -e 's;//.*$;;' -e '/\/\*/,/\*\//d' proto/google/rpc/error_details.proto
best_effort sed_inplace -e 's;//.*$;;' -e '/\/\*/,/\*\//d' proto/google/rpc/context/attribute_context.proto

# Remove extra proto-e files
best_effort find proto -type f -name '*.proto-e' -delete

# Remove downloaded archive + extracted tree
best_effort rm -rf googleapis-master master.zip

# Update build.rs
sed_inplace "s/googleads{}$current_version/googleads{}$GOOGLEADS_API_VERSION/g" build.rs
sed_inplace "s/googleads::$current_version/googleads::$GOOGLEADS_API_VERSION/g" src/lib.rs

# Update tests/*.rs
sed_inplace "s/googleads::$current_version/googleads::$GOOGLEADS_API_VERSION/g" tests/*.rs


# Update tests/test_helpers/*.rs
sed_inplace "s/googleads::$current_version/googleads::$GOOGLEADS_API_VERSION/g" tests/test_helpers/*.rs

# Update README.md
sed_inplace "s/Google Ads API $current_version/Google Ads API $GOOGLEADS_API_VERSION/g" README.md


