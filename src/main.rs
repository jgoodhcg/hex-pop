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
        .add_systems(Startup, setup)
        .run();
}

// Simple hex coordinate
#[derive(Component, Debug, Clone)]
struct HexCoord {
    q: i32, // column
    r: i32, // row
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
            
            commands.spawn((
                Sprite {
                    image: hex_image.clone(),
                    custom_size: Some(Vec2::new(hex_size * 2.0, hex_size * 2.0)),
                    ..default()
                },
                Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                coord.clone(),
            )).with_children(|parent| {
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
            
            // Log coordinates to console for debugging
            web_sys::console::log_1(&format!("Hex at ({}, {})", coord.q, coord.r).into());
        }
    }
}