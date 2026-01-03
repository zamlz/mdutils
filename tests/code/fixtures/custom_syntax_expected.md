# Custom Syntax Test

## Test 1: Default behavior (no syntax highlighting)

```python
print('{"message": "hello"}')
```
<!-- md-code: id="default_syntax"; bin="python3" -->

Output:
```
{"message": "hello"}

```
<!-- md-code-output: id="default_syntax" -->

## Test 2: JSON syntax highlighting

```python
print('{"message": "world"}')
```
<!-- md-code: id="json_syntax"; bin="python3"; syntax="json" -->

Output:
```json
{"message": "world"}

```
<!-- md-code-output: id="json_syntax" -->

## Test 3: Text syntax highlighting

```bash
echo "Some plain text output"
```
<!-- md-code: id="text_syntax"; bin="bash"; syntax="text" -->

Output:
```text
Some plain text output

```
<!-- md-code-output: id="text_syntax" -->

## Test 4: Combined fence and syntax

```python
print("Combined test")
```
<!-- md-code: id="combined"; bin="python3"; fence="~~~"; syntax="python" -->

Output:
~~~python
Combined test

~~~
<!-- md-code-output: id="combined" -->
