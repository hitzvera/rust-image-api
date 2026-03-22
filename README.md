# 🦀 Rust Image Processing API

High-performance image processing API built with Rust.

## Tech Stack

- **Framework:** Actix-web
- **Image Processing:** image crate
- **Async Runtime:** Tokio

## Features

- Image upload
- Resize/thumbnail generation
- Format conversion (JPEG, PNG, WebP)
- Compression optimization

## Performance Goals

- Target: 50k+ RPS for simple operations
- p99 latency < 50ms
- Memory usage < 50MB

## Getting Started

### Prerequisites

- Rust 1.70+
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/hitzvera/rust-image-api.git
cd rust-image-api

# Build
cargo build --release

# Run
cargo run
```

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/thumbnail` | Generate thumbnail |
| POST | `/api/v1/resize` | Resize image |
| POST | `/api/v1/convert` | Convert format |
| GET | `/health` | Health check |

## Benchmark

```bash
# Install wrk
brew install wrk  # macOS
apt install wrk   # Linux

# Run benchmark
wrk -t12 -c400 -d30s http://localhost:8080/health
```

## License

MIT

---

**Part of Python vs Rust Performance Comparison**
🐍 Python version: https://github.com/hitzvera/python-image-api
