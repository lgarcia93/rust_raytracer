extern crate nalgebra as na;

use image::Rgb;
use na::{Vector3};
use crate::outputimage::create_image;
use crate::ray;
use crate::sphere::Sphere;

pub struct Renderer {
    image_width: u32,
    aspect_ratio: f64,
    image_height: u32,
    viewport_width: f64,
    viewport_height: f64,
    focal_lenght: f64,
}

pub fn init_renderer(output_width: u32, aspect_ratio: f64) -> Renderer {
    return Renderer {
        image_width: output_width,
        aspect_ratio,
        image_height: (output_width as f64 / aspect_ratio) as u32,
        viewport_width: 2.0 * aspect_ratio,
        viewport_height: 2.0,
        focal_lenght: 1.0
    };
}

impl Renderer {
    pub fn render(&self, origin: Vector3<f64>, spheres: Vec<Sphere>, light_direction: Vector3<f64>) {
        let mut image = create_image(self.image_width, self.aspect_ratio);

        let horizontal_vec = Vector3::new(self.viewport_width, 0.0,  0.0);
        let vertical_vec = Vector3::new(0.0, self.viewport_height, 0.0);
        let focal_length_vec = Vector3::new(0.0, 0.0, self.focal_lenght);

        let lower_left_corner: Vector3<f64> =  origin - horizontal_vec / 2.0 - vertical_vec / 2.0 - focal_length_vec;

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                image.set_pixel_color(j, i, Rgb([0, 0, 0]));
            }
        }

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                for sphere in &spheres {
                    let u = j as f64 / (self.image_width - 1) as f64;
                    let v = i as f64 / (self.image_height - 1) as f64;
                    let direction = lower_left_corner + horizontal_vec * u + vertical_vec * v - origin;
                    
                    let ray = ray::new(origin, direction);
                    
                    let k= ray.origin - sphere.center; 
                    
                    let a= ray.direction.dot(&ray.direction);
                    let b = 2.0 * k.dot(&ray.direction);
                    let c= k.dot(&k) - sphere.radius * sphere.radius;

                    let discriminant = b * b - 4.0 * a * c;

                    let t1 = -b + discriminant.sqrt() / 2.0 * a;

                    if discriminant >= 0.0 {
                        let i_sect_point = ray.origin + ray.direction * t1;

                        let surface_normal = (i_sect_point - sphere.center).normalize();

                        let mut shading = light_direction.dot(&surface_normal);

                        // Clamp the shading result
                        if shading < 0.0 {
                            shading = 0.0
                        }

                        if shading > 1.0 {
                            shading = 1.0
                        }
                        
                        let pixel_color = ((255 as f64) * shading) as u8;
                        
                        image.set_pixel_color(j, i, Rgb([pixel_color, pixel_color,pixel_color]));
                    }
                }
            }
        }

        image.save(String::from( "~/render.png"))
    }
}