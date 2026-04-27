use serde::Deserialize;
use crate::image_processing::noise::NoiseConfig;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BlendType {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "gradient")]
    Gradient,
    #[serde(rename = "screen")]
    Screen,
}

// TODO: Apply function to the noise
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FunctionType {
    #[serde(rename = "logarithm_e")]
    LogarithmE,
    #[serde(rename = "sin")]
    Sin,
    #[serde(rename = "cos")]
    Cos,
    #[serde(rename = "tan")]
    Tan,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlendedLayer {
    pub noise: NoiseConfig,
    pub weight: f32,
}

pub(crate) const HEIGHT: u16 = 2500;
pub(crate) const WIDTH: u16 = 2500;
