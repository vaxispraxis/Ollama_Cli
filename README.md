# open-ollama-cli

Minimal Rust CLI that reads a prompt from stdin, sends it to Ollama, streams the reply, prints everything, and exits.

## Usage

```bash
# CLI args
cargo run --release -- hello robot

# stdin (piping)
echo "Hello" | cargo run --release
```

## Configuration
- `OLLAMA_HOST` (default `http://localhost:11434`)
- `OLLAMA_HOSTS` (legacy/alt name, used if `OLLAMA_HOST` is unset)
- `OLLAMA_MODEL` (default `dolphin3:8b`)

## Shell Examples

### bash / zsh
```bash
export OLLAMA_HOST="http://localhost:11434"
export OLLAMA_MODEL="dolphin3:8b"
cargo run --release -- hello robot
```

### nushell
```nu
$env.OLLAMA_HOST = "http://localhost:11434"
$env.OLLAMA_MODEL = "dolphin3:8b"
cargo run --release -- hello robot
```

## Notes
- Expects Ollama running locally unless `OLLAMA_HOST` is set.
