use std::path::Path;
use std::process::Command;
use std::time::Instant;

fn imagecli_bin() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.push("imagecli");
    path
}

fn run_unsharpen(input: &str, output: &str, sigma: f32, threshold: i32) -> std::time::Duration {
    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args([
            "-i", input, "-o", output,
            "unsharpen", "--sigma", &sigma.to_string(), "--threshold", &threshold.to_string(),
        ])
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli unsharpen --sigma {sigma} --threshold {threshold} failed");
    elapsed
}

fn images_are_identical(path_a: &str, path_b: &str) -> bool {
    let a = image::open(path_a).expect("failed to open image A").to_rgb8();
    let b = image::open(path_b).expect("failed to open image B").to_rgb8();

    if a.dimensions() != b.dimensions() {
        return false;
    }

    a.pixels().zip(b.pixels()).all(|(pa, pb)| pa == pb)
}

#[test]
fn unsharpen_sigma_1_threshold_3() {
    let fixture = "tests/fixtures/unsharpen/sigma_1.0_threshold_3.png";
    let output = "tests/fixtures/unsharpen/sigma_1.0_threshold_3_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_unsharpen("lena.png", output, 1.0, 3);
    println!("unsharpen sigma=1.0 threshold=3 latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "unsharpen sigma=1.0 threshold=3 output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn unsharpen_sigma_2_threshold_5() {
    let fixture = "tests/fixtures/unsharpen/sigma_2.0_threshold_5.png";
    let output = "tests/fixtures/unsharpen/sigma_2.0_threshold_5_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_unsharpen("lena.png", output, 2.0, 5);
    println!("unsharpen sigma=2.0 threshold=5 latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "unsharpen sigma=2.0 threshold=5 output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn unsharpen_sigma_4_threshold_10() {
    let fixture = "tests/fixtures/unsharpen/sigma_4.0_threshold_10.png";
    let output = "tests/fixtures/unsharpen/sigma_4.0_threshold_10_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_unsharpen("lena.png", output, 4.0, 10);
    println!("unsharpen sigma=4.0 threshold=10 latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "unsharpen sigma=4.0 threshold=10 output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}
