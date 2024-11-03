#!/usr/bin/env bash
## lint all code directories in the repostitory using cargo clippy.

for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo clippy --all-targets --all-features -- -D warnings)
done

echo "Format complete."