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
| `color` | `--temperature` (i32, default 0), `--tint` (i32, default 0), `--vibrance` (i32, default 0), `--saturation` (i32, default 0) | Color adjustment; temperature: -100 cool/blue to 100 warm/amber; tint: -100 green to 100 magenta; vibrance: smart saturation preferring muted colors; saturation: linear -100 grayscale to 100 oversaturated |
| `color-grade` | `--shadows-hue` (u32, default 0), `--shadows-sat` (u32, default 0), `--shadows-lum` (i32, default 0), `--midtones-hue` (u32, default 0), `--midtones-sat` (u32, default 0), `--midtones-lum` (i32, default 0), `--highlights-hue` (u32, default 0), `--highlights-sat` (u32, default 0), `--highlights-lum` (i32, default 0) | Tint shadows/midtones/highlights independently; hue: 0–360 (color wheel); sat: 0–100 (tint strength); lum: -100 to +100 (brightness shift per range) |
| `grain` | `--amount` / `-a` (u32, default 25), `--size` / `-s` (u32, default 25), `--roughness` / `-r` (u32, default 50), `--monochrome` / `-M` (bool, default false) | Film grain simulation; amount: 0–100 intensity; size: 0 fine to 100 coarse; roughness: 0 smooth dye clouds to 100 sharp silver halide; monochrome: use identical noise on all channels (for B&W film) |
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

# Warm color shift
imagecli -i input.png -o output.png color --temperature=40

# Desaturate with cool tint
imagecli -i input.png -o output.png color --saturation=-30 --temperature=-20

# Vibrance boost (preserves already-saturated colors)
imagecli -i input.png -o output.png color --vibrance=60

# Warm shadows only
imagecli -i input.png -o output.png color-grade --shadows-hue=30 --shadows-sat=60

# Teal shadows + orange highlights (classic cinematic look)
imagecli -i input.png -o output.png color-grade --shadows-hue=200 --shadows-sat=50 --highlights-hue=30 --highlights-sat=40

# Vintage 70s look: faded curve + warm color + color grading + vignette
imagecli -i input.png curve --darks=35 --highlights=-20 | imagecli color --temperature=30 --saturation=-15 | imagecli color-grade --shadows-hue=30 --shadows-sat=30 --highlights-hue=45 --highlights-sat=20 | imagecli vignette --amount -70 -o output.png

# Subtle default film grain
imagecli -i input.png -o output.png grain

# Heavy fine-grained silver halide look
imagecli -i input.png -o output.png grain --amount 60 --size 5 --roughness 90

# Soft coarse dye cloud grain
imagecli -i input.png -o output.png grain --amount 40 --size 80 --roughness 10

# Monochrome B&W film grain (no color noise)
imagecli -i input.png -o output.png grain --amount 45 --size 15 --roughness 80 --monochrome

# Piped chain: vignette then contrast curve
imagecli -i input.png vignette | imagecli curve --darks=-10 --highlights=10 -o output.png

# Debug: visualize a tone curve (no input image needed)
imagecli -o plot.png show-curve --darks=-30 --middarks=-15 --midhighlights=15 --highlights=30
```

## Adding a new command

1. Add a variant to the `Command` enum with its args
2. Add the processing logic in the `match cli.command` block in `main()`
3. The function must return a `DynamicImage`

## Verifying result

If the user ask to verify the result, generate a small thumbnail (512x512) and analyse it.

## Test image

`lena.png` — 512x512 RGB PNG used for testing.
