use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hex Pop".into(),
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_falling_triangles,
                handle_triangle_input,
                apply_triangle_gravity,
                detect_triangle_collision,
                convert_triangle_to_grid,
                apply_hex_physics,
                animate_settling,
                detect_matches,
                clear_matches,
                cascade_physics,
                check_game_over,
            ),
        )
        .run();
}

// ===== CONSTANTS =====

// Grid dimensions for mobile play
const GRID_WIDTH: i32 = 6;
const GRID_HEIGHT: i32 = 12;
const HEX_SIZE: f32 = 24.0; // radius, not width (reduced from 32.0 for mobile)
const GRID_VERTICAL_OFFSET: f32 = -80.0; // Negative moves grid down, positive moves up

// ===== DATA STRUCTURES =====

// Hex coordinate using axial coordinates
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32, // column
    r: i32, // row
}

// Windows 95 hex types based on asset filenames
#[derive(Debug, Clone, Copy, PartialEq)]
enum HexType {
    Blank,  // hex-w95-blank.png
    Cli,    // hex-w95-cli.png
    Corner, // hex-w95-corner.png
}

// Falling triangle of 3 hexes
#[derive(Component)]
struct FallingTriangle {
    hexes: [HexType; 3],      // Hex types in triangular formation
    grid_q: i32,              // Horizontal grid position (snapped to grid)
    world_y: f32,             // Smooth vertical world position
    fall_speed: f32,          // Current falling velocity
    position_cycle: usize,    // Current hex arrangement (0-2) for "rotation"
}

// Individual hex during settling animation
#[derive(Component)]
struct SettlingHex {
    hex_type: HexType,
    start_coord: HexCoord,
    target_coord: HexCoord,
    animation_timer: f32,
    animation_duration: f32,
}

// Individual stacked hex
#[derive(Component)]
struct StackedHex {
    hex_type: HexType,
}

// Game state resource
#[derive(Resource)]
struct GameState {
    stack: HashMap<HexCoord, HexType>,        // Landed pieces on grid
    triangle_spawn_timer: f32,                // Time until next triangle spawns
    has_falling_triangle: bool,               // Whether a triangle is currently falling
    settling_pieces: Vec<HexCoord>,           // Pieces currently animating to final positions
    game_speed: f32,                          // Game speed multiplier
    score: u32,                               // Player score
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stack: HashMap::new(),
            triangle_spawn_timer: 3.0,
            has_falling_triangle: false,
            settling_pieces: Vec::new(),
            game_speed: 1.0,
            score: 0,
        }
    }
}

// ===== IMPLEMENTATIONS =====

impl HexCoord {
    fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    fn to_world_pos(&self) -> Vec2 {
        axial_to_world(self.q, self.r)
    }

    fn neighbors(&self) -> Vec<HexCoord> {
        vec![
            HexCoord::new(self.q + 1, self.r),
            HexCoord::new(self.q - 1, self.r),
            HexCoord::new(self.q, self.r + 1),
            HexCoord::new(self.q, self.r - 1),
            HexCoord::new(self.q + 1, self.r - 1),
            HexCoord::new(self.q - 1, self.r + 1),
        ]
    }

    fn is_valid(&self) -> bool {
        // Check if coordinates are within grid bounds
        self.q >= 0 && self.q < GRID_WIDTH && self.r >= 0 && self.r < GRID_HEIGHT
    }

    fn lowest_supported_position(&self, stack: &HashMap<HexCoord, HexType>) -> HexCoord {
        // TODO: Find where this hex should fall to
        *self
    }
}

impl FallingTriangle {
    fn get_hex_world_positions(&self) -> [(Vec2, HexType); 3] {
        // TODO: Calculate world positions of 3 hexes based on grid_q, world_y, and position_cycle
        // TODO: Use triangular formation with current position_cycle arrangement
        [
            (Vec2::ZERO, self.hexes[0]),
            (Vec2::ZERO, self.hexes[1]),
            (Vec2::ZERO, self.hexes[2]),
        ]
    }

    fn cycle_positions(&mut self) {
        // TODO: Cycle the position arrangement (0->1->2->0)
        // TODO: This rotates which hex is in which position without rotating sprites
        self.position_cycle = (self.position_cycle + 1) % 3;
    }

    fn can_move_to_grid_q(&self, new_grid_q: i32, stack: &HashMap<HexCoord, HexType>) -> bool {
        // TODO: Check if triangle can move to new horizontal grid position
        // TODO: Ensure within grid bounds and no collision with existing pieces
        new_grid_q >= 0 && new_grid_q < GRID_WIDTH
    }
}

