Markdown Utils
==============

**Version:** 0.1.0

<!-- md-toc: -->
- [Why?](#why)
- [Features](#features)
- [Disclaimer](#disclaimer)
- [Usage](#usage)
  - [TOC Example](#toc-example)
  - [Table Example](#table-example)
  - [Code Example](#code-example)
  - [New Example](#new-example)
- [Idempotency](#idempotency)
- [Meta-Programming](#meta-programming)
- [Examples](#examples)
- [Development](#development)
  - [Build](#build)
  - [Run](#run)
  - [Test](#test)
  - [Debug](#debug)
- [Troubleshooting](#troubleshooting)
  - [Binary not found after `nix build`](#binary-not-found-after-nix-build)
  - [Command-specific issues](#command-specific-issues)
  - [Getting help](#getting-help)
<!-- md-toc: end -->

## Why?

A Rust CLI tool for markdown processing with multiple subcommands. This
is heavily based on some of the useful features I used when I used to
use GNU/Emacs and Org-mode. Spreadsheet and literate programming were
really cool features but were limited to GNU/Emacs. Emacs is really
awesome, but if I needed to edit markdown files, I wanted to be able to
do some of the same things.  And even more so, I wanted to do so in any
text editor! Editors like VIM, NeoVIM, Kakoune, Helix, etc. allow you
to take a selection of text and pipe it to some program in your path
and then replaces the highlighted text with it's output. This tool
does just that. It is not editor specific and utilizes HTML comments in
markdown (which is valid syntax) to tag and extend markdown with some
of these capabilities.

## Features

- **Table Formatting** - Auto-align and format markdown tables
- **Spreadsheet Formulas** - Excel-like formulas with vectors, matrices, and functions (`sum`, `avg`, `min`, `max`, `count`, `prod`)
- **Code Execution** - Execute code blocks and capture output directly in markdown
- **Table of Contents** - Auto-generate TOCs with GitHub-style anchors
- **Task Completion** - Mark checklist items as done with strikethrough and timestamp
- **Cross-Table References** - Reference data between tables using table IDs
- **Variables** - Store intermediate results in formulas with `let` statements
- **Matrix Operations** - Transpose (`.T`), matrix multiplication (`@`), ranges (`A1:C3`)
- **Idempotent** - Running commands multiple times produces the same result
- **Editor Agnostic** - Works with any editor that can pipe text (Vim, Neovim, Kakoune, Helix, etc.)
- **Meta-Programmable** - This README and all docs are generated using `md` itself!

## Disclaimer

This tool is entirely vibe-coded. It started off as an experiment in
vibe-coding and I figured it was simple enough to define and structurely
build up in an interative manner. I had the features and design in my
head but I just didn't have the time to work on a project like this. It's
simple enough where I can define everything exactly to what I need,
but also complex enough where it would take me a while.

In any case, you have been warned!

## Usage

`md` supports the following commands:
_(Please click on the links to see more detailed documentation)_

- [`new`](docs/new.md): Create simple elements like new tables
- [`table`](docs/table.md): Format and apply formulas to existing tables
- [`code`](docs/code.md): Evaluation of code blocks
- [`toc`](docs/toc.md): Generation of table of contents
- [`done`](docs/done.md): Mark checklist items as completed

All commands (with the exception of `new`) operate with the idea that
it reads from STDIN and then tranforms the input to produce some output
to STDOUT. The primary use-case for this functionality is with an editor
that can take the current selection and pipe it to this tool. This makes
the tool editor agnostic (for the most part).

```bash
cat EXAMPLE.md | md <command>
```

### TOC Example

```
cat EXAMPLE.md | md toc
```

Input:
~~~markdown
# Title Heading
<!-- md-toc: -->
## Heading 1
### Heading 2
~~~
<!-- md-code: id="toc-example"; bin="md toc"; syntax="markdown" -->

Output:
~~~markdown
# Title Heading
<!-- md-toc: -->
- [Heading 1](#heading-1)
  - [Heading 2](#heading-2)
<!-- md-toc: end -->
## Heading 1
### Heading 2

~~~
<!-- md-code-output: id="toc-example" -->

### Table Example

```bash
cat EXAMPLE.md | md table
```

Input:
~~~markdown
| x   | y   | z   |
| --- | --- | --- |
| 2   | 3   | 4   |
|     |     |     |
<!-- md-table: _2 = _1 ^ 3 -->
~~~
<!-- md-code: id="table-example"; bin="md table"; syntax="markdown" -->

Output:
~~~markdown
| x   | y   | z   |
| --- | --- | --- |
| 2   | 3   | 4   |
| 8   | 27  | 64  |
<!-- md-table: _2 = _1 ^ 3 -->
~~~
<!-- md-code-output: id="table-example" -->

### Code Example

```bash
cat EXAMPLE.md | md code
```
Input:
~~~markdown
```python
def collatz_sequence(n):
    while n != 1:
        yield n
        n = n // 2 if n % 2 == 0 else 3 * n + 1
    yield 1

start_num = 300
result_sequence = list(collatz_sequence(start_num))
print(f"Collatz sequence for {start_num}:\n{result_sequence}")
```
<!-- md-code: id="code-test"; bin="python3" -->
~~~
<!-- md-code: id="code-example"; bin="md code"; syntax="markdown" -->

Output:
~~~markdown
```python
def collatz_sequence(n):
    while n != 1:
        yield n
        n = n // 2 if n % 2 == 0 else 3 * n + 1
    yield 1

start_num = 300
result_sequence = list(collatz_sequence(start_num))
print(f"Collatz sequence for {start_num}:\n{result_sequence}")
```
<!-- md-code: id="code-test"; bin="python3" -->

Output:
```
Collatz sequence for 300:
[300, 150, 75, 226, 113, 340, 170, 85, 256, 128, 64, 32, 16, 8, 4, 2, 1]

```
<!-- md-code-output: id="code-test" -->
~~~
<!-- md-code-output: id="code-example" -->

### New Example

```bash
md new table:7:7
```
<!-- md-code: id="new-example"; bin="bash"; syntax="markdown" -->

Output:
```markdown
|     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- |
|     |     |     |     |     |     |     |
|     |     |     |     |     |     |     |
|     |     |     |     |     |     |     |
|     |     |     |     |     |     |     |
|     |     |     |     |     |     |     |
|     |     |     |     |     |     |     |
|     |     |     |     |     |     |     |
```
<!-- md-code-output: id="new-example" -->

### Done Example

```bash
cat EXAMPLE.md | md done
```

Input:
~~~markdown
- [ ] Buy groceries
- [ ] Walk the dog
- [x] Already completed
~~~
<!-- md-code: id="table-example"; bin="md done"; syntax="markdown" -->

Output:
~~~markdown
- [x] ~~Buy groceries~~ `COMPLETED: 2026-02-07 17:44:08`
- [x] ~~Walk the dog~~ `COMPLETED: 2026-02-07 17:44:08`
- [x] Already completed
~~~
<!-- md-code-output: id="table-example" -->


Note: Already checked items (`- [x]`) are left unchanged.

## Idempotency

Idempotency means that the following is true,
```
command(document) == command(command(document))
```

This tool is __ONLY__ idempotent if the following conditions are met!
- code blocks are running deterministic code
- table formulas are deterministic and not self-referencial

## Meta-Programming

The tool is designed to allow meta-programming of itself! This means that
this very `README.md` file is built and modified using the same `md`
tool. This includes all files in the [`docs/`](docs/) sub-folder.

## Examples

The [`examples/`](examples/) directory contains real-world usage examples:

- [`data-analysis.md`](examples/data-analysis.md) - Data analysis workflows
- [`grades.md`](examples/grades.md) - Grade calculation spreadsheet
- [`meta-programming.md`](examples/meta-programming.md) - Using `md` to document itself
- [`project-docs.md`](examples/project-docs.md) - Project documentation example
- [`scientific-computation.md`](examples/scientific-computation.md) - Scientific computing examples

Each example demonstrates different features and can be processed with `md` commands.

## Development

This project uses Nix flakes for reproducible builds and development environments.

### Build

Build the project using Nix:
```bash
nix build
```

The binary will be available at `md`.

### Run

Run the built binary:
```bash
md table < input.md
```

Or run directly through the development shell:
```bash
nix develop --command cargo run -- table < input.md
```

### Test

Run all tests (389 total tests including unit, integration, and doc tests):
```bash
nix develop --command cargo test
```

Run specific test:
```bash
nix develop --command cargo test test_name
```

Run tests for a specific module:
```bash
nix develop --command cargo test table::
nix develop --command cargo test code::
nix develop --command cargo test toc::
```

### Debug

Enter the development shell for interactive development:
```bash
nix develop
```

Inside the shell, you can use standard Cargo commands:
```bash
cargo build              # Build the project
cargo run -- table       # Run the project with table subcommand
cargo test               # Run tests
cargo check              # Check for errors without building
cargo clippy             # Run linter
cargo fmt                # Format code
```

For debugging with GDB or LLDB:
```bash
nix develop --command cargo build
nix develop --command gdb ./target/debug/md
```

## Troubleshooting

### Binary not found after `nix build`

After running `nix build`, the binary is available at `./result/bin/md`. You can either:
- Use the full path: `./result/bin/md table < input.md`
- Add to PATH: `export PATH="$PWD/result/bin:$PATH"`
- Copy to a location in your PATH: `cp result/bin/md ~/.local/bin/`

### Command-specific issues

For troubleshooting specific to each command, see:
- **Table formulas** - See [docs/table.md](docs/table.md#troubleshooting)
- **Code execution** - See [docs/code.md](docs/code.md#troubleshooting)
- **TOC generation** - See [docs/toc.md](docs/toc.md#troubleshooting)
- **Task completion** - See [docs/done.md](docs/done.md#troubleshooting)

### Getting help

- Check the detailed documentation in [`docs/`](docs/)
- View examples in [`examples/`](examples/)
- Report issues on GitHub (if repository is public)
