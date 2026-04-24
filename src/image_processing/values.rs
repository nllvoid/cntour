use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlendType {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "gradient")]
    Gradient,
}

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
    pub colored: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlendLayer {
    pub noise: NoiseConfigDto,
    pub weight: f32,
}

#[derive(Deserialize, Debug)]
pub struct BlendedRequest {
    pub layers: Vec<BlendLayer>,
    pub colored: Option<String>,
    pub blend_type: BlendType,
}

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

pub(crate) const HEIGHT: u16 = 2500;
pub(crate) const WIDTH: u16 = 2500;

