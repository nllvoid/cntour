use crate::image_processing::util::get_random_seed;
use noise_functions::{
    CellDistanceSq, Noise, OpenSimplex2s, OpenSimplexNoise, Perlin, Simplex, ValueCubic,
};
use rayon::prelude::*;

pub(crate) const HEIGHT: u16 = 2500;
pub(crate) const WIDTH: u16 = 2500;
const SCALE: f32 = 0.001;
const CURL_MULTIPLIERS: [f32; 3] = [500.0, 300.0, 200.0];

#[derive(Debug)]
pub enum NoiseConfig {
    Perlin {
        octaves: u32,
        gain: f32,
        lacunarity: f32,
        seed: i32,
        sharp: bool,
        curl: bool,
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

    let perlin_samplers = if let NoiseConfig::Perlin { octaves, gain, lacunarity, .. } = &config {
        Some((
            Perlin.fbm(*octaves, *gain, *lacunarity).seed(resolved_seed),
            Perlin.fbm(*octaves, *gain, *lacunarity).seed(resolved_seed + 1),
            Perlin.fbm(*octaves, *gain, *lacunarity).seed(resolved_seed + 2),
        ))
    } else {
        None
    };

    let mut raw: Vec<f32> = (0..HEIGHT as usize * WIDTH as usize)
        .into_par_iter()
        .map(|i| {
            let x = (i % WIDTH as usize) as f32;
            let y = (i / WIDTH as usize) as f32;

            match &config {
                NoiseConfig::Perlin { sharp, curl, .. } => {
                    let (p0, p1, p2) = perlin_samplers.as_ref().unwrap();
                    let sample = p0.sample2([x * SCALE, y * SCALE]);

                    if *sharp && !*curl {
                        (sample * 4.0).fract()
                    } else if !*sharp && *curl {
                        let (vx1, vy1) = curl_perlin_cached(p0, x, y, 1.0, SCALE);
                        let (vx2, vy2) = curl_perlin_cached(
                            p1,
                            x + vx1 * CURL_MULTIPLIERS[0],
                            y + vy1 * CURL_MULTIPLIERS[0],
                            1.0,
                            SCALE * 3.0,
                        );
                        let (vx3, vy3) = curl_perlin_cached(
                            p2,
                            x + vx2 * CURL_MULTIPLIERS[1],
                            y + vy2 * CURL_MULTIPLIERS[1],
                            1.0,
                            SCALE * 8.0,
                        );

                        p0.sample2([
                            (x + vx3 * CURL_MULTIPLIERS[2]) * SCALE,
                            (y + vy3 * CURL_MULTIPLIERS[2]) * SCALE,
                        ])
                    } else if *sharp && *curl {
                        let (vx1, vy1) = curl_perlin_cached(p0, x, y, 1.0, SCALE);
                        let (vx2, vy2) = curl_perlin_cached(
                            p1,
                            x + vx1 * CURL_MULTIPLIERS[0],
                            y + vy1 * CURL_MULTIPLIERS[0],
                            1.0,
                            SCALE * 3.0,
                        );
                        let (vx3, vy3) = curl_perlin_cached(
                            p2,
                            x + vx2 * CURL_MULTIPLIERS[1],
                            y + vy2 * CURL_MULTIPLIERS[1],
                            1.0,
                            SCALE * 8.0,
                        );

                        (p0.sample2([
                            (x + vx3 * CURL_MULTIPLIERS[2]) * SCALE,
                            (y + vy3 * CURL_MULTIPLIERS[2]) * SCALE,
                        ]) * 4.0).fract()
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
                    .sample2([x * SCALE * 2.0, y * SCALE * 2.0]),
                NoiseConfig::CellDistance { jitter } => CellDistanceSq::default()
                    .jitter(*jitter)
                    .sample2([x * SCALE, y * SCALE]),
                NoiseConfig::ValueCubic { octaves, gain, lacunarity, .. } => ValueCubic
                    .fbm(*octaves, *gain, *lacunarity)
                    .seed(resolved_seed)
                    .sample2([x * SCALE, y * SCALE]),
                NoiseConfig::Simplex { octaves, gain, lacunarity, .. } => Simplex
                    .fbm(*octaves, *gain, *lacunarity)
                    .seed(resolved_seed)
                    .sample2([x * SCALE, y * SCALE]),
            }
        })
        .collect();

    raw = raw.iter().map(|&n| ((n + 1.0) * 0.5 * 6.0).fract()).collect();

    let min = raw.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = raw.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;

    raw.iter()
        .map(|v| (((v - min) / range) * 255.0).clamp(0.0, 255.0) as u8)
        .collect()
}

pub fn curl_perlin_cached<N>(noise: &N, x: f32, y: f32, epsilon: f32, vscale: f32) -> (f32, f32)
where
    N: noise_functions::Sample<2> + Sync,
{
    let vx = noise.sample2([x * vscale, y * vscale + epsilon])
        - noise.sample2([x * vscale, y * vscale - epsilon]);
    let vy = -(noise.sample2([x * vscale + epsilon, y * vscale])
        - noise.sample2([x * vscale - epsilon, y * vscale]));
    (vx, vy)
}