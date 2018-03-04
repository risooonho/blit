#![feature(test)]

extern crate blit;
extern crate image;
extern crate test;

use test::Bencher;

use blit::*;

const SIZE: usize = 1000;
const ITERATIONS: i32 = 100;

#[bench]
fn blit_buffer(b: &mut Bencher) {
    let mut buffer: Vec<u32> = vec![0; SIZE * SIZE];

    let img = image::open("examples/smiley_rgb.png").unwrap();
    let rgb = img.as_rgb8().unwrap();
    let blit = rgb.to_blit_buffer(Color::from_u32(0xFF00FF));

    b.iter(|| {
        for x in 0..ITERATIONS {
            blit.blit(&mut buffer, SIZE, (x, 0));
        }
    });
}

#[bench]
fn blit_buffer_rect(b: &mut Bencher) {
    let mut buffer: Vec<u32> = vec![0; SIZE * SIZE];

    let img = image::open("examples/smiley_rgb.png").unwrap();
    let rgb = img.as_rgb8().unwrap();
    let blit = rgb.to_blit_buffer(Color::from_u32(0xFF00FF));
    let size = blit.size();

    b.iter(|| {
        for x in 0..ITERATIONS {
            blit.blit_rect(&mut buffer, SIZE, (x, 0), (0, 0, size.0, size.1));
        }
    });
}

#[bench]
fn blit_buffer_exact_fit(b: &mut Bencher) {
    let img = image::open("examples/smiley_rgb.png").unwrap();
    let rgb = img.as_rgb8().unwrap();
    let blit = rgb.to_blit_buffer(Color::from_u32(0xFF00FF));

    let size = blit.size();
    let mut buffer: Vec<u32> = vec![0; (size.0 * size.1) as usize];

    b.iter(|| {
        for _ in 0..ITERATIONS {
            blit.blit(&mut buffer, SIZE, (0, 0));
        }
    });
}

#[bench]
fn blit_img_rgb(b: &mut Bencher) {
    let mut buffer: Vec<u32> = vec![0; SIZE * SIZE];

    let img = image::open("examples/smiley_rgb.png").unwrap();
    let rgb = img.as_rgb8().unwrap();

    b.iter(|| {
        for x in 0..ITERATIONS {
            rgb.blit(&mut buffer, SIZE, (x, 0), Color::from_u32(0xFF00FF));
        }
    });
}

#[bench]
fn blit_img_rgba(b: &mut Bencher) {
    let mut buffer: Vec<u32> = vec![0; SIZE * SIZE];

    let img = image::open("examples/smiley_rgba.png").unwrap();
    let rgb = img.as_rgba8().unwrap();

    b.iter(|| {
        for x in 0..ITERATIONS {
            rgb.blit(&mut buffer, SIZE, (x, 0), Color::from_u32(0xFF00FF));
        }
    });
}