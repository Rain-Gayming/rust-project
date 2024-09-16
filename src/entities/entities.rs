
use bevy::prelude::*;

#[derive(Component)]
pub struct KeyboardMovable;

#[derive(Component)]
pub struct EntityValues{
    pub speed: f32,
    pub jump_height: f32,
    pub is_grounded: bool,
}