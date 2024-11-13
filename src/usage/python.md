### Python Integration

std::autodiff is fully handled by the Rust compiler and therefore should not cause any issues with Python integration.
An example for maturin/PyO3 is provided below. You will still need to enable `lto=fat` in your Cargo.toml and adjust 
the module name to match your project, otherwise python won't be able to find your functions.
The first `#[pyfunction]` macro will only be applied to the original function `f`. 
We therefore add a small wrapper function `df_py` and apply the `#[pyfunction]` macro to it.

```toml

```rs 
#![feature(autodiff)]
use std::autodiff::autodiff;
use pyo3::prelude::*;

#[pyfunction]
#[autodiff(df, Reverse, Active, Active)]
fn f(x: f32) -> f32 {
    x * x
}

// Will return x*x and 2*x
#[pyfunction]
fn df_py(x: f32) -> (f32, f32) {
    df(x, 1.0)
}

// Remember to adjust the name of the module to match your project
#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(f_py, m)?)?;
    m.add_function(wrap_pyfunction!(df_py, m)?)?;
    Ok(())
}
```
