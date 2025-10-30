#!/usr/bin/env sh

GIT_DIR="/var/git"
count=0
repo=""

for d in "$GIT_DIR"/*; do
    [ -d "$d" ] || continue
    count=$((count + 1))
    repo="$d"
done

if [ "$count" -eq 1 ]; then
    echo "Found one repository: $repo"
    exec ./basic-git-ui "$repo"
else
    echo "Error: Expected exactly one directory in $GIT_DIR, found $count." >&2
    exit 1
fi
