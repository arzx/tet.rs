use crate::MenuBackground;
use crate::states::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

pub fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MenuRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(64.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Button,
                        StartButton,
                        BackgroundColor(Color::srgb(0.2, 0.5, 0.8)),
                    ))
                    .with_children(|button| {
                        button.spawn(Text::new("Start"));
                    });
                    col.spawn((
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(64.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Button,
                        QuitButton,
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    ))
                    .with_children(|button| {
                        button.spawn(Text::new("Quit"));
                    });
                });
        });
}

pub fn menu_button_system(
    mut changed: Query<
        (
            Entity,
            &Interaction,
            Option<&StartButton>,
            Option<&QuitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (_entity, interaction, is_start, is_quit) in changed.iter_mut() {
        if *interaction == Interaction::Pressed {
            if is_start.is_some() {
                next_state.set(AppState::InGame);
            } else if is_quit.is_some() {
                app_exit.write(AppExit::Success);
            }
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands,
    roots: Query<Entity, With<MenuRoot>>,
    backgrounds: Query<Entity, With<MenuBackground>>,
) {
    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }
    for entity in backgrounds.iter() {
        commands.entity(entity).despawn();
    }
}
