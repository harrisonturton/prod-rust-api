#!/bin/bash

DIR=$(pwd | xargs -I {} basename {})

if [[ "$DIR" != "web" ]]; then
	echo "generate_schemas can only be called from the root /web directory"
	exit 1
fi;

# Will generate JSON schema for every "*_schema.ts" file.
find src -name "*_schema.ts" | xargs -I {} sh -c "typescript-json-schema {} '*' > '{}.json'"
