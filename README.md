# Example of creating packages

## Description

This repository contains an example of creating packages using pyo3 and integrating Rust with Python.

## Requirements

- [uv](https://docs.astral.sh/uv/getting-started/installation/)
- [Rust toolchain](https://rustup.rs/)

## Creation of virtual environment and install

`uv sync`

This creates `.venv`, installs the dev dependencies and builds the Rust extension via maturin.

## Rebuild after changing Rust code

`uv run maturin develop --uv`

Add `--release` for an optimized build.

## Tests

`uv run pytest`

## Build wheel and sdist

`uv build`

## More information

[pyo3](https://pyo3.rs/latest/getting-started)
[maturin](https://www.maturin.rs/installation)
[uv](https://docs.astral.sh/uv/)
