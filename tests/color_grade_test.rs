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

fn run_color_grade(input: &str, output: &str, args: &[&str]) -> std::time::Duration {
    let mut cmd_args = vec!["-i", input, "-o", output, "color-grade"];
    cmd_args.extend_from_slice(args);

    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(&cmd_args)
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli color-grade {args:?} failed");
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
fn color_grade_warm_shadows() {
    let fixture = "tests/fixtures/color-grade/warm_shadows.png";
    let output = "tests/fixtures/color-grade/warm_shadows_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_color_grade("lena.png", output, &["--shadows-hue=30", "--shadows-sat=60"]);
    println!("color-grade warm shadows latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "color-grade warm shadows output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn color_grade_teal_orange() {
    let fixture = "tests/fixtures/color-grade/teal_orange.png";
    let output = "tests/fixtures/color-grade/teal_orange_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_color_grade(
        "lena.png",
        output,
        &["--shadows-hue=200", "--shadows-sat=50", "--highlights-hue=30", "--highlights-sat=40"],
    );
    println!("color-grade teal/orange latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "color-grade teal/orange output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn color_grade_full() {
    let fixture = "tests/fixtures/color-grade/full_grade.png";
    let output = "tests/fixtures/color-grade/full_grade_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_color_grade(
        "lena.png",
        output,
        &[
            "--shadows-hue=240", "--shadows-sat=40", "--shadows-lum=-10",
            "--midtones-hue=120", "--midtones-sat=20", "--midtones-lum=5",
            "--highlights-hue=50", "--highlights-sat=35", "--highlights-lum=10",
        ],
    );
    println!("color-grade full (all ranges) latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "color-grade full output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}
