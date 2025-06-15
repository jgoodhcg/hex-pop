# Hex Pop Development Commands

# Default recipe - shows available commands
default:
    @just --list

# Build and serve for web development
dev:
    @echo "🔨 Building for web..."
    cargo build --target wasm32-unknown-unknown --release
    @echo "🕸️  Generating WASM bindings..."
    wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/hex-pop.wasm
    @echo "🚀 Starting server at http://localhost:8000"
    python3 -m http.server 8000

# Quick native build for testing
build:
    @echo "🔨 Building native..."
    cargo build

# Build only for web (no server)
build-web:
    @echo "🔨 Building for web..."
    cargo build --target wasm32-unknown-unknown --release
    @echo "🕸️  Generating WASM bindings..."
    wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/hex-pop.wasm
    @echo "✅ Web build complete!"

# Serve existing build
serve:
    @echo "🚀 Starting server at http://localhost:8000"
    python3 -m http.server 8000

# Clean everything
clean:
    @echo "🧹 Cleaning..."
    cargo clean
    rm -rf pkg/
    @echo "✅ Clean complete!"

# Run tests
test:
    cargo test

# Check for issues without building
check:
    cargo check

# Auto-rebuild web build on file changes (run server separately with 'just serve')
watch:
    @echo "👀 Watching for web build changes... (Ctrl+C to stop)"
    @echo "💡 Run 'just serve' in another terminal"
    @echo "⏰ First build is slow, incremental builds are much faster!"
    cargo watch --ignore pkg/ --ignore target/ -x 'build --target wasm32-unknown-unknown --release' -s 'wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/hex-pop.wasm'

# Fast debug web build watching (faster builds, larger files)
watch-debug:
    @echo "👀 Watching for debug web builds... (Ctrl+C to stop)"
    @echo "💡 Run 'just serve' in another terminal"
    @echo "🚀 Debug builds are faster but larger files"
    cargo watch --ignore pkg/ --ignore target/ -x 'build --target wasm32-unknown-unknown' -s 'wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/debug/hex-pop.wasm'