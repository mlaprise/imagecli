use image::DynamicImage;

/// Build a natural cubic spline through a set of (x, y) control points.
/// Returns coefficients (a, b, c, d) for each segment where:
///   S_i(x) = a_i + b_i*(x - x_i) + c_i*(x - x_i)^2 + d_i*(x - x_i)^3
pub(crate) fn cubic_spline_coeffs(xs: &[f64], ys: &[f64]) -> Vec<(f64, f64, f64, f64)> {
    let n = xs.len() - 1;
    let mut h = vec![0.0; n];
    for i in 0..n {
        h[i] = xs[i + 1] - xs[i];
    }

    // Solve tridiagonal system for c coefficients (natural spline: c[0] = c[n] = 0)
    let mut alpha = vec![0.0; n + 1];
    for i in 1..n {
        alpha[i] = (3.0 / h[i]) * (ys[i + 1] - ys[i]) - (3.0 / h[i - 1]) * (ys[i] - ys[i - 1]);
    }

    let mut l = vec![1.0; n + 1];
    let mut mu = vec![0.0; n + 1];
    let mut z = vec![0.0; n + 1];
    for i in 1..n {
        l[i] = 2.0 * (xs[i + 1] - xs[i - 1]) - h[i - 1] * mu[i - 1];
        mu[i] = h[i] / l[i];
        z[i] = (alpha[i] - h[i - 1] * z[i - 1]) / l[i];
    }

    let mut c = vec![0.0; n + 1];
    let mut b = vec![0.0; n];
    let mut d = vec![0.0; n];
    for j in (0..n).rev() {
        c[j] = z[j] - mu[j] * c[j + 1];
        b[j] = (ys[j + 1] - ys[j]) / h[j] - h[j] * (c[j + 1] + 2.0 * c[j]) / 3.0;
        d[j] = (c[j + 1] - c[j]) / (3.0 * h[j]);
    }

    (0..n).map(|i| (ys[i], b[i], c[i], d[i])).collect()
}

/// Evaluate the cubic spline at a given x value.
pub(crate) fn cubic_spline_eval(xs: &[f64], coeffs: &[(f64, f64, f64, f64)], x: f64) -> f64 {
    // Find the right segment
    let n = coeffs.len();
    let i = if x <= xs[0] {
        0
    } else if x >= xs[n] {
        n - 1
    } else {
        let mut i = 0;
        for j in 1..n {
            if x < xs[j] {
                break;
            }
            i = j;
        }
        i
    };

    let dx = x - xs[i];
    let (a, b, c, d) = coeffs[i];
    a + b * dx + c * dx * dx + d * dx * dx * dx
}

/// Build a 256-entry LUT from a set of spline control points (on 0-100 scale).
pub(crate) fn build_curve_lut(xs: &[f64], ys: &[f64]) -> [u8; 256] {
    let coeffs = cubic_spline_coeffs(xs, ys);
    let mut lut = [0u8; 256];
    for i in 0..256 {
        let x = i as f64 / 255.0 * 100.0; // map 0-255 to 0-100
        let y = cubic_spline_eval(xs, &coeffs, x);
        let out = (y / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8;
        lut[i] = out;
    }
    lut
}

pub fn apply(img: DynamicImage, darks: i32, middarks: i32, mids: i32, midhighlights: i32, highlights: i32) -> DynamicImage {
    let xs = [0.0, 25.0, 50.0, 75.0, 100.0];
    let ys = [
        (0.0 + darks as f64).clamp(0.0, 100.0),
        (25.0 + middarks as f64).clamp(0.0, 100.0),
        (50.0 + mids as f64).clamp(0.0, 100.0),
        (75.0 + midhighlights as f64).clamp(0.0, 100.0),
        (100.0 + highlights as f64).clamp(0.0, 100.0),
    ];
    let lut = build_curve_lut(&xs, &ys);
    let mut rgb = img.to_rgb8();
    for pixel in rgb.pixels_mut() {
        pixel[0] = lut[pixel[0] as usize];
        pixel[1] = lut[pixel[1] as usize];
        pixel[2] = lut[pixel[2] as usize];
    }
    DynamicImage::ImageRgb8(rgb)
}
