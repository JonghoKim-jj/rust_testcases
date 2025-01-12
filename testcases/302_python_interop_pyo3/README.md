# Requirements
PyO3 requires Rust 1.63+

PyO3 supports:
- CPython 3.7+
- PyPy 3.9+
- GraalPy 3.10+

(PyO3 User Guide)[https://pyo3.rs/v0.23.4/]

``` sh
python -m venv .venv
source .venv/bin/activate
pip install maturin
maturin init
```

# Build
Run `maturin develop`