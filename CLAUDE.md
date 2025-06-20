# Hex Pop - Game Development Project

## Project Overview
**Hex Pop** is a hexagon-based puzzle game built with Bevy v0.16.1, targeting web browsers (WebAssembly). It combines Tetris-like falling mechanics with match-3 puzzle elements using a hexagonal grid system.

## Game Mechanics (Current Design)
- **Falling Groups**: 3 colored hexagons fall from the top in a triangular formation
- **Player Controls**: 
  - Rotate the 3 hexes around each other (A/D keys)
  - Move entire group left/right (arrow keys)
  - Slam down for instant drop (S key)
- **Physics**: Individual hexes land and stack with simple gravity
- **Matching**: Groups of 3+ adjacent same-colored hexes pop
- **Chain Reactions**: After clearing, remaining hexes fall to fill gaps
- **Game Over**: When stack reaches the top

## Current Progress

### ✅ Completed
- Basic Bevy project setup with web deployment (WASM)
- Hex coordinate system using axial coordinates (q, r)
- Complete game architecture outlined in code
- Basic spawning system: 3-hex groups appear every 3 seconds
- Dark neon theme with color palette:
  - Background: Very dark blue (#050514)
  - Hex bodies: Dark green (all hexes)  
  - Neon borders: Bright colors (#4deeea, #74ee15, #ffe700, #f000ff, #001eff)
- Two-layer hex rendering (dark body + bright neon border)

### 🚧 Currently Working On
- **spawn_falling_groups**: Implemented (timer-based spawning)
- **spawn_hex_group**: Implemented (helper function for spawning logic)
- All other systems are stubbed with TODO comments

### ⏳ Next Priorities
1. **falling_group_physics**: Make groups fall with gravity
2. **handle_input**: Implement rotation and movement controls
3. **check_group_landing**: Collision detection with stack/ground
4. **lock_individual_hexes**: Convert falling group to individual stacked hexes

## Technical Architecture

### Key Components
```rust
struct HexCoord { q: i32, r: i32 }           // Axial hex coordinates
struct FallingGroup {                        // Group of 3 falling hexes
    hexes: [HexColor; 3],
    center_pos: Vec2,
    rotation: usize,
    fall_speed: f32,
    horizontal_speed: f32,
}
struct StackedHex { color: HexColor }        // Individual locked hex
struct GameState {                           // Game state resource
    stack: HashMap<HexCoord, HexColor>,
    next_group_timer: f32,
    game_speed: f32,
    score: u32,
}
```

### Update Systems (In Order)
1. `spawn_falling_groups` ✅ - Timer-based group spawning
2. `handle_input` ⏳ - A/D rotation, arrow keys, S slam
3. `falling_group_physics` ⏳ - Gravity + movement
4. `check_group_landing` ⏳ - Collision detection
5. `lock_individual_hexes` ⏳ - Convert to individual hexes
6. `apply_individual_gravity` - Each hex finds resting place
7. `detect_matches` - Find 3+ connected same colors
8. `clear_matches` - Remove matched hexes
9. `cascade_gravity` - Fill gaps after clearing
10. `check_game_over` - Stack height detection

### Visual Design
- **Theme**: Dark cyberpunk/neon aesthetic
- **Assets**: White PNG images tinted in code
- **Current**: Two-layer approach (dark body + neon border)
- **Future**: Multiple image files for special hexes (faces, effects)

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
- Dark blue background
- After 3 seconds: triangle of 3 hexes appears at top
- Hexes have dark green bodies with bright neon borders (magenta, blue, cyan)
- Currently stationary (physics not implemented yet)

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

**Current Status**: Foundation complete, ready to implement core falling mechanics.
**Next Session**: Focus on `falling_group_physics` to see hexes actually fall and move.