mod fractal;

use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        eprintln!(
            "\n\nUsage: {} <output_file> <0|1> <max_iterations> [fractal_params...]\n",
            args[0]
        );
        eprintln!("fractal_params:  Choice 0 (Mandelbrot): requires <real_center> <imag_center> <zoom_factor>");
        eprintln!("fractal_params:  Choice 1 (Julia): requires <re_c> <im_c>\n");

        eprintln!("images/EXAMPLE.png: cargo run --release images/EXAMPLE.png 0 1000 -1.0 0.0 1.0");

        let img = fractal::mandelbrot(1000, -1.0, 0.0, 1.0);
        img.save("images/EXAMPLE.png")
            .expect("Unable to save example fractal");

        std::process::exit(1);
    }

    let output_file = &args[1];
    let set_choice: u8 = args[2]
        .parse()
        .expect("Set choice (arg 2) must be a 0 or 1");

    if args.len() != 7 && set_choice == 0 {
        eprintln!("Not enough arguements for mandelbrot.");
    }
    let max_recursion: u32 = args[3]
        .parse()
        .expect("Max iterations (arg 3) must be a positive integer.");

    let img;
    let time_elapsed;
    match set_choice {
        0 => {
            if args.len() != 7 {
                eprintln!("Incorrect number of arguments for Mandelbrot (choice 0).");
                eprintln!(
                    "Usage: {} <output> 0 <max_iter> <real_center> <imag_center> <zoom>",
                    args[0]
                );
                std::process::exit(1);
            }
            let real_center: f64 = args[4]
                .parse()
                .expect("Real center (arg 4) must be a floating-point number");
            let imaginary_center: f64 = args[5]
                .parse()
                .expect("Imaginary center (arg 5) must be a floating-point number");
            let zoom_factor: f64 = args[6]
                .parse()
                .expect("Zoom factor (arg 6) must be a floating-point number");

            let start_time = Instant::now();
            img = fractal::mandelbrot(max_recursion, real_center, imaginary_center, zoom_factor);
            time_elapsed = start_time.elapsed();
        }
        1 => {
            if args.len() != 6 {
                eprintln!("Incorrect number of arguments for Julia (choice 1).");
                eprintln!("Usage: {} <output> 1 <max_iter> <re_c> <im_c>", args[0]);
                std::process::exit(1);
            }
            let re_c: f64 = args[4]
                .parse()
                .expect("Real constant C (arg 4) must be a floating-point number");
            let im_c: f64 = args[5]
                .parse()
                .expect("Imaginary constant C (arg 5) must be a floating-point number");

            let start_time = Instant::now();
            img = fractal::julia(max_recursion, re_c, im_c);
            time_elapsed = start_time.elapsed();
        }
        _ => panic!("Invalid choice: '{}'. Please enter 0 or 1.", set_choice),
    }

    img.save(output_file).expect("Unable to save image");
    println!("Time to run fractal: {:.3}s", time_elapsed.as_secs_f64());
}
