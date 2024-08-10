#!/bin/bash

if [ -z "$1" ]; then
  echo "Error: must supply Google Ads API version, e.g., 'v17'"
  exit 1
fi
GOOGLEADS_API_VERSION=$1

# Determine the current Google Ads API version from build.rs
current_version=$(grep -oE 'googleads/v[0-9]+' build.rs | grep -oE 'v[0-9]+')

if [ "$current_version" == "$GOOGLEADS_API_VERSION" ]; then
  echo "Nothing Done. Already at target version"
  exit 0
fi

echo "Updating googleads-rs to $GOOGLEADS_API_VERSION"

rm -rf proto
mkdir proto
# download latest googleapis
curl https://github.com/googleapis/googleapis/archive/master.zip -o master.zip -L --silent && unzip -q master

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

# remove optional keyword
find proto -type f | xargs sed -i '' -e 's/^ *optional//g'

# remove comments from 2 proto files to avoid doc test errors
sed -i '' -e 's;//.*$;;' -e '/\/\*/,/\*\//d' proto/google/rpc/error_details.proto
sed -i '' -e 's;//.*$;;' -e '/\/\*/,/\*\//d' proto/google/rpc/context/attribute_context.proto

# remove extra proto-e files
find proto -type f -name '*.proto-e' -delete

# remove orig googleapis files
rm -rf googleapis-master master.zip

## update version references in rust code

# Update build.rs
sed -i '' "s/googleads\/$current_version/googleads\/$GOOGLEADS_API_VERSION/g" build.rs

# Update src/lib.rs
sed -i '' "s/googleads::$current_version/googleads::$GOOGLEADS_API_VERSION/g" src/lib.rs

# Update README.md
sed -i '' "s/Google Ads API $current_version/Google Ads API $GOOGLEADS_API_VERSION/g" README.md


