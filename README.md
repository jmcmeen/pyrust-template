# pyrust-template

A complete, batteries-included template for writing **custom Python modules in Rust**
using [PyO3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/). Clone it,
rename one thing, and start writing fast native code that imports like any normal
Python package.

```python
import pyrust_template as rs

rs.add(40, 2)                       # 42   (runs in Rust)
rs.dot([1, 2, 3], [4, 5, 6])        # 32.0
acc = rs.Accumulator()
acc.add(1.5); acc.add(2.5)
acc.total                           # 4.0
```

## Why PyO3 + maturin?

Writing against the raw CPython C API is verbose and error-prone (manual
reference counting, boilerplate for every function). [PyO3](https://pyo3.rs/)
maps Rust functions, structs, exceptions, and standard containers to Python
automatically, with memory safety guaranteed by the Rust compiler.
[maturin](https://www.maturin.rs/) is the build backend that compiles the crate
and packages it into a wheel — no `setup.py`, no CMake, just `cargo` under the
hood.

## Requirements

- [uv](https://docs.astral.sh/uv/) — the project's package/environment manager
  (`curl -LsSf https://astral.sh/uv/install.sh | sh`). uv manages Python itself,
  so you don't need a separate Python install.
- A [Rust toolchain](https://rustup.rs/) (`cargo`/`rustc`):
  - **All platforms:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - **Windows** also needs the MSVC build tools (rustup will prompt you).

uv pulls in `maturin` automatically at build time — you do not need to install
it yourself. maturin invokes `cargo`, which pulls the Rust dependencies
(`pyo3`).

> **Tip:** use a uv-managed Python (the default here). System Python packages on
> Debian/Ubuntu often lack the dev libraries required to link the extension;
> uv's standalone builds include them.

## Quick start

```bash
# Create a venv (uv-managed Python) and do an editable install with dev deps
uv venv --managed-python
uv pip install -e ".[dev]"

# Run the Python tests
uv run pytest

# Run the pure-Rust unit tests
cargo test

# Try the demo
uv run python examples/demo.py
```

Or use the Makefile shortcuts: `make dev`, `make test`, `make cargo-test`,
`make demo` (`make help` lists everything). Override the Python version with
e.g. `make dev PYTHON=3.11`.

## Project layout

```text
pyrust_template/
├── pyproject.toml              # Build system + metadata (start here)
├── Cargo.toml                  # `_core` extension + Cargo workspace root
├── Makefile                    # Convenience commands
├── src/lib.rs                  # PyO3 bindings (Rust <-> Python glue)
├── crates/mathops/
│   ├── Cargo.toml
│   └── src/lib.rs              # Pure Rust logic (no PyO3) + cargo tests
├── python/pyrust_template/
│   ├── __init__.py             # Python wrapper; re-exports the Rust symbols
│   ├── _core.pyi               # Type stubs for editors / mypy
│   └── py.typed                # PEP 561 marker
├── tests/                      # pytest suite
├── examples/demo.py
└── .github/workflows/ci.yml    # Build & test on Linux/macOS/Windows
```

**The key idea:** keep real logic in a plain-Rust crate (`mathops`) with no PyO3
dependency, and confine all Python-binding code to the `_core` crate (`src/lib.rs`).
Your logic stays reusable and independently testable with `cargo test`; the
binding file is a thin translation layer. (Splitting into two crates is also
what makes `cargo test` portable — see below.)

## How it fits together

1. `pyproject.toml` declares the build backend (`maturin`) and, under
   `[tool.maturin]`, that this is a mixed project: Python source under
   `python/`, with the compiled module dropped in as `pyrust_template._core`.
2. `Cargo.toml` builds the root crate as a `cdylib` named `_core`, and depends
   on the pure-logic `mathops` crate.
3. `src/lib.rs`'s `#[pymodule] fn _core(...)` block exposes Rust functions and
   classes to Python. This function name **must** match the `[lib] name` in
   `Cargo.toml` and the `module-name` in `pyproject.toml`.
4. `__init__.py` re-exports those symbols so users write
   `from pyrust_template import add`.

> **Why two crates?** The `_core` crate links libpython (PyO3 needs it). A
> `cargo test` on such a crate builds a test *executable* that loads libpython
> at startup — which fails on, e.g., macOS framework Python with a
> `@rpath/Python3.framework... no LC_RPATH` dyld error. Putting the real logic
> (and its `cargo test` tests) in the separate `mathops` crate, which links no
> Python, sidesteps that entirely: those tests run on every platform. The
> `_core` crate carries no Rust tests and sets `test = false` so cargo never
> builds that fragile harness. PyO3's `extension-module` feature (which would
> stop libpython from linking, but breaks linking the test exe on Linux) is
> therefore left off in `Cargo.toml` and enabled only at build time via
> `tool.maturin.features` in `pyproject.toml`.

## Customizing for your own module

1. **Rename the package.** Replace `pyrust_template` everywhere it appears:
   the `python/pyrust_template/` directory, and the strings in `pyproject.toml`,
   `Cargo.toml`, `__init__.py`, and the tests. The `_core` and `mathops` crate
   names (and the `#[pymodule]` function / `module-name` suffix `_core`) can stay.

   ```bash
   git grep -l pyrust_template   # find every occurrence
   ```

2. **Add your Rust.** Put logic in the `mathops` crate
   (`crates/mathops/src/lib.rs`) — keep it free of PyO3 so it stays testable.

3. **Bind it.** Add a `#[pyfunction]` (functions) or `#[pyclass]` +
   `#[pymethods]` (classes) in `src/lib.rs` that calls into `mathops`, register
   it inside the `#[pymodule]` block with `m.add_function(...)` /
   `m.add_class::<...>()`, then mirror the signature in `_core.pyi`.

4. **Rebuild.** Re-run `uv pip install -e .` (or `make build`). Editable
   installs do **not** auto-recompile Rust — you must rebuild after changing any
   `.rs` file.

5. **Test.** Add Python cases under `tests/` (`uv run pytest`) and pure-Rust
   cases in a `#[cfg(test)]` module inside `crates/mathops` (`cargo test`).

## Binding cheat sheet

```rust
// Function with a defaulted arg
#[pyfunction]
#[pyo3(signature = (x, factor = 2.0))]
fn scale(x: f64, factor: f64) -> f64 { x * factor }

// A class
#[pyclass]
struct Widget { size: i64 }

#[pymethods]
impl Widget {
    #[new]
    fn new(size: i64) -> Self { Widget { size } }
    fn poke(&self) { /* ... */ }
    #[getter]
    fn size(&self) -> i64 { self.size }
}

// Exceptions: return a PyResult and map errors to Python exception types.
// use pyo3::exceptions::PyValueError;
// fn checked(...) -> PyResult<f64> { something().map_err(PyValueError::new_err) }
```

- A `Vec<T>` argument accepts any Python sequence; returning `Vec<T>` yields a
  Python `list`. Tuples, `HashMap`, `Option`, etc. convert automatically.
- For zero-copy NumPy array access, add the
  [`numpy`](https://docs.rs/numpy/) crate and take a `PyReadonlyArray1<f64>`.
- Release the GIL around heavy Rust loops with `py.allow_threads(|| ...)`.

See the [PyO3 user guide](https://pyo3.rs/) for the full API.

## Building wheels for distribution

`make wheel` (`uv build --wheel`) builds a wheel into `dist/` for your current
platform and Python version. For redistributable, manylinux-compatible wheels
across many Python versions, use [`maturin`](https://www.maturin.rs/) in CI
(e.g. `maturin generate-ci github`), or the
[`cibuildwheel`](https://cibuildwheel.readthedocs.io/) action.

> If you would rather ship a single wheel that works across Python versions,
> enable PyO3's `abi3` feature (e.g. `features = ["abi3-py39"]` on the `pyo3`
> dependency). This template builds one wheel per Python version instead, which
> keeps the CI matrix meaningful.
