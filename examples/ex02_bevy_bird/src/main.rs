// ex02-bevy-bird\src\main.rs
// Imports:
use bevy::{camera::ScalingMode, prelude::*};
use ex02_bevy_bird::*;

// Flat Structure: Single-level Submodules:
mod flat_submodule;

// Nested Structure: Submodules with their own Submodules:
mod nested_submodule;

// Events:
#[derive(Event)]
struct EndGame;

// Resources / States:
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameMode {
    #[default]
    Waiting,
    Started,
}

// Components:
#[derive(Component)]
#[require(Gravity, Velocity)] // Bevy automatic component instantiation
struct Player;

#[derive(Component)]
struct Gravity(f32);

impl Default for Gravity {
    fn default() -> Self {
        Gravity(400.0)
    }
}

#[derive(Component, Default)]
struct Velocity(f32);

fn main() -> AppExit {
    flat_submodule::hello_flat_module();
    nested_submodule::handle_auth();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(PipePlugin)
        .init_state::<GameMode>()
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, gravity.run_if(in_state(GameMode::Started)))
        // Run this system ONLY if the gameplay is actually live
        .add_systems(
            FixedUpdate,
            check_in_bounds.run_if(in_state(GameMode::Started)),
        )
        .add_systems(Update, controls)
        // FIX 1: Use .add_observer instead of .observe
        .add_observer(respawn_on_endgame)
        .run()
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: CANVAS_SIZE.x,
                max_height: CANVAS_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            image: asset_server.load("bevy-bird.png"),
            ..default()
        },
        Transform::from_xyz(-CANVAS_SIZE.x / 4.0, 0.0, 1.0),
    ));
}

// Use Res<Time<Fixed>> instead of Res<Time> for FixedUpdate physics loops
fn gravity(
    mut transforms: Query<(&mut Transform, &mut Velocity, &Gravity)>,
    time: Res<Time<Fixed>>,
) {
    for (mut transform, mut velocity, gravity) in &mut transforms {
        // time.delta_secs() works correctly on Time<Fixed>
        velocity.0 -= gravity.0 * time.delta_secs();
        transform.translation.y += velocity.0 * time.delta_secs();
    }
}

// The Single Query Trap (Crash Vulnerability)
// If you plan to add states where the player doesn't exist,
// transition your system inputs from Single back to a standard fallible Query,
// or ensure your systems only execute during the active state loop
// using .run_if(in_state(GameMode::Started))
fn controls(
    mut velocity: Single<&mut Velocity, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    current_state: Res<State<GameMode>>,
    mut next_state: ResMut<NextState<GameMode>>,
) {
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        if *current_state.get() == GameMode::Started {
            velocity.0 += 200.0;
        } else {
            next_state.set(GameMode::Started);
            velocity.0 += 200.0;
        }
    }
}

fn check_in_bounds(player: Single<&Transform, With<Player>>, mut commands: Commands) {
    if player.translation.y < -CANVAS_SIZE.y / 2.0 - PLAYER_SIZE
        || player.translation.y > CANVAS_SIZE.y / 2.0 + PLAYER_SIZE
    {
        commands.trigger(EndGame);
    }
}

// FIX 2: Correct layout parameters for modern Bevy observers
fn respawn_on_endgame(
    _event: On<EndGame>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameMode>>,
) {
    // FIX 3: Use .single() instead of .get_single()
    let player_entity = player_query.single();

    match player_entity {
        Ok(player) => {
            commands.entity(player).insert((
                Transform::from_xyz(-CANVAS_SIZE.x / 4.0, 0.0, 1.0),
                Velocity(0.0),
            ));
        }

        Err(e) => {
            println!("Error at: fn respawn_on_endgame > match player_entity");
            println!("{e}");
        }
    }

    next_state.set(GameMode::Waiting);
    println!("Game Reset Successful!");
}
