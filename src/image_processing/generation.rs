use noise_functions::{CellDistanceSq, Noise, OpenSimplex2s, OpenSimplexNoise, Perlin, Simplex, ValueCubic};
use crate::image_processing::util::get_random_seed;

pub(crate) const HEIGHT: u16 = 2500;
pub(crate) const WIDTH: u16 = 2500;
const WARP_SCALE: f32 = 0.003;
const SCALE: f32 = 0.001;

#[derive(Debug)]
pub enum NoiseConfig {
    Perlin {
        octaves: u32,
        gain: f32,
        lacunarity: f32,
        seed: i32,
        sharp: bool,
    },
    OpenSimplex {
        seed: i32,
    },
    CellDistance {
        jitter: f32,
    },
    ValueCubic {
        octaves: u32,
        gain: f32,
        lacunarity: f32,
        seed: i32,
    },
    Simplex {
        octaves: u32,
        gain: f32,
        lacunarity: f32,
        seed: i32,
    },
}

pub fn fill_with_noise(config: NoiseConfig) -> Vec<u8> {
    let resolved_seed = match &config {
        NoiseConfig::Perlin { seed, .. }
        | NoiseConfig::ValueCubic { seed, .. }
        | NoiseConfig::Simplex { seed, .. }
        | NoiseConfig::OpenSimplex { seed } => get_random_seed(*seed),
        NoiseConfig::CellDistance { .. } => 0,
    };

    let mut raw: Vec<f32> = Vec::with_capacity(HEIGHT as usize * WIDTH as usize);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let val = match &config {
                NoiseConfig::Perlin { octaves, gain, lacunarity, sharp, .. } => {
                    let sample = Perlin
                        .fbm(*octaves, *gain, *lacunarity)
                        .seed(resolved_seed)
                        .sample2([x as f32 * SCALE, y as f32 * SCALE]);
                    if *sharp {
                        (sample * 4.0).fract()
                    } else {
                        sample
                    }
                }
                NoiseConfig::OpenSimplex { .. } => OpenSimplex2s
                    .improve2_x()
                    .translate_xy(
                        OpenSimplex2s.seed(resolved_seed * 3),
                        OpenSimplex2s.seed(resolved_seed / 3),
                    )
                    .sample2([x as f32 * SCALE * 2.0, y as f32 * SCALE * 2.0]),
                NoiseConfig::CellDistance { jitter } => CellDistanceSq::default()
                    .jitter(*jitter)
                    .sample2([x as f32 * SCALE, y as f32 * SCALE]),
                NoiseConfig::ValueCubic { octaves, gain, lacunarity, .. } => ValueCubic
                    .fbm(*octaves, *gain, *lacunarity)
                    .seed(resolved_seed)
                    .sample2([x as f32 * SCALE, y as f32 * SCALE]),
                NoiseConfig::Simplex { octaves, gain, lacunarity, .. } => Simplex
                    .fbm(*octaves, *gain, *lacunarity)
                    .seed(resolved_seed)
                    .sample2([x as f32 * SCALE, y as f32 * SCALE]),
            };
            raw.push(val);
        }
    }

    raw = raw
        .iter()
        .map(|&n| {
            let remapped = (n + 1.0) * 0.5;
            (remapped * 6.0).fract()
        })
        .collect();

    let min = raw.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = raw.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;

    raw.iter()
        .map(|v| (((v - min) / range) * 255.0).clamp(0.0, 255.0) as u8)
        .collect()
}