use bevy::prelude::*;
use bevy::window::{Window, WindowResolution, WindowPlugin};
use bevy::app::AppExit;
//todo: add tetris music
//todo: get assets for the bricks
//todo: 10x20 grid
//todo: make the menu as a plugin
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState{
    #[default]
    MainMenu,
    InGame,
}

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct QuitButton;

fn spawn_menu(mut commands: Commands){
    commands.spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }, MenuRoot))
        .with_children(|parent| {
            parent.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((Node {
                            width: Val::Px(220.0),
                            height: Val::Px(64.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Button,
                        StartButton,
                        BackgroundColor(Color::srgb(0.2, 0.5, 0.8))
                    ))
                    .with_children(|button| {
                        button.spawn(Text::new("Start"));
                    });
                    col.spawn((Node {
                            width: Val::Px(220.0),
                            height: Val::Px(64.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Button,
                        QuitButton,
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.2))
                    ))
                    .with_children(|button| {
                        button.spawn(Text::new("Quit"));
                    });
                });
        });
}


fn menu_button_system(
    mut commands: Commands,
    mut changed: Query<
    (Entity, &Interaction, Option<&StartButton>, Option<&QuitButton>),
    (Changed<Interaction>, With<Button>),
    >,
    menu: Query<Entity, With<MenuRoot>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,

) {
    for (_entity, interaction, is_start, is_quit) in changed.iter_mut(){
        if *interaction == Interaction::Pressed {
            if is_start.is_some() {
                next_state.set(AppState::InGame);
                for m in menu.iter() {
                    commands.entity(m).despawn();
                }
            } else if is_quit.is_some() {
                app_exit.write(AppExit::Success);
            }
        }
    }
}

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
            ..default()
        }))
        .init_state::<AppState>()
        .add_systems(Startup, 
            (setup, spawn_menu)
        )
        .add_systems(Update, menu_button_system)
        .run();
}