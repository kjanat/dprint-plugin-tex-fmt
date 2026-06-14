#!/usr/bin/env bash
# End-to-end test: build the wasm plugin, then run each fixture pair through
# the real dprint binary and diff against the committed target file.
#
# Usage:
#   bash scripts/e2e.sh            # uses an already-built wasm if present
#   REBUILD=1 bash scripts/e2e.sh  # always rebuilds the wasm first
set -euo pipefail

WASM=target/wasm32-unknown-unknown/wasm-release/dprint_plugin_tex_fmt.wasm

if [[ "${REBUILD:-}" == "1" || ! -f "$WASM" ]]; then
	echo "Building wasm..."
	cargo build --profile wasm-release --target wasm32-unknown-unknown
fi

if ! command -v dprint &>/dev/null; then
	echo "error: dprint not found. Install with: cargo install dprint" >&2
	exit 1
fi

pass=true

for source_file in tests/fixtures/*/source/*.tex; do
	fixture=$(basename "$(dirname "$(dirname "$source_file")")")
	filename=$(basename "$source_file")
	target_file="tests/fixtures/$fixture/target/$filename"
	config_file="tests/fixtures/$fixture/config.json"

	actual=$(dprint fmt --config "$config_file" --stdin "$source_file" <"$source_file")

	if diff -u "$target_file" <(printf '%s\n' "$actual") >/dev/null 2>&1; then
		echo "PASS: $fixture/$filename"
	else
		echo "FAIL: $fixture/$filename"
		diff -u "$target_file" <(printf '%s\n' "$actual") || true
		pass=false
	fi
done

$pass && echo "All e2e tests passed."
