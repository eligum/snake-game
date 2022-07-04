use bevy::prelude::*;
use bevy::{core::FixedTimestep, render::camera::ScalingMode};
use rand::prelude::random;

const GRID_WIDTH: u32 = 20;
const GRID_HEIGHT: u32 = 20;
const CLEAR_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const ASPECT_RATIO: f32 = 1.0 / 1.0;
const SNAKE_COLOR: Color = Color::rgb(0.4, 1.0, 0.2);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.65, 0.0);
// const WALL_COLOR: Color = Color::BLACK;
// const PATH_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

impl SnakeSegments {
    fn iter(&self) -> std::slice::Iter<'_, Entity> {
        self.0.iter()
    }
}

#[derive(Component)]
struct Food;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
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

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Pathfinding Visualizer".to_string(),
            width: 800.0,
            height: 800.0,
            position: None,
            resizable: true,
            decorations: true,
            cursor_locked: false,
            cursor_visible: true,
            ..Default::default()
        })
        .insert_resource(SnakeSegments::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.25))
                .with_system(snake_movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_food),
        )
        .add_system(snake_movement_input.before(snake_movement))
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.bottom = -400.0;
    camera.orthographic_projection.top = 400.0;
    camera.orthographic_projection.left = -400.0 * ASPECT_RATIO;
    camera.orthographic_projection.right = 400.0 * ASPECT_RATIO;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_snake_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

#[rustfmt::skip]
fn snake_movement_input(
    kbd_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    // Only one entity has the SnakeHead component
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if kbd_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if kbd_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if kbd_input.pressed(KeyCode::Right) {
            Direction::Right
        } else if kbd_input.pressed(KeyCode::Down) {
            Direction::Down
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn spawn_snake_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn spawn_food(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * GRID_WIDTH as f32) as i32,
            y: (random::<f32>() * GRID_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width * (window.width() / GRID_WIDTH as f32),
            sprite_size.height * (window.height() / GRID_HEIGHT as f32),
            1.0,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, length: f32, tile_count: f32) -> f32 {
        let tile_size = length / tile_count;
        pos * tile_size - length / 2.0 + tile_size / 2.0
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
