use bevy::sprite::collide_aabb::Collision;
use bevy::window::WindowResized;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::collide_aabb::collide,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(window_resize)
        .add_system(bevy::window::close_on_esc)
        .add_system(move_ball)
        .add_system(handle_input)
        .add_system(collision)
        .add_system(handle_score.run_if(on_event::<IncrementScore>()))
        .add_event::<IncrementScore>()
        .run();
}

#[derive(Component)]
struct Direction {
    dir: Vec2,
}

#[derive(PartialEq, Eq)]
enum Player {
    Left,
    Right,
}

struct IncrementScore(Player);

#[derive(Component)]
struct Paddle {
    player: Player,
}

#[derive(Component)]
struct Score {
    player: Player,
    points: usize,
}

const BALL_SIZE: f32 = 25.0;
const BALL_SPEED: f32 = 400.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    let window = windows.single();
    let (height, width) = (window.height(), window.width());
    let score_pos_y = height / 2.0 - 100.0;
    let score_pos_x = width / 4.0;

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::TEAL,
    };

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..default()
    });

    // The bouncing ball
    let sprite = Sprite {
        color: Color::CYAN,
        custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
        ..default()
    };
    commands.spawn((
        SpriteBundle {
            sprite,
            ..default()
        },
        Direction {
            dir: Vec2::new(BALL_SPEED, BALL_SPEED),
        },
    ));

    // player left
    let sprite = Sprite {
        color: Color::AQUAMARINE,
        custom_size: Some(Vec2::new(25.0, 100.0)),
        ..default()
    };
    commands.spawn((
        SpriteBundle {
            sprite,
            transform: Transform::from_xyz(-100.0, 0.0, 0.0),
            ..default()
        },
        Paddle {
            player: Player::Left,
        },
    ));

    // player left score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("0", text_style.clone()),
            transform: Transform::from_xyz(-score_pos_x, score_pos_y, 0.0),
            ..default()
        },
        Score {
            player: Player::Left,
            points: 0,
        },
    ));

    // player right
    let sprite = Sprite {
        color: Color::AQUAMARINE,
        custom_size: Some(Vec2::new(25.0, 100.0)),
        ..default()
    };
    commands.spawn((
        SpriteBundle {
            sprite,
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
        Paddle {
            player: Player::Right,
        },
    ));

    // player right score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("0", text_style),
            transform: Transform::from_xyz(score_pos_x, score_pos_y, 0.0),
            ..default()
        },
        Score {
            player: Player::Right,
            points: 0,
        },
    ));
}

fn move_ball(
    time: Res<Time>,
    mut ball_pos: Query<(&mut Direction, &mut Transform)>,
    mut inc_score: EventWriter<IncrementScore>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let (height, width) = (window.height() / 2.0, window.width() / 2.0);

    let (mut direction, mut xform) = ball_pos.get_single_mut().expect("ball not found");
    let dir = direction.dir * time.delta_seconds();

    let new_pos = Vec2::new(xform.translation.x, xform.translation.y) + dir;
    if new_pos.y < -height {
        direction.dir.y = direction.dir.y.abs();
    } else if new_pos.y > height {
        direction.dir.y = -(direction.dir.y.abs());
    }

    if new_pos.x < -width {
        inc_score.send(IncrementScore(Player::Right));
    } else if new_pos.x > width {
        inc_score.send(IncrementScore(Player::Left));
    } else {
        let cur_pos = Vec2::new(xform.translation.x, xform.translation.y) + dir;
        xform.translation.x = cur_pos.x;
        xform.translation.y = cur_pos.y;
    }
}

const PADDLE_MARGIN: f32 = 25.0;

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
    time: Res<Time>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let (height, width) = (window.height() / 2.0, window.width() / 2.0);

    for (paddle, mut xform) in query.iter_mut() {
        match paddle.player {
            Player::Left => {
                xform.translation.x = -width + PADDLE_MARGIN;

                let new_loc = xform.translation.y
                    + if keys.pressed(KeyCode::W) {
                        time.delta_seconds() * 1.5 * BALL_SPEED
                    } else if keys.pressed(KeyCode::S) {
                        -time.delta_seconds() * 1.5 * BALL_SPEED
                    } else {
                        0.0
                    };
                if new_loc > -height && new_loc < height {
                    xform.translation.y = new_loc;
                }
            }
            Player::Right => {
                xform.translation.x = width - PADDLE_MARGIN;

                let new_loc = xform.translation.y
                    + if keys.pressed(KeyCode::O) {
                        time.delta_seconds() * 1.5 * BALL_SPEED
                    } else if keys.pressed(KeyCode::L) {
                        -time.delta_seconds() * 1.5 * BALL_SPEED
                    } else {
                        0.0
                    };
                if new_loc > -height && new_loc < height {
                    xform.translation.y = new_loc;
                }
            }
        }
    }
}

fn collision(
    paddles: Query<(&Paddle, &Transform)>,
    mut ball_pos: Query<(&mut Direction, &Transform)>,
) {
    let (mut ball_dir, ball_pos) = ball_pos.single_mut();
    let ball_size = Vec2::new(BALL_SIZE, BALL_SIZE);
    let paddle_size = Vec2::new(25.0, 100.0);

    for (_, pos) in paddles.iter() {
        match collide(
            ball_pos.translation,
            ball_size,
            pos.translation,
            paddle_size,
        ) {
            Some(Collision::Left) => ball_dir.dir.x = -(ball_dir.dir.x.abs()),
            Some(Collision::Right) => ball_dir.dir.x = ball_dir.dir.x.abs(),
            _ => {}
        }
    }
}

fn window_resize(
    mut resizer: EventReader<WindowResized>,
    mut query: Query<(&mut Transform, &Score)>,
) {
    if let Some(event) = resizer.iter().next() {
        for (mut text_pos, score) in query.iter_mut() {
            text_pos.translation.y = event.height / 2.0 - 100.0;

            match score.player {
                Player::Left => text_pos.translation.x = -(event.width / 4.0),
                Player::Right => text_pos.translation.x = event.width / 4.0,
            }
        }
    }
}

fn handle_score(
    mut ball_pos: Query<(&Direction, &mut Transform)>,
    mut scores: EventReader<IncrementScore>,
    mut query: Query<(&mut Text, &mut Score)>,
) {
    for score in scores.iter() {
        for (mut text, mut which) in query.iter_mut() {
            match score {
                IncrementScore(Player::Left) if which.player == Player::Left => {
                    which.points += 1;
                    text.sections[0].value = format!("{}", which.points);
                }
                IncrementScore(Player::Right) if which.player == Player::Right => {
                    which.points += 1;
                    text.sections[0].value = format!("{}", which.points);
                }
                _ => {}
            }
        }
    }

    let (_, mut xform) = ball_pos.get_single_mut().expect("missing ball");
    xform.translation.x = 0.0;
    xform.translation.y = 0.0;
}
