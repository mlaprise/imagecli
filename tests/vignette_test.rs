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

fn run_vignette(input: &str, output: &str, args: &[&str]) -> std::time::Duration {
    let mut cmd_args = vec!["-i", input, "-o", output, "vignette"];
    cmd_args.extend_from_slice(args);

    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(&cmd_args)
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli vignette {args:?} failed");
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
fn vignette_dark_default() {
    let fixture = "tests/fixtures/vignette/dark_default.png";
    let output = "tests/fixtures/vignette/dark_default_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_vignette("lena.png", output, &[]);
    println!("vignette default latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "vignette default output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn vignette_light_rectangular() {
    let fixture = "tests/fixtures/vignette/light_rect.png";
    let output = "tests/fixtures/vignette/light_rect_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_vignette(
        "lena.png",
        output,
        &["--amount", "50", "--midpoint", "30", "--roundness", "-80"],
    );
    println!("vignette light rectangular latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "vignette light rectangular output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn vignette_heavy_circular() {
    let fixture = "tests/fixtures/vignette/heavy_circle.png";
    let output = "tests/fixtures/vignette/heavy_circle_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_vignette(
        "lena.png",
        output,
        &["--amount", "-90", "--midpoint", "40", "--roundness", "80", "--feather", "70"],
    );
    println!("vignette heavy circular latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "vignette heavy circular output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}
