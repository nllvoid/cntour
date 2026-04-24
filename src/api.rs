use actix_web::{web, HttpResponse};

use crate::image_processing::values::{BlendedRequest, SingleRequest, HEIGHT, WIDTH};
use crate::image_processing::{color, generation, util};
use crate::image_processing::util::encode_png;
use crate::image_processing::blend::blend_noises;
use crate::image_processing::color::generate_random_palette;
use crate::image_processing::palettes::get_palette;

fn render(grayscale: &[u8], colored: String) -> Vec<u8> {
    if colored != "grayscale".to_string() {
        let palette: Option<Vec<[u8; 3]>> = match colored.as_str() {
            "random"                   => generate_random_palette(6).into(),
            colored                 => Some(get_palette(&colored)),
        };
        let img = color::grayscale_to_rgb_image(grayscale, WIDTH as u32, HEIGHT as u32, palette.expect("!"));
        encode_png(image::DynamicImage::ImageRgb8(img))
    } else {
        let img = util::grayscale_array_to_image(grayscale, WIDTH as u32, HEIGHT as u32);
        encode_png(image::DynamicImage::ImageLuma8(img))
    }
}

pub async fn generate_single(body: web::Json<SingleRequest>) -> HttpResponse {
    let body = body.into_inner();
    log::info!("POST /image/generate/single");
    log::info!("  noise:   {:?}", body.noise);
    log::info!("  colored: {:?}", body.colored);

    let grayscale = generation::fill_with_noise(body.noise.into());
    let colored = body.colored.unwrap_or("random".to_string());
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

    let grayscale = blend_noises(&pairs, body.blend_type, WIDTH.into(), HEIGHT.into());
    let colored = body.colored.unwrap_or("random".to_string());
    let png = render(&grayscale, colored);

    log::info!("  ✓ done, png {} bytes", png.len());
    HttpResponse::Ok().content_type("image/png").body(png)
}