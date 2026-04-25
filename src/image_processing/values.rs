use serde::Deserialize;

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

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
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

#[derive(Deserialize, Debug)]
pub struct SingleRequest {
    pub noise: NoiseConfig,
    pub colored: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlendLayer {
    pub noise: NoiseConfig,
    pub weight: f32,
}

#[derive(Deserialize, Debug)]
pub struct BlendedRequest {
    pub layers: Vec<BlendLayer>,
    pub colored: Option<String>,
    pub blend_type: BlendType,
}

pub(crate) const HEIGHT: u16 = 2500;
pub(crate) const WIDTH: u16 = 2500;
