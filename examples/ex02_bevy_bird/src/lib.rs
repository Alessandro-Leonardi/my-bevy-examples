use bevy::{image::ImageLoaderSettings, prelude::*};

// Constants:
pub const CANVAS_SIZE: Vec2 = Vec2::new(480.0, 270.0);
pub const PLAYER_SIZE: f32 = 25.0;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, spawn_pipes.run_if(run_once));
    }
}

fn spawn_pipes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image: Handle<Image> = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            settings
                .sampler
                .get_or_init_descriptor()
                .set_filter(bevy::image::ImageFilterMode::Nearest);
        })
        .load("pipe.png");

    commands.spawn((
        Sprite {
            image: image,
            custom_size: Some(Vec2::new(32.0, 160.0)),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::axes(8.0, 19.0),
                center_scale_mode: SliceScaleMode::Stretch,
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
