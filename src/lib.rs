use pyo3::prelude::*;

pub mod qumode;
pub mod qubit;
pub mod compilation;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from hybrid-cv-dv!".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
