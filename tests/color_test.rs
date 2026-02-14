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

fn run_color(input: &str, output: &str, args: &[&str]) -> std::time::Duration {
    let mut cmd_args = vec!["-i", input, "-o", output, "color"];
    cmd_args.extend_from_slice(args);

    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(&cmd_args)
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli color {args:?} failed");
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
fn color_warm() {
    let fixture = "tests/fixtures/color/warm.png";
    let output = "tests/fixtures/color/warm_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_color("lena.png", output, &["--temperature=40"]);
    println!("color warm (temperature=40) latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "color warm output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn color_cool_desaturated() {
    let fixture = "tests/fixtures/color/cool_desat.png";
    let output = "tests/fixtures/color/cool_desat_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_color("lena.png", output, &["--saturation=-30", "--temperature=-20"]);
    println!("color cool+desat (saturation=-30 temperature=-20) latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "color cool+desat output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn color_vibrance_tint() {
    let fixture = "tests/fixtures/color/vibrance_tint.png";
    let output = "tests/fixtures/color/vibrance_tint_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_color("lena.png", output, &["--vibrance=60", "--tint=25"]);
    println!("color vibrance+tint (vibrance=60 tint=25) latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "color vibrance+tint output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}
