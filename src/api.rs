use actix_web::{web, HttpResponse};
use image::ImageFormat;
use serde::Deserialize;
use std::io::Cursor;

use crate::image_processing::generation::NoiseConfig;
use crate::image_processing::{color, generation, util};

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NoiseConfigDto {
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

impl From<NoiseConfigDto> for NoiseConfig {
    fn from(dto: NoiseConfigDto) -> Self {
        match dto {
            NoiseConfigDto::Perlin {
                octaves,
                gain,
                lacunarity,
                seed,
                sharp,
                curl,
            } => NoiseConfig::Perlin {
                octaves,
                gain,
                lacunarity,
                seed,
                sharp,
                curl,
            },
            NoiseConfigDto::OpenSimplex { seed } => NoiseConfig::OpenSimplex { seed },
            NoiseConfigDto::CellDistance { jitter } => NoiseConfig::CellDistance { jitter },
            NoiseConfigDto::ValueCubic {
                octaves,
                gain,
                lacunarity,
                seed,
            } => NoiseConfig::ValueCubic {
                octaves,
                gain,
                lacunarity,
                seed,
            },
            NoiseConfigDto::Simplex {
                octaves,
                gain,
                lacunarity,
                seed,
            } => NoiseConfig::Simplex {
                octaves,
                gain,
                lacunarity,
                seed,
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SingleRequest {
    pub noise: NoiseConfigDto,
    pub colored: Option<bool>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlendLayer {
    pub noise: NoiseConfigDto,
    pub weight: f32,
}

#[derive(Deserialize, Debug)]
pub struct BlendedRequest {
    pub layers: Vec<BlendLayer>,
    pub colored: Option<bool>,
}

fn encode_png(img: image::DynamicImage) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Png).unwrap();
    buf.into_inner()
}

fn render(grayscale: &[u8], colored: bool) -> Vec<u8> {
    let w = generation::WIDTH as u32;
    let h = generation::HEIGHT as u32;
    if colored {
        let img = color::grayscale_to_rgb_image(grayscale, w, h);
        encode_png(image::DynamicImage::ImageRgb8(img))
    } else {
        let img = util::grayscale_array_to_image(grayscale, w, h);
        encode_png(image::DynamicImage::ImageLuma8(img))
    }
}

pub async fn generate_single(body: web::Json<SingleRequest>) -> HttpResponse {
    let body = body.into_inner();
    log::info!("POST /image/generate/single");
    log::info!("  noise:   {:?}", body.noise);
    log::info!("  colored: {:?}", body.colored);

    let grayscale = generation::fill_with_noise(body.noise.into());
    let colored = body.colored.unwrap_or(false);
    let png = render(&grayscale, colored);

    log::info!("done, png {} bytes", png.len());
    HttpResponse::Ok().content_type("image/png").body(png)
}

pub async fn generate_blended(body: web::Json<BlendedRequest>) -> HttpResponse {
    let body = body.into_inner();
    log::info!("→ POST /image/generate/blended");
    log::info!("  layers:  {}", body.layers.len());
    for (i, layer) in body.layers.iter().enumerate() {
        log::info!("  layer[{}]: {:?} weight={}", i, layer.noise, layer.weight);
    }
    log::info!("  colored: {:?}", body.colored);

    if body.layers.is_empty() || body.layers.len() > 16 {
        log::info!("  ✗ invalid layer count");
        return HttpResponse::BadRequest().body("layers must be between 1 and 16");
    }

    let vecs: Vec<Vec<u8>> = body
        .layers
        .iter()
        .map(|l| generation::fill_with_noise(l.noise.clone().into()))
        .collect();

    let weights: Vec<f32> = body.layers.iter().map(|l| l.weight).collect();

    let pairs: Vec<(&[u8], f32)> = vecs
        .iter()
        .zip(weights.iter())
        .map(|(v, &w)| (v.as_slice(), w))
        .collect();

    let grayscale = util::blend_noises(&pairs);
    let colored = body.colored.unwrap_or(false);
    let png = render(&grayscale, colored);

    log::info!("  ✓ done, png {} bytes", png.len());
    HttpResponse::Ok().content_type("image/png").body(png)
}
