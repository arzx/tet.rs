mod menu;
mod states;
mod game;
mod board;

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use menu::{menu_button_system, spawn_menu, cleanup_menu};
use states::AppState;
use board::{Board, BOARD_HEIGHT, BOARD_WIDTH, Cell};
//todo: add tetris music
//todo: get assets for the bricks
#[derive(Component)]
pub struct MenuCamera;

#[derive(Component)]
pub struct MenuBackground;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image: Handle<Image> = asset_server.load("background.png");
    commands.spawn((Camera2d, MenuCamera));
    commands.spawn((
        Sprite {
            image: background_image,
            image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
            custom_size: Some(Vec2::new(1200.0, 1800.0)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        MenuBackground
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".to_string(),
                // use integers (u32) and chain the builder method:
                resolution: WindowResolution::new(1200, 1800).with_scale_factor_override(1.0),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), spawn_menu)
        .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
        .add_systems(Update, menu_button_system)
        .add_systems(OnEnter(AppState::InGame), game::setup_ingame)
        .insert_resource(Board { cells: [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT] })
        .run();
}
