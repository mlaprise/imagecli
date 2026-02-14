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

fn run_channel(input: &str, output: &str, channel: &str) -> std::time::Duration {
    let start = Instant::now();
    let status = Command::new(imagecli_bin())
        .args(["-i", input, "-o", output, "channel", channel])
        .status()
        .expect("failed to execute imagecli");
    let elapsed = start.elapsed();
    assert!(status.success(), "imagecli channel {channel} failed");
    elapsed
}

fn images_are_identical(path_a: &str, path_b: &str) -> bool {
    let a = image::open(path_a).expect("failed to open image A");
    let b = image::open(path_b).expect("failed to open image B");

    let a = a.as_bytes();
    let b = b.as_bytes();

    a == b
}

#[test]
fn channel_red() {
    let fixture = "tests/fixtures/channel/red.png";
    let output = "tests/fixtures/channel/red_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_channel("lena.png", output, "red");
    println!("channel red latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "channel red output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn channel_green() {
    let fixture = "tests/fixtures/channel/green.png";
    let output = "tests/fixtures/channel/green_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_channel("lena.png", output, "green");
    println!("channel green latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "channel green output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}

#[test]
fn channel_blue() {
    let fixture = "tests/fixtures/channel/blue.png";
    let output = "tests/fixtures/channel/blue_actual.png";
    assert!(Path::new(fixture).exists(), "fixture missing: {fixture}");

    let elapsed = run_channel("lena.png", output, "blue");
    println!("channel blue latency: {elapsed:?}");

    assert!(
        images_are_identical(fixture, output),
        "channel blue output differs from fixture"
    );
    std::fs::remove_file(output).ok();
}
