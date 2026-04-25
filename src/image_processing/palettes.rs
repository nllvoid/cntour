pub fn palette_aurora() -> Vec<[u8; 3]> {
    vec![
        [111, 45, 189],
        [166, 99, 204],
        [178, 152, 220],
        [184, 208, 235],
        [185, 250, 248],
    ]
}

pub fn palette_glacier() -> Vec<[u8; 3]> {
    vec![
        [3, 4, 94],
        [0, 119, 182],
        [0, 180, 216],
        [144, 224, 239],
        [202, 240, 248],
    ]
}

pub fn palette_neon() -> Vec<[u8; 3]> {
    vec![
        [247, 37, 133],
        [114, 9, 183],
        [58, 12, 163],
        [67, 97, 238],
        [76, 201, 240],
    ]
}

pub fn palette_ocean() -> Vec<[u8; 3]> {
    vec![
        [5, 102, 141],
        [2, 128, 144],
        [0, 168, 150],
        [2, 195, 154],
        [240, 243, 189],
    ]
}

pub fn palette_psychedelic() -> Vec<[u8; 3]> {
    vec![
        [255, 0, 80],
        [255, 140, 0],
        [255, 235, 0],
        [0, 200, 255],
        [60, 0, 255],
        [200, 0, 255],
    ]
}

pub fn palette_sunset() -> Vec<[u8; 3]> {
    vec![
        [57, 0, 153],
        [158, 0, 89],
        [255, 0, 84],
        [255, 84, 0],
        [255, 189, 0],
    ]
}

pub fn palette_viridis() -> Vec<[u8; 3]> {
    vec![
        [72, 33, 115],
        [46, 111, 142],
        [41, 175, 127],
        [189, 223, 38],
    ]
}

pub fn get_palette(name: &str) -> Vec<[u8; 3]> {
    let palettes = [
        "aurora",
        "glacier",
        "neon",
        "ocean",
        "psychedelic",
        "sunset",
        "viridis",
    ];

    assert_eq!(palettes.contains(&name), true, "Palette name not listed");

    match name {
        "aurora" => palette_aurora(),
        "glacier" => palette_glacier(),
        "neon" => palette_neon(),
        "ocean" => palette_ocean(),
        "psychedelic" => palette_psychedelic(),
        "sunset" => palette_sunset(),
        "viridis" => palette_viridis(),
        _ => palette_aurora(),
    }
}
