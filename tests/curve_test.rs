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

fn run_curve(input: &str, output: &str, args: &[&str]) -> std::time::Duration {
    let mut cmd_args = vec!["-i", input, "-o", output, "curve"];
    cmd_args.extend_from_slice(args);

    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(&cmd_args)
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli curve {args:?} failed");
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
fn curve_s_contrast() {
    let fixture = "tests/fixtures/curve/s_curve.png";
    let output = "tests/fixtures/curve/s_curve_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_curve("lena.png", output, &["--darks=-15", "--highlights=15"]);
    println!("curve S-curve latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "curve S-curve output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn curve_faded() {
    let fixture = "tests/fixtures/curve/faded.png";
    let output = "tests/fixtures/curve/faded_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_curve("lena.png", output, &["--darks=20", "--highlights=-10"]);
    println!("curve faded/matte latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "curve faded output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn curve_full_5pt() {
    let fixture = "tests/fixtures/curve/full_5pt.png";
    let output = "tests/fixtures/curve/full_5pt_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_curve(
        "lena.png",
        output,
        &["--darks=-30", "--middarks=-15", "--mids=5", "--midhighlights=15", "--highlights=30"],
    );
    println!("curve full 5-point latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "curve full 5-point output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}
