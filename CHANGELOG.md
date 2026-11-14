# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.1 - 2026-11-14

### Changed

- CI: daily automated update of flake inputs and cargo dependencies via pull request.
- CI: `cargo-deny` checks (advisories, licenses, bans, sources) on every pull request,
  plus a scheduled advisory audit.
- Stricter linting: `unsafe_code` is forbidden and additional clippy lints are enabled.

### Fixed

- `md toc`: heading anchors now match GitHub's slug rule. Consecutive hyphens are no
  longer collapsed and leading/trailing hyphens are no longer trimmed, so
  `## Media & Apps` is linked as `#media--apps` instead of `#media-apps` (#1).
  Re-run `md toc` on existing documents to refresh links to headings containing
  punctuation between words.

## 0.1.0

Initial release.
