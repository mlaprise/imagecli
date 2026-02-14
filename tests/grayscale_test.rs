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

fn images_are_identical(path_a: &str, path_b: &str) -> bool {
    let a = image::open(path_a).expect("failed to open image A");
    let b = image::open(path_b).expect("failed to open image B");

    let a = a.as_bytes();
    let b = b.as_bytes();

    a == b
}

#[test]
fn grayscale_default() {
    let fixture = "tests/fixtures/grayscale/default.png";
    let output = "tests/fixtures/grayscale/default_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(["-i", "lena.png", "-o", output, "grayscale"])
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli grayscale failed");
    println!("grayscale latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "grayscale output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn grayscale_dimensions_preserved() {
    let output = "tests/fixtures/grayscale/dims_actual.png";

    let status = Command::new(imagecli_bin())
        .args(["-i", "lena.png", "-o", output, "grayscale"])
        .status()
        .expect("failed to execute imagecli");
    assert!(status.success());

    let original = image::open("lena.png").unwrap();
    let result = image::open(output).unwrap();
    assert_eq!(original.width(), result.width());
    assert_eq!(original.height(), result.height());
    std::fs::remove_file(output).ok();
}

#[test]
fn grayscale_idempotent() {
    let pass1 = "tests/fixtures/grayscale/idempotent_pass1.png";
    let pass2 = "tests/fixtures/grayscale/idempotent_pass2.png";

    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(["-i", "lena.png", "-o", pass1, "grayscale"])
        .status()
        .expect("failed to execute imagecli");
    assert!(status.success());

    let status = Command::new(imagecli_bin())
        .args(["-i", pass1, "-o", pass2, "grayscale"])
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success());
    println!("grayscale idempotent (2 passes) latency: {elapsed:?}");

    assert!(
        images_are_identical(pass1, pass2),
        "grayscale is not idempotent: second pass differs from first"
    );
    std::fs::remove_file(pass1).ok();
    std::fs::remove_file(pass2).ok();
}
