#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
proc_macro_dir="$repo_root/crates/bevy_auto_plugin_proc_macros"
docs_src="$repo_root/docs"
docs_gen="$proc_macro_dir/docs_gen"

if [[ ! -d "$docs_src" ]]; then
  echo "docs source not found at $docs_src" >&2
  exit 1
fi

version=$(rg -n '^version = ' "$proc_macro_dir/Cargo.toml" | head -n1 | sed -E 's/.*"([^"]+)".*/\1/')
if [[ -z "${version}" ]]; then
  echo "failed to read version from $proc_macro_dir/Cargo.toml" >&2
  exit 1
fi

rm -rf "$docs_gen"
mkdir -p "$docs_gen"
cp -R "$docs_src/" "$docs_gen/"
printf "%s\n" "$version" > "$docs_gen/.version"

echo "Prepared docs_gen for bevy_auto_plugin_proc_macros v$version"

echo "Running: cargo publish $*"
(cwd="$repo_root"; cd "$cwd"; cargo publish "$@")
