# Hex Pop

A hexagon-based puzzle game built with Bevy v0.16.1, designed to run in web browsers.

## 🎮 About

Hex Pop is a 2D puzzle game featuring hexagonal gameplay mechanics. Built with Rust and the Bevy game engine, it's optimized for web deployment with WebAssembly.

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [just](https://github.com/casey/just) command runner: `brew install just`
- [cargo-watch](https://github.com/watchexec/cargo-watch) for development: `cargo install cargo-watch`
- [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen) for web builds: `cargo install wasm-bindgen-cli`
- WebAssembly target: `rustup target add wasm32-unknown-unknown`

### Development Workflow

1. **Start the file watcher** (auto-rebuilds on code changes):
   ```bash
   just watch
   ```

2. **Start the web server** (in another terminal):
   ```bash
   just serve
   ```

3. **Open your browser** to `http://localhost:8000`

4. **Make changes** to your code and refresh the browser to see updates!

## 📋 Available Commands

Run `just` to see all available commands:

- `just dev` - One-time build and serve
- `just watch` - Auto-rebuild on file changes (release build)
- `just watch-debug` - Auto-rebuild with debug builds (faster compilation)
- `just serve` - Start web server for existing build
- `just build` - Native build for testing
- `just build-web` - Web build only
- `just clean` - Clean all build artifacts
- `just test` - Run tests
- `just check` - Quick code check without building