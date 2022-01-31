#!/bin/bash

DIR=$(pwd | xargs -I {} basename {})

if [[ "$DIR" != "web" ]]; then
	echo "generate_schemas can only be called from the root /web directory"
	exit 1
fi;

FILES=$(find src -name "*_schema.ts")
TOTAL_COUNT=$(echo $FILES | wc -w | tr -d ' ')

echo "Generating $TOTAL_COUNT JSON schema files in web/src/service/*"

current_count=1
for file in $FILES; do
	index="$current_count/$TOTAL_COUNT"
	echo "----------------------------------------------------------"
	echo "[$index] Input: $file"

	base_dir=$(dirname $file)
	id=$(basename $file "_schema.ts") # Make schema id the prefix to _schema.ts
	typescript-json-schema --required --strictNullChecks --id $id $file "*" -o "${base_dir}/${id}_schema.json"

	echo "[$index] Output: ${base_dir}/${id}_schema.json"
	current_count=$(($current_count+1))
done

echo "----------------------------------------------------------"
echo "Thanks for waiting <3"
