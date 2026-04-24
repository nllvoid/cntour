use crate::image_processing::values::BlendType;

fn linear(layers: &[(&[u8], f32)], i: usize) -> f32 {
    let total_w: f32 = layers.iter().map(|(_, w)| w).sum();
    let val: f32 = layers.iter().map(|(data, w)| data[i] as f32 * w).sum();
    val / total_w
}

fn gradient(layers: &[(&[u8], f32)], i: usize, x: usize, y: usize, width: usize, height: usize) -> f32 {
    let mut total_w: f32 = 0.0;
    let mut val: f32 = 0.0;

    for (data, user_w) in layers {
        let x1 = data[y * width + (x + 1).min(width - 1)] as f32;
        let x0 = data[y * width + x.saturating_sub(1)] as f32;
        let y1 = data[(y + 1).min(height - 1) * width + x] as f32;
        let y0 = data[y.saturating_sub(1) * width + x] as f32;

        let dx = (x1 - x0) / 2.0;
        let dy = (y1 - y0) / 2.0;
        let grad = (dx * dx + dy * dy).sqrt();

        let w = user_w * (grad + 0.0001);
        total_w += w;
        val += data[i] as f32 * w;
    }

    val / total_w
}

pub fn blend_noises(
    layers: &[(&[u8], f32)],
    blend_type: BlendType,
    width: usize,
    height: usize,
) -> Vec<u8> {
    let len = width * height;
    let mut combined = vec![0u8; len];

    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;

            let result = match blend_type {
                BlendType::Linear => linear(layers, i),
                BlendType::Gradient => gradient(layers, i, x, y, width, height),
            };

            combined[i] = result.clamp(0.0, 255.0) as u8;
        }
    }

    combined
}