---
name: process-image
description: Apply image processing to a file using imagecli, or preview/display an image. Use when the user asks to edit, adjust, transform, view, check, or verify an image (blur, sharpen, resize, color grade, curves, vignette, grain, etc.), or to emulate a film stock look.
argument-hint: <input-file> [description of desired look]
allowed-tools: Bash(cargo run *), Bash(cargo build *), Read, Write, Glob
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
   - **Grain**: fine or coarse, smooth dye clouds or sharp silver halide, monochrome or color noise
   - **Tonal range**: how it handles blacks (crushed or lifted), highlights (rolled off or clipped)
   - **Signature look**: what makes this stock instantly recognizable

2. **Map each characteristic to imagecli commands**. Explain the mapping so the user can see the reasoning. For example:
   - Lifted blacks → `curve --darks=N`
   - Warm highlights → `color-grade --highlights-hue=30 --highlights-sat=N`
   - Muted saturation → `color --saturation=-N`
   - Fine silver halide grain → `grain --amount=45 --size=15 --roughness=80`
   - Monochrome B&W grain → `grain --amount=N --monochrome`

3. **Build and run the pipeline**, then verify as usual.

## Available commands

| Command | Key args | What it does |
|---------|----------|--------------|
| `blur` | `--sigma` (f32) | Gaussian blur |
| `unsharpen` | `--sigma`, `--threshold` | Sharpen via unsharp mask |
| `grayscale` | none | Black and white |
| `resize` | `-s` (u32) | Resize longest side |
| `channel` | `red`/`green`/`blue` | Extract single channel |
| `curve` | `--darks`, `--middarks`, `--mids`, `--midhighlights`, `--highlights` | Tone curve (5-point spline, each shifts control point on 0-100 scale) |
| `color` | `--temperature`, `--tint`, `--vibrance`, `--saturation` | Color adjustments (-100 to 100) |
| `color-grade` | `--shadows-hue/sat/lum`, `--midtones-hue/sat/lum`, `--highlights-hue/sat/lum` | Split-tone color grading (hue 0-360, sat 0-100, lum -100 to 100) |
| `grain` | `--amount` / `-a`, `--size` / `-s`, `--roughness` / `-r`, `--monochrome` / `-M` | Film grain (amount/size/roughness 0-100, monochrome flag for B&W) |
| `vignette` | `--amount`, `--midpoint`, `--roundness`, `--feather` | Vignette effect (amount -100 to 100, others 0-100) |
| `show-curve` | same as `curve` | Debug: renders 256x256 curve plot (no input needed) |

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
- **Heavy fine-grained silver halide**: `grain --amount=60 --size=5 --roughness=90`
- **Soft coarse dye cloud grain**: `grain --amount=40 --size=80 --roughness=10`

## Previewing an image

When the user asks to view, check, or verify an image:

1. If the image is larger than 512px on its longest side, create a temporary thumbnail:
   ```bash
   cargo run --release -- -i <file> resize -s 512 -o /tmp/preview_thumb.png
   ```
   Then read `/tmp/preview_thumb.png`.

2. If the image is small enough (512px or less), read it directly.

3. Describe what you see: subject, colors, exposure, any notable qualities.

## Presets

Presets let users save and reuse processing pipelines. They are stored as JSON files in the `presets/` directory at the project root.

### Saving a preset

When the user likes a result and wants to save it (e.g., "save this as a preset", "remember this look"), or when you finish a film stock emulation:

1. Ask for a preset name if the user didn't provide one.
2. Write a JSON file to `presets/<name>.json` with this structure:

```json
{
  "name": "Vintage 70s",
  "description": "Faded warm look with lifted blacks and vignette",
  "pipeline": [
    { "command": "curve", "args": { "darks": 35, "highlights": -20 } },
    { "command": "color", "args": { "temperature": 30, "saturation": -15 } },
    { "command": "color-grade", "args": { "shadows-hue": 30, "shadows-sat": 30, "highlights-hue": 45, "highlights-sat": 20 } },
    { "command": "vignette", "args": { "amount": -70 } }
  ]
}
```

Rules:
- `pipeline` is an ordered array of steps, executed left-to-right via pipes.
- Each step has `command` (the imagecli subcommand name) and `args` (an object of only the non-default arguments).
- Omit arguments that are left at their default value.

### Applying a preset

When the user asks to apply a preset (e.g., "apply the vintage preset", "use my portra look"):

1. Read `presets/<name>.json`.
2. Convert the `pipeline` array into a piped `cargo run --release --` command chain:
   - First step gets `-i <input>`, last step gets `-o <output>`.
   - Each `args` object is expanded to `--key=value` flags.
3. Execute and verify as usual.

### Listing presets

When the user asks to list or see available presets, glob `presets/*.json` and display each preset's name and description.

## Output naming

Unless the user specifies an output filename, save as `<input_basename>_<effect>.<ext>` (e.g., `photo_warm.jpg`).
