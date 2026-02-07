# open-ollama-cli

Minimal Rust CLI that reads a prompt from stdin, sends it to Ollama, streams the reply, prints everything, and exits.

## Usage

```bash
# CLI args
cargo run --release -- hello robot

# stdin (piping)
echo "Hello" | cargo run --release
```

## Notes
- Expects Ollama running at `http://localhost:11434`.
- Uses model name `ollama` (change in `src/main.rs` if needed).
