mod fractal;
mod input;

use input::{get_recursion_depth, get_set_choice, get_output_filename};

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

    let output_file = format!("images/{}.png", get_output_filename());
    img.save(output_file).expect("Unable to save image");
}

