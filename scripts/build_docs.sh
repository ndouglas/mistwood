#!/usr/bin/env bash

# This script is used to build the documentation for the project.
#
# There are currently two sources of documentation:
# - `mdbook` (source in docs/src, output in docs/book)
# - `rustdoc` (source in src, output in target/doc)

base_dir="$(git rev-parse --show-toplevel)";
pushd "${base_dir}" > /dev/null; # cwd -> base_dir
cargo +nightly doc \
  --all-features \
  --no-deps \
  --document-private-items \
  --target-dir docs/src/rustdoc;
pushd docs > /dev/null; # base_dir -> base_dir/docs
mdbook build .;
popd > /dev/null; # base_dir/docs -> base_dir
popd > /dev/null; # base_dir -> cwd

# cd $BASE_DIR
# cargo +nightly doc --all-features
# (cd guide
#     (cd src && sed -e "s#{ROOT_PATH}#${1-/}#g" links.md.template > links.md)
#     mdbook build
#     mkdir -p book/{assets,rustdoc}
#     cp -r assets/* book/assets/
#     cp -r ../target/doc/* book/rustdoc/
# )