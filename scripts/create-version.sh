#!/bin/bash

if [[ -z "$1" ]]; then
    echo "Usage: $0 [token]"
    exit 1
fi

curl \
    -H "Authorization: Bearer $1" \
    -X PUT \
    http://localhost:4000/api/v1/packages/1/versions \
    -F 'name=0.1.0' \
    -F 'version_number=0.1.0' \
    -F 'changelog=Initial release!' \
    -F 'kubejs=any' \
    -F 'loaders=forge,fabric,quilt,neoforge' \
    -F 'minecraft=1.20.1,1.20.2,1.20.3,1.20.4,1.20.5,1.20.6,1.21,1.21.1,1.21.2,1.21.3' \
    -F 'file=@test-package.tgz'
