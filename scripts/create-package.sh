#!/bin/bash

if [[ -z "$1" ]]; then
    echo "Usage: $0 [token]"
    exit 1
fi

curl \
    -H "Authorization: Bearer $1" \
    -X PUT \
    http://localhost:4000/api/v1/packages \
    -H "Content-Type: application/json" \
    -d '{
            "name": "Test Package",
            "slug": "my_package",
            "description": "Test package!",
            "readme": "MY PACKAGE LETS GO"
        }'
