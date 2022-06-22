use image::{GenericImageView, ImageBuffer, RgbImage, Rgb};
use num::{Complex, BigInt};

use num_rational::BigRational;
type FloatType = f64;

fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
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

fn main() {
    let max_iter = 1000;
    let x = -1.4 as FloatType;
    let y = 0 as FloatType;
    let r = 0.001 as FloatType;
    let upper_left = Complex::<FloatType>::new(&x - &r, &y - &r);
    let lower_right = Complex::<FloatType>::new(&x + &r,  &y + &r);

    // height / width
    let scale = (&lower_right.im - &upper_left.im) / (&lower_right.re - &upper_left.re);
    let img_w = 1200;
    let img_h = (img_w as FloatType * scale).abs().ceil() as u32;

    let mut img = ImageBuffer::<Rgb<u8>, _>::new(img_w, img_h);


    // fill with white
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([0u8, 0u8, 0u8]);



        let point = pixel_to_point((img_w as usize, img_h as usize), (x as usize, y as usize), &upper_left, &lower_right);
        let mut z = Complex::<FloatType>::new(0 as FloatType, 0 as FloatType);
        let mut it_count = 0;
        while it_count < max_iter {
            if z.norm() > 2 as FloatType {
                break;
            }
            z = z.powi(2) + &point;
            it_count += 1;
        }

        // let color = ((it_count as FloatType * 0.5 * std::f64::consts::PI / 390.0).sin() * 255.0) as u8;
        let color = 255u8 - (255.0 as FloatType * (it_count as FloatType /max_iter as FloatType)) as u8;

        *pixel = Rgb([color, color, color]);

        if x == 0 {
            println!("Progress: {}/{}", y, img_h);
        }
    }



    img.save("img.png").unwrap();
}
