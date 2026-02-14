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

fn run_resize(input: &str, output: &str, size: u32) -> std::time::Duration {
    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(["-i", input, "-o", output, "resize", "-s", &size.to_string()])
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli resize -s {size} failed");
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
fn resize_128() {
    let fixture = "tests/fixtures/resize/size_128.png";
    let output = "tests/fixtures/resize/size_128_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_resize("lena.png", output, 128);
    println!("resize 128 latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "resize 128 output differs from fixture"
    );

    let img = image::open(output).unwrap();
    assert_eq!(img.width(), 128);
    assert_eq!(img.height(), 128);

    std::fs::remove_file(output).ok();
}

#[test]
fn resize_256() {
    let fixture = "tests/fixtures/resize/size_256.png";
    let output = "tests/fixtures/resize/size_256_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_resize("lena.png", output, 256);
    println!("resize 256 latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "resize 256 output differs from fixture"
    );

    let img = image::open(output).unwrap();
    assert_eq!(img.width(), 256);
    assert_eq!(img.height(), 256);

    std::fs::remove_file(output).ok();
}

#[test]
fn resize_1024_noop() {
    let fixture = "tests/fixtures/resize/size_1024.png";
    let output = "tests/fixtures/resize/size_1024_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_resize("lena.png", output, 1024);
    println!("resize 1024 (no-op) latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "resize 1024 output differs from fixture"
    );

    // lena.png is 512x512, so resize to 1024 should be a no-op
    let img = image::open(output).unwrap();
    assert_eq!(img.width(), 512, "expected no-op: image should stay 512px wide");
    assert_eq!(img.height(), 512, "expected no-op: image should stay 512px tall");

    std::fs::remove_file(output).ok();
}
