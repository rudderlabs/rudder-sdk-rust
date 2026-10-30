# Published Rust SDK acceptance

The published-package workflow runs the full central E2E suite for the exact stable release version. It uses the existing release App and waits for delivery validation and cleanup. The central workflow checks the release tag against the immutable event commit and `master` ancestry. It installs the exact public registry package and verifies native package metadata and integrity evidence.

Acceptance starts only after the existing publisher succeeds. Existing native build, security, and publication checks remain in place. The final notification posts an accepted root message only when exact public crate verification, full E2E delivery, and cleanup succeed. If upload succeeded but acceptance failed, was cancelled, or was skipped, it says "published; validation failed or incomplete". If the publication job failed, was cancelled, or was skipped for an applicable stable release, it reports publication as failed or unconfirmed and does not claim the crate is absent or accepted.

The details thread includes the exact central run link when dispatch returned one; otherwise the release run remains the diagnostic fallback. The root contains the SDK, version, UTC date, and team mentions; links and check results stay in a non-broadcast thread reply without unfurls. Missing `SLACK_BOT_TOKEN` or `SLACK_RELEASE_CHANNEL_ID` fails the notification job and is reported in the run summary; use the existing organization configuration rather than creating credentials.

Full source-candidate E2E before publication is a separate rollout. Do not treat native tests or this post-publication check as that gate.

## Validation-only retry

1. Open the failed central run from the shared action summary.
2. Resolve the validation failure.
3. Run this workflow at the existing release tag:

```sh
gh workflow run published-sdk-e2e.yml --repo rudderlabs/rudder-sdk-rust --ref vX.Y.Z
```

The retry has no upload path or publishing credential. It derives the exact version and commit from the selected tag. It does not use `latest`, publish a package, or recreate a tag. Tags predating this workflow need an exact-version dispatch through the central workflow. Stable `X.Y.Z` versions are supported; prerelease acceptance is not implemented.

A published package remains published if acceptance fails. Do not rerun a successful upload job to retry E2E. A successful validation-only retry does not change an earlier failed run or its notification. Record the new acceptance run in the release ticket and existing Slack release thread.

## Retry after a workflow fix

The retry workflow must exist on the default branch and selected release tag.
A tag retry loads workflow code and pins stored at that tag. A fix merged later
to master does not change that retry. For workflow fixes or older tags without
the retry workflow, use the current central workflow. Copy the full commit SHA
from the original release run. Replace the version, commit placeholder and request
ID before running:

```sh
gh workflow run server-sdk-validation.yml --repo rudderlabs/rudder-sdk-e2e-tests --ref main \
  -f selection=auto -f source_repository=rudderlabs/rudder-sdk-rust \
  -f source_sha=RELEASE_EVENT_COMMIT_SHA -f package_version=X.Y.Z \
  -f suite=full -f request_id=rust-published-X_Y_Z-retry-UNIQUE_ID
```

The central fallback cannot publish or notify. Release-run reruns also stay silent
after the first attempt. Record new acceptance evidence in the release ticket and
existing release thread. If notification fails, repair the existing thread separately.

## Rollout

The caller pins shared-action #14 at
`2d1262f86e519195dc723a8d244a8ecc8bcedb21`. The verifier requires full live
delivery and verified deletion of the run's source, destination, and connection.

Coordinate activation with central #46/#47 and the other callers under
[SDK-6016](https://linear.app/rudderstack/issue/SDK-6016). The central lifecycle
must be active before this caller runs. No legacy result fallback is provided.
Native publication and registry checks remain unchanged. Full release-PR
candidate support for this SDK is separate work.

Hosted published-package acceptance remains pending the next normal release
under [SDK-5797](https://linear.app/rudderstack/issue/SDK-5797).
Do not create a release solely to test this wiring.
