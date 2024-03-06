mod window;
mod helper;

use bevy::{app::AppExit, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use window::{CustomPlugin, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::input::ButtonInput;
use bevy_framepace::Limiter;
use helper::MathHelper;

const SQUARE_THICKNESS : f32 = 25.0;

#[derive(Component, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component)]
struct Snake{
    is_dead: bool
}

fn setup(mut commands: Commands, mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    commands.spawn(Camera2dBundle::default());
    settings.limiter = Limiter::from_framerate(10.0);
}

fn generate_borders(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    let top_border = Mesh2dHandle(meshes.add(
                Cuboid::new(WINDOW_WIDTH + SQUARE_THICKNESS, SQUARE_THICKNESS , 1.0)));
    let bottom_border = Mesh2dHandle(meshes.add(
        Cuboid::new(WINDOW_WIDTH + SQUARE_THICKNESS, SQUARE_THICKNESS, 1.0)));
    let left_border = Mesh2dHandle(meshes.add(
            Cuboid::new(SQUARE_THICKNESS, WINDOW_HEIGHT + SQUARE_THICKNESS, 1.0)));
    let right_border = Mesh2dHandle(meshes.add(
            Cuboid::new(SQUARE_THICKNESS, WINDOW_HEIGHT + SQUARE_THICKNESS, 1.0)));
    
    commands.spawn(MaterialMesh2dBundle{
        mesh:top_border,
        material: materials.add(Color::rgb(1.0,1.0,1.0)),
        transform: Transform::from_xyz(0.0, -(WINDOW_HEIGHT / 2.0 - SQUARE_THICKNESS / 2.0), 0.0),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle{
        mesh:bottom_border,
        material: materials.add(Color::rgb(1.0,1.0,1.0)),
        transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - SQUARE_THICKNESS / 2.0, 0.0),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle{
        mesh:left_border,
        material: materials.add(Color::rgb(1.0,1.0,1.0)),
        transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0 + SQUARE_THICKNESS / 2.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle{
        mesh:right_border,
        material: materials.add(Color::rgb(1.0,1.0,1.0)),
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0 - SQUARE_THICKNESS/ 2.0, 0.0, 0.0),
        ..default()
    });
}

fn spawn_snake(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    let cube = Mesh2dHandle(meshes.add(Rectangle::new(SQUARE_THICKNESS, SQUARE_THICKNESS)));
    commands.spawn((Snake{ is_dead: false}, Direction::Up, MaterialMesh2dBundle{
        mesh: cube,
        material: materials.add(Color::rgb(0.0,1.0,0.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }));
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Direction, With<Snake>>) {
    for mut direction in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) && *direction != Direction::Down {
            *direction = Direction::Up;
        }
        if keys.pressed(KeyCode::KeyS) && *direction != Direction::Up {
            *direction = Direction::Down;
        }
        if keys.pressed(KeyCode::KeyA) && *direction != Direction::Right {
            *direction = Direction::Left;
        }
        if keys.pressed(KeyCode::KeyD) && *direction != Direction::Left {
            *direction = Direction::Right;
        }
    }
}

fn move_snake(mut query: Query<(&Direction, &mut Transform), With<Snake>>){
    for (direction, mut transform) in query.iter_mut(){
        match *direction{
            Direction::Up => transform.translation.y += 10.0,
            Direction::Down => transform.translation.y -= 10.0,
            Direction::Left => transform.translation.x -= 10.0,
            Direction::Right => transform.translation.x += 10.0,
        }
    }
}

fn check_border_collision(mut query : Query<(&Transform, &mut Snake)>){
    for(snake_transform, mut snake) in query.iter_mut(){
        let snake_position = snake_transform.translation;
        println!("Snake Y:{:?}", snake_position.y);
        println!("Snake X:{:?}", snake_position.x);
        let hit_vertical_border = ((snake_position.y + SQUARE_THICKNESS/ 2.0) >= MathHelper::round(WINDOW_HEIGHT / 2.0 - SQUARE_THICKNESS, 1))
                                    || ((snake_position.y - SQUARE_THICKNESS / 2.0) <= MathHelper::round(-(WINDOW_HEIGHT / 2.0 - SQUARE_THICKNESS), 1));
        let hit_horizontal_border = ((snake_position.x + SQUARE_THICKNESS / 2.0) >= MathHelper::round(WINDOW_WIDTH / 2.0 - SQUARE_THICKNESS, 1))
                                        || ((snake_position.x - SQUARE_THICKNESS / 2.0) <= MathHelper::round(-(WINDOW_WIDTH / 2.0 - SQUARE_THICKNESS), 1));

        snake.is_dead = hit_horizontal_border || hit_vertical_border;
    }
}

fn handle_snake_death(mut query: Query<&Snake>, mut exit: EventWriter<AppExit>){
    for snake in query.iter_mut(){
        if snake.is_dead {
            exit.send(AppExit);
        }
    }
}

fn main() {
    App::new()
          .add_plugins((CustomPlugin, bevy_framepace::FramepacePlugin))
          .add_systems(Startup, setup)
          .add_systems(Startup, generate_borders)
          .add_systems(Startup, spawn_snake)
          .add_systems(Update, handle_input)
          .add_systems(Update, move_snake)
          .add_systems(Update, check_border_collision)
          .add_systems(Update, handle_snake_death)
          .run();
}