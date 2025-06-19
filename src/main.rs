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
        .add_systems(Update, (check_asset_loading, debug_display))
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
}

#[derive(Resource)]
struct HexTexture(Handle<Image>);

#[derive(Component)]
struct DebugText;

#[derive(Resource, Default)]
struct DebugState {
    last_logged_state: Option<String>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let hex_size = 50.0;
    
    // Load hexagon texture
    let hex_texture = asset_server.load("hexagon.png");
    commands.insert_resource(HexTexture(hex_texture.clone()));
    
    // Create a hex grid
    let grid_radius = 3; // Creates roughly a 7x7 hex grid
    
    for q in -grid_radius..=grid_radius {
        let r1 = (-grid_radius).max(-q - grid_radius);
        let r2 = grid_radius.min(-q + grid_radius);
        
        for r in r1..=r2 {
            let hex_coord = HexCoord::new(q, r);
            let world_pos = hex_coord.to_world_pos(hex_size);
            
            commands.spawn((
                Sprite {
                    image: hex_texture.clone(),
                    custom_size: Some(Vec2::new(hex_size * 1.8, hex_size * 1.8)),
                    ..default()
                },
                Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                Hexagon { coord: hex_coord },
            ));
        }
    }
    
    // Debug text
    commands.spawn((
        Text::new("Hex Grid Loaded"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        DebugText,
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
        if let Some(hex_texture_res) = hex_texture {
            let load_state = asset_server.get_load_state(&hex_texture_res.0);
            
            let status = match load_state {
                Some(bevy::asset::LoadState::NotLoaded) => "PNG: Not loaded",
                Some(bevy::asset::LoadState::Loading) => "PNG: Loading...",
                Some(bevy::asset::LoadState::Loaded) => {
                    if images.get(&hex_texture_res.0).is_some() {
                        "PNG: Loaded! Grid Ready"
                    } else {
                        "PNG: Loaded but not accessible"
                    }
                },
                Some(bevy::asset::LoadState::Failed(_)) => "PNG: FAILED TO LOAD",
                None => "PNG: Unknown state",
            };
            
            **text = status.to_string();
        } else {
            **text = "PNG: No texture resource".to_string();
        }
    }
}