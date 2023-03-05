// extern crate dict_derive;
extern crate ndarray;

use dict_derive::IntoPyObject;
use ndarray::{prelude::*, OwnedRepr};
use numpy::{PyArray1, ToPyArray};
use pyo3::Python;

/**
 * The return object in Rust
 */
pub struct SARes {
    s1: Array1<f64>,
    s1_conf: Array1<f64>,
    st: Array1<f64>,
    st_conf: Array1<f64>,
    s2: Array1<f64>,
    s2_conf: Array1<f64>,
}

/**
 * The return dictionary of the sensitivity analysis functions, for interoperability with Python (SALib)
*/
#[derive(IntoPyObject)]
#[allow(non_snake_case)]
pub struct PyRes<'a> {
    S1: &'a PyArray1<f64>,
    S1_conf: &'a PyArray1<f64>,
    ST: &'a PyArray1<f64>,
    ST_conf: &'a PyArray1<f64>,
    S2: &'a PyArray1<f64>,
    S2_conf: &'a PyArray1<f64>,
}


impl SARes {
    
    /// Create a new instance of the return object
    pub fn new(d: usize) -> Self {
        let x: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> = ArrayBase::zeros((d,));
        Self {
            s1: x.clone(),
            s1_conf: x.clone(),
            st: x.clone(),
            st_conf: x.clone(),
            s2: x.clone(),
            s2_conf: x.clone(),
        }
    }

    /// Convert the return object to a dictionary for interoperability with Python (SALib)
    pub fn to_py<'a>(&self, py: Python<'a>) -> PyRes<'a> {
        PyRes {
            S1: self.s1.to_pyarray(py),
            S1_conf: self.s2.to_pyarray(py),
            ST: self.st.to_pyarray(py),
            ST_conf: self.st_conf.to_pyarray(py),
            S2: self.s2.to_pyarray(py),
            S2_conf: self.s2_conf.to_pyarray(py),
        }
    }

    /// Set the first order sensitivity indices
    pub fn set_s1(&mut self, s1: Array1<f64>) {
        self.s1 = s1;
    }

    /// Set the total order sensitivity indices
    pub fn set_st(&mut self, st: Array1<f64>) {
        self.st = st;
    }
}

// impl std::ops::Deref for SARes {
//     type Target = Array1<f64>;

//     fn deref(&self) -> &Self::Target {
//         &self.s1
//     }
// }

// impl std::ops::DerefMut for SARes {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.s1
//     }
// }
