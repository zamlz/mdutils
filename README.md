Markdown Utils (Name Pending)
=============================

A simple Rust utility for processing markdown (currently echoes STDIN to STDOUT).

## Development

This project uses Nix flakes for reproducible builds and development environments.

### Build

Build the project using Nix:
```bash
nix build
```

The binary will be available at `./result/bin/mdutils`.

### Run

Run the built binary:
```bash
./result/bin/mdutils
```

Or run directly through the development shell:
```bash
nix develop --command cargo run
```

### Test

Run tests:
```bash
nix develop --command cargo test
```

### Debug

Enter the development shell for interactive development:
```bash
nix develop
```

Inside the shell, you can use standard Cargo commands:
```bash
cargo build       # Build the project
cargo run         # Run the project
cargo test        # Run tests
cargo check       # Check for errors without building
cargo clippy      # Run linter
cargo fmt         # Format code
```

For debugging with GDB or LLDB:
```bash
nix develop --command cargo build
nix develop --command gdb ./target/debug/mdutils
```
