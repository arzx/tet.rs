use bevy::prelude::*;
use crate::MenuCamera;


//todo: 10x20 grid
//todo: show score, level and next tetris item
pub fn setup_ingame(mut commands: Commands, mut camera_query: Query<&mut Transform, With<MenuCamera>>) {
    if let Ok(mut transform) = camera_query.single_mut() {
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
    }
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(25.0),
                right: Val::Px(20.0),
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(64.0),
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::FlexEnd,
                            ..default()
                        },
                    ))
                    .with_children(|col| {
                        col.spawn((
                            Text::new("Score:  "),
                            TextFont {
                                font_size: 40.0,
                                ..default()
                            },
                        ));
                    });
                    col.spawn((
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(64.0),
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::FlexEnd,
                            ..default()
                        },
                    ))
                    .with_children(|col| {
                        col.spawn((
                            Text::new("Next:  "),
                            TextFont {
                                font_size: 40.0,
                                ..default()
                            },
                        ));
                    });
                });
        });
}