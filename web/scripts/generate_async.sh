#!/bin/bash

DIR=$(pwd | xargs -I {} basename {})

if [[ "$DIR" != "web" ]]; then
	echo "generate_schemas can only be called from the root /web directory"
	exit 1
fi;

FILES=$(find src -name "*_schema.ts")

function generate {
	file=$1
	echo "Generating $file"
	base_dir=$(dirname $file)
	id=$(basename $file "_schema.ts") # Make schema id the prefix to _schema.ts
	typescript-json-schema --aliasRefs --required --strictNullChecks --id $id $file "*" -o "${base_dir}/${id}_schema.json"
	echo "Finished generating $file"
}

for file in $FILES; do
	generate $file &
done

# Otherwise we'd exit early
wait
