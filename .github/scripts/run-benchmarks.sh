#!/usr/bin/env bash
set -euo pipefail

output=${1:-results.csv}
binary=./target/release/bigint-benchmark
libraries=(rug malachite dashu ibig num-bigint)
benchmarks=(
  "e:100000:e_100k"
  "e:1000000:e_1m"
  "e:10000000:e_10m"
  "fib:10000000:fib_10m"
  "fib:100000000:fib_100m"
  "fib_hex:100000000:fib_hex_100m"
)

printf 'library' >"$output"
for benchmark in "${benchmarks[@]}"; do
  IFS=: read -r _ _ column <<<"$benchmark"
  printf ',%s' "$column" >>"$output"
done
printf '\n' >>"$output"

for library in "${libraries[@]}"; do
  printf '%s' "$library" >>"$output"
  for benchmark in "${benchmarks[@]}"; do
    IFS=: read -r task size _ <<<"$benchmark"
    milliseconds=$(
      "$binary" --task "$task" --lib "$library" -n "$size" benchmark |
        awk -v library="$library" '$1 == library { print $2 }'
    )
    if [[ -z "$milliseconds" ]]; then
      printf 'No result for %s/%s/%s\n' "$library" "$task" "$size" >&2
      exit 1
    fi
    python3 -c \
      'import sys; print(f",{int(sys.argv[1]) / 1000:.3f}", end="")' \
      "$milliseconds" >>"$output"
  done
  printf '\n' >>"$output"
done
