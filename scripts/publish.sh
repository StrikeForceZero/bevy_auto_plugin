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

dry_run_requested="false"
for arg in "$@"; do
  if [[ "$arg" == "--dry-run" ]]; then
    dry_run_requested="true"
  fi
done

dirty_files=$(git -C "$repo_root" status --porcelain=v1 | awk '{print $2}')
allow_dirty_arg=""
if [[ -n "$dirty_files" ]]; then
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if [[ "$file" != crates/bevy_auto_plugin_proc_macros/docs_gen/* ]]; then
      echo "Refusing to publish with dirty file outside docs_gen: $file" >&2
      exit 1
    fi
    if [[ "$file" == "crates/bevy_auto_plugin_proc_macros/docs_gen/.version" ]]; then
      continue
    fi
    rel="${file#crates/bevy_auto_plugin_proc_macros/docs_gen/}"
    src_file="$repo_root/$rel"
    gen_file="$repo_root/$file"
    if [[ ! -f "$src_file" ]]; then
      echo "docs_gen file has no matching source: $gen_file (expected $src_file)" >&2
      exit 1
    fi
    if ! cmp -s "$src_file" "$gen_file"; then
      echo "docs_gen file differs from source: $gen_file" >&2
      exit 1
    fi
  done <<< "$dirty_files"
  allow_dirty_arg="--allow-dirty"
fi

if [[ "$dry_run_requested" == "true" ]]; then
  echo "Running: cargo publish --dry-run ${allow_dirty_arg} $*"
  (cwd="$repo_root"; cd "$cwd"; cargo publish --dry-run ${allow_dirty_arg} "$@")
  if [[ -n "$dirty_files" ]]; then
    echo "Dirty state: safe (docs_gen matches docs)"
  else
    echo "Dirty state: clean"
  fi
  exit 0
fi

echo "Running: cargo publish --dry-run ${allow_dirty_arg} $*"
(cwd="$repo_root"; cd "$cwd"; cargo publish --dry-run ${allow_dirty_arg} "$@")

echo "Running: cargo publish ${allow_dirty_arg} $*"
(cwd="$repo_root"; cd "$cwd"; cargo publish ${allow_dirty_arg} "$@")
