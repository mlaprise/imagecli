use std::path::Path;
use std::process::Command;
use std::time::Instant;

fn imagecli_bin() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();
    // integration test binaries live in target/debug/deps/
    path.pop(); // remove test binary name
    path.pop(); // remove "deps"
    path.push("imagecli");
    path
}

fn run_blur(input: &str, output: &str, sigma: f32) -> std::time::Duration {
    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(["-i", input, "-o", output, "blur", "--sigma", &sigma.to_string()])
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli blur --sigma {sigma} failed");
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
fn blur_sigma_1() {
    let fixture = "tests/fixtures/blur/sigma_1.0.png";
    let output = "tests/fixtures/blur/sigma_1.0_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_blur("lena.png", output, 1.0);
    println!("blur sigma=1.0 latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "blur sigma=1.0 output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn blur_sigma_2() {
    let fixture = "tests/fixtures/blur/sigma_2.0.png";
    let output = "tests/fixtures/blur/sigma_2.0_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_blur("lena.png", output, 2.0);
    println!("blur sigma=2.0 latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "blur sigma=2.0 output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn blur_sigma_5() {
    let fixture = "tests/fixtures/blur/sigma_5.0.png";
    let output = "tests/fixtures/blur/sigma_5.0_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_blur("lena.png", output, 5.0);
    println!("blur sigma=5.0 latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "blur sigma=5.0 output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}
