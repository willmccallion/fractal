# Mandelbrot Fractal

## Option 1: Interactive Mandelbrot

* Run cargo run --release
* Mouse wheel to zoom in and out
* Left click and drag to move around

## Option 2: Generate image

### Input Parameters

* **Parameter 1**: <output_path.png> | Filepath to output location of image.
* **Parameter 2**: <set choice> | **0**: Mandelbrot, **1**: Julia
* **Parameter 3**: <recursion depth> | Number greater than 0.

#### Mandelbrot

* **Parameter 4**: <real center> | The **real** component of z.
* **Parameter 5**: <imaginary center> | The **imaginary** component of z.
* **Paramter 6**: <zoom factor> | Zoom amount into the fractal.

#### Julia

* **Parameter 4**: <real constant> | The **real** part of the constant.
* **Parameter 5**: <imaginary constant> | The **imaginary** part of the constant.

### Examples of Option 2

* **Mandelbrot**: cargo run --release <output_path.png> 0 1000 -1.0 0.0 1.0
* **Julia**: cargo run --release <output_path.png> 1 100 -0.795814377 -0.19144677
