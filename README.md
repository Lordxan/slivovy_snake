# Slivovy Snake

A classic Snake game built with **Rust** and [Bevy](https://bevyengine.org/) game engine, compiled to **WebAssembly** for native browser gameplay.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-2021%2B-blue)
![Bevy](https://img.shields.io/badge/Bevy-0.18-005a8d)
![Platform](https://img.shields.io/badge/platform-Web%2FWebAssembly-green)

## Features

- 🐍 **Classic Snake gameplay** on a 20×20 grid
- 🎮 **Smooth WASM-based rendering** with WebGL2 support
- ⚡ **Progressive speed increase** as you eat food
- 💾 **Persistent high scores** stored in browser local storage
- ✨ **Particle effects** for visual feedback
- 🎨 **Minimal dark aesthetic** with a retro-modern feel

## Architecture

The project follows Bevy's ECS (Entity Component System) plugin-based architecture:

| Module | Description |
|--------|-------------|
| `core` | Constants, utilities, and global state |
| `plugins/snake` | Snake movement, growth, and life cycle |
| `plugins/input` | Keyboard input handling (WASD / Arrow keys) |
| `plugins/rendering` | Canvas rendering pipeline |
| `plugins/collision` | Wall boundaries and self-collision detection |
| `plugins/food` | Food spawning and consumption |
| `plugins/particle` | Particle effect system |
| `plugins/game_events` | Game state transitions |
| `plugins/game_flow` | Start, pause, and game-over flow |
| `plugins/ui` | HUD, score, and game-over overlays |

## Prerequisites

- **Rust** (rustc 1.74+)
- **cargo**
- **wasm-bindgen** CLI tool

```bash
# Install wasm-bindgen if missing
cargo install wasm-bindgen-cli
```

## Building

### Development build (native — for testing logic)

```bash
cargo run
```

### Web / WebAssembly build

```bash
# Using Makefile (recommended)
make build-web

# Manual steps
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
  --out-dir ./web/pkg --out-name "slivovy_snake" \
  ./target/wasm32-unknown-unknown/release/slivovy_snake.wasm
```

## Running

### Local development server

```bash
# Using Makefile
make serve
# Opens http://localhost:8080

# Or manually
cd web
python3 -m http.server 8080
```

### Deploy to web

The compiled output lives in `web/pkg/`. These files can be served by any static HTTP server (Nginx, GitHub Pages, Cloudflare Pages, Vercel, etc.).

## Controls

| Action | Key |
|--------|-----|
| Move | **Arrow Keys** or **WASD** |
| Restart | (on game over) |

## Configuration

Key constants can be adjusted in `src/core/constants.rs`:

| Constant | Default | Description |
|----------|---------|-------------|
| `GRID_WIDTH` | 20 | Grid width in tiles |
| `GRID_HEIGHT` | 20 | Grid height in tiles |
| `MOVEMENT_INTERVAL` | 100ms | Base tick rate |
| `MIN_MOVEMENT_INTERVAL` | 50ms | Fastest possible tick rate |
| `SPEED_STEP_FOODS` | 5 | Food eaten before speed-up |
| `SPEED_STEP_MILLIS` | 8 | Ms subtracted per speed-up |
| `TILE_SIZE` | 32px | Tile pixel size |

## Build Pipeline

```
┌──────────────┐     ┌──────────────────┐     ┌──────────────┐
│  Rust Source │───▶│  Cargo (wasm32)  │───▶│    .wasm     │
└──────────────┘     └──────────────────┘     └──────┬───────┘
                                                      │
                                              wasm-bindgen
                                                      │
                                                      ▼
                                               ┌──────────────┐
                                               │  web/pkg/    │
                                               │  (JS + Wasm) │
                                               └──────┬───────┘
                                                      │
                                              Serve via HTTP
                                                      │
                                                      ▼
                                               ┌──────────────┐
                                               │   Browser    │
                                               │   (Player)   │
                                               └──────────────┘
```

## Project Structure

```
snake-slivovy/
├── Cargo.toml              # Rust project manifest
├── Makefile                # Build & serve automation
├── src/
│   ├── main.rs             # Application entry point
│   ├── core/               # Core modules & constants
│   │   ├── constants.rs    # Grid, colors, timing
│   │   ├── global.rs       # Global state
│   │   ├── mod.rs
│   │   └── utils.rs        # Helpers
│   └── plugins/            # Bevy plugins (game systems)
│       ├── mod.rs
│       ├── collision.rs
│       ├── food.rs
│       ├── game_events.rs
│       ├── game_flow.rs
│       ├── game.rs
│       ├── input.rs
│       ├── particle.rs
│       ├── rendering.rs
│       ├── snake.rs
│       └── ui.rs
└── web/
    ├── index.html          # Entry HTML
    └── pkg/                # [build output] WASM + JS bindings
```

## Development

```bash
# Run native build for fast iteration
cargo run

# Watch mode (rebuilds on source changes)
cargo run -- --nocapture

# Build web target
make build-web

# Serve web build
make serve
```

## License

This project is licensed under the [MIT License](LICENSE).

## Repository

[https://github.com/Lordxan/slivovy-snake](https://github.com/Lordxan/slivovy-snake)