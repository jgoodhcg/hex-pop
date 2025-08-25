use bevy::prelude::*;
use wasm_bindgen::JsCast;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hex Drop Game".into(),
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut window_query: Query<&mut Window>,
) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.15, 0.18, 0.25)), // Dark blue-grey
            ..default()
        },
    ));

    // Get actual canvas size from browser
    let (canvas_width, canvas_height) = if let Some(web_window) = web_sys::window() {
        if let Some(canvas) = web_window.document().and_then(|doc| doc.get_element_by_id("bevy")) {
            let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
            (canvas.client_width() as f32, canvas.client_height() as f32)
        } else {
            (800.0, 600.0) // Fallback
        }
    } else {
        (800.0, 600.0) // Fallback
    };
    
    // Set Bevy window to match actual canvas size
    if let Ok(mut window) = window_query.single_mut() {
        window.resolution.set(canvas_width, canvas_height);
        web_sys::console::log_1(&format!("✅ Updated Bevy window to match canvas: {}x{}", canvas_width, canvas_height).into());
    }
    
    // Now both should be the same
    let screen_width = canvas_width;
    let screen_height = canvas_height;
    
    // Grid math calculations
    let hex_radius = 30.0;
    let hex_width = hex_radius * 1.732; // sqrt(3) * radius
    let hex_height = hex_radius * 2.0;
    let row_spacing = hex_height * 0.75;
    
    let cols_needed = ((screen_width / hex_width) as i32) + 2;
    let rows_needed = ((screen_height / row_spacing) as i32) + 2;
    
    // Just spawn ONE hexagon at center (0,0 coordinate)
    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(hex_radius, 6))),
        MeshMaterial2d(materials.add(Color::srgb(0.4, 0.8, 0.95))), // Bright cyan so we can see it
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    // Add 0,0 label
    commands.spawn((
        Text2d::new("0,0"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    
    // Console logging for WASM
    web_sys::console::log_1(&format!("🌐 Canvas size: {}x{}", screen_width, screen_height).into());
    web_sys::console::log_1(&"🔷 Hex dimensions:".into());
    web_sys::console::log_1(&format!("   Radius: {}", hex_radius).into());
    web_sys::console::log_1(&format!("   Width: {:.1}", hex_width).into());
    web_sys::console::log_1(&format!("   Height: {:.1}", hex_height).into());
    web_sys::console::log_1(&format!("   Row spacing: {:.1}", row_spacing).into());
    web_sys::console::log_1(&"🔢 Grid calculations:".into());
    web_sys::console::log_1(&format!("   Columns needed: {}", cols_needed).into());
    web_sys::console::log_1(&format!("   Rows needed: {}", rows_needed).into());
    web_sys::console::log_1(&format!("   Total hexes would be: {}", cols_needed * rows_needed).into());
    web_sys::console::log_1(&"✅ Spawned single hexagon at (0,0)".into());
}
