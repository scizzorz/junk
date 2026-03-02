[private]
default:
  @just --list

# Build a release distribution
release:
  cargo build --release

# Build the project
build:
  cargo build

# Run the project
run:
  cargo run

# Format the code
format:
  cargo fmt

# Lint the code
lint:
  cargo clippy
