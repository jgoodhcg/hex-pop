use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hex Pop".into(),
                resolution: (800.0, 600.0).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_sprite)
        .run();
}

#[derive(Component)]
struct RotatingSprite;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.0, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        RotatingSprite,
    ));
}

fn rotate_sprite(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingSprite>>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs() * 0.5);
    }
}
