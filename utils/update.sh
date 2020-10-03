#!/bin/bash

rm -rf proto
mkdir proto
wget https://github.com/googleapis/googleapis/archive/master.zip && unzip -q master.zip
mv googleapis-master/google proto
mv googleapis-master/grafeas proto
find proto -type f -not -name '*.proto' -delete
find proto -type f | xargs sed -i'' -e 's/^ *optional//g'
rm -rf googleapis-master master.zip
