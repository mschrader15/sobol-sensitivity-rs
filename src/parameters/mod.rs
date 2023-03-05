//! Structures and traits that can be used to build model parameters for equations of state.
mod sobol_res;
pub use sobol_res::{SARes, PyRes};

pub struct CommonArgs {
    pub d: usize,
    pub num_resamples: usize,
    pub second_order: bool,
    pub step: usize,
    pub confidence_interval: f64,
}


/// This is the start of an OOP approach to the Sobol sensitivity analysis 
impl CommonArgs {
    /// Creates a new [`CommonArgs`].
    pub fn new(d: usize, num_resamples: usize, second_order: bool, confidence_interval: f64, ) -> Self {
        // create the step & req length.
        let mut step = d + 2;
        if second_order {
            step = 2 * d + 2;
        }
        Self {
            d,
            num_resamples,
            second_order,
            step,
            confidence_interval,
        }
    }
}


impl Default for CommonArgs {
    fn default() -> Self {
        Self {
            d: 1,
            num_resamples: 1000,
            second_order: false,
            step: 3,
            confidence_interval: 0.95,
        }
    }
}