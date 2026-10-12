use pyo3::prelude::*;

mod examples;

// Old style
// use pyo3::prelude::*;
// use examples::{check_position, fibonacci, Number};
//
// mod examples;
//
// #[pymodule]
// fn fib(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
//
//     Ok(())
// }
//
// #[pymodule]
// fn numeric(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_class::<Number>()?;
//
//     Ok(())
// }
// #[pymodule]
// fn example(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(check_position, m)?)?;
//
//     m.add_wrapped(wrap_pymodule!(fib))?;
//     m.add_wrapped(wrap_pymodule!(numeric))?;
//
//     let sys = PyModule::import(m.py(), "sys")?;
//     let sys_modules = sys.getattr("modules")?;
//
//     sys_modules.set_item("example.fib", m.getattr("fib")?)?;
//     sys_modules.set_item("example.numeric", m.getattr("numeric")?)?;
//
//     Ok(())
// }

// New style
#[pymodule]
mod example {
    use pyo3::prelude::*;

    #[pymodule_export]
    use crate::examples::check_position;

    #[pymodule]
    mod fib {
        #[pymodule_export]
        use crate::examples::fibonacci;
    }

    #[pymodule]
    mod numeric {
        #[pymodule_export]
        use crate::examples::Number;
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let sys = PyModule::import(m.py(), "sys")?;
        let modules = sys.getattr("modules")?;
        modules.set_item("example.fib", m.getattr("fib")?)?;
        modules.set_item("example.numeric", m.getattr("numeric")?)?;
        Ok(())
    }
}
