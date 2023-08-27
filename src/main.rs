#![feature(portable_simd)]

extern crate core;
use std::simd::SimdPartialOrd;
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
const IMAGE_SIZE: usize = 1024;

// @formatter:off
type F64simd        = std::simd::f64x16;
type I64simd        = std::simd::i64x16;
type Mask64simd  = std::simd::mask64x16;
const SIMD_LANES: usize            = 16;
// @formatter:on


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

fn mandelbrot_simd(cx: F64simd, cy: F64simd) -> I64simd {
    let mut zx = F64simd::splat(0.0);
    let mut zy = F64simd::splat(0.0);

    let mut cnt = I64simd::splat(0);
    let mut escaped = Mask64simd::splat(false);

    for _ in 0..MAX_ITERATIONS {
        (zx, zy) = {
            (
                zx * zx - zy * zy + cx,
                zx * zy + zx * zy + cy
            )
        };

        escaped |= (zx * zx + zy * zy).simd_ge(F64simd::splat(4.0));

        if escaped.all() {
            break;
        }

        cnt += escaped.select(
            I64simd::splat(0),
            I64simd::splat(1),
        );
    }

    cnt
}

fn render_mandelbrot() {
    let mut buffer = vec![0u8; IMAGE_SIZE * IMAGE_SIZE];

    let x1 = (0.0 - 0.8) * 2.5;
    let x2 = (1.0 - 0.8) * 2.5;
    let y1 = (0.0 - 0.5) * 2.5;
    let y2 = (1.0 - 0.5) * 2.5;

    let now = Instant::now();
    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE {
            let cx = (x as f64 / IMAGE_SIZE as f64) * (x2 - x1) + x1;
            let cy = (y as f64 / IMAGE_SIZE as f64) * (y2 - y1) + y1;

            let cnt = mandelbrot(cx, cy);

            buffer[y * IMAGE_SIZE + x] = (cnt % 256) as u8;
        }
    }
    println!("render_mandelbrot elapsed: {}ms", now.elapsed().as_millis());

    save_image(&buffer, "test_output/mandelbrot.png");
}

fn save_image(buffer: &[u8], filename: &str) {
    let mut image = image::ImageBuffer::new(IMAGE_SIZE as u32, IMAGE_SIZE as u32);
    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE {
            let index = y * IMAGE_SIZE + x;
            let color = buffer[index];
            let color = image::Rgb([color, color, color]);
            image.put_pixel(x as u32, y as u32, color);
        }
    }
    image.save(filename).unwrap();
}

fn render_mandelbrot_simd() {
    let mut buffer = vec![128u8; IMAGE_SIZE * IMAGE_SIZE];

    let x1 = (0.0 - 0.8) * 2.5;
    let x2 = (1.0 - 0.8) * 2.5;
    let y1 = (0.0 - 0.5) * 2.5;
    let y2 = (1.0 - 0.5) * 2.5;

    let now = Instant::now();

    let x_init = (0..SIMD_LANES)
        .map(|i| i as f64)
        .collect::<Vec<_>>();
    let x_init = F64simd::from_slice(x_init.as_slice());

    for y in 0..IMAGE_SIZE {
        for x in 0..IMAGE_SIZE / SIMD_LANES {
            let cx = (x_init + F64simd::splat((x * SIMD_LANES) as f64)) / F64simd::splat(IMAGE_SIZE as f64);
            let cx = cx * F64simd::splat(x2 - x1) + F64simd::splat(x1);

            let cy = ((y as f64 / IMAGE_SIZE as f64) * (y2 - y1)) + y1;
            let cy = F64simd::splat(cy);

            let cnt = mandelbrot_simd(cx, cy);

            cnt.as_array()
                .iter()
                .enumerate()
                .for_each(|(i, &c)| {
                    buffer[y * IMAGE_SIZE + x * SIMD_LANES + i] = (c % 256) as u8;
                });
        }
    }
    println!("render_mandelbrot_simd elapsed: {}ms", now.elapsed().as_millis());

    save_image(&buffer, "test_output/mandelbrot_simd.png");
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