// ex02-bevy-bird\src\main.rs
// Imports:
use bevy::{camera::ScalingMode, prelude::*};
use ex02_bevy_bird::*;

use bevy::color::palettes::tailwind::{RED_400, SLATE_50};
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};

// Flat Structure: Single-level Submodules:
mod flat_submodule;

// Nested Structure: Submodules with their own Submodules:
mod nested_submodule;

// OBS: What to select a default image filter:
// .add_plugins(DefaultPlugins.set(ImagePlugin {
//  default_sampler:
//      ImageSamplerDescriptor::nearest(),
// }))

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Event)]
pub struct ScorePoint;

#[derive(Component)]
struct ScoreText;

fn main() -> AppExit {
    flat_submodule::hello_flat_module();
    nested_submodule::handle_auth();

    let mut app = App::new();

    app.init_resource::<Score>()
        .add_plugins(DefaultPlugins)
        .add_plugins(PipePlugin)
        .init_state::<GameMode>()
        .add_observer(|_trigger: On<ScorePoint>, mut score: ResMut<Score>| {
            score.0 += 1;
        })
        .add_systems(Startup, startup)
        // Run this system ONLY if the gameplay is actually live
        .add_systems(
            Update,
            (controls, score_update.run_if(resource_changed::<Score>)),
        )
        .add_systems(
            FixedUpdate,
            (
                gravity.run_if(in_state(GameMode::Started)),
                check_in_bounds.run_if(in_state(GameMode::Started)),
                check_collisions,
            )
                .chain(),
        )
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

    commands.spawn((
        Node {
            width: percent(100),
            margin: px(20.0).top(),
            ..default()
        },
        Text::new("0"),
        TextLayout::justify(Justify::Center),
        TextFont {
            font_size: 33.0.into(),
            ..default()
        },
        TextColor(SLATE_50.into()),
        ScoreText,
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
    mut score: ResMut<Score>,
) {
    // FIX 3: Use .single() instead of .get_single()
    let player_entity = player_query.single();

    match player_entity {
        Ok(player) => {
            score.0 = 0;
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

fn check_collisions(
    mut commands: Commands,
    player: Single<(&Sprite, Entity), With<Player>>,
    pipe_segments: Query<(&Sprite, Entity), Or<(With<PipeTop>, With<PipeBottom>)>>,
    pipe_gaps: Query<(&Sprite, Entity), With<PointsGate>>,
    mut gizmos: Gizmos,
    transform_helper: TransformHelper,
) -> Result<()> {
    let player_transform = transform_helper.compute_global_transform(player.1)?;

    let player_collider =
        BoundingCircle::new(player_transform.translation().xy(), PLAYER_SIZE / 2.0);

    gizmos.circle_2d(
        player_transform.translation().xy(),
        PLAYER_SIZE / 2.0,
        RED_400,
    );

    for (sprite, entity) in &pipe_segments {
        let pipe_transform = transform_helper.compute_global_transform(entity)?;

        let pipe_collider = Aabb2d::new(
            pipe_transform.translation().xy(),
            sprite.custom_size.unwrap() / 2.0,
        );

        gizmos.rect_2d(
            pipe_transform.translation().xy(),
            sprite.custom_size.unwrap(),
            RED_400,
        );

        if player_collider.intersects(&pipe_collider) {
            commands.trigger(EndGame);
        }
    }

    for (sprite, entity) in &pipe_gaps {
        let gap_transform = transform_helper.compute_global_transform(entity)?;

        let gap_collider = Aabb2d::new(
            gap_transform.translation().xy(),
            sprite.custom_size.unwrap().xy(),
        );

        gizmos.rect_2d(
            gap_transform.translation().xy(),
            sprite.custom_size.unwrap().xy(),
            RED_400,
        );

        if player_collider.intersects(&gap_collider) {
            commands.trigger(ScorePoint);

            commands.entity(entity).despawn();
        }
    }

    Ok(())
}

fn score_update(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut span in &mut query {
        span.0 = score.0.to_string();
    }
}
