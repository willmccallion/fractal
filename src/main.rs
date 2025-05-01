mod fractal;

use std::io::{Read, BufRead}; // Import BufRead for read_line

fn main() {
    println!("Welcome to the fractal creator: ");

    let set_choice = get_set_choice();
    
    let max_recursion = get_recursion_depth();

    let mut img;
    match set_choice {
        48 => img = fractal::mandelbrot(max_recursion),
        49 => img = fractal::julia(max_recursion),
        _ => panic!("Invalid choice: '{}'. Please enter 0 or 1.", set_choice),
    }
    
    let output_file = "images/output.png";
    img.save(output_file).expect("Unable to save image");
}

fn get_set_choice() -> u8 {
    println!("Enter [0] for Mandelbrot\nEnter [1] for Julia.");
    let mut set_choice_buf: [u8; 1] = [0; 1];
    std::io::stdin().read_exact(&mut set_choice_buf).expect("Failed to read fractal choice");
    let mut discard = String::new();
    std::io::stdin().read_line(&mut discard).expect("Failed to read rest of choice line");

    set_choice_buf[0]
}

fn get_recursion_depth() -> u32 {
    println!("Enter the maximum recursion depth (e.g., 50):");
    let mut max_recursion_str = String::new();
    std::io::stdin().read_line(&mut max_recursion_str).expect("Failed to read max recursion");

    max_recursion_str
        .trim()
        .parse()
        .expect("Please enter a valid number for max recursion")
}
