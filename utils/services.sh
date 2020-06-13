#!/bin/bash

# PACKAGE_NAME <tab> PATH_TO_PROTO
grep '^package' -r proto \
| sed 's/:/ /g' \
| sed 's/;//g' \
| sed 's/\.type/\.r#type/g' \
| awk '{print $3 "\t" $1}' \
| sort
