use image::{RgbImage, Rgb};
use std::{time::Instant, thread, sync::{Arc, Mutex}, borrow::Borrow, ops::Rem};

fn main() {
    const WIDTH: u32 = 3840 * 1;
    const HEIGHT: u32 = 3840 * 1;
    let img = Arc::new(Mutex::new(RgbImage::new(WIDTH, HEIGHT)));
    let pixel_todo: Arc<Mutex<(u32, u32)>> = Arc::new(Mutex::new((0, 0)));
    const MAX: u32 = 2000;

    let colors: Arc<Vec<(f64, f64, f64)>> = Arc::new(vec![
        (228.0, 023.0, 232.0),
        (232.0, 123.0, 023.0),
        (027.0, 232.0, 023.0),
        (023.0, 132.0, 232.0),
    ]);

    let mut handles = vec![];

    let start_time = Instant::now();

    for _ in 0..12 {
        let img = Arc::clone(& img);
        let pixel_todo = Arc::clone(&pixel_todo);
        let colors = Arc::clone(&colors);

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

                //let mut img_lock = img.lock().unwrap();

                put_color(Arc::clone(&img), m, MAX, &x_y, colors.borrow()); 

            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let time_elapsed = start_time.elapsed();

    println!("Multi-thread:\n\tMax Samples: {}\n\tTime taken: {}ms", MAX, time_elapsed.as_millis());

    let img_lock = img.lock().unwrap();

    img_lock.save("temp.bmp").unwrap();
}

fn put_color(img: Arc<Mutex<RgbImage>>, m: u32, max: u32, x_y: &(u32, u32), colors: &Vec<(f64, f64, f64)>) {
    let v = (m as f64 / 10.0).rem(colors.len() as f64); 
    let d = v.rem(1.0);

    let c1 = colors.get(v as usize).unwrap();
    let c2 = colors.get((v as usize + 1) % colors.len()).unwrap();

    let mut img_lock = img.lock().unwrap();

    if m == max {
        img_lock.put_pixel(x_y.0, x_y.1, Rgb([0, 0, 0]));
    } else {
        img_lock.put_pixel(x_y.0, x_y.1, Rgb([
                                             ((1.0 - d) * c1.0 + d * c2.0) as u8,
                                             ((1.0 - d) * c1.1 + d * c2.1) as u8,
                                             ((1.0 - d) * c1.2 + d * c2.2) as u8
        ]));
    }
}
