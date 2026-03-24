use crate::image_processing::util::get_current_time;
use image::{Rgb, RgbImage};
use rand::prelude::*;

pub fn generate_random_palette(count: u8) -> Vec<[u8; 3]> {
    let mut rng = SmallRng::seed_from_u64(get_current_time().as_millis() as u64);

    let hue_offset: f32 = rng.random_range(0.0..360.0);
    let saturation: f32 = rng.random_range(0.6..1.0); // always vibrant

    (0..count)
        .map(|i| {
            let hue = (hue_offset + (i as f32 / count as f32) * 360.0) % 360.0;
            let value: f32 = rng.random_range(0.4..1.0); // spread brightness

            let h = hue / 60.0;
            let s = saturation;
            let v = value;
            let f = h - h.floor();
            let p = v * (1.0 - s);
            let q = v * (1.0 - s * f);
            let t = v * (1.0 - s * (1.0 - f));

            let (r, g, b) = match h.floor() as u32 {
                0 => (v, t, p),
                1 => (q, v, p),
                2 => (p, v, t),
                3 => (p, q, v),
                4 => (t, p, v),
                _ => (v, p, q),
            };

            [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
        })
        .collect()
}

pub fn sort_palette_by_value(mut palette: Vec<[u8; 3]>) -> Vec<[u8; 3]> {
    let slice = palette.as_mut_slice();
    slice.sort_unstable_by(|a, b| {
        let rgb_a = prisma::Rgb::new(
            a[0] as f32 / 255.0,
            a[1] as f32 / 255.0,
            a[2] as f32 / 255.0,
        );
        let rgb_b = prisma::Rgb::new(
            b[0] as f32 / 255.0,
            b[1] as f32 / 255.0,
            b[2] as f32 / 255.0,
        );

        let hsv_a: prisma::Hsv<f32> = rgb_a.into();
        let hsv_b: prisma::Hsv<f32> = rgb_b.into();

        hsv_a
            .value()
            .partial_cmp(&hsv_b.value())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    palette
}

pub fn grayscale_to_rgb_image(data: &[u8], width: u32, height: u32) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    let palette = sort_palette_by_value(generate_random_palette(4));

    let stops = palette.len() - 1;
    for (i, &val) in data.iter().enumerate() {
        let t = val as f32 / 255.0 * stops as f32;
        let lo = (t.floor() as usize).min(stops - 1);
        let blend = t - lo as f32;

        let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * blend) as u8;
        let [r, g, b] = [0, 1, 2].map(|c| lerp(palette[lo][c], palette[lo + 1][c]));

        img.put_pixel((i as u32) % width, (i as u32) / width, Rgb([r, g, b]));
    }
    img
}
