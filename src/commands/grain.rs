use image::DynamicImage;

use crate::utils::smoothstep;

/// Deterministic hash-based noise: integer bit-mixing to produce [-1, 1].
fn hash(x: i64, y: i64, seed: u64) -> f64 {
    let mut h = (x as u64).wrapping_mul(374761393)
        ^ (y as u64).wrapping_mul(668265263)
        ^ seed.wrapping_mul(1274126177);
    h = h.wrapping_mul(1274126177);
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    // Map to [-1, 1]
    (h & 0xFFFFFFFF) as f64 / 2147483647.5 - 1.0
}

/// Value noise via bilinear interpolation of hash values at grid corners.
fn value_noise(x: f64, y: f64, cell_size: f64, seed: u64) -> f64 {
    let gx = (x / cell_size).floor();
    let gy = (y / cell_size).floor();
    let fx = x / cell_size - gx;
    let fy = y / cell_size - gy;

    let gx = gx as i64;
    let gy = gy as i64;

    let c00 = hash(gx, gy, seed);
    let c10 = hash(gx + 1, gy, seed);
    let c01 = hash(gx, gy + 1, seed);
    let c11 = hash(gx + 1, gy + 1, seed);

    // Smoothstep interpolation factors
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);

    let top = c00 + sx * (c10 - c00);
    let bot = c01 + sx * (c11 - c01);
    top + sy * (bot - top)
}

pub fn apply(img: DynamicImage, amount: u32, size: u32, roughness: u32, monochrome: bool) -> DynamicImage {
    let amount = amount.clamp(0, 100);
    let size = size.clamp(0, 100);
    let roughness = roughness.clamp(0, 100);

    let strength = amount as f64 / 100.0;
    let cell_size = 1.0 + (size as f64 / 100.0) * 4.0;
    let roughness_t = roughness as f64 / 100.0;

    let mut rgb = img.to_rgb8();

    // Per-channel seeds and sub-pixel offsets (emulsion layer misalignment)
    let channel_seeds: [u64; 3] = [42, 137, 251];
    let channel_offsets: [(f64, f64); 3] = [(0.0, 0.0), (0.37, 0.71), (-0.53, 0.29)];

    // Monochrome uses a single seed/offset for all channels
    let mono_seed: u64 = 42;

    for y in 0..rgb.height() {
        for x in 0..rgb.width() {
            let pixel = rgb.get_pixel(x, y);
            let lum = (0.2126 * pixel[0] as f64
                + 0.7152 * pixel[1] as f64
                + 0.0722 * pixel[2] as f64)
                / 255.0;

            // Luminance mask: grain peaks in midtones, suppressed in blacks/whites
            let mask = smoothstep(0.0, 0.25, lum) * (1.0 - smoothstep(0.75, 1.0, lum));

            // For monochrome: compute noise once, reuse for all channels
            let mono_noise = if monochrome {
                let smooth = value_noise(x as f64, y as f64, cell_size, mono_seed);
                let fine = hash(x as i64, y as i64, mono_seed);
                let grain = smooth + roughness_t * (fine - smooth);
                grain * strength * mask
            } else {
                0.0 // unused
            };

            let pixel = rgb.get_pixel_mut(x, y);
            for c in 0..3 {
                let noise = if monochrome {
                    mono_noise
                } else {
                    let (ox, oy) = channel_offsets[c];
                    let px = x as f64 + ox;
                    let py = y as f64 + oy;
                    let seed = channel_seeds[c];

                    // Smooth value noise (dye clouds)
                    let smooth = value_noise(px, py, cell_size, seed);
                    // Fine hash noise (silver halide grit)
                    let fine = hash(x as i64, y as i64, seed);
                    // Blend based on roughness
                    let grain = smooth + roughness_t * (fine - smooth);
                    grain * strength * mask
                };

                let v = pixel[c] as f64;

                // Density blend (subtractive compositing)
                let new_v = if noise > 0.0 {
                    // Darken
                    v - v * noise
                } else {
                    // Lighten
                    v + (255.0 - v) * noise.abs()
                };
                pixel[c] = new_v.round().clamp(0.0, 255.0) as u8;
            }
        }
    }

    DynamicImage::ImageRgb8(rgb)
}
