use serde::Deserialize;

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