Command: `code` (Code Execution)
================================

<!-- md-toc: -->
<!-- md-toc: end -->

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

**Directive parameters:**
- `id="unique-id"` (required) - Unique identifier for the code block
- `bin="command"` (required) - The command to run (e.g., `"python3"`, `"node"`, `"bash"`)
- `timeout=N` (optional) - Timeout in seconds (default: 30)
- `fence="..."` (optional) - Custom fence for output block (e.g., `"~~~"`, `"````"`) - defaults to input block's fence
- `syntax="..."` (optional) - Syntax highlighting language for output block (e.g., `"json"`, `"text"`) - defaults to no syntax

**Example - Python code execution:**

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


**Example - With custom timeout:**

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

**Example - Bash script:**

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

**Example - Command with arguments:**

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

**Example - Custom fence for output:**

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

**Example - Custom syntax highlighting:**

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

**Output block management:**
- Output blocks are automatically created after code blocks when they produce output
- If you run the command again, existing output blocks are updated
- Empty output (no stdout/stderr) does not create an output block
- Output blocks are marked with `<!-- md-code-output: id="..." -->` for tracking

**Multiple executions:**

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

**Important notes:**
- Each code block must have a unique `id`
- The `bin` parameter is required - it specifies what command to run
- Regular code blocks without `md-code` directives are completely ignored
- Both stdout and stderr are captured in the output
- The tool is idempotent - running it multiple times updates the same output blocks
- Use `fence` parameter to customize the fence style of output blocks
- Use `syntax` parameter to add syntax highlighting to output blocks
