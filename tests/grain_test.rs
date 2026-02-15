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

fn run_grain(input: &str, output: &str, args: &[&str]) -> std::time::Duration {
    let mut cmd_args = vec!["-i", input, "-o", output, "grain"];
    cmd_args.extend_from_slice(args);

    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(&cmd_args)
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli grain {args:?} failed");
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
fn grain_default() {
    let fixture = "tests/fixtures/grain/default.png";
    let output = "tests/fixtures/grain/default_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_grain("lena.png", output, &[]);
    println!("grain default latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "grain default output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn grain_fine_sharp() {
    let fixture = "tests/fixtures/grain/fine_sharp.png";
    let output = "tests/fixtures/grain/fine_sharp_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_grain(
        "lena.png",
        output,
        &["--amount", "60", "--size", "5", "--roughness", "90"],
    );
    println!("grain fine sharp latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "grain fine sharp output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn grain_coarse_smooth() {
    let fixture = "tests/fixtures/grain/coarse_smooth.png";
    let output = "tests/fixtures/grain/coarse_smooth_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_grain(
        "lena.png",
        output,
        &["--amount", "40", "--size", "80", "--roughness", "10"],
    );
    println!("grain coarse smooth latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "grain coarse smooth output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}
