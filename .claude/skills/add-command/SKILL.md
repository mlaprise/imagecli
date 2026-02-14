---
name: add-command
description: Add a new image processing subcommand to imagecli. Use when the user wants to implement a new image operation.
argument-hint: <command-name> [description]
disable-model-invocation: true
allowed-tools: Read, Edit, Write, Bash(cargo build *), Bash(cargo run *), Bash(cargo check *), Grep, Glob
---

# Add Command

Add a new image processing subcommand to the imagecli CLI.

## Steps

1. **Read `src/main.rs`** to understand the current structure, the `Command` enum, and existing implementations.
2. **Add a variant** to the `Command` enum with clap derive attributes for the new args.
3. **Implement the processing logic** in the `match cli.command` block in `main()`. The match arm must produce a `DynamicImage`.
4. **Build**: Run `cargo build --release` to verify it compiles.
5. **Test**: Run the new command on `lena.png` (512x512 test image) and verify the output visually.
6. **Update CLAUDE.md**: Add the new command to the Commands table and add a usage example.

## Conventions

- All operations take a `DynamicImage` and return a `DynamicImage`.
- Use the `image` crate for pixel manipulation.
- Numeric params: use `i32` for signed ranges, `u32` for unsigned, `f32` for floats.
- Follow existing patterns for default values and arg naming.
- Keep all code in `src/main.rs` (single-file architecture).
