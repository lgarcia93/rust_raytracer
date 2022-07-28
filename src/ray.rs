extern crate nalgebra as na;
use na::{Vector3};

pub struct Ray  {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}

pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
    return Ray{
        origin,
        direction
    };
}
