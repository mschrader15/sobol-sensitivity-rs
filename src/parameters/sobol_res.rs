// extern crate dict_derive;
extern crate ndarray;

use ndarray::{prelude::*, OwnedRepr};
use numpy::{ToPyArray, PyArray1};
use dict_derive::IntoPyObject;
use pyo3::Python;


// results
pub struct SARes {
    s1: Array1<f64>,
    s1_conf: Array1<f64>,
    st: Array1<f64>,
    st_conf: Array1<f64>,
    s2: Array1<f64>,
    s2_conf: Array1<f64>,
}

#[derive(IntoPyObject)]
pub struct PyRes<'a>{
    S1: &'a PyArray1<f64>,
    S1_conf: &'a PyArray1<f64>,
    ST: &'a PyArray1<f64>,
    ST_conf: &'a PyArray1<f64>,
    S2: &'a PyArray1<f64>,
    S2_conf: &'a PyArray1<f64>,
}



impl SARes {
    // initialize an empty res holder
    pub fn new (d: usize) -> Self {
        // initialize an empty array with the desired dimensions
        let mut x: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> = ArrayBase::zeros((d, ));
        Self{
            s1: x.clone(),
            s1_conf: x.clone(),
            st: x.clone(),
            st_conf: x.clone(),
            s2: x.clone(),
            s2_conf: x.clone(),
        }
    }

    pub fn to_py(&self, py: Python) -> PyRes {
        PyRes{
            S1: self.s1.to_pyarray(py),
            S1_conf: self.s2.to_pyarray(py),
            ST: self.st.to_pyarray(py),
            ST_conf: self.st_conf.to_pyarray(py),
            S2: self.s2.to_pyarray(py),
            S2_conf: self.s2_conf.to_pyarray(py),
        }
    }

    pub fn set_s1(&mut self, s1: Array1<f64>) {
        self.s1 = s1;
    }

    pub fn set_st(&mut self, st: Array1<f64>) {
        self.st = st;
    }
}

impl std::ops::Deref for SARes {
    type Target = Array1<f64>;

    fn deref(&self) -> &Self::Target {
        &self.s1
    }
}

impl std::ops::DerefMut for SARes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s1
    }
}

