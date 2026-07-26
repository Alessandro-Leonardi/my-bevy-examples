// Imports:
use bevy::{camera::ScalingMode, image, prelude::*};

// Flat Structure: Single-level Submodules:
mod flat_submodule;

// Nested Structure: Submodules with their own Submodules:
mod nested_submodule;
use crate::nested_submodule::nested_hello::hello_nested_submodule;

// Components:
#[derive(Component)]
#[require(Gravity(1000.0), Velocity)]
struct Player;

#[derive(Component)]
struct Gravity(f32);

#[derive(Component, Default)]
struct Velocity;

fn main() -> AppExit {
    // TO BE DELETED:
    flat_submodule::hello_flat_module();
    nested_submodule::handle_auth();
    hello_nested_submodule();

    // main function starts here:
    //
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run()

    // Implicitly returns AppExit.
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) -> () {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 480.0,
                max_height: 270.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(25.0)),
            image: asset_server.load("bevy-bird.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
