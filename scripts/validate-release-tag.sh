#!/usr/bin/env bash

set -euo pipefail

if [[ ! "${RELEASE_TAG}" =~ ^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]]; then
  echo "Release tag ${RELEASE_TAG} must be a stable vX.Y.Z version." >&2
  exit 1
fi

crate_version="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')"
expected_tag="v${crate_version}"

if [ "${RELEASE_TAG}" != "${expected_tag}" ]; then
  echo "Release tag ${RELEASE_TAG} does not match Cargo.toml package.version ${crate_version}." >&2
  echo "Expected release tag: ${expected_tag}" >&2
  exit 1
fi
