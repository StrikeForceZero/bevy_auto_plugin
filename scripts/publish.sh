#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
proc_macro_dir="$repo_root/crates/bevy_auto_plugin_proc_macros"
docs_src="$repo_root/docs"
docs_gen="$proc_macro_dir/docs_gen"

bash "$repo_root/scripts/docs.sh"

dry_run_requested="false"
filtered_args=()
for arg in "$@"; do
  if [[ "$arg" == "--dry-run" ]]; then
    dry_run_requested="true"
    continue
  fi
  if [[ "$arg" == "--workspace" ]]; then
    continue
  fi
  filtered_args+=("$arg")
done
publish_args=(--workspace "${filtered_args[@]}")

dirty_files=$(git -C "$repo_root" status --porcelain=v1 | awk '{print $2}')
allow_dirty_args=()
if [[ -n "$dirty_files" ]]; then
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if [[ "$file" != crates/bevy_auto_plugin_proc_macros/docs_gen/* ]]; then
      echo "Refusing to publish with dirty file outside docs_gen: $file" >&2
      exit 1
    fi
  done <<< "$dirty_files"
fi

docs_gen_root="$docs_gen"
if [[ -d "$docs_gen/docs" ]]; then
  docs_gen_root="$docs_gen/docs"
fi

if [[ -d "$docs_gen_root" ]]; then
  while IFS= read -r gen_file; do
    if [[ "$(basename "$gen_file")" == ".version" ]]; then
      continue
    fi
    rel="${gen_file#"$docs_gen_root/"}"
    src_file="$docs_src/$rel"
    if [[ ! -f "$src_file" ]]; then
      echo "docs_gen file has no matching source: $gen_file (expected $src_file)" >&2
      exit 1
    fi
    if ! cmp -s "$src_file" "$gen_file"; then
      echo "docs_gen file differs from source: $gen_file" >&2
      exit 1
    fi
  done < <(find "$docs_gen_root" -type f -print)
  allow_dirty_args=(--allow-dirty)
fi

if [[ "$dry_run_requested" == "true" ]]; then
  echo "Running: cargo +stable publish --dry-run ${allow_dirty_args[*]} ${publish_args[*]}"
  (cwd="$repo_root"; cd "$cwd"; cargo +stable publish --dry-run "${allow_dirty_args[@]}" "${publish_args[@]}")
  if [[ -n "$dirty_files" ]]; then
    echo "Dirty state: safe (docs_gen matches docs)"
  else
    echo "Dirty state: clean"
  fi
  exit 0
fi

echo "Running: cargo +stable publish --dry-run ${allow_dirty_args[*]} ${publish_args[*]}"
(cwd="$repo_root"; cd "$cwd"; cargo +stable publish --dry-run "${allow_dirty_args[@]}" "${publish_args[@]}")

echo "Running: cargo +stable publish ${allow_dirty_args[*]} ${publish_args[*]}"
(cwd="$repo_root"; cd "$cwd"; cargo +stable publish "${allow_dirty_args[@]}" "${publish_args[@]}")
