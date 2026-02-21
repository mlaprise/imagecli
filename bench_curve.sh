#!/usr/bin/env bash
set -e

INPUT="${1:-sample.jpg}"
OUTPUT="/tmp/bench_output.jpg"
BIN="./target/release/imagecli"

if [ ! -f "$BIN" ]; then
  echo "Building release binary..."
  cargo build --release
fi

echo "Input: $INPUT"
echo "Pipeline: grayscale → curve → color-grade → vignette"
echo "---"

start=$(python3 -c 'import time; print(time.time())')

$BIN -i "$INPUT" grayscale |
  $BIN curve --darks=20 --middarks=-10 --midhighlights=15 --highlights=-10 -o "$OUTPUT"

end=$(python3 -c 'import time; print(time.time())')

elapsed=$(python3 -c "print(f'{($end - $start) * 1000:.0f}')")
size=$(wc -c <"$INPUT" | tr -d ' ')
dims=$(identify -format '%wx%h' "$INPUT" 2>/dev/null || echo "unknown")

echo "Dimensions: $dims"
echo "File size:  $((size / 1024)) KB"
echo "Total time: ${elapsed} ms"
