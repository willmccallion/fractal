use std::io::{self, Read, Write};

pub fn get_output_filename() -> String {
    println!("What is the filename of this set? ");
    let mut max_recursion_str = String::new();
    io::stdin().read_line(&mut max_recursion_str).expect("Failed to read filename");
    
    max_recursion_str = max_recursion_str.trim().to_string();
    max_recursion_str
}

pub fn get_set_choice() -> u8 {
    println!("Enter [0] for Mandelbrot\nEnter [1] for Julia.");
    let mut set_choice_buf: [u8; 1] = [0; 1];
    io::stdin().read_exact(&mut set_choice_buf).expect("Failed to read fractal choice");
    let mut discard = String::new();
    io::stdin().read_line(&mut discard).expect("Failed to read rest of choice line");

    set_choice_buf[0]
}

pub fn get_recursion_depth() -> u32 {
    println!("Enter the maximum recursion depth (e.g., 50):");
    let mut max_recursion_str = String::new();
    io::stdin().read_line(&mut max_recursion_str).expect("Failed to read max recursion");

    max_recursion_str
        .trim()
        .parse()
        .expect("Please enter a valid number for max recursion")
}

pub fn get_mandeltbrot_param() -> (f64, f64, f64) {
    println!("Enter Mandelbrot parameters: real_center imaginary_center zoomFactor");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read parameters");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        panic!("Expected 3 parameters: real_center imaginary_center zoomFactor");
    }

    let real_center: f64 = parts[0].parse().expect("Invalid real_center");
    let imaginary_center: f64 = parts[1].parse().expect("Invalid imaginary_center");
    let zoom_factor: f64 = parts[2].parse().expect("Invalid zoomFactor");

    if zoom_factor <= 0.0 {
        panic!("zoomFactor must be positive");
    }

    (real_center, imaginary_center, zoom_factor)
}

pub fn get_julia_param() -> (f64, f64) {
    println!("Enter Julia parameters: ReC ImC (e.g., -0.8 0.156)");

    io::stdout().flush().expect("Could not flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line for Julia parameters");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        panic!(
            "Expected exactly 2 parameters (ReC ImC), but got {} parts: {:?}",
            parts.len(), parts
        );
    }

    let re_c: f64 = parts[0]
        .parse()
        .expect("Invalid floating-point number for ReC");
    let im_c: f64 = parts[1]
        .parse()
        .expect("Invalid floating-point number for ImC");

    (re_c, im_c)
}
