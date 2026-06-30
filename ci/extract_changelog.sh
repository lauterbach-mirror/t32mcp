#!/bin/sh
# Extract the release notes for a single version out of CHANGELOG.md.
#
# Usage: ci/extract_changelog.sh <tag>
#   <tag> may be "v0.1.0" or "0.1.0"; a leading "v" is stripped.
#
# Prints every line below the matching "## [<version>]" heading up to (but not
# including) the next "## [" heading. Used by the release job to feed
# release-cli's `description:` field.
set -eu

VERSION="${1#v}"

NOTES="$(
  awk -v ver="$VERSION" '
    /^## \[/ {
      if (found) { exit }
      if (index($0, "[" ver "]")) { found = 1; next }
    }
    found { print }
  ' CHANGELOG.md
)"

# Trim leading and trailing blank lines. Uses awk (not GNU-only sed loops) so it
# also works with BusyBox in the alpine-based release jobs.
NOTES="$(printf '%s\n' "$NOTES" | awk '
  NF { started = 1 }
  started { buf[++n] = $0; if (NF) last = n }
  END { for (i = 1; i <= last; i++) print buf[i] }
')"

if [ -z "$NOTES" ]; then
  printf 'ERROR: no CHANGELOG.md section found for version %s.\n' "$VERSION" >&2
  printf 'Add a "## [%s] - <date>" section before tagging.\n' "$VERSION" >&2
  exit 1
fi

printf '%s\n' "$NOTES"
