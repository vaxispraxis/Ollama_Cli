use std::io::{self, Read, Write};
use std::env;

use serde::Serialize;

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prefer CLI args as the prompt; fall back to stdin for piping.
    let args = env::args().skip(1).collect::<Vec<String>>();
    let mut prompt = if !args.is_empty() {
        args.join(" ")
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };
    if prompt.is_empty() {
        eprintln!("No input provided. Use CLI args or pipe stdin.");
        std::process::exit(1);
    }
    if prompt.ends_with('\n') {
        // Avoid sending trailing newline-only prompt.
        while prompt.ends_with('\n') || prompt.ends_with('\r') {
            prompt.pop();
            if prompt.ends_with('\r') {
                prompt.pop();
            }
        }
    }

    let host = env::var("OLLAMA_HOST")
        .or_else(|_| env::var("OLLAMA_HOSTS"))
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "http://localhost:11434".to_string());
    let model = env::var("OLLAMA_MODEL")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "dolphin3:8b".to_string());

    let payload = GenerateRequest {
        model: &model,
        prompt: &prompt,
        stream: true,
    };

    let url = format!("{}/api/generate", host.trim_end_matches('/'));
    let body = serde_json::to_string(&payload)?;
    let resp = match ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_string(&body)
    {
        Ok(r) => r,
        Err(ureq::Error::Status(code, r)) => {
            let text = r.into_string().unwrap_or_default();
            eprintln!("Ollama returned HTTP {}: {}", code, text);
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("Failed to reach Ollama at {}: {}", url, err);
            std::process::exit(1);
        }
    };

    // Stream line-delimited JSON and print response text as it arrives.
    let mut buffer = String::new();
    let mut stdout = io::stdout();
    let mut reader = resp.into_reader();

    loop {
        let mut chunk = [0u8; 4096];
        let n = reader.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        buffer.push_str(std::str::from_utf8(&chunk[..n])?);

        while let Some(pos) = buffer.find('\n') {
            let line = buffer[..pos].trim().to_string();
            buffer.drain(..pos + 1);
            if line.is_empty() {
                continue;
            }
            if let Ok(obj) = serde_json::from_str::<serde_json::Value>(&line) {
                if let Some(token) = obj.get("response").and_then(|v| v.as_str()) {
                    write!(stdout, "{}", token)?;
                    stdout.flush()?;
                }
            }
        }
    }

    Ok(())
}
