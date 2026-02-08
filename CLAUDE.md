# imagecli

A single-binary Rust CLI for image processing. Operations are composable via unix pipes.

## Build

```bash
cargo build --release
# binary: ./target/release/imagecli
```

## Architecture

- Single file: `src/main.rs`
- Dependencies: `clap` (CLI parsing with derive), `image` (image processing)
- All image operations are clap subcommands in the `Command` enum
- I/O: `-i <file>` for input (or stdin), `-o <file>` for output (or stdout as PNG)
- Piping works because stdin/stdout use PNG format by default

## Commands

| Command | Args | Description |
|---------|------|-------------|
| `blur` | `--sigma` (f32, default 2.0) | Gaussian blur |
| `unsharpen` | `--sigma` (f32, default 2.0), `--threshold` (i32, default 5) | Unsharp mask |
| `grayscale` | none | Convert to black and white |
| `resize` | `--output-size` / `-s` (u32, required) | Resize longest side to target; no-op if image is already smaller |
| `channel` | `red`/`green`/`blue` (positional) | Extract a single RGB channel as grayscale |
| `curve` | `--darks`, `--middarks`, `--mids`, `--midhighlights`, `--highlights` (i32, default 0 each) | Tone curve via 5-point cubic spline on a 0–100 scale; each arg shifts the control point up/down from identity |
| `vignette` | `--amount` / `-a` (i32, default -50), `--midpoint` / `-m` (u32, default 50), `--roundness` / `-r` (i32, default 0), `--feather` / `-f` (u32, default 50) | Lightroom-style vignette; amount: -100 darken to 100 lighten; midpoint: 0–100; roundness: -100 rect to 100 circle; feather: 0–100 |
| `show-curve` | same args as `curve` | Debug: renders a 256x256 plot of the tone curve (no input image needed); shows grid, identity diagonal, spline curve, and control points |

## Usage examples

```bash
# Single operation
imagecli -i input.png -o output.png blur --sigma 3

# Piped chain: grayscale then sharpen
imagecli -i input.png grayscale | imagecli unsharpen --sigma 3 --threshold 5 -o output.png

# Resize then extract red channel
imagecli -i input.png resize -s 256 | imagecli channel red -o output.png

# S-curve contrast boost
imagecli -i input.png -o output.png curve --darks=-15 --highlights=15

# Faded/matte film look
imagecli -i input.png -o output.png curve --darks=20 --highlights=-10

# Dark vignette with defaults
imagecli -i input.png -o output.png vignette

# Rectangular light vignette, tight midpoint
imagecli -i input.png -o output.png vignette --amount 50 --midpoint 30 --roundness -80

# Piped chain: vignette then contrast curve
imagecli -i input.png vignette | imagecli curve --darks=-10 --highlights=10 -o output.png

# Debug: visualize a tone curve (no input image needed)
imagecli -o plot.png show-curve --darks=-30 --middarks=-15 --midhighlights=15 --highlights=30
```

## Adding a new command

1. Add a variant to the `Command` enum with its args
2. Add the processing logic in the `match cli.command` block in `main()`
3. The function must return a `DynamicImage`

## Test image

`lena.png` — 512x512 RGB PNG used for testing.
