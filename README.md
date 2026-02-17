# imagecli

A single-binary Rust CLI for image processing, designed to be used as a tool by AI agents. Operations are composable via unix pipes, making it easy for agents to chain transformations in a single shell command.

## Why

AI agents (like Claude Code) can process images, but they need simple, predictable CLI tools to do it. imagecli gives them:

- **Single binary, zero config** — `cargo build --release` and it's ready
- **Composable via pipes** — chain operations with `|`, no temp files needed
- **Deterministic args** — every parameter is a named flag with sensible defaults
- **stdin/stdout by default** — pipes use PNG format, so agents don't need to manage intermediate files

## Install

```bash
cargo build --release
# binary: ./target/release/imagecli
```

## Usage

```bash
# Single operation
imagecli -i input.png -o output.png blur --sigma 3

# Piped chain: grayscale then sharpen
imagecli -i input.png grayscale | imagecli unsharpen --sigma 3 --threshold 5 -o output.png

# Cinematic teal/orange color grade
imagecli -i input.png -o output.png color-grade --shadows-hue=200 --shadows-sat=50 --highlights-hue=30 --highlights-sat=40

# Vintage 70s look: faded curve + warm color + color grading + vignette
imagecli -i input.png curve --darks=35 --highlights=-20 \
  | imagecli color --temperature=30 --saturation=-15 \
  | imagecli color-grade --shadows-hue=30 --shadows-sat=30 --highlights-hue=45 --highlights-sat=20 \
  | imagecli vignette --amount -70 -o output.png
```

## Commands

| Command | Description |
|---------|-------------|
| `blur` | Gaussian blur |
| `unsharpen` | Unsharp mask (sharpen) |
| `grayscale` | Convert to black and white |
| `resize` | Resize longest side to target |
| `channel` | Extract a single RGB channel |
| `curve` | Tone curve via 5-point cubic spline |
| `color` | Temperature, tint, vibrance, saturation |
| `color-grade` | Split-tone shadows/midtones/highlights |
| `vignette` | Lightroom-style vignette |
| `show-curve` | Debug: render a tone curve plot |

Run `imagecli <command> --help` for detailed argument info.

## Using with AI agents

imagecli is built to be called by AI agents that need image processing capabilities. The CLI surface is intentionally simple: named flags, numeric values, and predictable behavior.

### Claude Code Skills

This project includes ready-made [Claude Code Skills](https://docs.anthropic.com/en/docs/claude-code/skills) in `.claude/skills/`:

- **`process-image`** — Claude automatically translates natural language requests ("make it warmer", "add a cinematic look") into imagecli commands, runs them, and verifies the result
- **`add-command`** — Scaffolds a new image processing subcommand (`/add-command`)
- **`preview`** — Quick visual inspection of an image file

To use them, open this project in Claude Code and ask it to process an image. The skills are picked up automatically.

### Other agents

Any agent with shell access can use imagecli. The key patterns:

```bash
# Input from file, output to file
imagecli -i input.jpg -o output.jpg <command> [args]

# Input from stdin, output to stdout (PNG format)
cat input.png | imagecli blur --sigma 2 | imagecli grayscale > output.png

# Chain operations with pipes
imagecli -i photo.jpg resize -s 1024 | imagecli color --temperature=20 -o result.jpg
```

### Film Emulation

You can tap into Claude builting knownledge of film caracteristic and convert this easily into a processing pipeline.

https://github.com/user-attachments/assets/bc833bbb-569b-4736-91f6-ce398e29bb47

## License

MIT
