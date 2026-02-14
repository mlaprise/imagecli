---
name: process-image
description: Apply image processing to a file using imagecli. Use when the user asks to edit, adjust, or transform an image (blur, sharpen, resize, color grade, curves, vignette, etc.), or to emulate a film stock look.
argument-hint: <input-file> [description of desired look]
allowed-tools: Bash(cargo run *), Bash(cargo build *), Read
---

# Process Image

Apply image processing to an image using the `imagecli` CLI.

## Workflow

1. **Understand the request**: Determine which imagecli commands and parameters achieve the desired look.
2. **Build the command**: Construct a `cargo run --release --` pipeline. Chain multiple operations with unix pipes if needed.
3. **Run it**: Execute the command. Use `-i <input>` for input and `-o <output>` for output.
4. **Verify**: Read the output image to visually confirm the result. If the image is larger than 512px, pipe through `resize -s 512` to a temp file for preview.
5. **Iterate**: If the result isn't right, adjust parameters and re-run.

## Film stock emulation

When the user asks to emulate a known film stock (e.g., Kodak Portra 400, Fuji Velvia 50, Ilford HP5, CineStill 800T, etc.):

1. **List the film's characteristics** before doing anything. Present a brief breakdown covering:
   - **Contrast**: overall curve shape (low, medium, high), shadow/highlight behavior
   - **Color palette**: dominant hues, color cast, warm/cool bias
   - **Saturation**: vivid or muted, which colors pop vs. which are subdued
   - **Grain**: fine or coarse (note: imagecli has no grain, so skip this in the pipeline)
   - **Tonal range**: how it handles blacks (crushed or lifted), highlights (rolled off or clipped)
   - **Signature look**: what makes this stock instantly recognizable

2. **Map each characteristic to imagecli commands**. Explain the mapping so the user can see the reasoning. For example:
   - Lifted blacks → `curve --darks=N`
   - Warm highlights → `color-grade --highlights-hue=30 --highlights-sat=N`
   - Muted saturation → `color --saturation=-N`

3. **Build and run the pipeline**, then verify as usual.

## Available commands

| Command | Key args | What it does |
|---------|----------|--------------|
| `blur` | `--sigma` (f32) | Gaussian blur |
| `unsharpen` | `--sigma`, `--threshold` | Sharpen via unsharp mask |
| `grayscale` | none | Black and white |
| `resize` | `-s` (u32) | Resize longest side |
| `channel` | `red`/`green`/`blue` | Extract single channel |
| `curve` | `--darks`, `--middarks`, `--mids`, `--midhighlights`, `--highlights` | Tone curve (5-point spline) |
| `color` | `--temperature`, `--tint`, `--vibrance`, `--saturation` | Color adjustments |
| `color-grade` | `--shadows-hue/sat/lum`, `--midtones-hue/sat/lum`, `--highlights-hue/sat/lum` | Split-tone color grading |
| `vignette` | `--amount`, `--midpoint`, `--roundness`, `--feather` | Vignette effect |

## Piping pattern

```bash
cargo run --release -- -i input.jpg command1 [args] | cargo run --release -- command2 [args] -o output.jpg
```

Only the first command in the pipe uses `-i`, only the last uses `-o`. Intermediate steps use stdin/stdout (PNG format).

## Common looks

- **Cinematic teal/orange**: `color-grade --shadows-hue=200 --shadows-sat=50 --highlights-hue=30 --highlights-sat=40`
- **Vintage/faded**: `curve --darks=20 --highlights=-10` piped with warm color and vignette
- **High contrast B&W**: `grayscale` piped with `curve --darks=-15 --highlights=15`
- **Warm golden hour**: `color --temperature=40 --vibrance=20`

## Output naming

Unless the user specifies an output filename, save as `<input_basename>_<effect>.<ext>` (e.g., `photo_warm.jpg`).
