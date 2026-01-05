Command: `code` (Code Execution)
================================

<!-- md-toc: -->
- [Executing Code](#executing-code)
- [Supported Directive parameters](#supported-directive-parameters)
- [Examples](#examples)
  - [Python code execution](#python-code-execution)
  - [With custom timeout](#with-custom-timeout)
  - [Bash script](#bash-script)
  - [Command with arguments](#command-with-arguments)
  - [Custom fence for output](#custom-fence-for-output)
  - [Custom syntax highlighting](#custom-syntax-highlighting)
- [Output block management](#output-block-management)
- [Multiple execution Behaviour](#multiple-execution-behaviour)
- [Troubleshooting](#troubleshooting)
  - [Code blocks not executing](#code-blocks-not-executing)
  - [Timeout errors](#timeout-errors)
  - [Command not found](#command-not-found)
  - [Output not updating](#output-not-updating)
  - [No output block created](#no-output-block-created)
  - [Duplicate ID errors](#duplicate-id-errors)
  - [Custom fence not working](#custom-fence-not-working)
  - [Syntax highlighting issues](#syntax-highlighting-issues)
<!-- md-toc: end -->

## Executing Code

The `code` subcommand allows you to execute code blocks in markdown
files and automatically capture their output. This is useful for creating
executable documentation, tutorials, or notebooks.

**Basic usage:**
```bash
md code < document.md
```

**How it works:**
- Code blocks with `md-code` directives are executed automatically
- Output is automatically captured and inserted into the document
- Regular code blocks without directives are left untouched
- All content is preserved exactly as-is

**Basic directive syntax:**

Code blocks are marked with HTML comments using the `<!-- md-code: -->` marker immediately after the code fence.

~~~markdown
```python
print("Hello, world!")
```
<!-- md-code: id="hello"; bin="python3" -->
~~~

**Important notes:**
- Each code block must have a unique `id`
- The `bin` parameter is required - it specifies what command to run
- Regular code blocks without `md-code` directives are completely ignored
- Both stdout and stderr are captured in the output
- The tool is idempotent - running it multiple times updates the same output blocks assuming code is deterministic
- Use `fence` parameter to customize the fence style of output blocks
- Use `syntax` parameter to add syntax highlighting to output blocks

## Supported Directive parameters

- `id="unique-id"` (required) - Unique identifier for the code block
- `bin="command"` (required) - The command to run (e.g., `"python3"`, `"node"`, `"bash"`)
- `timeout=N` (optional) - Timeout in seconds (default: 30)
- `fence="..."` (optional) - Custom fence for output block (e.g., `"~~~"`, `"````"`) - defaults to input block's fence
- `syntax="..."` (optional) - Syntax highlighting language for output block (e.g., `"json"`, `"text"`) - defaults to no syntax

## Examples

### Python code execution

Input:
~~~markdown
```python
x = 10
y = 20
print(f"The sum is {x + y}")
```
<!-- md-code: id="sum"; bin="python3" -->
~~~
<!-- md-code: id="simple-python"; bin="md code"; syntax="markdown" -->

Output:
~~~markdown
```python
x = 10
y = 20
print(f"The sum is {x + y}")
```
<!-- md-code: id="sum"; bin="python3" -->

Output:
```
The sum is 30

```
<!-- md-code-output: id="sum" -->
~~~
<!-- md-code-output: id="simple-python" -->


### With custom timeout

~~~markdown
```python
import time
time.sleep(2)
print("Done!")
```
<!-- md-code: id="slow"; bin="python3"; timeout=1 -->
~~~
<!-- md-code: id="code-timeout"; bin="md code"; syntax="markdown" -->

Output:
~~~markdown
Error: Code execution timed out after 1 seconds

~~~
<!-- md-code-output: id="code-timeout" -->

### Bash script

~~~markdown
```bash
echo "hello world"
```
<!-- md-code: id="bash-hello"; bin="bash" -->
~~~
<!-- md-code: id="bash-example"; bin="md code"; syntax="markdown" -->

Output:
~~~markdown
```bash
echo "hello world"
```
<!-- md-code: id="bash-hello"; bin="bash" -->

Output:
```
hello world

```
<!-- md-code-output: id="bash-hello" -->
~~~
<!-- md-code-output: id="bash-example" -->

### Command with arguments

~~~markdown
```python
print("unbuffered output")
```
<!-- md-code: id="unbuf"; bin="python3 -u" -->
~~~
<!-- md-code: id="command-with-args"; bin="md code"; syntax="markdown" -->

Output:
~~~markdown
```python
print("unbuffered output")
```
<!-- md-code: id="unbuf"; bin="python3 -u" -->

Output:
```
unbuffered output

```
<!-- md-code-output: id="unbuf" -->
~~~
<!-- md-code-output: id="command-with-args" -->

### Custom fence for output

~~~markdown
```python
print("Using tildes for output")
```
<!-- md-code: id="custom"; bin="python3"; fence="~~~~" -->
~~~
<!-- md-code-disabled: id="custom-fence"; bin="md code"; syntax="markdown" -->
<!-- meta programming custom fence is currently broken :( -->

Output:
~~~markdown
```python
print("Using tildes for output")
```
<!-- md-code: id="custom"; bin="python3"; fence="~~~~" -->

Output:
~~~~
Using tildes for output

~~~~
<!-- md-code-output: id="custom" -->
~~~
<!-- md-code-output: id="custom-fence" -->

### Custom syntax highlighting

~~~markdown
```python
import json
print(json.dumps({"status": "success", "value": 42}))
```
<!-- md-code: id="json"; bin="python3"; syntax="json" -->
~~~
<!-- md-code: id="custom-syntax"; bin="md code"; syntax="markdown" -->

Output:
~~~markdown
```python
import json
print(json.dumps({"status": "success", "value": 42}))
```
<!-- md-code: id="json"; bin="python3"; syntax="json" -->

Output:
```json
{"status": "success", "value": 42}

```
<!-- md-code-output: id="json" -->
~~~
<!-- md-code-output: id="custom-syntax" -->

## Output block management

- Output blocks are automatically created after code blocks when they produce output
- If you run the command again, existing output blocks are updated
- Empty output (no stdout/stderr) does not create an output block
- Output blocks are marked with `<!-- md-code-output: id="..." -->` for tracking

## Multiple execution Behaviour

You can have multiple code blocks in the same document, each with unique IDs:

~~~markdown
```python
print("First block")
```
<!-- md-code: id="first"; bin="python3" -->

```python
print("Second block")
```
<!-- md-code: id="second"; bin="python3" -->
~~~
<!-- md-code: id="multi-code-blocks"; bin="md code"; syntax="markdown" -->

Output:
~~~markdown
```python
print("First block")
```
<!-- md-code: id="first"; bin="python3" -->

Output:
```
First block

```
<!-- md-code-output: id="first" -->

```python
print("Second block")
```
<!-- md-code: id="second"; bin="python3" -->

Output:
```
Second block

```
<!-- md-code-output: id="second" -->
~~~
<!-- md-code-output: id="multi-code-blocks" -->

## Troubleshooting

### Code blocks not executing

Make sure your code block directive includes all required fields:

1. **Unique `id` attribute** - Each code block must have a distinct ID
2. **`bin` attribute** - Specifies the interpreter/command to run
3. **Directive placement** - The comment must immediately follow the code fence

**Correct example:**
~~~markdown
```python
print("hello")
```
<!-- md-code: id="hello"; bin="python3" -->
~~~

**Common mistakes:**
~~~markdown
# Missing bin attribute
<!-- md-code: id="hello" -->

# Missing id attribute
<!-- md-code: bin="python3" -->

# Directive not immediately after code block
```python
print("hello")
```

Some text here
<!-- md-code: id="hello"; bin="python3" -->
~~~

### Timeout errors

If your code takes too long to execute, you'll see:
```
Error: Code execution timed out after 30 seconds
```

**Solutions:**
- Add a custom timeout: `<!-- md-code: id="slow"; bin="python3"; timeout=60 -->`
- Default timeout is 30 seconds
- Maximum timeout is configurable in your code

### Command not found

If you see errors like `sh: python3: not found`, the interpreter isn't in your PATH.

**Solutions:**
- Use full path to interpreter: `bin="/usr/bin/python3"`
- Verify the command exists: `which python3`
- Check your shell environment

### Output not updating

If running `md code` doesn't update your output:

**Check that:**
1. You're actually running the command: `cat file.md | md code > output.md`
2. The code block has changed since last run
3. The output is being written to the file (not just viewed)
4. Output blocks are marked with `<!-- md-code-output: id="..." -->`

**The tool is idempotent** - running it multiple times on the same input produces the same output.

### No output block created

Output blocks are only created when there's output to capture.

**Reasons for no output:**
- Code produces no stdout/stderr
- Code executed successfully but silently
- Only want output? Add a print statement!

### Duplicate ID errors

Each code block must have a unique ID within the document.

**Error:** `Duplicate code block ID: 'example'`

**Solution:** Rename one of the IDs:
```
<!-- md-code: id="example-1"; bin="python3" -->
<!-- md-code: id="example-2"; bin="python3" -->
```

### Custom fence not working

When using custom fences, make sure:
- The fence parameter uses the correct syntax: `fence="~~~~"`
- The fence is longer than any fence used in the code itself
- Both opening and closing fences match

### Syntax highlighting issues

If syntax highlighting isn't working in output blocks:
- Use the `syntax` parameter: `syntax="json"`
- The syntax name must be supported by your markdown viewer
- Common values: `json`, `text`, `python`, `bash`, `xml`
