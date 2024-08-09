#!/bin/bash

rm -rf proto
mkdir proto
#wget https://github.com/googleapis/googleapis/archive/master.zip && unzip -q master
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
mv googleapis-master/google/ads/googleads/v17 proto/google/ads/googleads
######################################################################################

find proto -type f -not -name '*.proto' -delete
find proto -type f | xargs sed -i'' -e 's/^ *optional//g'
#rm -rf googleapis-master master.zip
find proto -type f -name '*.proto-e' -delete
