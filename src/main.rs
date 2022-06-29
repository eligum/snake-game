use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.4, 1.0, 0.2);
const GRID_WIDTH: u32  = 10;
const GRID_HEIGHT: u32 = 10;
// const WALL_COLOR: Color = Color::BLACK;
// const PATH_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct SnakeHead;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(mut command: Commands) {
    command
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            // transform: Transform {
            //     scale: Vec3::new(10.0, 10.0, 10.0),
            //     ..default()
            // },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 9, y: 9 })
        .insert(Size::square(0.8));
}

fn snake_movement(
    kbd_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if kbd_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.0;
        }
        if kbd_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.0;
        }
        if kbd_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.0;
        }
        if kbd_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.0;
        }
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        println!("{:#?}", sprite_size);
        println!("{:#?}", transform);
        println!("---");
        transform.scale = Vec3::new(
            sprite_size.width / GRID_WIDTH as f32 * window.width() as f32,
            sprite_size.height / GRID_HEIGHT as f32 * window.height() as f32,
            1.0,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let title_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (title_size / 2.0)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, GRID_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, GRID_HEIGHT as f32),
            0.0,
        );
    }
}