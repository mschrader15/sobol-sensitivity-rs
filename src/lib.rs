
use parameters::CommonArgs;
use pyo3::prelude::*;
use numpy::PyReadonlyArray1;

mod analyze_lib;
mod parameters;

use parameters::PyRes;


#[pyfunction]
fn analyze<'a>(
    py: Python<'a>,
    y: PyReadonlyArray1<'a, f64>,
    d: usize,
    calc_second_order: bool,
    num_resamples: usize,
    conf_level: f32,
    seed: usize,
    parallel: bool,
) -> PyResult<PyRes<'a>> {
    
    // read y
    let y = y.as_array();
    let args = CommonArgs::new(d, num_resamples, calc_second_order, seed, conf_level);
    let res = analyze_lib::sobol::analyze(&y, &args).to_py(py);

    Ok(res)
}

/// A Python module implemented in Rust.
/// This should be a Class with a method imo
/// I also need to implement a module docstring
#[pymodule]
fn sobol_sensitivity(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(analyze, m)?)?;

    Ok(())
}


