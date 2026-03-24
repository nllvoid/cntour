# cntour

A Rust API for generating noise-based images.

## Configuration

The server is configured via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Host address to bind to |
| `PORT` | `8080` | Port to listen on |
| `ALLOWED_ORIGIN` | `*` | CORS allowed origin. Set to a specific origin (e.g. `https://example.com`) to restrict access, or leave as `*` to allow any. |

```sh
HOST=0.0.0.0 PORT=8080 ALLOWED_ORIGIN=https://example.com cargo run
```

## Endpoints

The examples below use `$BASE_URL` — set it to your server's address, e.g.:

```sh
export BASE_URL=http://0.0.0.0:8080
```

---

### `GET /images/health`

Check API health status.

```sh
curl -X GET "$BASE_URL/images/health"
```

---

### `POST /image/generate/single`

Generate a single noise-based image.

```sh
curl -X POST "$BASE_URL/image/generate/single" \
  -H "Content-Type: application/json" \
  -d '{
    "noise": {
      "type": "perlin",
      "octaves": 2,
      "gain": 0.4,
      "lacunarity": 1.9,
      "seed": 99,
      "sharp": true
    },
    "colored": true
  }' \
  --output single.png
```
![Single](single.png)
---

### `POST /image/generate/blended`

Generate a blended noise-based image from multiple weighted layers (up to 16 layers).

```sh
curl -X POST "$BASE_URL/image/generate/blended" \
  -H "Content-Type: application/json" \
  -d '{
    "layers": [
      {
        "noise": { "type": "perlin", "octaves": 4, "gain": 0.5, "lacunarity": 2.0, "seed": 99, "sharp": false },
        "weight": 40.0
      },
      {
        "noise": { "type": "open_simplex", "seed": 1 },
        "weight": 60.0
      }
    ],
    "colored": true
  }' \
  --output blended.png
```
![Blended](blended.png)

---

## Noise Types

### `perlin`

Classic Perlin noise. Smooth gradient-based noise with good isotropy. Supports `sharp` mode which applies `fract(noise * 6.0)` — creating tight contour lines like light refracting through liquid.

| Parameter | Type | Description |
|-----------|------|-------------|
| `octaves` | `u32` | Number of noise layers stacked. More octaves = more detail. 2–4 recommended for liquid look. |
| `gain` | `f32` | Amplitude multiplier per octave. `0.5` is standard. Higher (0.6–0.7) = more mid-frequency detail, muddier. Lower (0.3–0.4) = dominant base wave, bigger blobs. |
| `lacunarity` | `f32` | Frequency multiplier per octave. `2.0` doubles frequency each octave. Lower (1.5–1.8) = bigger, slower features. |
| `seed` | `i32` | Random seed. Different seeds produce different spatial layouts of the same noise pattern. |
| `sharp` | `bool` | **Required.** Applies `fract(noise * 6.0)` to the output — produces repeating contour bands. Best used on one layer only in blended mode; multiple sharp layers cause interference artifacts. |

```json
{ "type": "perlin", "octaves": 2, "gain": 0.4, "lacunarity": 1.8, "seed": 99, "sharp": false }
```

---

### `value_cubic`

Cubic Value noise. Uses cubic interpolation between random grid values. Produces the smoothest, most liquid-like blobs of all noise types. Highly recommended as the primary layer for organic/liquid aesthetics.

| Parameter | Type | Description |
|-----------|------|-------------|
| `octaves` | `u32` | Number of noise layers stacked. 2 octaves gives the largest, smoothest blobs. |
| `gain` | `f32` | Amplitude multiplier per octave. `0.4` recommended — keeps higher octaves subtle so base blob shape dominates. |
| `lacunarity` | `f32` | Frequency multiplier per octave. `1.8` recommended for large smooth features. |
| `seed` | `i32` | Random seed. Use different seeds across blended layers to create color variation without changing structure. |

```json
{ "type": "value_cubic", "octaves": 2, "gain": 0.4, "lacunarity": 1.8, "seed": 99 }
```

---

### `simplex`

Simplex noise (Ken Perlin, 2001). Faster than Perlin with fewer directional artifacts. Slightly more angular than OpenSimplex. Good secondary layer for adding detail on top of ValueCubic.

| Parameter | Type | Description |
|-----------|------|-------------|
| `octaves` | `u32` | Number of stacked noise layers. |
| `gain` | `f32` | Amplitude multiplier per octave. |
| `lacunarity` | `f32` | Frequency multiplier per octave. |
| `seed` | `i32` | Random seed. |

```json
{ "type": "simplex", "octaves": 2, "gain": 0.4, "lacunarity": 1.8, "seed": 13 }
```

---

### `open_simplex`

OpenSimplex noise (Kurt Spencer, 2014). Created to avoid the Simplex patent. Smoother and more isotropic than Simplex — looks the same in all directions with no angular bias. Good as a smooth base or blending layer.

| Parameter | Type | Description |
|-----------|------|-------------|
| `seed` | `i32` | Random seed. |

```json
{ "type": "open_simplex", "seed": 77 }
```

---

### `cell_distance`

Worley / cellular noise based on distance to the nearest cell point. Low jitter produces smooth membrane-like boundaries (surface tension effect). High jitter produces chaotic, broken cell structures. Best used as a minor layer (5–15% weight) to add organic edge variation.

| Parameter | Type | Description |
|-----------|------|-------------|
| `jitter` | `f32` | Controls how randomly cell points are displaced from their grid positions. `0.0` = perfect grid. `1.0` = fully random. `0.3–0.5` recommended for liquid look. |

```json
{ "type": "cell_distance", "jitter": 0.35 }
```