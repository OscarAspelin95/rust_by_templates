# Problem
You want to leverage Python for convenience and Rust for performance.

# Solution
Use `PyO3` and `maturin` to create bindings.

# Requirements
`uv` package manager

# Usage
- Run `uv venv` to create a virtual environment.
- Run `uv pip install maturin`.
- Run `uv run maturin develop` to generate bindings.
- Run `uv run main.py` to run python script.

# Development
Make changes to the Rust and Python code as needed. Then, it should be enough to run
- `uv run maturin develop`
- `uv run main.py`

However, sometimes you have to run
- `uv cache clean --force`

# Notes
This is a very basic example. See [PyO3](https://github.com/PyO3/pyo3) for more exhaustive examples.
