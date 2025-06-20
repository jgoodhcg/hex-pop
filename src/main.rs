use bevy::prelude::*;
use std::collections::HashMap;
use rand::Rng;

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
                spawn_falling_groups,
                handle_input,
                falling_group_physics,
                check_group_landing,
                lock_individual_hexes,
                apply_individual_gravity,
                detect_matches,
                clear_matches,
                cascade_gravity,
                check_game_over,
            ),
        )
        .run();
}

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
    Blank,   // hex-w95-blank.png
    Cli,     // hex-w95-cli.png  
    Corner,  // hex-w95-corner.png
}

// Falling group of 3 hexes
#[derive(Component)]
struct FallingGroup {
    hexes: [HexType; 3], // Exactly 3 hexes
    center_pos: Vec2,     // World position of group center
    rotation: usize,      // 0, 1, or 2 (which hex is "top")
    fall_speed: f32,
    horizontal_speed: f32, // For left/right movement
}

// Individual stacked hex
#[derive(Component)]
struct StackedHex {
    hex_type: HexType,
}

// Game state resource
#[derive(Resource)]
struct GameState {
    stack: HashMap<HexCoord, HexType>,
    next_group_timer: f32,
    game_speed: f32,
    score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stack: HashMap::new(),
            next_group_timer: 3.0,
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

    fn to_world_pos(&self, hex_size: f32) -> Vec2 {
        let x = hex_size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let y = hex_size * (1.5 * self.r as f32);
        Vec2::new(x, y)
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
        // TODO: Define grid bounds
        true
    }

    fn lowest_supported_position(&self, stack: &HashMap<HexCoord, HexType>) -> HexCoord {
        // TODO: Find where this hex should fall to
        *self
    }
}

impl FallingGroup {
    fn get_hex_positions(&self, hex_size: f32) -> [(Vec2, HexType); 3] {
        // TODO: Calculate world positions of 3 hexes based on center_pos and rotation
        [
            (Vec2::ZERO, self.hexes[0]),
            (Vec2::ZERO, self.hexes[1]),
            (Vec2::ZERO, self.hexes[2]),
        ]
    }

    fn rotate_clockwise(&mut self) {
        // TODO: Change rotation index
        self.rotation = (self.rotation + 1) % 3;
    }

    fn can_move_to(&self, new_pos: Vec2, stack: &HashMap<HexCoord, HexType>) -> bool {
        // TODO: Check if group can move to new position without collision
        true
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

// ===== SYSTEMS =====

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Spawn camera with dark background
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.753, 0.753, 0.753)), // Windows 95 Light Gray (192,192,192)
            ..default()
        },
    ));
}

fn spawn_falling_groups(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    falling_groups: Query<&FallingGroup>,
) {
    // Only spawn if no falling group exists and timer is up
    if falling_groups.is_empty() {
        game_state.next_group_timer -= time.delta_secs();
        
        if game_state.next_group_timer <= 0.0 {
            spawn_hex_group(&mut commands, &asset_server);
            game_state.next_group_timer = 3.0; // Reset timer
        }
    }
}

fn handle_input(mut falling_groups: Query<&mut FallingGroup>, keyboard: Res<ButtonInput<KeyCode>>) {
    // TODO: A/D rotation, arrow keys move, S slam
}

fn falling_group_physics(
    mut falling_groups: Query<(&mut FallingGroup, &mut Transform)>,
    time: Res<Time>,
) {
    // TODO: Gravity + horizontal movement
}

fn check_group_landing(falling_groups: Query<&FallingGroup>, game_state: Res<GameState>) {
    // TODO: Detect when individual hexes should lock
}

fn lock_individual_hexes(
    mut commands: Commands,
    falling_groups: Query<(Entity, &FallingGroup)>,
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
) {
    // TODO: Convert each hex in group to individual StackedHex
}

fn apply_individual_gravity(
    mut stacked_hexes: Query<(&mut Transform, &StackedHex, &HexCoord)>,
    mut game_state: ResMut<GameState>,
) {
    // TODO: Each hex falls to its lowest supported position
}

fn detect_matches(game_state: Res<GameState>) {
    // TODO: Find groups of 3+ same color (flood fill)
}

fn clear_matches(
    mut commands: Commands,
    stacked_hexes: Query<(Entity, &HexCoord, &StackedHex)>,
    mut game_state: ResMut<GameState>,
) {
    // TODO: Remove matched hexes, add to score
}

fn cascade_gravity(
    mut stacked_hexes: Query<(&mut Transform, &StackedHex, &HexCoord)>,
    mut game_state: ResMut<GameState>,
) {
    // TODO: After clearing, drop remaining hexes again
}

fn check_game_over(game_state: Res<GameState>) {
    // TODO: Stack height detection
}

// ===== HELPER FUNCTIONS =====

fn spawn_hex_group(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Create 3 random Windows 95 hex types
    let hexes = [
        HexType::random(),
        HexType::random(),
        HexType::random(),
    ];
    
    // Spawn the falling group entity
    commands.spawn((
        FallingGroup {
            hexes,
            center_pos: Vec2::new(0.0, 300.0), // Start at top center
            rotation: 0,
            fall_speed: 100.0,
            horizontal_speed: 0.0,
        },
        Transform::from_xyz(0.0, 300.0, 0.0),
    )).with_children(|parent| {
        // Spawn 3 hex sprites as children
        let hex_radius = 64.0; // Actual hexagon radius in your 128x128 images
        let gap = 4.0; // Small breathing room between hexagons
        let hex_width = hex_radius * 3.0_f32.sqrt() + gap; // Width with gap
        let hex_height = hex_radius * 1.5 + gap; // Vertical spacing with gap
        
        let positions = [
            Vec2::new(0.0, hex_height/2.0),                      // Top
            Vec2::new(-hex_width/2.0, -hex_height/2.0),         // Bottom left  
            Vec2::new(hex_width/2.0, -hex_height/2.0),          // Bottom right
        ];
        
        for (i, &hex_type) in hexes.iter().enumerate() {
            // Spawn hex using Windows 95 image files
            parent.spawn((
                Sprite {
                    image: asset_server.load(hex_type.to_image_path()),
                    color: Color::WHITE, // Use white to show original image colors
                    custom_size: Some(Vec2::new(128.0, 128.0)), // Keep original image size
                    ..default()
                },
                Transform::from_xyz(positions[i].x, positions[i].y, 0.0),
            ));
        }
    });
}

fn find_connected_group(
    start: &HexCoord,
    hex_type: HexType,
    stack: &HashMap<HexCoord, HexType>,
) -> Vec<HexCoord> {
    // TODO: Flood fill for matching detection
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

