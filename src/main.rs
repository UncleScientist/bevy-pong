use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(move_ball)
        .run();
}

#[derive(Component)]
struct Direction {
    dir: Vec2,
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

    let cur_pos = Vec2::new(xform.translation.x, xform.translation.y) + dir;
    if cur_pos.y < -height || cur_pos.y > height {
        direction.dir.y = -direction.dir.y;
    }
    if cur_pos.x < -width || cur_pos.x > width {
        direction.dir.x = -direction.dir.x;
    }
    let cur_pos = Vec2::new(xform.translation.x, xform.translation.y) + dir;

    xform.translation.x = cur_pos.x;
    xform.translation.y = cur_pos.y;
}
