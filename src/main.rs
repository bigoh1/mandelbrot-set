use image::{ImageBuffer, Rgb};
use colors_transform::{Color, Hsl};

use num::Complex;

use std::collections::HashMap;
type FloatType = f64;

// use rayon::prelude::*;

fn pixel_to_point(bounds: (u32, u32),
                  pixel: (u32, u32),
                  upper_left: &Complex<FloatType>,
                  lower_right: &Complex<FloatType>)
                  -> Complex<FloatType>
{
    let (width, height) = (&lower_right.re - &upper_left.re,
                           &upper_left.im - &lower_right.im);
    Complex {
        re: &upper_left.re + (pixel.0 as FloatType) * width  / (bounds.0 as FloatType),
        im: &upper_left.im - (pixel.1 as FloatType) * height / (bounds.1 as FloatType)
    }
}

fn escape_time(c: Complex<FloatType>, max_iter: usize) -> Option<usize> {
    // let c = pixel_to_point(img_shape, point, &upper_left, &lower_right);
    let mut z = Complex::<FloatType>::new(0 as FloatType, 0 as FloatType);
    let mut it_count = 0;

    while it_count < max_iter {
        if z.norm_sqr() > 4 as FloatType {
            return Some(it_count);
        }
        z = z * z + c;
        it_count += 1;
    }

    None
}

fn iter_to_rgb(iter_count: Option<usize>, max_iter: usize, color_mapping: &Vec<Rgb<u8>>) -> Rgb::<u8> {
    match iter_count {
        None => Rgb::<u8>::from([0, 0, 0]),
        Some(count) => {
            let mut mapping = [Rgb::<u8>::from([0, 0, 0]); 16];
            let i = count % color_mapping.len();
            return color_mapping[i];
        }
    }
}


fn main() {
    let max_iter = 10000;

    // 1.04180483110546, y = 0.346342664848392
    let x: FloatType = -0.7497065;
    let y: FloatType = 0.0314565;
    let r: FloatType = 0.001;

    let upper_left = Complex::<FloatType>::new(&x - &r, &y - &r);
    let lower_right = Complex::<FloatType>::new(&x + &r,  &y + &r);

    let scale = (&lower_right.im - &upper_left.im) / (&lower_right.re - &upper_left.re);
    
    let img_w = 1200;
    let img_h = (img_w as FloatType * scale).abs().ceil() as u32;

    let mut img = ImageBuffer::<Rgb<u8>, _>::new(img_w, img_h);

    let mut mapping = vec![Rgb::<u8>::from([0, 0, 0]); 16];
    mapping[0] = Rgb::<u8>::from([66, 30, 15]);
    mapping[1] = Rgb::<u8>::from([25, 7, 26]);
    mapping[2] = Rgb::<u8>::from([9, 1, 47]);
    mapping[3] = Rgb::<u8>::from([4, 4, 73]);
    mapping[4] = Rgb::<u8>::from([0, 7, 100]);
    mapping[5] = Rgb::<u8>::from([12, 44, 138]);
    mapping[6] = Rgb::<u8>::from([24, 82, 177]);
    mapping[7] = Rgb::<u8>::from([57, 125, 209]);
    mapping[8] = Rgb::<u8>::from([134, 181, 229]);
    mapping[9] = Rgb::<u8>::from([211, 236, 248]);
    mapping[10] = Rgb::<u8>::from([241, 233, 191]);
    mapping[11] = Rgb::<u8>::from([248, 201, 95]);
    mapping[12] = Rgb::<u8>::from([255, 170, 0]);
    mapping[13] = Rgb::<u8>::from([204, 128, 0]);
    mapping[14] = Rgb::<u8>::from([153, 87, 0]);
    mapping[15] = Rgb::<u8>::from([106, 52, 3]);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([0u8, 0u8, 0u8]);

        let c = pixel_to_point((img_w, img_h), (x, y), &upper_left, &lower_right);
        let iter_count = escape_time(c, max_iter);

        let rgb = iter_to_rgb(iter_count, max_iter, &mapping);
        *pixel = rgb;

        if x == 0 {
            println!("Progress: {}/{}", y, img_h);
        }
    }

    img.save("img.png").unwrap();
}
