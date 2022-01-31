DIR=$(pwd | xargs -I {} basename {})

if [[ "$DIR" != "web" ]]; then
	echo "format can only be called from the root /web directory"
	exit 1
fi;

echo "formatting files..."

yarn prettier --write .
