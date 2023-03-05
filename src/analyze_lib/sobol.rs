use std::ops::{Sub, Mul};
use rand::prelude::*;
use rand::distributions::Uniform;
use statrs::distribution::{Normal, ContinuousCDF};

use crate::parameters;

use ndarray::prelude::*;
use parameters::{CommonArgs, SARes};


// create a struct for storing the array slices
#[derive(Clone)]
struct ArraySlices<'slice> {
    pub a: ArrayView1<'slice, f64>,
    pub b: ArrayView1<'slice, f64>,
    pub ab: Array2<f64>,
    pub ba: Array2<f64>,
}

impl<'slice> ArraySlices<'slice> {
    pub fn new(y: &'slice Array1<f64>, n: usize, args: &CommonArgs) -> ArraySlices<'slice> {
        // create the slices. 
        // a starts at 0, with step size args.step (d+2) or (2 * d + 2)
        let a  = y.slice(s![..;args.step]);
        // b starts at step - 1 (d+1), with step size args.step (d+2)
        let b = y.slice(s![(args.step - 1)..;args.step]);
        
        let mut ab = Array2::zeros((n, args.d));
        let mut ba = ab.clone();
        
        // iterate over the dimensions, creates the AB and BA matrices
        for n in 0..args.d {
            ab.slice_mut(s![.., n]).assign(&y.slice(s![(n + 1)..;args.step]));
            if args.second_order{
                ba.slice_mut(s![.., n]).assign(&y.slice(s![(n + args.d + 1)..;args.step]));
            }
        }
    
        ArraySlices { a, b, ab, ba }
    }
}


// calculate the first order sensitivity indices
fn calc_s1(slices: &ArraySlices, y: &Array1<f64>, res: &mut SARes) -> () {
    // calculate the first order sensitivity indices
    // inner = (AB - A)
    let mut inner = slices.ab.to_owned().sub(&slices.a);
    // innner = B * (inner)
    inner.columns_mut().into_iter().map(|x| x.mul(&slices.b));
    // 1/N SUM (inner)
    let vi = inner.mean_axis(Axis(0)).unwrap();
    // Si = Vi / V
    res.set_s1(vi / y.std(0.0)); 
}

fn calc_total(slices: &ArraySlices, y: &Array1<f64>, res: &mut SARes) -> () {
    // calculate the total order sensitivity indices
    // inner = (A - AB)
    let mut inner = slices.a.sub(&slices.ab);
    // inner = inner^2
    inner.mapv_inplace(|x| x.powi(2));
    // 1/(2N) SUM (inner) / Var(Y)
    res.set_st(0.5 * (inner.mean_axis(Axis(0)).unwrap() / y.std(0.0))); 
}

// fn calc_s2(slices: &ArraySlices, y: &Array1<f64>, res: &mut SARes) -> () {
//     // calculate the second order sensitivity indices
//     // inner = (BA - AB)
//     let mut inner = slices.ba.sub(&slices.ab);
//     // inner = inner^2
//     inner.mapv_inplace(|x| x.powi(2));
//     // 1/(2N) SUM (inner) / Var(Y)
//     res.set_s2(0.5 * (inner.mean_axis(Axis(0)).unwrap() / y.std(0.0))); 
// }




// The Sobol Sensitivity Analysis Method
// Input is a Sequence of Model Outputs and the Dimensions of the Problem
// Output is a object containing the first order, total order, and second order sensitivity indices
pub fn analyze(y: &ArrayView1<f64>, args: &CommonArgs) -> SARes {
    // Have to convert d into usize.
    let d: usize = args.d.try_into().unwrap();
    // Create the output holder.
    let mut res = SARes::new(d);

    // get N
    let n: usize;
    if args.second_order{
        n = y.len() / (2 * d + 2);
    } else {
        n = y.len() / (d + 2);
    }

    // create a normalized version of y array.
    let y_norm = (y - y.mean().expect("y should not be empty")) / y.std(0.);

    // assert that the size is correct
    if args.second_order {
        assert!(y_norm.dim() % (2 * d + 2) == 0);
    } else {
        assert!(y_norm.dim() % (d + 2) == 0);
    };

    // create the necessary slices
    let y_slices = ArraySlices::new(&y_norm, n, args);
    
    
    // calculate the first order sensitivity indices
    calc_s1(&y_slices, &y_norm, &mut res);
    
    // calculate the total order sensitivity indices
    calc_total(&y_slices, &y_norm, &mut res);

    // calculate the second order sensitivity indices
    if args.second_order {
        // calc_s2(&y_slices, &y_norm, &mut res);
    }
    
    // bootstrap the Confidence Intervals (parallelized with rayon)
    // create a random sample of integers with dimension num_bootstraps * num_samples
    // rand::SeedableRng::seed_from_u64();
    let rng = SeedableRng::seed_from_u64(0);
    let mut bootstrap_samples = Array2::zeros((args.num_resamples, n));

    // create a normal distribution
    let normal = Normal::new(0.0, 1.0).unwrap();
    // create a cdf for the normal distribution
    let cdf = normal.cdf(0.0);

    let pff = match args.confidence_interval {
        0.95 => 1.959963984540054,
        0.99 => 2.5758293035489004,
        // if we can't find the confidence interval, compute it (expensive)
        _ => Normal::new(0.0, 1.0, ).unwrap().inverse_cdf(args.confidence_interval),
    };

    // return res
    res
    
}
