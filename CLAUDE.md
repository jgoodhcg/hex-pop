# Hex Pop - Game Development Project

## Project Overview
**Hex Pop** is a hexagon-based puzzle game built with Bevy v0.16.1, targeting web browsers (WebAssembly). It combines Tetris-like falling mechanics with match-3 puzzle elements using a hexagonal grid system.

## Game Mechanics (Current Design)
- **Falling Triangles**: 3-hex triangular groups fall from the top with smooth vertical movement
- **Player Controls**: 
  - Left/Right arrows: Move triangle horizontally (snaps to grid columns)
  - Down arrow: Fast drop (increased fall speed)
  - Space bar: Cycle hex positions in triangle (rotation without sprite rotation)
- **Collision System**: Entire triangle stops falling when any hex touches the grid or landed pieces
- **Hex Physics**: After landing, individual hexes fall following point-top hex rules:
  - Fall until supported by grid bottom OR two adjacent pieces below
  - If only one piece below, slide to unsupported side and continue falling
- **Settling Animation**: 200-300ms smooth movement to final resting positions
- **Matching**: 3+ hexes of same type in any direction clear (lines, not just clusters)
- **Chain Reactions**: After clearing, remaining hexes fall and settle using hex physics
- **Game Over**: When stack reaches the spawn area

## Current Progress

### ✅ Completed
- Basic Bevy project setup with web deployment (WASM)
- Hex coordinate system using axial coordinates (q, r)
- Windows 95 theme with gray background and themed hex assets
- Grid system: 6×12 hex tessellation with proper spacing and centering
- Mobile-optimized sizing with grid positioned for falling piece space
- Three Windows 95 hex types: Blank, CLI, Corner (based on asset filenames)
- Perfect grid-to-rectangle fitting with padding

### 🚧 Currently Working On
- Triangle falling mechanics system design
- Input handling system for keyboard controls
- Collision detection for triangle-to-grid interactions
- Hex physics for post-landing settling

### ⏳ Next Priorities
1. **spawn_falling_triangles**: Create triangular falling pieces above grid
2. **handle_triangle_input**: Left/right movement, fast drop, position cycling
3. **apply_triangle_gravity**: Smooth vertical falling with collision detection
4. **settle_landed_pieces**: Convert triangles to grid pieces with hex physics
5. **animate_settling**: Smooth movement to final resting positions

## Technical Architecture

### Key Components
```rust
struct HexCoord { q: i32, r: i32 }           // Axial hex coordinates
struct FallingTriangle {                     // Triangle of 3 falling hexes
    hexes: [HexType; 3],                     // Hex types in triangle
    grid_q: i32,                             // Horizontal grid position (snapped)
    world_y: f32,                            // Smooth vertical world position
    fall_speed: f32,                         // Current falling velocity
    position_cycle: usize,                   // Current hex arrangement (0-2)
}
struct StackedHex { hex_type: HexType }      // Individual landed hex
struct GameState {                           // Game state resource
    stack: HashMap<HexCoord, HexType>,       // Landed pieces on grid
    falling_triangle: Option<FallingTriangle>, // Active falling triangle
    spawn_timer: f32,                        // Time until next triangle
    score: u32,
}
struct SettlingHex {                         // Animating hex during physics
    hex_type: HexType,
    start_pos: HexCoord,
    target_pos: HexCoord,
    animation_timer: f32,
}
```

### Update Systems (In Order)
1. `spawn_falling_triangles` ⏳ - Timer-based triangle spawning above grid
2. `handle_triangle_input` ⏳ - Keyboard input for movement/cycling/fast drop
3. `apply_triangle_gravity` ⏳ - Smooth vertical falling with collision detection
4. `detect_triangle_collision` ⏳ - Check when triangle hits grid/pieces
5. `convert_triangle_to_grid` ⏳ - Place triangle hexes on grid, start settling
6. `apply_hex_physics` ⏳ - Individual hex falling with point-top rules
7. `animate_settling` ⏳ - Smooth movement to final positions
8. `detect_matches` ⏳ - Find 3+ connected same hex types
9. `clear_matches` ⏳ - Remove matched hexes
10. `cascade_physics` ⏳ - Re-apply hex physics after clearing
11. `check_game_over` ⏳ - Stack height detection

### Visual Design
- **Theme**: Windows 95 retro aesthetic with gray background
- **Assets**: Three themed hex images (blank, CLI, corner)
- **Grid**: 6×12 tessellated layout, mobile-optimized, positioned toward bottom
- **Animation**: Smooth falling and settling movements
- **UI**: Keyboard controls (future: on-screen buttons for mobile)

## Development Setup

### Prerequisites
- Rust (latest stable)
- `just` command runner: `brew install just`
- `cargo-watch`: `cargo install cargo-watch`
- `wasm-bindgen-cli`: `cargo install wasm-bindgen-cli`
- WASM target: `rustup target add wasm32-unknown-unknown`

### Development Workflow
```bash
just watch      # Auto-rebuild on changes (in one terminal)
just serve      # Start web server (in another terminal)
# Open http://localhost:8000
```

### Current Test
Run the game and you should see:
- Windows 95 light gray background
- Black rectangular play area positioned toward bottom of screen
- 6×12 grid of blank hex icons with coordinate labels
- Perfect tessellation within the black rectangle
- Grid ready for falling triangle implementation

## Planned Features

### Core Game Loop
- [ ] Physics: falling, collision, stacking
- [ ] Input: rotation, movement, slam
- [ ] Matching: flood fill algorithm for connected groups
- [ ] Scoring: points for matches, chain bonuses
- [ ] Progressive difficulty: increasing fall speed

### Special Hexes (Future)
- Bomb hexes (clear surrounding area)
- Rainbow hexes (match any color)
- Lightning hexes (clear entire row/column)
- Skull hexes (penalty mechanics)

### Polish (Future)
- Particle effects for matches/clearing
- Sound effects and music
- Animations for falling, rotating, clearing
- Mobile touch controls
- High score system

## Code Philosophy
- **Single file architecture**: Everything in `src/main.rs` for simplicity
- **Step-by-step development**: Implement one system at a time, test visually
- **Clean separation**: Systems handle timing/logic, helpers handle pure functionality
- **Future-proof**: Architecture supports special hexes and effects

## Current Challenges
1. **Hex positioning math**: Getting rotation and collision detection right
2. **ECS learning curve**: Understanding Bevy's entity-component-system
3. **Visual feedback**: Making sure each step has visible results
4. **Asset pipeline**: Deciding between single images with tinting vs multiple image files

## Resources
- [Bevy Documentation](https://bevy-cheatbook.github.io/)
- [Hex Grid Guide](https://www.redblobgames.com/grids/hexagons/)
- [Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#coordinates-axial)

---

**Current Status**: Grid system complete, ready to implement falling triangle mechanics.
**Next Session**: Implement triangle spawning, input handling, and basic falling physics.