#!/usr/bin/env bash

set -euo pipefail

crate_version="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')"
expected_tag="v${crate_version}"

if [ "${RELEASE_TAG}" != "${expected_tag}" ]; then
  echo "Release tag ${RELEASE_TAG} does not match Cargo.toml package.version ${crate_version}." >&2
  echo "Expected release tag: ${expected_tag}" >&2
  exit 1
fi
