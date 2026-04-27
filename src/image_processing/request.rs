use serde::Deserialize;
use crate::image_processing::noise::NoiseConfig;
use crate::image_processing::values::{BlendedLayer, BlendType};

#[derive(Deserialize, Debug)]
pub struct SingleRequest {
    pub noise: NoiseConfig,
    pub colored: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BlendedRequest {
    pub layers: Vec<BlendedLayer>,
    pub colored: Option<String>,
    pub blend_type: BlendType,
}