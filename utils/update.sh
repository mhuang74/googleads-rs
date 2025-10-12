#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Define a function to handle errors gracefully
safe_run() {
  "$@" || echo "Command failed: $*" >&2
}

if [ -z "$1" ]; then
  echo "Error: must supply Google Ads API version, e.g., 'v17'"
  exit 1
fi
GOOGLEADS_API_VERSION=$1

# Determine the current Google Ads API version from build.rs
current_version=$(grep -oE 'googleads\{\}v[0-9]+' build.rs | grep -oE 'v[0-9]+')

if [ "$current_version" == "$GOOGLEADS_API_VERSION" ]; then
  echo "Nothing Done. Already at target version $GOOGLEADS_API_VERSION"
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
safe_run find proto -type f -not -name '*.proto' -delete

# Remove optional keyword
safe_run find proto -type f | while read -r file; do
  sed -i '' -e 's/^ *optional//g' "$file" || echo "Failed to process $file"
done

# Remove comments from 2 proto files to avoid doc test errors
safe_run sed -i '' -e 's;//.*$;;' -e '/\/\*/,/\*\//d' proto/google/rpc/error_details.proto
safe_run sed -i '' -e 's;//.*$;;' -e '/\/\*/,/\*\//d' proto/google/rpc/context/attribute_context.proto

# Remove extra proto-e files
safe_run find proto -type f -name '*.proto-e' -delete

# Remove orig googleapis files (uncomment if needed)
safe_run rm -rf googleapis-master master.zip

# Update version references in Rust code
# Update build.rs
safe_run sed -i '' "s/googleads{}$current_version/googleads{}$GOOGLEADS_API_VERSION/g" build.rs

# Update src/lib.rs
safe_run sed -i '' "s/googleads::$current_version/googleads::$GOOGLEADS_API_VERSION/g" src/lib.rs

# Update README.md
safe_run sed -i '' "s/Google Ads API $current_version/Google Ads API $GOOGLEADS_API_VERSION/g" README.md


