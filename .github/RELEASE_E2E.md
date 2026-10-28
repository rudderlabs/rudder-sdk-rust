# Published Rust SDK acceptance

The published-package workflow runs the full central E2E suite for the exact stable release version. It uses the existing release App and waits for delivery validation and cleanup. The central workflow checks the release tag against the immutable event commit and `master` ancestry. It installs the exact public registry package and verifies native package metadata and integrity evidence.

Acceptance starts only after the existing publisher succeeds. Existing native build, security, and publication checks remain in place. The final notification posts an accepted root message only when exact public crate verification, full E2E delivery, and cleanup succeed. If upload succeeded but acceptance failed, was cancelled, or was skipped, it says "published; validation failed or incomplete". If the publication job failed, was cancelled, or was skipped for an applicable stable release, it reports publication as failed or unconfirmed and does not claim the crate is absent or accepted.

The root contains the SDK, version, UTC date, and team mentions; links and check results stay in a non-broadcast thread reply without unfurls. When the dispatcher returns a central run URL, the thread links to it; otherwise the Rust release run remains the diagnostic fallback. Missing `SLACK_BOT_TOKEN` or `SLACK_RELEASE_CHANNEL_ID` fails the notification job and is reported in the run summary; use the existing organization configuration rather than creating credentials.

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

## Rollout

Tracked in [SDK-5968](https://linear.app/rudderstack/issue/SDK-5968). Keep this PR draft until [central PR #32](https://github.com/rudderlabs/rudder-sdk-e2e-tests/pull/32), the shared action, and the [SDK-5977](https://linear.app/rudderstack/issue/SDK-5977) workflow are reviewed and available. The central cleanup-budget fix is committed as `52f6bf7`; [SDK-5978](https://linear.app/rudderstack/issue/SDK-5978) distinguishes ordinary trusted inheritance from additional published-package hardening, so inheritance alone is not a merge blocker.

The current Rust entry point still calls the composite action; it does not call a reusable shared workflow yet. As inspected, the shared action main and PR #5 contain only `test.yml` in `.github/workflows/`, while central published-package support is on PR #32 rather than main. Do not merge or use this dispatch on a release until the shared workflow is independently reviewed and available. Then replace the local token/dispatch steps with a reusable call pinned to its reviewed full commit SHA, pass only `RELEASE_PRIVATE_KEY`, and retain the `run-id`/`run-url` outputs and Rust-owned publisher/notification policy. Hosted acceptance remains pending the next normal release. Do not create a release solely to test this wiring.
