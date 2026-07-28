use bevy::{image::ImageLoaderSettings, prelude::*};

// Constants:
pub const CANVAS_SIZE: Vec2 = Vec2::new(480.0, 270.0);
pub const PLAYER_SIZE: f32 = 25.0;

const PIPE_SIZE: Vec2 = Vec2::new(32.0, CANVAS_SIZE.y);
const GAP_SIZE: f32 = 100.0;
const PIPE_SPEED: f32 = 20.0;

pub struct PipePlugin;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PipeTop;

#[derive(Component)]
pub struct PipeBottom;

#[derive(Component)]
pub struct PointsGate;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, spawn_pipes.run_if(run_once))
            .add_systems(FixedUpdate, shift_pipes_to_the_left);
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

    let gap_y_position = 0.0;
    let pipe_offset = PIPE_SIZE.y / 2.0 + GAP_SIZE / 2.0;

    // POSSIBLE ERROR WITH THE FOLLOWING TWO VARIABLES:
    let image_mode = SpriteImageMode::Auto;
    let transform = Transform::from_xyz(0.0, 0.0, 1.0);

    commands.spawn((
        transform,
        Visibility::Visible,
        Pipe,
        children![
            (
                Sprite {
                    image: image.clone(),
                    custom_size: Some(PIPE_SIZE),
                    image_mode: image_mode.clone(),
                    ..default()
                },
                Transform::from_xyz(0.0, pipe_offset + gap_y_position, 1.0,),
                PipeTop
            ),
            (
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(10.0, GAP_SIZE)),
                    ..default()
                },
                Transform::from_xyz(0.0, gap_y_position, 1.0),
                PointsGate,
            ),
            (
                Sprite {
                    image,
                    custom_size: Some(PIPE_SIZE),
                    image_mode,
                    ..default()
                },
                Transform::from_xyz(0.0, -pipe_offset + gap_y_position, 1.0,),
                PipeBottom,
            )
        ],
    ));
}

pub fn shift_pipes_to_the_left(mut pipes: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut pipe in &mut pipes {
        pipe.translation.x -= PIPE_SPEED * time.delta_secs();
    }
}
