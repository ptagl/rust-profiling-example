# Profiling Rust with `tracing`

A small Rust example showing how to generate **profiling** traces with `tracing` and inspect them with Perfetto UI.

## What it does

This demo instruments a Rust application with `profiling` and `tracing`, then exports a JSON trace that can be opened in:

- `https://ui.perfetto.dev/`
- `chrome://tracing`

## Run

```bash
cargo run --features tracing
```

After the program exits, a **trace** file such as `trace-*.json` will be generated in the current directory.

## Open the trace

Upload the generated JSON file to Perfetto UI:

https://ui.perfetto.dev/

## Notes
This project is intentionally **minimal** and is meant as a companion example for a [blog post](https://ptagl.github.io/blog/posts/rust-profiling-with-tracing) about profiling Rust applications with tracing.