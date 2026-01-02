# Test: Directives Inside Markdown Code Fences Are Ignored

This code block should execute:

```python
print("This should execute")
```
<!-- md-code: id="real"; execute; bin="python3" -->

Output:
```
This should execute

```
<!-- md-code-output: id="real" -->

This is a markdown example showing code - the directive inside should NOT execute:

```markdown
# Example of md-code directive
\```python
print("Example code")
\```
<!-- md-code: id="example"; execute; bin="python3" -->
```

Another real code block:

```bash
echo "Real output"
```
<!-- md-code: id="real2"; execute; bin="bash" -->

Output:
```
Real output

```
<!-- md-code-output: id="real2" -->
