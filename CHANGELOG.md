# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.1 - 2026-11-21

### Changed

- Replaced the unmaintained `chrono` crate (RUSTSEC-2026-0543) with `jiff` for the
  timestamp written by `md done`. Output format is unchanged (#9).
- Updated `jiff` to 1.0 (#10).
- Updated flake inputs and cargo dependencies (#8).

## 0.2.0 - 2026-11-14

### Added

- `md code`: render a block's delimited output as a markdown table, like Org-mode
  `#+RESULTS:` tables. New directive parameters `table="tsv"|"csv"|"pipe"`,
  `delimiter`, `columns` and `header`. Without them behaviour is unchanged (#3).

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
