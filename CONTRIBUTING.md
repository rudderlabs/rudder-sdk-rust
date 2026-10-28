# Contributing to RudderStack Rust SDK

Thanks for taking the time and for your help improving this project!

## Table of Contents

- [RudderStack Contributor Agreement](#rudderstack-contributor-agreement)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Committing](#committing)
- [Getting Help](#getting-help)

## RudderStack Contributor Agreement

To contribute to this project, we need you to sign to [Contributor License Agreement (“CLA”)][cla] for the first commit you make. By agreeing to the [CLA][cla]
we can add you to list of approved contributors and review the changes proposed by you.

## Submitting a Pull Request

Do you have an improvement?

1. Submit an [issue][issue] describing your proposed change.
2. We will try to respond to your issue promptly.
3. Fork this repo, develop and test your code changes. See the project's [README](README.md) for further information about working in this repository.
4. Submit a pull request against this repo's `main` branch.
   - Include instructions on how to test your changes.
5. Your branch may be merged once all configured checks pass, including:
   - A review from appropriate maintainers

## Rust SDK releases

A published, stable `vX.Y.Z` GitHub release triggers **Publish to crates.io** once.
The workflow verifies the release tag, checks out that exact commit, compares
Cargo metadata with the tag, rechecks the tag before publishing, and publishes
`rudderanalytics` via crates.io OIDC. After publication it requests the **full**
central server-SDK E2E suite against that exact public registry version and the
verified release commit. The central validator owns registry propagation,
installation without local overrides, live locks and cleanup. Native Rust tests
(`cargo test --all-features`) run in CI; they are not a full source-candidate E2E.

A successful publish with failed or cancelled full acceptance is **published but
not accepted**. Check the exact central run linked in the acceptance job, resolve
the issue, and do not announce completion or unpublish automatically. To retry
validation **without republishing or creating a tag**, dispatch **Publish to
crates.io** from `master` with `version` (e.g. `2.0.1`, no `v`) and `commit`
(the full 40-character SHA of that version's existing published release tag).
The workflow rejects mismatched tags/versions/commits and other workflow refs;
there is no fallback to the latest package. The protected `release` environment
must permit both the published tag and the `master` retry. The release App must
have Actions write on `rudder-sdk-e2e-tests` for acceptance.

Published-package support is dependent on SDK-5961 (central Rust mode) and
SDK-5962 (reviewed shared-action `package-version` and release SHA inputs).
Until those are on main and the fixed action pin in `publish.yml` is replaced,
acceptance intentionally fails closed. This implementation has not been live
validated: first hosted acceptance is deferred to the next normal release.

## Committing

We prefer squash or rebase commits so that all changes from a branch are
committed to master as a single commit. All pull requests are squashed when
merged, but rebasing prior to merge gives you better control over the commit
message.

We look forward to your feedback on improving this project.

<!----variable's---->

[slack]: https://resources.rudderstack.com/join-rudderstack-slack
[issue]: https://github.com/rudderlabs/rudder-sdk-rust/issues/new
[cla]: https://rudderlabs.wufoo.com/forms/rudderlabs-contributor-license-agreement

## Getting Help

If you have a question about rudder or have encountered problems using it,
start by asking a question on [Slack][slack].
