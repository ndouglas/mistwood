#!/usr/bin/env bash

function die() {
  >&2 echo "$1";
  exit 1;
}

# This script is used to build the documentation for the project.
#
# There are currently two sources of documentation:
# - `mdbook` (source in docs/src, output in docs/book)
# - `rustdoc` (source in src, output in target/doc)

base_dir="$(git rev-parse --show-toplevel)";
pushd "${base_dir}" > /dev/null; # cwd -> base_dir
cargo doc \
  --all-features \
  --no-deps \
  --document-private-items \
  --target-dir docs/src/rustdoc || die "Failed to build rustdoc";
pushd docs > /dev/null; # base_dir -> base_dir/docs
mdbook build . || die "Failed to build mdbook";
find . -type f -not \( -perm 644 -o -perm 755 \) -exec chmod 755 {} \;
popd > /dev/null; # base_dir/docs -> base_dir
popd > /dev/null; # base_dir -> cwd
