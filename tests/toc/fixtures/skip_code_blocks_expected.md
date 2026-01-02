# Test: Headers Inside Code Blocks Should Be Skipped

<!-- md-toc: -->
- [Real Section 1](#real-section-1)
- [Real Section 2](#real-section-2)
  - [Real Subsection](#real-subsection)
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
