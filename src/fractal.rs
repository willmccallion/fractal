use image::{ImageBuffer, Rgb, RgbImage};
use rayon::prelude::*;

const IMAGE_W: f64 = 4096.0;
const IMAGE_H: f64 = 2160.0;

pub fn mandelbrot(
    max_recursion: u32,
    real_center: f64,
    imaginary_center: f64,
    zoom_factor: f64,
) -> RgbImage {
    let mut img = ImageBuffer::new(IMAGE_W as u32, IMAGE_H as u32);

    let scale = 2.0 / IMAGE_H as f64;
    let max_iterations = max_recursion as usize;

    img.par_enumerate_pixels_mut()
        .for_each(|(img_x, img_y, pixel)| {
            let x_pixel = img_x as f64 - (IMAGE_W as f64 / 2.0);
            let y_pixel = (IMAGE_H as f64 / 2.0) - 1.0 - img_y as f64;

            let re_c = real_center + (x_pixel / zoom_factor) * scale;
            let im_c = imaginary_center + (y_pixel / zoom_factor) * scale;

            let depth = recursive_fractal_sequence(re_c, im_c, 0.0, 0.0, 0, max_iterations);

            let color = if depth >= max_iterations {
                Rgb([0, 0, 0])
            } else {
                let alpha = depth as f64 / max_iterations as f64;
                Rgb([0, (alpha * 255.0) as u8, (alpha * 153.0) as u8])
            };

            *pixel = color;
        });

    img
}
pub fn julia(max_recursion: u32, re_c: f64, im_c: f64) -> RgbImage {
    let mut img = ImageBuffer::new(IMAGE_W as u32, IMAGE_H as u32);

    let scale = 3.0 / IMAGE_H as f64;
    let max_iterations = max_recursion as usize;

    img.par_enumerate_pixels_mut()
        .for_each(|(img_x, img_y, pixel)| {
            let x_pixel_int = img_x as i32 - IMAGE_W as i32 / 2;
            let y_pixel_int = IMAGE_H as i32 / 2 - 1 - img_y as i32;

            let x_pixel = x_pixel_int as f64;
            let y_pixel = y_pixel_int as f64;

            let re_z = x_pixel * scale;
            let im_z = y_pixel * scale;
            let depth = recursive_fractal_sequence(re_c, im_c, re_z, im_z, 0, max_iterations);

            let (red, green, blue) = if depth >= max_iterations {
                (255, 255, 255)
            } else {
                let alpha = depth as f64 / max_iterations as f64;
                (0, (alpha * 255.0) as u8, (alpha * 153.0) as u8)
            };

            *pixel = Rgb([red, green, blue]);
        });

    img
}

fn recursive_fractal_sequence(
    re_c: f64,
    im_c: f64,
    re_z: f64,
    im_z: f64,
    depth: usize,
    max_iterations: usize,
) -> usize {
    if depth >= max_iterations || (re_z * re_z + im_z * im_z) > 4.0 {
        return depth;
    }

    let new_re_z: f64 = re_z * re_z - im_z * im_z + re_c;
    let new_im_z: f64 = (2.0 * im_z * re_z) + im_c;

    recursive_fractal_sequence(re_c, im_c, new_re_z, new_im_z, depth + 1, max_iterations)
}
