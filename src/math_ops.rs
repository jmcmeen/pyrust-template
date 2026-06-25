// math_ops.rs
//
// Pure Rust logic with no PyO3 dependency. Keeping your real logic free of
// binding code makes it independently testable (see the `#[cfg(test)]` block
// at the bottom, runnable with `cargo test`) and reusable from other Rust.

/// Add two integers.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Dot product of two equal-length slices. Returns `Err` if the lengths differ;
/// the binding layer translates that into a Python `ValueError`.
pub fn dot(a: &[f64], b: &[f64]) -> Result<f64, String> {
    if a.len() != b.len() {
        return Err("dot: input vectors must have equal length".to_string());
    }
    Ok(a.iter().zip(b).map(|(x, y)| x * y).sum())
}

/// Return the running cumulative sum of the input.
pub fn cumulative_sum(values: &[f64]) -> Vec<f64> {
    let mut out = Vec::with_capacity(values.len());
    let mut running = 0.0;
    for &v in values {
        running += v;
        out.push(running);
    }
    out
}

/// A running total you can keep adding to. Plain Rust — no PyO3 here. The
/// binding layer (src/lib.rs) wraps this in a `#[pyclass]`.
pub struct Accumulator {
    total: f64,
}

impl Accumulator {
    pub fn new(start: f64) -> Self {
        Accumulator { total: start }
    }

    pub fn add(&mut self, value: f64) -> f64 {
        self.total += value;
        self.total
    }

    pub fn reset(&mut self, start: f64) {
        self.total = start;
    }

    pub fn total(&self) -> f64 {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), Ok(32.0));
    }

    #[test]
    fn test_dot_length_mismatch() {
        assert!(dot(&[1.0, 2.0], &[1.0]).is_err());
    }

    #[test]
    fn test_cumulative_sum() {
        assert_eq!(cumulative_sum(&[1.0, 2.0, 3.0]), vec![1.0, 3.0, 6.0]);
        assert_eq!(cumulative_sum(&[]), Vec::<f64>::new());
    }

    #[test]
    fn test_accumulator() {
        let mut acc = Accumulator::new(0.0);
        assert_eq!(acc.add(5.0), 5.0);
        assert_eq!(acc.add(2.5), 7.5);
        acc.reset(100.0);
        assert_eq!(acc.total(), 100.0);
    }
}
