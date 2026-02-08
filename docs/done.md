Command: `done` (Mark Checklist Items as Done)
=============================================

<!-- md-toc: -->
- [Marking Tasks as Done](#marking-tasks-as-done)
- [Features](#features)
- [Examples](#examples)
  - [Single Task](#single-task)
  - [Multiple Tasks](#multiple-tasks)
  - [Nested Tasks](#nested-tasks)
  - [Mixed Content](#mixed-content)
- [Idempotency](#idempotency)
- [Troubleshooting](#troubleshooting)
  - [Task not being marked](#task-not-being-marked)
  - [Already checked items not changing](#already-checked-items-not-changing)
  - [Code block content being modified](#code-block-content-being-modified)
  - [Timestamp format](#timestamp-format)
<!-- md-toc: end -->

## Marking Tasks as Done

The `done` subcommand marks open markdown checklist items as completed with strikethrough and a timestamp.

**How it works:**
- Open items (`- [ ]`) are transformed to checked items with strikethrough and timestamp
- Already checked items (`- [x]`) are left unchanged
- Already strikethrough items are left unchanged (idempotent)
- Non-checklist lines pass through unchanged
- Content inside code blocks is not modified

**Transformation:**
```
- [ ] task → - [x] ~~task~~ `COMPLETED: YYYY-MM-DD HH:MM:SS`
```

## Features

- **Strikethrough text**: Task text is wrapped in `~~strikethrough~~`
- **Completion timestamp**: Human-readable format `YYYY-MM-DD HH:MM:SS`
- **Preserves indentation**: Nested items maintain their indentation level
- **Idempotent**: Running multiple times produces the same result
- **Code block aware**: Items inside code fences are not modified
- **Selective processing**: Only open (`- [ ]`) items are transformed

## Examples

### Single Task

Input:
~~~markdown
- [ ] Buy groceries
~~~
<!-- md-code: id="simple-example"; bin="md done"; syntax="markdown" -->

Output:
~~~markdown
- [x] ~~Buy groceries~~ `COMPLETED: 2026-02-07 18:09:01`
~~~
<!-- md-code-output: id="simple-example" -->

### Multiple Tasks

Input:
~~~markdown
- [ ] First task
- [ ] Second task
- [x] Already done manually
- [ ] Third task
~~~
<!-- md-code: id="multiple-example"; bin="md done"; syntax="markdown" -->

Output:
~~~markdown
- [x] ~~First task~~ `COMPLETED: 2026-02-07 18:09:01`
- [x] ~~Second task~~ `COMPLETED: 2026-02-07 18:09:01`
- [x] Already done manually
- [x] ~~Third task~~ `COMPLETED: 2026-02-07 18:09:01`
~~~
<!-- md-code-output: id="multiple-example" -->

Note: The already checked item (`- [x] Already done manually`) is left unchanged.

### Nested Tasks

Input:
~~~markdown
- [ ] Parent task
  - [ ] Child task 1
  - [ ] Child task 2
    - [ ] Grandchild task
~~~
<!-- md-code: id="nested-example"; bin="md done"; syntax="markdown" -->

Output:
~~~markdown
- [x] ~~Parent task~~ `COMPLETED: 2026-02-07 18:09:01`
  - [x] ~~Child task 1~~ `COMPLETED: 2026-02-07 18:09:01`
  - [x] ~~Child task 2~~ `COMPLETED: 2026-02-07 18:09:01`
    - [x] ~~Grandchild task~~ `COMPLETED: 2026-02-07 18:09:01`
~~~
<!-- md-code-output: id="nested-example" -->

### Mixed Content

Input:
~~~markdown
# Project Tasks

- [ ] Complete feature
- Regular bullet point
- [x] Already done

Some paragraph text.

```python
# This is a code block
- [ ] This is NOT a task
```
~~~
<!-- md-code: id="mixed-example"; bin="md done"; syntax="markdown" -->

Output:
~~~markdown
# Project Tasks

- [x] ~~Complete feature~~ `COMPLETED: 2026-02-07 18:09:01`
- Regular bullet point
- [x] Already done

Some paragraph text.

```python
# This is a code block
- [ ] This is NOT a task
```
~~~
<!-- md-code-output: id="mixed-example" -->

Note: The checklist-like text inside the code block is not modified.

## Idempotency

Running `md done` multiple times on the same content produces the same result:

```
md done(output) == output
```

This works because:
1. Already strikethrough items (containing `~~`) are left unchanged
2. The timestamp is only added once during the first transformation

## Troubleshooting

### Task not being marked

**Problem:** Running `md done` doesn't mark a task as done.

**Common causes:**

1. **Already checked item**
   - Items with `- [x]` or `- [X]` are left unchanged
   - Only open items `- [ ]` are transformed

2. **Not a valid checklist format**
   - Must be exactly `- [ ]` with a space after the dash
   - Invalid: `-[ ]`, `* [ ]`, `+ [ ]`
   - Valid: `- [ ]` ✓

3. **Inside a code block**
   - Items inside ``` or ~~~ fenced code blocks are not processed
   - This is intentional to avoid modifying code examples

### Already checked items not changing

**This is expected behavior.** The `done` command only processes open tasks (`- [ ]`).

If you want to add strikethrough to an already checked item, you would need to:
1. Uncheck it first: change `- [x]` to `- [ ]`
2. Run `md done`

### Code block content being modified

If content inside code blocks is being modified unexpectedly:

**Check:**
- Are your code fences properly closed?
- Are you using standard fences (``` or ~~~)?
lear Is the fence on its own line?

### Timestamp format

The timestamp format is fixed as `YYYY-MM-DD HH:MM:SS` using local time.

**Example:** `2024-01-15 14:30:00`

This format is:
- Human-readable
- Sortable
- Machine-parseable (ISO 8601 compatible)
