extern crate nalgebra as na;
use na::{Vector3};

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64
}