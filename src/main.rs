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
        .add_systems(Update, falling_hex)
        .run();
}

// Simple hex coordinate
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32, // column
    r: i32, // row
}

#[derive(Component)]
struct FallingHex {
    speed: f32, // pixels per second
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn a falling hex at the top
    commands.spawn((
        Sprite {
            image: asset_server.load("hexagon.png"),
            custom_size: Some(Vec2::new(60.0, 60.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 300.0, 0.0), // Start at top
        FallingHex { speed: 100.0 },
    ));
}

// System to handle falling hex physics
fn falling_hex(mut falling_hexes: Query<(&mut Transform, &FallingHex)>, time: Res<Time>) {
    for (mut transform, falling_hex) in falling_hexes.iter_mut() {
        // Move hex downward
        transform.translation.y -= falling_hex.speed * time.delta_secs();

        // Stop at bottom of screen (y = -300)
        if transform.translation.y < -300.0 {
            transform.translation.y = -300.0;
        }
    }
}
