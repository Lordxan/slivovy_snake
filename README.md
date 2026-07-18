# slivovy_snake

A Snake game built with **Rust** and **Bevy**, targeting **WebAssembly** for the web.

## Tech Stack

- **Rust** (2021 edition)
- **Bevy** 0.18.x — ECS-based game engine with **WebGL2** renderer
- **WebAssembly** (wasm32-unknown-unknown) — web deployment target
- **wasm-pack** — builds the game to WebAssembly

## Browser Compatibility

The game uses the **WebGL2** renderer, providing broad browser support:

| Browser  | Version |
|----------|---------|
| Chrome   | 56+     |
| Firefox  | 27+     |
| Edge     | 79+     |
| Safari   | 12+     |
| Opera    | 43+     |

WebGL2 is supported on virtually all modern desktop and mobile browsers, including iOS Safari and Android Chrome.

## Requirements

- Rust toolchain (rustc 1.80+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) — for web builds

## Quick Start

```bash
# Build the game to WebAssembly
make build-web

# Serve the game locally
make serve
```

Then open **http://localhost:8080** in your browser.

## Build

### One-command build (recommended)

```bash
make build-web
```

This runs `wasm-pack build --target web --out-dir pkg` which:
1. Compiles the game to `wasm32-unknown-unknown`
2. Generates JS glue code via `wasm-bindgen`
3. Outputs everything to `pkg/`

### Manual build

```bash
wasm-pack build --target web --out-dir pkg
```

### With cargo (raw wasm)

```bash
cargo build --release --target wasm32-unknown-unknown
# Output: target/wasm32-unknown-unknown/release/slivovy_snake.wasm
```

## Run Locally

### Option A: Python HTTP server

```bash
make serve
# or:
python3 -m http.server 8080 --directory web
```

### Option B: Node.js serve

```bash
npx serve web -p 8080
```

### Option C: Custom script

```bash
./server.sh 8080
```

### Option D: Rust basic-http-server

```bash
cargo install basic-http-server
basic-http-server web/
```

## Project Structure

```
slivovy_snake/
├── Cargo.toml              # Package + library config (cdylib, rlib)
├── Makefile                # Build + serve commands
├── README.md               # This file
├── server.sh               # Simple HTTP server script
├── .cargo/
│   └── config.toml         # Default wasm32-unknown-unknown target
├── web/
│   ├── index.html          # HTML entry point
│   └── index.js            # JS loader (backwards compat)
├── src/
│   ├── main.rs             # Thin wrapper — calls slivovy_snake::main()
│   ├── lib.rs              # Library entry point (used by wasm-pack)
│   ├── game.rs             # Game logic + visual sync systems
│   ├── spawn.rs            # Entity spawning (camera, grid, snake, food)
│   └── input.rs            # Keyboard input handling
└── target/
    └── wasm32-unknown-unknown/  # WebAssembly build output
```

## Rendering Architecture

- **20x20 grid** with 32px tiles rendered as a dark green background with subtle grid lines
- **Snake** as colored rectangles — bright green head (1.15x scale) and green body (0.92x scale)
- **Food** as a red square with custom size
- **Visuals sync** locked to the 100ms movement tick rate for smooth, deterministic updates
- **WebGL2** renderer via Bevy's `webgl2` feature — compatible with Chrome, Firefox, Edge, Safari, and Opera
