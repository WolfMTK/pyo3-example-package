use examples::{check_position, fibonacci, Number};
use pyo3::{prelude::*, wrap_pymodule};

mod examples;

#[pymodule]
fn fib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;

    Ok(())
}

#[pymodule]
fn numeric(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Number>()?;

    Ok(())
}

#[pymodule]
fn example(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_position, m)?)?;

    m.add_wrapped(wrap_pymodule!(fib))?;
    m.add_wrapped(wrap_pymodule!(numeric))?;

    let sys = PyModule::import_bound(m.py(), "sys")?;
    let sys_modules = sys.getattr("modules")?;

    sys_modules.set_item("example.fib", m.getattr("fib")?)?;
    sys_modules.set_item("example.numeric", m.getattr("numeric")?)?;

    Ok(())
}
