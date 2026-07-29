use std::time::Duration;

use bevy::{
    camera::ScalingMode, image::ImageLoaderSettings, prelude::*, time::common_conditions::on_timer,
};

// Constants:
pub const CANVAS_SIZE: Vec2 = Vec2::new(480.0, 270.0);
pub const PLAYER_SIZE: f32 = 25.0;

const PIPE_SIZE: Vec2 = Vec2::new(32.0, CANVAS_SIZE.y);
const GAP_SIZE: f32 = 100.0;
const PIPE_SPEED: f32 = 150.0;

// Resources / States:
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameMode {
    #[default]
    Waiting,
    Started,
}

// Components:
#[derive(Component)]
#[require(Gravity, Velocity)] // Bevy automatic component instantiation
pub struct Player;

#[derive(Component)]
pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(400.0)
    }
}

#[derive(Component, Default)]
pub struct Velocity(pub f32);

pub struct PipePlugin;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PipeTop;

#[derive(Component)]
pub struct PipeBottom;

#[derive(Component)]
pub struct PointsGate;

// Events:
#[derive(Event)]
pub struct EndGame;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            spawn_pipes.run_if(on_timer(Duration::from_millis(1000))),
        )
        .add_systems(FixedUpdate, (shift_pipes_to_the_left, despawn_pipes));
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
    let transform = Transform::from_xyz(CANVAS_SIZE.x / 2.0, 0.0, 1.0);

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

fn optional_count_pipes(query: Query<&Pipe>) {
    info!("> {} pipes exist.", query.iter().len());
}

fn optional_change_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: CANVAS_SIZE.x * 4.0,
                max_height: CANVAS_SIZE.y * 4.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn despawn_pipes(mut commands: Commands, pipes: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in pipes.iter() {
        if transform.translation.x < -(CANVAS_SIZE.x / 2.0 + PIPE_SIZE.x) {
            commands.entity(entity).despawn();
        }
    }
}
