
use parameters::CommonArgs;
use pyo3::prelude::*;
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};

mod analyze;
mod parameters;

#[pyfunction]
fn sobol_anaylze(
    _py: Python,
    y: PyReadonlyArray1<f64>,
    d: usize,
    calc_second_order: bool,
    num_resamples: usize,
    conf_level: f32,
    seed: usize,
    parallel: bool,
) -> PyResult<()>{
    
    // read y
    let y = y.as_array();
    let args = CommonArgs::new(d, num_resamples, calc_second_order);

    analyze::sobol::sobol_anaylze(&y, &args);


    // convert the 
    // if parallel {

    // }

    // Ok(y_norm.into_pyarray(_py))
    // y_norm.int
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn sobol_sensitivity(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(sobol_anaylze, m)?)?;

    Ok(())
}


