// lib.rs
//
// The binding layer: this is the only file that knows about both Rust and
// Python. It exposes the pure-Rust functions from math_ops.rs to Python via
// PyO3. The compiled result is importable as `pyrust_template._core`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

mod math_ops;

// --- Free functions --------------------------------------------------------

/// Add two integers and return the result.
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    math_ops::add(a, b)
}

/// Return the dot product of two equal-length sequences of floats.
///
/// A `Vec<f64>` argument accepts any Python sequence of floats; the `Err`
/// returned by `math_ops::dot` becomes a Python `ValueError`.
#[pyfunction]
fn dot(a: Vec<f64>, b: Vec<f64>) -> PyResult<f64> {
    math_ops::dot(&a, &b).map_err(PyValueError::new_err)
}

/// Return the running cumulative sum of a sequence of floats.
#[pyfunction]
fn cumulative_sum(values: Vec<f64>) -> Vec<f64> {
    math_ops::cumulative_sum(&values)
}

// --- Example class ---------------------------------------------------------

/// A running total you can keep adding to. A thin `#[pyclass]` wrapper around
/// the plain-Rust `math_ops::Accumulator`, to show stateful objects.
#[pyclass]
struct Accumulator {
    inner: math_ops::Accumulator,
}

#[pymethods]
impl Accumulator {
    #[new]
    #[pyo3(signature = (start = 0.0))]
    fn new(start: f64) -> Self {
        Accumulator {
            inner: math_ops::Accumulator::new(start),
        }
    }

    /// Add a value and return the new total.
    fn add(&mut self, value: f64) -> f64 {
        self.inner.add(value)
    }

    /// Reset the running total.
    #[pyo3(signature = (start = 0.0))]
    fn reset(&mut self, start: f64) {
        self.inner.reset(start)
    }

    /// The current running total.
    #[getter]
    fn total(&self) -> f64 {
        self.inner.total()
    }

    fn __repr__(&self) -> String {
        format!("<Accumulator total={}>", self.inner.total())
    }
}

// PyO3's #[pymodule] defines the entry point. The function name MUST match the
// module name in pyproject.toml's `module-name` and the `[lib] name` in
// Cargo.toml (here: `_core`).
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(dot, m)?)?;
    m.add_function(wrap_pyfunction!(cumulative_sum, m)?)?;
    m.add_class::<Accumulator>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