impl HexType {
    fn to_image_path(&self) -> &'static str {
        match self {
            HexType::Blank => "hex-w95-blank.png",
            HexType::Cli => "hex-w95-cli.png",
            HexType::Corner => "hex-w95-corner.png",
        }
    }

    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => HexType::Blank,
            1 => HexType::Cli,
            _ => HexType::Corner,
        }
    }
}

// ===== HELPER FUNCTIONS =====

fn inner_radius() -> f32 {
    return HEX_SIZE * (3_f32.sqrt() / 2_f32);
}

/// Point-top axial grid bounding box (all hexes fully inside)
///
/// `size` = radius (centre → any corner)
/// returns (span_x, span_y)
fn board_span(size: f32, cols: i32, rows: i32) -> (f32, f32) {
    let w = cols as f32;
    let h = rows as f32;

    let span_x = w * size * 3_f32.sqrt() + ((2_f32 * inner_radius()) / 2_f32);

    let span_y = h * size * (3_f32 / 2_f32) + (0.25_f32 * ((3_f32 / 2_f32) * size));

    (span_x, span_y)
}

fn axial_to_world(q: i32, r: i32) -> Vec2 {
    let (span_x, span_y) = board_span(HEX_SIZE, GRID_WIDTH, GRID_HEIGHT);

    // Top-left corner of the board area (shifted down for falling piece space)
    let board_top_left = Vec2::new(-span_x / 2.0, span_y / 2.0 + GRID_VERTICAL_OFFSET);

    // Hex dimensions
    let w = HEX_SIZE * 3.0_f32.sqrt(); // full width
    let h = HEX_SIZE * 2.0; // full height

    // Position relative to top-left corner of board:
    // (0,0) at: 1w right, 1/2h down (even row)
    // (0,1) at: 1/2w right, 1 1/4h down (odd row - shifted left by w/2)
    // Pattern: odd rows shift left by w/2, even rows don't shift
    let x_shift = if r % 2 == 1 { w / 2.0 } else { 0.0 };
    let x_offset = q as f32 * w - x_shift + w;
    let y_offset = r as f32 * (3.0 * h / 4.0) + h / 2.0;

    // Convert to world coordinates (Bevy y-up, so down is negative)
    board_top_left + Vec2::new(x_offset, -y_offset)
}

// ===== SYSTEMS =====

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera with Windows 95 gray background
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.753, 0.753, 0.753)), // Windows 95 Light Gray (192,192,192)
            ..default()
        },
    ));

    let (span_x, span_y) = board_span(HEX_SIZE, GRID_WIDTH, GRID_HEIGHT);
    
    // Add padding around the board
    let padding = 20.0; // Adjust this value for more/less padding
    
    // draw a black background with padding around the grid
    commands.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(span_x + 2.0 * padding, span_y + 2.0 * padding)),
            ..default()
        },
        Transform::from_xyz(0.0, GRID_VERTICAL_OFFSET, -1.0), // behind tiles, moved with grid
    ));

    // Spawn grid of hex icons with coordinate text
    for r in 0..GRID_HEIGHT {
        for q in 0..GRID_WIDTH {
            let pos = axial_to_world(q, r);

            // Spawn hex sprite
            commands.spawn((
                Sprite {
                    image: asset_server.load("hex-w95-blank.png"),
                    custom_size: Some(Vec2::new(48.0, 48.0)), // Scaled down for mobile
                    ..default()
                },
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ));

            // Spawn coordinate text
            commands.spawn((
                Text2d::new(format!("{},{}\n{},{}", q, r, pos.x.trunc(), pos.y.trunc())),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_xyz(pos.x, pos.y, 1.0),
            ));
        }
    }
}

fn spawn_falling_triangles(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    // TODO: Check if no triangle is falling and timer is up
    // TODO: Spawn new triangle above the grid center
    // TODO: Create triangle entity with 3 child hex sprites
    // TODO: Set initial position above grid, random hex types
    // TODO: Reset spawn timer
}

fn handle_triangle_input(
    mut triangles: Query<&mut FallingTriangle>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // TODO: Left/Right arrows: Move triangle grid_q position within bounds
    // TODO: Down arrow: Increase fall_speed for fast drop
    // TODO: Space bar: Cycle position_cycle (0->1->2->0) for "rotation"
    // TODO: Ensure grid_q stays within 0..GRID_WIDTH bounds
}

fn apply_triangle_gravity(
    mut triangles: Query<&mut FallingTriangle>,
    time: Res<Time>,
) {
    // TODO: Decrease world_y based on fall_speed and time
    // TODO: Update triangle world_y position smoothly
    // TODO: Apply different fall speeds (normal vs fast drop)
}

