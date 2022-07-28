use crate::renderer::init_renderer;
extern crate nalgebra as na;
use na::{Vector3};
use crate::sphere::Sphere;

mod ray;
mod sphere;
mod outputimage;
mod renderer;

fn main() {
    let renderer = init_renderer(1920, 16.0/9.0 as f64);
    
    let origin: Vector3<f64> = Vector3::new(0.0, 0.0, 1.0);
    
    let sphere1 = Sphere{
        center: Vector3::new(1.0, 1.0, -1.0),
        radius: 2.0,
    };

    let sphere2 = Sphere{
        center: Vector3::new(-1.0, 1.0, -3.0),
        radius: 1.1,
    };

    let sphere3 = Sphere{
        center: Vector3::new(0.0, 0.2, -3.0),
        radius: 1.1,
    };
    
    let light_direction: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0);

    renderer.render(origin, vec![sphere1], light_direction.normalize());
}