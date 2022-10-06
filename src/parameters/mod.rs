//! Structures and traits that can be used to build model parameters for equations of state.
mod sobol_res;
pub use sobol_res::SARes;

pub struct CommonArgs {
    pub d: usize,
    pub num_resamples: usize,
    pub second_order: bool,
    pub step: usize,
}

impl CommonArgs {
    pub fn new(d: usize, num_resamples: usize, second_order: bool) -> Self {
        // create the step & req length.
        let step = d + 2;
        if second_order {
            step = 2 * d + 2;
        }
        Self {
            d,
            num_resamples,
            second_order,
            step,
        }
    }
}
