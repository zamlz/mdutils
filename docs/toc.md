Command: `toc` (Table-of-Contents Generation)
=============================================

<!-- md-toc: -->
<!-- md-toc: end -->


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

**Features:**
- **GitHub-style slugs**: Links use the same anchor format as GitHub
- **Automatic indentation**: H2 sections indented, H3 further indented, etc.
- **Duplicate handling**: Headers with the same text get unique slugs (e.g., `#section`, `#section-1`, `#section-2`)
- **Update support**: Re-running replaces the old TOC with updated content
- **Smart parsing**: Only includes headers after the TOC marker (prevents self-reference)

**Updating an existing TOC:**

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
