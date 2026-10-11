# Musializer-RS Release Notes

## v1.0.1 — AGPL-3.0-or-later License Update

### Summary

This release updates Musializer-RS from GPL-3.0-or-later to AGPL-3.0-or-later.

The AGPL license is appropriate for this project because it includes networked interactions and remote deployment scenarios. The Affero clause ensures that any modified version of the software that is used over a network must also provide the corresponding source code to the users interacting with it.

### What changed

- Switched the project license from GPL-3.0-or-later to AGPL-3.0-or-later.
- Updated the root `LICENSE` file to the AGPL v3 text.
- Updated crate metadata in:
  - `crates/musializer-core/Cargo.toml`
  - `crates/musializer-desktop/Cargo.toml`
- Updated project documentation and release-facing metadata in:
  - `README.md`
  - `mobile/README.md`
  - `docs/PACKAGE_DISTRIBUTION_GUIDE.md`
  - `distribution/winget/GabinRoy.MusializerRS.yaml`

### Why this matters

Musializer-RS is a desktop and mobile application ecosystem with remote distribution and networked service patterns. The AGPL license provides stronger copyleft protections for software that may be made available through network access, while remaining fully compatible with the project’s open-source distribution goals.

### Distribution and packaging impact

For maintainers publishing binaries, packages, and installer artifacts:

- Include the `LICENSE` file in distribution artifacts.
- Keep the AGPL notice visible in release notes and package metadata.
- Ensure release documentation clearly states the license change for end users and packagers.

### Upgrade guidance for users

If you are redistributing or modifying Musializer-RS, ensure you comply with the AGPL terms, especially when providing access to modified builds over a network.

### Compatibility note

This change affects licensing only. It does not change the application features, build pipeline, or runtime behavior.

---

### Changelog highlights

- AGPLv3 licensing alignment
- Release metadata refresh
- Distribution documentation update
- Clearer licensing guidance for upstream/downstream users

