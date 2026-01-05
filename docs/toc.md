Command: `toc` (Table-of-Contents Generation)
=============================================

<!-- md-toc: -->
- [Creating a TOC](#creating-a-toc)
- [Features](#features)
- [Updating an existing TOC](#updating-an-existing-toc)
- [Troubleshooting](#troubleshooting)
  - [TOC not generating](#toc-not-generating)
  - [TOC not updating](#toc-not-updating)
  - [Duplicate header anchors](#duplicate-header-anchors)
  - [Special characters in headers](#special-characters-in-headers)
  - [Wrong indentation levels](#wrong-indentation-levels)
  - [Headers in code blocks showing in TOC](#headers-in-code-blocks-showing-in-toc)
  - [Multiple TOC markers](#multiple-toc-markers)
  - [Links not working](#links-not-working)
<!-- md-toc: end -->

## Creating a TOC

The `toc` subcommand automatically generates or updates a table of contents from markdown headers.

**How it works:**
- Add a `<!-- md-toc: -->` marker where you want the TOC
- The command scans all headers in the document
- Generates clickable links with proper indentation
- Adds `<!-- md-toc: end -->` after the TOC
- Re-running updates the existing TOC

**Example:**

Input:
```markdown
# My Documentation
<!-- md-toc: -->

## Introduction
### Background
### Goals

## Implementation
### Architecture
### Testing

## Conclusion
```
<!-- md-code: id="toc-example"; bin="md toc"; syntax="markdown" -->

Output:
```markdown
# My Documentation
<!-- md-toc: -->
- [Introduction](#introduction)
  - [Background](#background)
  - [Goals](#goals)
- [Implementation](#implementation)
  - [Architecture](#architecture)
  - [Testing](#testing)
- [Conclusion](#conclusion)
<!-- md-toc: end -->

## Introduction
### Background
### Goals

## Implementation
### Architecture
### Testing

## Conclusion

```
<!-- md-code-output: id="toc-example" -->

## Features

- **GitHub-style slugs**: Links use the same anchor format as GitHub
- **Automatic indentation**: H2 sections indented, H3 further indented, etc.
- **Duplicate handling**: Headers with the same text get unique slugs (e.g., `#section`, `#section-1`, `#section-2`)
- **Update support**: Re-running replaces the old TOC with updated content
- **Smart parsing**: Only includes headers after the TOC marker (prevents self-reference)

## Updating an existing TOC

```markdown
# Document
<!-- md-toc: -->
- [Old Section](#old-section)
<!-- md-toc: end -->

## New Section 1
## New Section 2
```
<!-- md-code: id="toc-update-example"; bin="md toc"; syntax="markdown" -->

After running `md toc`:
```markdown
# Document
<!-- md-toc: -->
- [New Section 1](#new-section-1)
- [New Section 2](#new-section-2)
<!-- md-toc: end -->

## New Section 1
## New Section 2

```
<!-- md-code-output: id="toc-update-example" -->

**Note:** If no `<!-- md-toc: -->` marker is found, the document is returned unchanged.

## Troubleshooting

### TOC not generating

**Problem:** Running `md toc` doesn't create or update a table of contents.

**Common causes:**

1. **Missing TOC marker**
   - Make sure you have `<!-- md-toc: -->` in your document
   - The marker must be on its own line
   - Check for typos in the marker

2. **No headers after marker**
   - Headers must appear AFTER the TOC marker
   - Headers before the marker are ignored (prevents self-reference)
   - Make sure you have at least one header (H1-H6) after the marker

3. **Incorrect header syntax**
   - Use `#` for headers, not underline style
   - Valid: `## Section` ✓
   - Invalid: `Section\n=======` ✗ (not supported yet)

**Example of correct placement:**
```markdown
# My Document
<!-- md-toc: -->

## Section 1
## Section 2
```

### TOC not updating

If your TOC exists but isn't updating when you run `md toc`:

**Check:**
1. Are you writing output to a file? `md toc < input.md > output.md`
2. Are you overwriting the same file? Consider: `md toc < input.md | sponge input.md`
3. Is the end marker present? Look for `<!-- md-toc: end -->`

### Duplicate header anchors

When you have multiple headers with the same text, GitHub-style slugs append numbers:

```markdown
## Setup → #setup
## Setup → #setup-1  
## Setup → #setup-2
```

This is intentional behavior to ensure all links are unique.

### Special characters in headers

Headers with special characters are converted to GitHub-style slugs:

- Spaces → hyphens: `My Header` → `#my-header`
- Punctuation removed: `What? How!` → `#what-how`
- Numbers preserved: `Section 1` → `#section-1`
- Underscores preserved: `my_header` → `#my_header`

### Wrong indentation levels

If your TOC indentation looks wrong:

**Check:**
- H1 headers have no indentation
- H2 headers have 2 spaces (`  `)
- H3 headers have 4 spaces (`    `)
- Each level adds 2 more spaces

**Example:**
```markdown
- [H1 Item](#h1-item)
  - [H2 Item](#h2-item)
    - [H3 Item](#h3-item)
```

### Headers in code blocks showing in TOC

Currently, headers inside code blocks ARE included in the TOC. This is a known limitation.

**Workaround:**
- Use indentation instead of code fences for small code samples
- Or use `#` without a space in code blocks: `#Not a header`

### Multiple TOC markers

If you have multiple `<!-- md-toc: -->` markers in one document:
- Each TOC only includes headers AFTER its own marker
- Each TOC is independent
- Useful for per-section TOCs in long documents

### Links not working

If TOC links don't navigate correctly:

**Causes:**
- Your markdown viewer might not support anchor links
- The slug generation might not match your viewer's implementation
- Try clicking directly on a header to see the URL format used

**Note:** This tool uses GitHub-style slugs, which work on:
- GitHub
- GitLab
- Most static site generators (Hugo, Jekyll, etc.)
- VS Code markdown preview
