#![windows_subsystem = "windows"]

use bevy::prelude::*;
use rand::Rng;

// Grid configuration constants
const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;
const CELL_SIZE: f32 = 30.0;
const MOVE_INTERVAL_SECONDS: f32 = 0.15;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

// Marker component for UI elements so we can easily despawn them
#[derive(Component)]
struct GameOverMenu;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Food;

#[derive(Resource, Default)]
struct SnakeSegments(Vec<Entity>);

#[derive(Resource)]
struct MoveTimer(Timer);

#[derive(Resource, Default, Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Resource, Default)]
struct LastInputDirection(Direction);

#[derive(Resource, Default)]
struct Score(u32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Snake".into(),
                    resolution: (
                        (GRID_WIDTH as f32 * CELL_SIZE) as u32,
                        (GRID_HEIGHT as f32 * CELL_SIZE) as u32,
                    )
                        .into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
        )
        .init_state::<GameState>()
        .init_resource::<SnakeSegments>()
        .init_resource::<LastInputDirection>()
        .init_resource::<Score>()
        .insert_resource(MoveTimer(Timer::from_seconds(
            MOVE_INTERVAL_SECONDS,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (setup_camera, setup_game))
        .add_systems(Update, (handle_input, game_loop, position_translation))
        .add_systems(
            Update,
            (handle_input, game_loop, position_translation).run_if(in_state(GameState::Playing)), // Only run game logic while playing
        )
        // GameOver systems
        .add_systems(OnEnter(GameState::GameOver), setup_game_over_ui)
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over_ui)
        .add_systems(
            Update,
            handle_restart_input.run_if(in_state(GameState::GameOver)),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_game(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    segments.0.clear();

    let head_pos = Position {
        x: GRID_WIDTH / 2,
        y: GRID_HEIGHT / 2,
    };

    let head_entity = commands
        .spawn((
            SnakeHead,
            SnakeSegment,
            head_pos,
            Sprite {
                color: Color::srgb(0.2, 0.8, 0.2),
                custom_size: Some(Vec2::splat(CELL_SIZE - 2.0)),
                ..default()
            },
            Transform::default(),
        ))
        .id();

    segments.0.push(head_entity);

    spawn_food(&mut commands, &segments);
}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut last_dir: ResMut<LastInputDirection>) {
    let current_dir = last_dir.0;

    let new_dir = if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        Direction::Up
    } else if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        Direction::Down
    } else if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        Direction::Left
    } else if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        Direction::Right
    } else {
        return;
    };

    if new_dir != current_dir.opposite() {
        last_dir.0 = new_dir;
    }
}

fn game_loop(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    last_dir: Res<LastInputDirection>,
    mut segments: ResMut<SnakeSegments>,
    mut head_query: Query<&mut Position, With<SnakeHead>>,
    mut positions_query: Query<&mut Position, (Without<SnakeHead>, Without<Food>)>,
    food_query: Query<(Entity, &Position), (With<Food>, Without<SnakeHead>)>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>, // <-- Added parameter
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let Ok(mut head_pos) = head_query.single_mut() else {
        return;
    };
    let old_head_pos = *head_pos;

    // 1. Move Head
    match last_dir.0 {
        Direction::Up => head_pos.y += 1,
        Direction::Down => head_pos.y -= 1,
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
    }

    // 2. Check Wall Collisions
    if head_pos.x < 0 || head_pos.x >= GRID_WIDTH || head_pos.y < 0 || head_pos.y >= GRID_HEIGHT {
        next_state.set(GameState::GameOver);
        return;
    }

    // 3. Check Self Collisions
    for &segment_entity in segments.0.iter().skip(1) {
        if let Ok(seg_pos) = positions_query.get(segment_entity) {
            if *seg_pos == *head_pos {
                next_state.set(GameState::GameOver);
                return;
            }
        }
    }

    // 4. Check Food Collision
    let mut ate_food = false;
    for (food_entity, food_pos) in food_query.iter() {
        if *food_pos == *head_pos {
            commands.entity(food_entity).despawn();
            ate_food = true;
            score.0 += 10;
            info!("Score: {}", score.0);
            spawn_food(&mut commands, &segments);
            break;
        }
    }

    // 5. Move Body Segments down the chain
    let mut previous_position = old_head_pos;
    for &segment_entity in segments.0.iter().skip(1) {
        if let Ok(mut pos) = positions_query.get_mut(segment_entity) {
            let temp = *pos;
            *pos = previous_position;
            previous_position = temp;
        }
    }

    // 6. Grow Snake if food was eaten
    if ate_food {
        let new_segment = commands
            .spawn((
                SnakeSegment,
                previous_position,
                Sprite {
                    color: Color::srgb(0.4, 0.9, 0.4),
                    custom_size: Some(Vec2::splat(CELL_SIZE - 2.0)),
                    ..default()
                },
                Transform::default(),
            ))
            .id();

        segments.0.push(new_segment);
    }
}

