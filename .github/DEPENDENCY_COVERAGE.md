# Resolved dependency inventory

The committed `Cargo.lock` records the development/CLI resolution, including optional dependencies, for GitHub's dependency graph. It does not constrain downstream library consumers or change `Cargo.toml` requirements.

CI runs locked tests separately for default TLS, rustls TLS, and each TLS variant with the CLI enabled. A single all-features run does not validate these alternatives.

Refresh with `cargo update`, review the lock diff, and run the four CI feature combinations before committing. After merge, verify concrete Cargo versions and transitive packages in GitHub. The lock describes the selected development resolution, not every downstream version or target platform.

Rollback: revert the lock, ignore-rule change, and CI matrix together. Existing Dependabot policy stays unchanged.
