use pyo3::prelude::*;

#[pyfunction]
fn compute_fibonacci_number(n: u64) -> PyResult<u64> {
    if n < 2 {
        return Ok(n);
    }

    let (mut a, mut b) = (0, 1);
    for _ in 2..n + 1 {
        (a, b) = (b, a + b);
    }
    Ok(b)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_examples(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_fibonacci_number, m)?)?;
    Ok(())
}
