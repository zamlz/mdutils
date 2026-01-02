# Test: Headers Inside Code Blocks Should Be Skipped

<!-- md-toc: -->
<!-- md-toc: end -->

## Real Section 1

This is real content.

```markdown
# Fake Header in Code Block
## Another Fake Header
### Nested Fake Header

<!-- md-toc: -->

## This TOC directive should NOT be processed
```

## Real Section 2

More real content.

~~~
## This should also be ignored

<!-- md-toc: -->
~~~

### Real Subsection
