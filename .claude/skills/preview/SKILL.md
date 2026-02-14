---
name: preview
description: Preview an image file by displaying it. Use when the user asks to view, check, or verify an image.
argument-hint: <image-file>
allowed-tools: Read, Bash(cargo run *)
---

# Preview Image

Display an image for visual inspection.

## Workflow

1. If the image is larger than 512px on its longest side, create a temporary thumbnail:
   ```bash
   cargo run --release -- -i $ARGUMENTS resize -s 512 -o /tmp/preview_thumb.png
   ```
   Then read `/tmp/preview_thumb.png`.

2. If the image is small enough (512px or less), read it directly.

3. Describe what you see: subject, colors, exposure, any notable qualities.
