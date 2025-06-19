use bevy::prelude::*;

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
        .add_systems(Update, (falling_row_system, spawn_new_row_system))
        .run();
}

// Simple hex coordinate
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32, // column
    r: i32, // row
}

// Game grid dimensions
const GRID_WIDTH: i32 = 8; // hex columns
const GRID_HEIGHT: i32 = 12; // hex rows
const ROW_WIDTH: i32 = 6; // hexes per falling row

// Hex colors for matching
#[derive(Debug, Clone, Copy, PartialEq)]
enum HexColor {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
}

// Game state components
#[derive(Component)]
struct StackedHex {
    color: HexColor,
}

#[derive(Component)]
struct FallingRow {
    hexes: Vec<HexColor>, // Left to right order
    fall_speed: f32,      // pixels per second
    row_position: i32,    // Current row (starts at top)
}

#[derive(Resource)]
struct GameState {
    stack: std::collections::HashMap<HexCoord, HexColor>, // Locked hexes
    game_speed_multiplier: f32,
    lines_cleared: u32,
    next_row_timer: f32,
    base_fall_speed: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stack: std::collections::HashMap::new(),
            game_speed_multiplier: 1.0,
            lines_cleared: 0,
            next_row_timer: 3.0,   // 3 seconds between rows
            base_fall_speed: 50.0, // pixels per second
        }
    }
}

impl HexCoord {
    fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    // Convert hex coordinate to world position
    fn to_world_pos(&self, hex_size: f32) -> Vec2 {
        let x = hex_size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let y = hex_size * (1.5 * self.r as f32);
        Vec2::new(x, y)
    }

    // Get all 6 adjacent hex coordinates
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

    // Check if coordinate is within game grid bounds
    fn is_valid_grid_pos(&self) -> bool {
        self.q >= 0 && self.q < GRID_WIDTH && self.r >= 0 && self.r < GRID_HEIGHT
    }
}

impl HexColor {
    fn to_bevy_color(&self) -> Color {
        match self {
            HexColor::Red => Color::srgb(1.0, 0.2, 0.2),
            HexColor::Blue => Color::srgb(0.2, 0.2, 1.0),
            HexColor::Green => Color::srgb(0.2, 1.0, 0.2),
            HexColor::Yellow => Color::srgb(1.0, 1.0, 0.2),
            HexColor::Purple => Color::srgb(1.0, 0.2, 1.0),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Load hexagon image
    let hex_image = asset_server.load("hexagon.png");

    // Settings
    let hex_size = 30.0;
    let grid_radius = 2; // Creates a small hexagon of hexagons

    // Create hexagon pattern
    for q in -grid_radius..=grid_radius {
        let r1 = (-grid_radius).max(-q - grid_radius);
        let r2 = grid_radius.min(-q + grid_radius);

        for r in r1..=r2 {
            let coord = HexCoord::new(q, r);
            let world_pos = coord.to_world_pos(hex_size);

            commands
                .spawn((
                    Sprite {
                        image: hex_image.clone(),
                        custom_size: Some(Vec2::new(hex_size * 2.0, hex_size * 2.0)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                    coord.clone(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text2d::new(format!("{},{}", coord.q, coord.r)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Transform::from_xyz(0.0, 0.0, 1.0),
                    ));
                });
        }
    }
}

// System to handle falling row physics
fn falling_row_system(
    mut commands: Commands,
    mut falling_rows: Query<(Entity, &mut FallingRow, &mut Transform)>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let hex_size = 30.0;

    for (entity, falling_row, mut transform) in falling_rows.iter_mut() {
        // Apply gravity
        let current_speed = falling_row.fall_speed * game_state.game_speed_multiplier;
        transform.translation.y -= current_speed * time.delta_secs();

        // Check if row should lock into place
        let target_row_y = (GRID_HEIGHT - falling_row.row_position - 1) as f32 * hex_size * 1.5;

        if transform.translation.y <= target_row_y {
            // Lock the row into the stack
            let start_q = (GRID_WIDTH - ROW_WIDTH) / 2; // Center the row

            for (i, &color) in falling_row.hexes.iter().enumerate() {
                let coord = HexCoord::new(start_q + i as i32, falling_row.row_position);
                game_state.stack.insert(coord.clone(), color);

                // Spawn visual hex for stacked piece
                let world_pos = coord.to_world_pos(hex_size);
                commands.spawn((
                    Sprite {
                        image: asset_server.load("hexagon.png"),
                        color: color.to_bevy_color(),
                        custom_size: Some(Vec2::new(hex_size * 2.0, hex_size * 2.0)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                    coord,
                    StackedHex { color },
                ));
            }

            // Remove falling row entity
            commands.entity(entity).despawn();

            // Increase game speed slightly
            game_state.game_speed_multiplier += 0.05;
        }
    }
}

// System to spawn new falling rows
fn spawn_new_row_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    falling_rows: Query<&FallingRow>,
) {
    // Only spawn if no falling row exists and timer is up
    if falling_rows.is_empty() {
        game_state.next_row_timer -= time.delta_secs();

        if game_state.next_row_timer <= 0.0 {
            spawn_falling_row(&mut commands, &mut game_state, &asset_server);
            game_state.next_row_timer = 3.0; // Reset timer
        }
    }
}

fn spawn_falling_row(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    asset_server: &Res<AssetServer>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Generate random colors for the row
    let colors = [
        HexColor::Red,
        HexColor::Blue,
        HexColor::Green,
        HexColor::Yellow,
        HexColor::Purple,
    ];
    let row_hexes: Vec<HexColor> = (0..ROW_WIDTH)
        .map(|_| colors[rng.gen_range(0..colors.len())])
        .collect();

    let hex_size = 30.0;
    let start_q = (GRID_WIDTH - ROW_WIDTH) / 2;
    let spawn_row = 0; // Start at top
    let spawn_y = (GRID_HEIGHT + 2) as f32 * hex_size * 1.5; // Above grid

    // Create falling row entity
    commands
        .spawn((
            FallingRow {
                hexes: row_hexes.clone(),
                fall_speed: game_state.base_fall_speed,
                row_position: spawn_row,
            },
            Transform::from_xyz(0.0, spawn_y, 0.0),
        ))
        .with_children(|parent| {
            // Spawn visual hexes as children
            for (i, &color) in row_hexes.iter().enumerate() {
                let coord = HexCoord::new(start_q + i as i32, spawn_row);
                let world_pos = coord.to_world_pos(hex_size);

                parent.spawn((
                    Sprite {
                        image: asset_server.load("hexagon.png"),
                        color: color.to_bevy_color(),
                        custom_size: Some(Vec2::new(hex_size * 2.0, hex_size * 2.0)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, 0.0, 0.0),
                    coord,
                ));
            }
        });
}
