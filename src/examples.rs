use std::hash::{DefaultHasher, Hash, Hasher};

use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        1 => 1, // position
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[pyfunction]
pub fn check_position(x: i32) -> PyResult<()> {
    if x < 0 {
        Err(PyValueError::new_err("x is negative"))
    } else {
        Ok(())
    }
}

#[pyclass]
pub struct Number(i32);

#[pymethods]
impl Number {
    #[new]
    fn new(value: i32) -> Self {
        Self(value)
    }

    fn __repr__(&self) -> String {
        format!("Number({})", self.0)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __bool__(&self) -> bool {
        self.0 != 0
    }
}
