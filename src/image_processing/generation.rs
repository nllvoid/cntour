use crate::image_processing::util::get_random_seed;
use crate::image_processing::values::{NoiseConfig, HEIGHT, WIDTH};
use noise_functions::modifiers::{Fbm, Seeded};
use noise_functions::{
    CellDistanceSq, Noise, OpenSimplex2s, OpenSimplexNoise, Perlin, Simplex, ValueCubic,
};
use rayon::prelude::*;

const SCALE: f32 = 0.001;
const CURL_MULTIPLIERS: [f32; 3] = [500.0, 300.0, 200.0];

pub fn fill_with_noise(config: NoiseConfig) -> Vec<u8> {
    let resolved_seed = match &config {
        NoiseConfig::Perlin { seed, .. }
        | NoiseConfig::ValueCubic { seed, .. }
        | NoiseConfig::Simplex { seed, .. }
        | NoiseConfig::OpenSimplex { seed } => get_random_seed(*seed),
        NoiseConfig::CellDistance { .. } => 0,
    };

    let perlin_samplers = get_perlin_samplers(config.clone(), resolved_seed);

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
                        multiple_perlins_without_sharp(vec![p0, p1, p2], x, y)
                    } else if *sharp && *curl {
                        multiple_perlins_with_sharp(vec![p0, p1, p2], x, y)
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
                NoiseConfig::ValueCubic {
                    octaves,
                    gain,
                    lacunarity,
                    ..
                } => ValueCubic
                    .fbm(*octaves, *gain, *lacunarity)
                    .seed(resolved_seed)
                    .sample2([x * SCALE, y * SCALE]),
                NoiseConfig::Simplex {
                    octaves,
                    gain,
                    lacunarity,
                    ..
                } => Simplex
                    .fbm(*octaves, *gain, *lacunarity)
                    .seed(resolved_seed)
                    .sample2([x * SCALE, y * SCALE]),
            }
        })
        .collect();

    raw = raw
        .par_iter()
        .map(|&n| ((n + 1.0) * 0.5 * 6.0).fract())
        .collect();

    let min = raw
        .par_iter()
        .cloned()
        .reduce(|| f32::INFINITY, |a, b| a.min(b));
    let max = raw
        .par_iter()
        .cloned()
        .reduce(|| f32::NEG_INFINITY, |a, b| a.max(b));
    let range = max - min;

    raw.par_iter()
        .map(|&v| (((v - min) / range) * 255.0).clamp(0.0, 255.0) as u8)
        .collect()
}

fn curl_perlin_cached<N>(noise: &N, x: f32, y: f32, epsilon: f32, vscale: f32) -> (f32, f32)
where
    N: noise_functions::Sample<2> + Sync,
{
    let vx = noise.sample2([x * vscale, y * vscale + epsilon])
        - noise.sample2([x * vscale, y * vscale - epsilon]);
    let vy = -(noise.sample2([x * vscale + epsilon, y * vscale])
        - noise.sample2([x * vscale - epsilon, y * vscale]));
    (vx, vy)
}

fn multiple_perlins_without_sharp(perlins: Vec<&Seeded<Fbm<Perlin>>>, x: f32, y: f32) -> f32 {
    let (vx1, vy1) = curl_perlin_cached(perlins[0], x, y, 1.0, SCALE);
    let (vx2, vy2) = curl_perlin_cached(
        perlins[1],
        x + vx1 * CURL_MULTIPLIERS[0],
        y + vy1 * CURL_MULTIPLIERS[0],
        1.0,
        SCALE * 3.0,
    );
    let (vx3, vy3) = curl_perlin_cached(
        perlins[2],
        x + vx2 * CURL_MULTIPLIERS[1],
        y + vy2 * CURL_MULTIPLIERS[1],
        1.0,
        SCALE * 8.0,
    );

    perlins[1].sample2([
        (x + vx3 * CURL_MULTIPLIERS[2]) * SCALE,
        (y + vy3 * CURL_MULTIPLIERS[2]) * SCALE,
    ])
}
fn multiple_perlins_with_sharp(perlins: Vec<&Seeded<Fbm<Perlin>>>, x: f32, y: f32) -> f32 {
    let (vx1, vy1) = curl_perlin_cached(perlins[0], x, y, 1.0, SCALE);
    let (vx2, vy2) = curl_perlin_cached(
        perlins[1],
        x + vx1 * CURL_MULTIPLIERS[0],
        y + vy1 * CURL_MULTIPLIERS[0],
        1.0,
        SCALE * 3.0,
    );
    let (vx3, vy3) = curl_perlin_cached(
        perlins[2],
        x + vx2 * CURL_MULTIPLIERS[1],
        y + vy2 * CURL_MULTIPLIERS[1],
        1.0,
        SCALE * 8.0,
    );

    (perlins[0].sample2([
        (x + vx3 * CURL_MULTIPLIERS[2]) * SCALE,
        (y + vy3 * CURL_MULTIPLIERS[2]) * SCALE,
    ]) * 4.0)
        .fract()
}

fn get_perlin_samplers(
    config: NoiseConfig,
    resolved_seed: i32,
) -> Option<(
    Seeded<Fbm<Perlin>>,
    Seeded<Fbm<Perlin>>,
    Seeded<Fbm<Perlin>>,
)> {
    let perlin_samplers = if let NoiseConfig::Perlin {
        octaves,
        gain,
        lacunarity,
        ..
    } = &config
    {
        Some((
            Perlin.fbm(*octaves, *gain, *lacunarity).seed(resolved_seed),
            Perlin
                .fbm(*octaves, *gain, *lacunarity)
                .seed(resolved_seed + 1),
            Perlin
                .fbm(*octaves, *gain, *lacunarity)
                .seed(resolved_seed + 2),
        ))
    } else {
        None
    };
    perlin_samplers
}