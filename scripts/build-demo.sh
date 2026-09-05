#!/bin/sh
set -eu

root_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
binary="$root_dir/target/release/khb"
output="$root_dir/site/public/demo"

if [ ! -x "$binary" ]; then
  echo "release binary missing; run cargo build --release first" >&2
  exit 1
fi

"$binary" demo --output "$output" --force
node "$root_dir/scripts/mark-demo.mjs" "$output/index.html"
