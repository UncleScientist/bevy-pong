use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(move_ball)
        .add_system(handle_input)
        .run();
}

#[derive(Component)]
struct Direction {
    dir: Vec2,
}

enum Player {
    Left,
    Right,
}

#[derive(Component)]
struct Paddle {
    player: Player,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..default()
    });
    let sprite = Sprite {
        color: Color::CYAN,
        custom_size: Some(Vec2::new(25.0, 25.0)),
        ..default()
    };
    commands.spawn((
        SpriteBundle {
            sprite,
            ..default()
        },
        Direction {
            dir: Vec2::new(200.0, 200.0),
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
}

fn move_ball(
    time: Res<Time>,
    mut ball_pos: Query<(&mut Direction, &mut Transform)>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let (height, width) = (window.height() / 2.0, window.width() / 2.0);

    let (mut direction, mut xform) = ball_pos.get_single_mut().expect("ball not found");
    let dir = direction.dir * time.delta_seconds();

    let new_pos = Vec2::new(xform.translation.x, xform.translation.y) + dir;
    if new_pos.y < -height || new_pos.y > height {
        direction.dir.y = -direction.dir.y;
    }
    if new_pos.x < -width || new_pos.x > width {
        direction.dir.x = -direction.dir.x;
    }

    let cur_pos = Vec2::new(xform.translation.x, xform.translation.y) + dir;

    xform.translation.x = cur_pos.x;
    xform.translation.y = cur_pos.y;
}

const PADDLE_SPEED: f32 = 300.0;
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
                        time.delta_seconds() * PADDLE_SPEED
                    } else if keys.pressed(KeyCode::S) {
                        -time.delta_seconds() * PADDLE_SPEED
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
                        time.delta_seconds() * PADDLE_SPEED
                    } else if keys.pressed(KeyCode::L) {
                        -time.delta_seconds() * PADDLE_SPEED
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
