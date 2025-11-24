use bevy::prelude::*;
//use crate::menu::{menu_button_system, spawn_menu};
use crate::MenuCamera;
//todo: on button press "play" switch to InGame state



pub fn setup_ingame(mut camera_query: Query<&mut Transform, With<MenuCamera>>) {
    if let Ok(mut transform) = camera_query.single_mut() {
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
    }
}