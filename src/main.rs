use image::{RgbImage, Rgb};
use std::{time::Instant, thread, sync::{Arc, Mutex}};

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    let img = Arc::new(Mutex::new(RgbImage::new(WIDTH, HEIGHT)));
    let pixel_todo: Arc<Mutex<(u32, u32)>> = Arc::new(Mutex::new((0, 0)));
    const MAX: u32 = 10000;

    let mut handles = vec![];

    let start_time = Instant::now();

    for _ in 0..12 {
        let img = Arc::clone(& img);
        let pixel_todo = Arc::clone(&pixel_todo);

        let handle = thread::spawn(move || { 
            loop {
                let mut m: u32 = 0;
                let mut z: (f64, f64) = (0.0, 0.0);
                let mut pix = pixel_todo.lock().unwrap();
                let x_y = pix.clone();

                pix.0 += 1;
                if pix.0 >= WIDTH {
                    pix.0 = 0;
                    pix.1 += 1;
                }
                if pix.1 >= HEIGHT {
                    break;
                }
                drop(pix);

                let c: (f64, f64) = (x_y.0 as f64 / WIDTH as f64 * 4.0 - 2.0, x_y.1 as f64 / HEIGHT as f64 * 4.0 - 2.0);

                while m < MAX {
                    if (z.0 * z.0 + z.1 * z.1) >= 4.0 { break; }

                    m += 1;

                    z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
                }

                let mut img_lock = img.lock().unwrap();

                if m == MAX {
                    img_lock.put_pixel(x_y.0, x_y.1, Rgb([0, 0, 0]));
                } else {
                    let v: u32 = 16;
                    let n: u8 = ((m % v) * 255 / v) as u8;
                    img_lock.put_pixel(x_y.0, x_y.1, Rgb([n, n, n]));
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let time_elapsed = start_time.elapsed();

    println!("Time taken: {}ms", time_elapsed.as_millis());

    let img_lock = img.lock().unwrap();

    img_lock.save("temp.bmp").unwrap();
}
