use std::i32::{MAX, MIN};
use std::io::Cursor;
use image::{GrayImage, ImageFormat, Luma};
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn grayscale_array_to_image(data: &[u8], width: u32, height: u32) -> GrayImage {
    assert_eq!(
        data.len(),
        (width as usize) * (height as usize),
        "Grayscale data length must equal width * height"
    );
    
    let mut img = GrayImage::new(width, height);
    for (i, &val) in data.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        img.put_pixel(x, y, Luma([val]));
    }
    img
}

pub fn get_current_time() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time should go forward")
}

pub fn get_random_seed(seed: i32) -> i32 {
    if seed == -1 {
        SmallRng::seed_from_u64(get_current_time().as_millis() as u64)
            .random_range(MIN..MAX)
    } else {
        seed
    }
}

pub fn encode_png(img: image::DynamicImage) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Png).unwrap();
    buf.into_inner()
}