
extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;
extern crate rand_distr;

use ndarray::prelude::*;

pub fn sobol_sample(
    n: int,
    d: usize,
    seed: Option<usize>,
) -> Array2<f64> {
    let mut rng = match seed {
        Some(seed) => rand::rngs::StdRng::seed_from_u64(seed as u64),
        None => rand::rngs::StdRng::from_entropy(),
    };
    let mut samples = Array2::zeros((n, d));
    for i in 0..n {
        for j in 0..d {
            samples[[i, j]] = rand_distr::StandardNormal.sample(&mut rng);
        }
    }
    samples
}