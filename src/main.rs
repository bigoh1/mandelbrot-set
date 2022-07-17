use image::{ImageBuffer, Rgb};
use colors_transform::{Color, Hsl};

use num::Complex;
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

fn iter_to_rgb(iter_count: Option<usize>, max_iter: usize) -> [u8; 3] {
    match iter_count {
        None => [0, 0, 0],
        Some(count) => {
            let color = Hsl::from(255.0 * (count as f32 / max_iter as f32), 255.0, 255.0);
            let rgb = color.to_rgb().as_tuple();
            [rgb.0 as u8, rgb.1 as u8, rgb.2 as u8]
        }
    }
}


fn main() {
    let max_iter = 1000;
    let x: FloatType = 0.0;
    let y: FloatType = 0.0;
    let r: FloatType = 2.0;

    let upper_left = Complex::<FloatType>::new(&x - &r, &y - &r);
    let lower_right = Complex::<FloatType>::new(&x + &r,  &y + &r);

    let scale = (&lower_right.im - &upper_left.im) / (&lower_right.re - &upper_left.re);
    
    let img_w = 1200;
    let img_h = (img_w as FloatType * scale).abs().ceil() as u32;

    let mut img = ImageBuffer::<Rgb<u8>, _>::new(img_w, img_h);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([0u8, 0u8, 0u8]);

        let c = pixel_to_point((img_w, img_h), (x, y), &upper_left, &lower_right);
        let iter_count = escape_time(c, max_iter);

        let rgb = iter_to_rgb(iter_count, max_iter);
        *pixel = Rgb(rgb);

        if x == 0 {
            println!("Progress: {}/{}", y, img_h);
        }
    }

    img.save("img.png").unwrap();
}
