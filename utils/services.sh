#!/bin/bash

# PACKAGE_NAME <tab> PATH_TO_PROTO
grep '^package' -r proto \
| grep -v 'BUILD' \
| grep -E "googleads\/v10|google\/longrunning|google\/api|google\/rpc" \
| sed 's/:/ /g' \
| sed 's/;//g' \
| sed 's/\.type/\.r#type/g' \
| awk '{print $3 " " $1}' \
| sed 's/0/@/g' \
| sed 's/\./0/g' \
| sort -k1,1 \
| sed 's/0/\./g' \
| sed 's/@/0/g' \
