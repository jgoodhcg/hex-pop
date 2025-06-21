# Hex Pop

A hexagon-based puzzle game built with Bevy v0.16.1, featuring Windows 95 retro aesthetics and running in web browsers.

## 🎮 Game Overview

Hex Pop combines Tetris-like falling mechanics with match-3 puzzle elements using a hexagonal grid system. Players control falling triangular groups of 3 hexes that land and settle according to simple physics rules.

### 🎯 Game Mechanics

- **Falling Triangles**: 3-hex triangular groups fall from above with smooth movement
- **Hex Physics**: Individual pieces follow point-top hexagon rules - fall until supported by grid bottom or two adjacent pieces below
- **Smart Settling**: If only one piece below, hexes slide to the unsupported side and continue falling
- **Type Matching**: Connect 3 hexes in any direction of the same type to clear them
- **Chain Reactions**: Clearing creates gaps, triggering physics cascades and potential chain matches

### 🎮 Controls

- **Left/Right Arrows**: Move triangle horizontally (snaps to grid columns)
- **Down Arrow**: Fast drop (increased fall speed)
- **Space Bar**: Cycle hex positions in triangle (rotation without sprite rotation)

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

### 📁 Project Structure

```
src/
├── main.rs          # Single-file architecture with all game systems
assets/
├── hex-w95-blank.png    # Windows 95 themed hex assets
├── hex-w95-cli.png
└── hex-w95-corner.png
```

## 🎯 Technical Details

- **Engine**: Bevy v0.16.1 with ECS architecture
- **Target**: WebAssembly for browser deployment
- **Grid**: 6×12 hexagonal tessellation using axial coordinates
- **Controls**: Keyboard-based (future: mobile touch support)
- **Theme**: Windows 95 retro styling with authentic color palette

