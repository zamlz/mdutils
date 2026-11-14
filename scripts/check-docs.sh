#!/usr/bin/env bash
# Checks that the docs are idempotent: running `md toc`, `md table` and
# `md code` over every doc must reproduce the committed file exactly.
#
# Uses the `md` found on PATH (the code examples call `md` themselves), e.g.
#   PATH="$PWD/result/bin:$PATH" scripts/check-docs.sh
set -u

if ! command -v md >/dev/null; then
  echo "error: md not found on PATH" >&2
  exit 2
fi

# The `md done` examples embed the current time, so those timestamps can never
# match a committed copy; mask them on both sides before comparing.
normalize() {
  sed -E 's/COMPLETED: [0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2}/COMPLETED: <timestamp>/g'
}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

failed=0
checked=0
for file in README.md docs/*.md examples/*.md; do
  for cmd in toc table code; do
    checked=$((checked + 1))
    if ! md "$cmd" < "$file" > "$tmp/out" 2> "$tmp/err"; then
      echo "FAIL: md $cmd < $file exited with an error"
      sed 's/^/  /' "$tmp/err"
      failed=1
    elif ! diff -u --label "$file" --label "md $cmd < $file" \
        <(normalize < "$file") <(normalize < "$tmp/out") > "$tmp/diff"; then
      echo "FAIL: md $cmd changes $file"
      cat "$tmp/diff"
      failed=1
    fi
  done
done

if [ "$failed" -ne 0 ]; then
  # Don't paper over a diff by committing md's output: docs must be written so
  # that md leaves them unchanged, so a diff means the doc or md needs fixing.
  echo "docs are not idempotent: fix the doc, or md itself if its output is wrong, until md leaves every doc unchanged" >&2
  exit 1
fi
echo "ok: $checked checks, docs are idempotent"
