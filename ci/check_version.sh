#!/bin/sh
# Verify the crate version in Cargo.toml matches the release tag.
#
# Usage: ci/check_version.sh <tag>
#   <tag> may be "v0.1.0" or "0.1.0"; a leading "v" is stripped.
set -eu

TAG="${1#v}"

# First `version = "..."` line in the [package] table.
CRATE_VERSION="$(
  awk -F'"' '/^\[/{p=0} /^\[package\]/{p=1} p && /^version[[:space:]]*=/{print $2; exit}' \
    t32mcp-rs/Cargo.toml
)"

if [ -z "$CRATE_VERSION" ]; then
  echo "ERROR: could not read version from t32mcp-rs/Cargo.toml" >&2
  exit 1
fi

if [ "$TAG" != "$CRATE_VERSION" ]; then
  echo "ERROR: version mismatch: tag=$TAG, Cargo.toml=$CRATE_VERSION" >&2
  echo "Bump 'version' in t32mcp-rs/Cargo.toml to $TAG before tagging." >&2
  exit 1
fi

echo "Version OK: $CRATE_VERSION matches tag."
