use bevy::prelude::*;
use bevy::window::{Window, WindowResolution, WindowPlugin};

//todo: add tetris music
//todo: menu with play and quite buttons
//todo: get assets for the bricks
//todo: 10x20 grid


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image: Handle<Image> = asset_server.load("background.png");
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: background_image,
            image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
            custom_size: Some(Vec2::new(1200.0, 1800.0)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                // use integers (u32) and chain the builder method:
                resolution: WindowResolution::new(1200, 1800)
                    .with_scale_factor_override(1.0),
                resizable: false,
                ..default()
            }),
            // fill in the other WindowPlugin fields from Default
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}