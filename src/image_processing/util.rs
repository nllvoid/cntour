use image::{GrayImage, Luma};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::{rng, Rng, SeedableRng};
use rand::prelude::SmallRng;

pub fn blend_noises(layers: &[(&[u8], f32)]) -> Vec<u8> {
    assert!(!layers.is_empty(), "Need at least one layer");

    let len = layers[0].0.len();
    assert!(
        layers.iter().all(|(v, _)| v.len() == len),
        "All layers must be same length"
    );

    let total_weight: f32 = layers.iter().map(|(_, w)| w).sum();

    (0..len)
        .map(|i| {
            let val: f32 = layers.iter().map(|(v, w)| v[i] as f32 * w).sum::<f32>() / total_weight;
            val.clamp(0.0, 255.0) as u8
        })
        .collect()
}

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
            .random_range(-2147483648..2147483647)
    } else {
        seed
    }
}