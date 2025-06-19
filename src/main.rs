use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

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
        .add_plugins(LogDiagnosticsPlugin::default())
        .init_resource::<DebugState>()
        .add_systems(Startup, setup)
        .add_systems(Update, check_asset_loading)
        .run();
}

// Hex coordinate system - using axial coordinates (q, r)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32, // Column
    pub r: i32, // Row
}

impl HexCoord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    
    // Convert hex coordinates to world position
    pub fn to_world_pos(&self, hex_size: f32) -> Vec2 {
        let x = hex_size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let y = hex_size * (3.0 / 2.0 * self.r as f32);
        Vec2::new(x, y)
    }
}

#[derive(Component)]
struct Hexagon {
    coord: HexCoord,
    in_play: bool,  // true = active game piece, false = border/inactive
}

#[derive(Resource)]
struct HexTexture(Handle<Image>);

#[derive(Component)]
struct DebugText;

#[derive(Resource, Default)]
struct DebugState {
    last_logged_state: Option<String>,
}

// Game board configuration for mobile
const GRID_WIDTH: i32 = 7;   // Hexes wide (good for mobile portrait)
const GRID_HEIGHT: i32 = 12; // Hexes tall (fits mobile screens)
const MOBILE_ASPECT_RATIO: f32 = 9.0 / 16.0; // Mobile portrait ratio

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    
    // Load hexagon texture
    let hex_texture = asset_server.load("hexagon.png");
    commands.insert_resource(HexTexture(hex_texture.clone()));
    
    // Define rectangular play area boundaries (in world coordinates)
    let play_area_width = 300.0;  // Straight vertical edges
    let play_area_height = 500.0; // Straight horizontal edges
    
    // Calculate hex size to fit nicely in play area
    let hex_size = 25.0; // Fixed size for consistent gameplay
    
    web_sys::console::log_1(&format!("Rectangular play area: {}x{} pixels", play_area_width, play_area_height).into());
    
    // Generate a larger hex grid, then determine which hexes are in/out of play
    let grid_radius = 10; // Generate plenty of hexes to fill and overflow the rectangle
    
    let mut in_play_count = 0;
    let mut border_count = 0;
    
    for q in -grid_radius..=grid_radius {
        let r1 = (-grid_radius).max(-q - grid_radius);
        let r2 = grid_radius.min(-q + grid_radius);
        
        for r in r1..=r2 {
            let hex_coord = HexCoord::new(q, r);
            let world_pos = hex_coord.to_world_pos(hex_size);
            
            // Only spawn hexes that are somewhat near our play area
            if world_pos.x.abs() <= play_area_width + hex_size && 
               world_pos.y.abs() <= play_area_height + hex_size {
                
                // Determine if this hex is inside the rectangular play area
                let in_play = world_pos.x.abs() <= play_area_width / 2.0 && 
                              world_pos.y.abs() <= play_area_height / 2.0;
                
                if in_play {
                    in_play_count += 1;
                } else {
                    border_count += 1;
                }
                
                // Choose color: normal for in-play, gray for border
                let sprite_color = if in_play {
                    Color::WHITE  // Normal hexagon colors
                } else {
                    Color::srgb(0.4, 0.4, 0.4)  // Grayed out border
                };
                
                commands.spawn((
                    Sprite {
                        image: hex_texture.clone(),
                        color: sprite_color,
                        custom_size: Some(Vec2::new(hex_size * 1.8, hex_size * 1.8)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                    Hexagon { 
                        coord: hex_coord,
                        in_play,
                    },
                ));
            }
        }
    }
    
    // Draw play area boundary rectangle for visualization
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 0.0, 0.3), // Semi-transparent yellow
            custom_size: Some(Vec2::new(play_area_width, play_area_height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0), // Behind hexagons
    ));
    
    // Super basic text test
    commands.spawn((
        Text::from("HELLO WORLD"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        },
    ));
}

fn check_asset_loading(
    hex_texture: Option<Res<HexTexture>>,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut debug_state: ResMut<DebugState>,
) {
    if let Some(hex_texture_res) = hex_texture {
        let load_state = asset_server.get_load_state(&hex_texture_res.0);
        
        let current_state = match load_state {
            Some(bevy::asset::LoadState::NotLoaded) => "PNG: Not loaded".to_string(),
            Some(bevy::asset::LoadState::Loading) => "PNG: Loading...".to_string(),
            Some(bevy::asset::LoadState::Loaded) => {
                if images.get(&hex_texture_res.0).is_some() {
                    "PNG: Successfully loaded and ready!".to_string()
                } else {
                    "PNG: Loaded but not in Images asset store".to_string()
                }
            },
            Some(bevy::asset::LoadState::Failed(err)) => {
                format!("PNG: Failed to load - Error: {:?}", err)
            },
            None => "PNG: Unknown load state".to_string(),
        };
        
        // Only log if state changed
        if debug_state.last_logged_state.as_ref() != Some(&current_state) {
            web_sys::console::log_1(&current_state.clone().into());
            debug_state.last_logged_state = Some(current_state);
        }
    }
}

fn debug_display(
    hex_texture: Option<Res<HexTexture>>,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut text_query: Query<&mut Text, With<DebugText>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Some(hex_texture_res) = hex_texture.as_ref() {
            let load_state = asset_server.get_load_state(&hex_texture_res.0);
            
            let status = match load_state {
                Some(bevy::asset::LoadState::NotLoaded) => "Loading...",
                Some(bevy::asset::LoadState::Loading) => "Loading...",
                Some(bevy::asset::LoadState::Loaded) => {
                    if images.get(&hex_texture_res.0).is_some() {
                        "Ready!"
                    } else {
                        "Error"
                    }
                },
                Some(bevy::asset::LoadState::Failed(_)) => "Failed",
                None => "Unknown",
            };
            
            // Update just the status part, keep the counts static
            **text = format!("Board: 300x500\nStatus: {}", status);
        }
    }
}