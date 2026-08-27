#!/bin/sh
set -eu

root_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
binary="$root_dir/target/release/khb"
output="$root_dir/site/public/demo"

if [ ! -x "$binary" ]; then
  echo "release binary missing; run cargo build --release first" >&2
  exit 1
fi

"$binary" build "$root_dir/examples/atlas/handoff.yaml" --output "$output" --force
