use bevy::prelude::*;

// Constants:
pub const CANVAS_SIZE: Vec2 = Vec2::new(480.0, 270.0);
pub const PLAYER_SIZE: f32 = 25.0;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        info!("Building the Plugin.");
    }
}
