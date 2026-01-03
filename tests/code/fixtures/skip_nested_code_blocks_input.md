# Test: Directives Inside Markdown Code Fences Are Ignored

This code block should execute:

```python
print("This should execute")
```
<!-- md-code: id="real"; bin="python3" -->

This is a markdown example showing code - the directive inside should NOT execute:

```markdown
# Example of md-code directive
\```python
print("Example code")
\```
<!-- md-code: id="example"; bin="python3" -->
```

Another real code block:

```bash
echo "Real output"
```
<!-- md-code: id="real2"; bin="bash" -->
