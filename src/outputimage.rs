pub use image::{ImageBuffer, RgbImage};

pub struct OutputImage {
    output: RgbImage,
    width: u32,
    height: u32,
    aspect_ratio: f64,
}

pub fn create_image(width: u32, aspect_ratio: f64) -> OutputImage {
    let height = (width as f64/ aspect_ratio) as u32;

    let output = ImageBuffer::new(width, height);

    return OutputImage {
        output,
        width,
        aspect_ratio,
        height,
    };
}

impl OutputImage {
    pub fn save(&self, path: String) {
        self.output.save("image.png").unwrap();
    }

    pub fn set_pixel_color(&mut self, x: u32, y: u32, color_value: image::Rgb<u8>) {
       *self.output.get_pixel_mut(x, y) = color_value;
    }
}