fn detect_triangle_collision(
    triangles: Query<&FallingTriangle>,
    game_state: Res<GameState>,
) {
    // TODO: For each triangle, calculate world positions of 3 hexes
    // TODO: Convert world positions to grid coordinates
    // TODO: Check if any hex hits grid bottom or existing pieces
    // TODO: Mark triangle for landing when collision detected
}

fn convert_triangle_to_grid(
    mut commands: Commands,
    triangles: Query<(Entity, &FallingTriangle)>,
    mut game_state: ResMut<GameState>,
) {
    // TODO: For triangles marked for landing:
    // TODO: Calculate final grid positions of 3 hexes
    // TODO: Add hexes to game_state.stack HashMap
    // TODO: Despawn triangle entity
    // TODO: Trigger hex physics for newly placed pieces
}

fn apply_hex_physics(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
) {
    // TODO: For each hex in stack, check support below
    // TODO: If supported by 2+ pieces or bottom: stay in place
    // TODO: If supported by 1 piece: slide to unsupported side
    // TODO: If unsupported: fall down one grid position
    // TODO: Create SettlingHex entities for pieces that need to move
    // TODO: Update game_state.stack with new positions
}

fn animate_settling(
    mut commands: Commands,
    mut settling_hexes: Query<(Entity, &mut SettlingHex, &mut Transform)>,
    time: Res<Time>,
) {
    // TODO: Update animation_timer for settling hexes
    // TODO: Interpolate Transform position from start to target
    // TODO: When animation complete, remove SettlingHex component
    // TODO: Convert to StackedHex component
}

fn detect_matches(game_state: Res<GameState>) {
    // TODO: Find lines of 3+ same hex type in any direction
    // TODO: Check all 6 hex directions from each position
    // TODO: Lines can be straight in any of the 6 hex directions
    // TODO: Mark matching lines for removal (not flood fill clusters)
}

fn clear_matches(
    mut commands: Commands,
    stacked_hexes: Query<(Entity, &HexCoord, &StackedHex)>,
    mut game_state: ResMut<GameState>,
) {
    // TODO: Remove matched hexes from game_state.stack
    // TODO: Despawn hex sprite entities
    // TODO: Add score based on match size
    // TODO: Trigger cascade physics for remaining pieces
}

fn cascade_physics(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
) {
    // TODO: After clearing matches, re-apply hex physics
    // TODO: Check for chain reactions (new matches after settling)
    // TODO: Continue until no more matches possible
}

fn check_game_over(game_state: Res<GameState>) {
    // TODO: Check if any pieces in stack reach spawn area (top rows)
    // TODO: Check if triangle cannot spawn due to collision
    // TODO: Trigger game over state if needed
}

// ===== HELPER FUNCTIONS =====

fn spawn_triangle(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // TODO: Create 3 random Windows 95 hex types
    // TODO: Spawn triangle entity above grid center
    // TODO: Create child hex sprites in triangular formation
    // TODO: Position sprites according to current position_cycle
}

fn get_triangle_hex_positions(triangle: &FallingTriangle) -> [(Vec2, HexType); 3] {
    // TODO: Calculate world positions of 3 hexes based on:
    // TODO: - triangle.grid_q (horizontal grid position)
    // TODO: - triangle.world_y (vertical world position)
    // TODO: - triangle.position_cycle (arrangement of hexes)
    // TODO: Return array of (world_position, hex_type) for each hex
    [(Vec2::ZERO, triangle.hexes[0]), (Vec2::ZERO, triangle.hexes[1]), (Vec2::ZERO, triangle.hexes[2])]
}

fn triangle_hex_to_grid_coord(world_pos: Vec2) -> Option<HexCoord> {
    // TODO: Convert world position to grid coordinate
    // TODO: Return None if outside grid bounds
    // TODO: Use inverse of axial_to_world calculation
    None
}

fn find_lines_from_position(
    start: &HexCoord,
    hex_type: HexType,
    stack: &HashMap<HexCoord, HexType>,
) -> Vec<Vec<HexCoord>> {
    // TODO: Find all lines of 3+ same type starting from this position
    // TODO: Check all 6 hex directions: NE, E, SE, SW, W, NW
    // TODO: For each direction, count consecutive matching hexes
    // TODO: Return vector of lines (each line is a vector of coordinates)
    vec![]
}

fn world_to_hex_coord(world_pos: Vec2, hex_size: f32) -> HexCoord {
    // TODO: Convert world position to hex grid
    HexCoord::new(0, 0)
}

fn get_lowest_empty_position(coord: &HexCoord, stack: &HashMap<HexCoord, HexType>) -> HexCoord {
    // TODO: Where should a hex fall to?
    *coord
}
