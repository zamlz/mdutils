#!/usr/bin/env python3
"""Checks that the docs render the way they are written.

A misplaced code fence can swallow the rest of a page into one giant code block
while the source stays perfectly valid (and idempotent). Each doc is rendered
with cmark-gfm, GitHub's markdown parser, and we check that:

  * a document with a TOC renders as many headings as the TOC has entries
  * no fence markers (``` or ~~~) show up as visible prose
  * no table rows, headings or checklist items show up as raw prose

Needs cmark-gfm on PATH. Run from the repository root.
"""
import glob
import html
import re
import subprocess
import sys

TOC_START = "<!-- md-toc: -->"
TOC_END = "<!-- md-toc: end -->"


def render(text):
    return subprocess.run(
        ["cmark-gfm", "-e", "table", "-e", "strikethrough", "-e", "autolink", "-e", "tasklist"],
        input=text, capture_output=True, text=True, check=True,
    ).stdout


def count_headings(rendered):
    return len(re.findall(r"<h[1-6][ >]", rendered))


def prose_lines(rendered):
    """The visible text of a rendered page, minus code blocks and code spans."""
    rendered = re.sub(r"<pre.*?</pre>", "", rendered, flags=re.S)
    rendered = re.sub(r"<code>.*?</code>", "", rendered, flags=re.S)
    text = html.unescape(re.sub(r"<[^>]+>", "\n", rendered))
    return [line.strip() for line in text.split("\n") if line.strip()]


def toc_marker_line(lines):
    """Index of the first TOC marker that is not inside a code fence."""
    fence = None
    for i, line in enumerate(lines):
        m = re.match(r" {0,3}(`{3,}|~{3,})(.*)$", line)
        if m:
            if fence is None:
                fence = m.group(1)
            elif m.group(1)[0] == fence[0] and len(m.group(1)) >= len(fence) and not m.group(2).strip():
                fence = None
            continue
        if fence is None and line.strip() == TOC_START:
            return i
    return None


def check(path):
    problems = []
    text = open(path, encoding="utf-8").read()
    lines = text.split("\n")
    rendered = render(text)

    start = toc_marker_line(lines)
    if start is not None:
        end = next((i for i in range(start, len(lines)) if lines[i].strip() == TOC_END), None)
        if end is None:
            problems.append("the TOC has no end marker")
        else:
            entries = sum(1 for line in lines[start + 1:end] if re.match(r"\s*- \[", line))
            before = count_headings(render("\n".join(lines[:start])))
            after = count_headings(rendered) - before
            if entries != after:
                problems.append(
                    f"the TOC has {entries} entries but only {after} headings render after it "
                    "(a code fence is probably swallowing the rest of the page)"
                )

    for line in prose_lines(rendered):
        if "```" in line or "~~~" in line:
            problems.append(f"fence marker in rendered text: {line[:80]!r}")
        elif re.match(r"^\|.*\|$", line):
            problems.append(f"raw table row in rendered text: {line[:80]!r}")
        elif re.match(r"^#{1,6} \S", line):
            problems.append(f"raw heading in rendered text: {line[:80]!r}")
        elif re.match(r"^[-*] \[[ xX]\]", line):
            problems.append(f"raw checklist item in rendered text: {line[:80]!r}")
    return problems


def main():
    failed = False
    files = ["README.md"] + sorted(glob.glob("docs/*.md") + glob.glob("examples/*.md"))
    for path in files:
        for problem in check(path):
            print(f"FAIL: {path}: {problem}")
            failed = True
    if failed:
        print("docs do not render as written: fix the markdown structure", file=sys.stderr)
        return 1
    print(f"ok: {len(files)} docs render as written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
