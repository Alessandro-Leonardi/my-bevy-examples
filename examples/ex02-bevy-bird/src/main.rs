// Imports:
use bevy::{camera::ScalingMode, prelude::*};

// Flat Structure: Single-level Submodules:
mod flat_submodule;

// Nested Structure: Submodules with their own Submodules:
mod nested_submodule;
use crate::nested_submodule::nested_hello::hello_nested_submodule;

// Resources / States:
// NOTE: Only derive States here. Bevy handles the underlying resource storage automatically.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameMode {
    #[default]
    Waiting,
    Started,
}

// Components:
#[derive(Component)]
#[require(Gravity, Velocity)] // Bevy 0.15+ automatic component instantiation
struct Player;

#[derive(Component)]
struct Gravity(f32);

// Providing defaults for #[require] compatibility
impl Default for Gravity {
    fn default() -> Self {
        Gravity(400.0)
    }
}

#[derive(Component, Default)]
struct Velocity(f32);

fn main() -> AppExit {
    // TO BE DELETED:
    flat_submodule::hello_flat_module();
    nested_submodule::handle_auth();
    hello_nested_submodule();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        // FIX 1: Register the State type so .run_if(in_state(...)) functions correctly
        .init_state::<GameMode>()
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, gravity.run_if(in_state(GameMode::Started)))
        .add_systems(Update, controls)
        .run()
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        Player,
        Sprite {
            custom_size: Some(Vec2::splat(25.0)),
            image: asset_server.load("bevy-bird.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn gravity(mut transforms: Query<(&mut Transform, &mut Velocity, &Gravity)>, time: Res<Time>) {
    for (mut transform, mut velocity, gravity) in &mut transforms {
        velocity.0 -= gravity.0 * time.delta_secs();
        println!("> Velocity: {:?}", velocity.0);
        transform.translation.y += velocity.0 * time.delta_secs();
    }
}

fn controls(
    mut velocity: Single<&mut Velocity, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    // FIX 2: Read current state via State<T>, queue transitions via NextState<T>
    current_state: Res<State<GameMode>>,
    mut next_state: ResMut<NextState<GameMode>>,
) {
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        if *current_state.get() == GameMode::Started {
            // Normal jumping behavior once the game is live
            velocity.0 += 200.0;
        } else {
            // FIX 3: Transition out of the Waiting state into Started on first click
            next_state.set(GameMode::Started);
            velocity.0 += 200.0; // Optional: Gives an immediate initial jump on start
        }
    }
}
