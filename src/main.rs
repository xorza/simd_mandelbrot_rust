#![feature(portable_simd)]

extern crate core;
use std::simd::{f64x16, i64x16, mask64x16, SimdPartialOrd};
use std::time::Instant;

fn main() {
    render_mandelbrot();
    render_mandelbrot();
    render_mandelbrot();

    render_mandelbrot_simd();
    render_mandelbrot_simd();
    render_mandelbrot_simd();
}

const MAX_ITERATIONS: i32 = 1023;

fn mandelbrot(cx: f64, cy: f64) -> i64 {
    let mut zx = 0.0;
    let mut zy = 0.0;

    let mut cnt = 0_i64;

    for _ in 0..MAX_ITERATIONS {
        let zx1 = zx * zx - zy * zy + cx;
        let zy1 = zx * zy + zx * zy + cy;
        zx = zx1;
        zy = zy1;

        if zx * zx + zy * zy > 4.0 {
            break;
        }

        cnt += 1;
    }

    cnt
}

fn mandelbrot_simd(cx: f64x16, cy: f64x16) -> i64x16 {
    let mut zx = f64x16::splat(0.0);
    let mut zy = f64x16::splat(0.0);

    let mut cnt = i64x16::splat(0);
    let mut escaped = mask64x16::splat(false);

    for _ in 0..MAX_ITERATIONS {
        {
            let zx1 = zx * zx - zy * zy + cx;
            let zy1 = zx * zy + zx * zy + cy;
            zx = zx1;
            zy = zy1;
        }

        escaped = escaped | (zx * zx + zy * zy).simd_ge(f64x16::splat(4.0));

        if escaped.all() {
            break;
        }

        cnt += escaped.select(
            i64x16::splat(0),
            i64x16::splat(1),
        );
    }

    cnt
}

fn render_mandelbrot() {
    let mut buffer = vec![0u8; 1024 * 1024];

    let now = Instant::now();
    for y in 0..1024 {
        for x in 0..1024 {
            let cx = (x as f64 - 780.0) * 0.003;
            let cy = (y as f64 - 512.0) * 0.003;
            let cnt = mandelbrot(cx, cy);

            buffer[y * 1024 + x] = (cnt % 256) as u8;
        }
    }
    println!("render_mandelbrot elapsed: {}ms", now.elapsed().as_millis());

    //save image to png
    let mut image = image::ImageBuffer::new(1024, 1024);
    for y in 0..1024 {
        for x in 0..1024 {
            let index = (y * 1024 + x) as usize;
            let color = buffer[index];
            let color = image::Rgb([color, color, color]);
            image.put_pixel(x, y, color);
        }
    }
    image.save("test_output/mandelbrot.png").unwrap();
}

fn render_mandelbrot_simd() {
    let mut buffer = vec![0u8; 1024 * 1024];

    let now = Instant::now();
    for y in 0..1024 {
        for x in 0..1024 / 16 {
            let x_arr = (0..16)
                .map(|i| ((x * 16 + i) as f64 - 780.0) * 0.003)
                .collect::<Vec<_>>();

            let cx = f64x16::from_slice(x_arr.as_slice());
            let cy = f64x16::splat((y as f64 - 512.0) * 0.003);
            let cnt = mandelbrot_simd(cx, cy);

            cnt.as_array()
                .iter()
                .enumerate()
                .for_each(|(i, &c)| {
                    buffer[y * 1024 + x * 16 + i] = (c % 256) as u8;
                });
        }
    }
    println!("render_mandelbrot_simd elapsed: {}ms", now.elapsed().as_millis());

    //save image to png
    let mut image = image::ImageBuffer::new(1024, 1024);
    for y in 0..1024 {
        for x in 0..1024 {
            let index = (y * 1024 + x) as usize;
            let color = buffer[index];
            let color = image::Rgb([color, color, color]);
            image.put_pixel(x, y, color);
        }
    }
    image.save("test_output/mandelbrot_simd.png").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mandelbrot() {
        render_mandelbrot();
    }

    #[test]
    fn test_mandelbrot_simd() {
        render_mandelbrot_simd();
    }
}