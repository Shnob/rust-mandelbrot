use image::{RgbImage, Rgb};
use std::time::Instant;

fn main() {
    let mut img = RgbImage::new(800, 800);
    const MAX: u32 = 1000;

    let start_time = Instant::now();

    for i in 0..img.width() {
        for j in 0..img.height() {
            
            let mut m: u32 = 0;
            let mut z: (f64, f64) = (0.0, 0.0);
            let c: (f64, f64) = (i as f64 / img.width() as f64 * 4.0 - 2.0, j as f64 / img.height() as f64 * 4.0 - 2.0);

            while m < MAX {
                if (z.0 * z.0 + z.1 * z.1) >= 4.0 { break; }

                m += 1;

                z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
            }

            if m == MAX {
                img.put_pixel(i, j, Rgb([0, 0, 0]));
            } else {
                let v: u32 = 16;
                let n: u8 = ((m % v) * 255 / v) as u8;
                img.put_pixel(i, j, Rgb([n, n, n]));
            }
        }
    }

    let time_elapsed = start_time.elapsed();

    println!("Time taken: {}ms", time_elapsed.as_millis());

    img.save("temp.bmp").unwrap();
}
