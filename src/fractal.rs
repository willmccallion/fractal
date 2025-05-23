use image::{ImageBuffer, Rgb, RgbImage};

const IMAGE_H: f64 = 1440.0;

pub fn mandelbrot(
    max_recursion: u32,
    real_center: f64,
    imaginary_center: f64,
    zoom_factor: f64,
) -> RgbImage {
    let image_w = (3.0 * IMAGE_H) / 2.0;
    let image_w_int = image_w as i32;
    let image_h_int = IMAGE_H as i32;

    let scale = 2.0 / IMAGE_H;

    let initial_re_z: f64 = 0.0;
    let initial_im_z: f64 = 0.0;

    let mut img = ImageBuffer::new(image_w as u32, IMAGE_H as u32);

    let max_iterations = max_recursion as usize;

    for img_x in 0..(image_w as usize) {
        for img_y in 0..(IMAGE_H as usize) {
            let x_pixel_int = img_x as i32 - image_w_int / 2;
            let y_pixel_int = image_h_int / 2 - 1 - img_y as i32;

            let x_pixel = x_pixel_int as f64;
            let y_pixel = y_pixel_int as f64;

            let re_c = real_center + ((x_pixel / zoom_factor) + 0.5) * scale;
            let im_c = imaginary_center + ((y_pixel / zoom_factor) - 0.5) * scale;

            let depth = recursive_fractal_sequence(
                re_c,
                im_c,
                initial_re_z,
                initial_im_z,
                0,
                max_iterations,
            );

            let red: u8;
            let green: u8;
            let blue: u8;

            if depth >= max_iterations {
                red = 255;
                green = 255;
                blue = 255;
            } else {
                let alpha = depth as f64 / max_iterations as f64;
                red = 0;
                green = (alpha * 255.0) as u8;
                blue = (alpha * 153.0) as u8;
            }

            let pixel = Rgb([red, green, blue]);

            img.put_pixel(img_x as u32, img_y as u32, pixel);
        }
    }

    img
}

pub fn julia(max_recursion: u32, re_c: f64, im_c: f64) -> RgbImage {
    let image_w = (4.0 * IMAGE_H) / 3.0;
    let image_w_int = image_w as i32;
    let image_h_int = IMAGE_H as i32;

    let scale = 3.0 / IMAGE_H;

    let mut img = ImageBuffer::new(image_w as u32, IMAGE_H as u32);

    let max_iterations = max_recursion as usize;

    for img_x in 0..(image_w as usize) {
        for img_y in 0..(IMAGE_H as usize) {
            let x_pixel_int = img_x as i32 - image_w_int / 2;
            let y_pixel_int = image_h_int / 2 - 1 - img_y as i32;

            let x_pixel = x_pixel_int as f64;
            let y_pixel = y_pixel_int as f64;

            let re_z = x_pixel * scale;
            let im_z = y_pixel * scale;
            let depth = recursive_fractal_sequence(re_c, im_c, re_z, im_z, 0, max_iterations);

            let red: u8;
            let green: u8;
            let blue: u8;

            if depth >= max_iterations {
                red = 255;
                green = 255;
                blue = 255;
            } else {
                let alpha = depth as f64 / max_iterations as f64;
                red = 0;
                green = (alpha * 255.0) as u8;
                blue = (alpha * 153.0) as u8;
            }

            let pixel = Rgb([red, green, blue]);

            img.put_pixel(img_x as u32, img_y as u32, pixel);
        }
    }

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

