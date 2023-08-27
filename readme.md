Rust Mandelbrot implementation with SIMD instructions using portable_simd.
Nightly toolchain used, as portable_simd is not supported in release one.

Macbook Air M2 2022, 1024*1024, 1023 iterations:

````
render_mandelbrot elapsed: 860ms
render_mandelbrot elapsed: 894ms
render_mandelbrot elapsed: 844ms
render_mandelbrot_simd elapsed: 215ms
render_mandelbrot_simd elapsed: 218ms
render_mandelbrot_simd elapsed: 217ms
````

![mandelbrot_simd.png](mandelbrot_simd.png)
