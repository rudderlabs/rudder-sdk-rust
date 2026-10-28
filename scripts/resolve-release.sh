#!/usr/bin/env bash
# Resolve a published release to its immutable commit; run without release credentials.
set -euo pipefail

if [[ "$GITHUB_EVENT_NAME" == release ]]; then
  [[ "$GITHUB_REF" == "refs/tags/$RELEASE_TAG" && "$RELEASE_TAG" == v* && "$RELEASE_PRERELEASE" == false && "$RELEASE_DRAFT" == false ]] || {
    echo 'Expected a published, non-prerelease tag event.' >&2; exit 1;
  }
  tag="$RELEASE_TAG"
elif [[ "$GITHUB_EVENT_NAME" == workflow_dispatch && "$GITHUB_REF" == refs/heads/master ]]; then
  [[ "$INPUT_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && "$INPUT_SHA" =~ ^[0-9a-f]{40}$ ]] || {
    echo 'Retry requires an exact stable version and lowercase commit SHA.' >&2; exit 1;
  }
  tag="v$INPUT_VERSION"
else
  echo 'Only a published release or a manual run from master is allowed.' >&2
  exit 1
fi

[[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo 'Invalid release tag.' >&2; exit 1; }
release="$(curl -fsS --retry 2 --max-time 30 \
  -H 'Accept: application/vnd.github+json' \
  -H "Authorization: Bearer $GH_TOKEN" \
  "https://api.github.com/repos/$GITHUB_REPOSITORY/releases/tags/$tag")"
[[ "$(jq -r '.tag_name' <<< "$release")" == "$tag" &&
   "$(jq -r '.draft' <<< "$release")" == false &&
   "$(jq -r '.prerelease' <<< "$release")" == false ]] || {
  echo 'Tag is not an existing published stable release.' >&2; exit 1;
}

# ^{} dereferences annotated tags; for lightweight tags use the direct ref.
remote="$(git ls-remote "https://github.com/$GITHUB_REPOSITORY.git" "refs/tags/$tag" "refs/tags/$tag^{}")"
sha="$(awk -v ref="refs/tags/$tag^{}" '$2 == ref { print $1 }' <<< "$remote")"
if [[ -z "$sha" ]]; then
  sha="$(awk -v ref="refs/tags/$tag" '$2 == ref { print $1 }' <<< "$remote")"
fi
[[ "$sha" =~ ^[0-9a-f]{40}$ ]] || { echo 'Release tag has no commit.' >&2; exit 1; }
if [[ "$GITHUB_EVENT_NAME" == release && "$sha" != "$RELEASE_EVENT_SHA" ]]; then
  echo 'Release tag no longer points to the event commit.' >&2; exit 1
fi
if [[ "$GITHUB_EVENT_NAME" == workflow_dispatch && "$sha" != "$INPUT_SHA" ]]; then
  echo 'Retry SHA does not match the published release tag.' >&2; exit 1
fi

{
  echo "tag=$tag"
  echo "version=${tag#v}"
  echo "sha=$sha"
} >> "$GITHUB_OUTPUT"
