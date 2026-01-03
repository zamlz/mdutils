# Custom Syntax Test

## Test 1: Default behavior (no syntax highlighting)

```python
print('{"message": "hello"}')
```
<!-- md-code: id="default_syntax"; execute; bin="python3" -->

## Test 2: JSON syntax highlighting

```python
print('{"message": "world"}')
```
<!-- md-code: id="json_syntax"; execute; bin="python3"; syntax="json" -->

## Test 3: Text syntax highlighting

```bash
echo "Some plain text output"
```
<!-- md-code: id="text_syntax"; execute; bin="bash"; syntax="text" -->

## Test 4: Combined fence and syntax

```python
print("Combined test")
```
<!-- md-code: id="combined"; execute; bin="python3"; fence="~~~"; syntax="python" -->