fn setup_game_over_ui(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            GameOverMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            // "Game Over" Header
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: FontSize::Px(60.0), // <-- Updated to FontSize::Px
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.2, 0.2)),
            ));

            // Final Score
            parent.spawn((
                Text::new(format!("Final Score: {}", score.0)),
                TextFont {
                    font_size: FontSize::Px(35.0), // <-- Updated to FontSize::Px
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
            ));

            // Restart Prompt
            parent.spawn((
                Text::new("Press 'R' to Restart"),
                TextFont {
                    font_size: FontSize::Px(25.0), // <-- Updated to FontSize::Px
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

fn handle_restart_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    head_query: Query<&mut Position, With<SnakeHead>>,
    mut score: ResMut<Score>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // Reset game state entities (snake body, score, head position)
        restart_game(&mut commands, &mut segments, head_query, &mut score);

        // Return to Playing state
        next_state.set(GameState::Playing);
    }
}

fn cleanup_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_food(commands: &mut Commands, segments: &SnakeSegments) {
    let mut rng = rand::thread_rng();

    let food_pos = loop {
        let pos = Position {
            x: rng.gen_range(0..GRID_WIDTH),
            y: rng.gen_range(0..GRID_HEIGHT),
        };

        if !segments.0.is_empty() {
            break pos;
        } else {
            break pos;
        }
    };

    commands.spawn((
        Food,
        food_pos,
        Sprite {
            color: Color::srgb(0.9, 0.2, 0.2),
            custom_size: Some(Vec2::splat(CELL_SIZE - 2.0)),
            ..default()
        },
        Transform::default(),
    ));
}

fn position_translation(mut query: Query<(&Position, &mut Transform)>) {
    let x_offset = (GRID_WIDTH as f32 * CELL_SIZE) / 2.0 - CELL_SIZE / 2.0;
    let y_offset = (GRID_HEIGHT as f32 * CELL_SIZE) / 2.0 - CELL_SIZE / 2.0;

    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            pos.x as f32 * CELL_SIZE - x_offset,
            pos.y as f32 * CELL_SIZE - y_offset,
            0.0,
        );
    }
}

fn restart_game(
    commands: &mut Commands,
    segments: &mut ResMut<SnakeSegments>,
    mut head_query: Query<&mut Position, With<SnakeHead>>,
    score: &mut ResMut<Score>,
) {
    // 1. Despawn extra body segments (skip head at index 0)
    for &segment_entity in segments.0.iter().skip(1) {
        commands.entity(segment_entity).despawn();
    }
    segments.0.truncate(1); // Keep only head

    // 2. Reset head position back to center using `single_mut`
    if let Ok(mut head_pos) = head_query.single_mut() {
        head_pos.x = GRID_WIDTH / 2;
        head_pos.y = GRID_HEIGHT / 2;
    }

    // 3. Reset Score
    score.0 = 0;
}
