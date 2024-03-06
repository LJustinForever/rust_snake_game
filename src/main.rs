mod window;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use window::{CustomPlugin, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::input::ButtonInput;
use bevy_framepace::Limiter;


const SQUARE_THICKNESS : f32 = 25.0;

#[derive(Component, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component)]
struct Snake;


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

fn setup(mut commands: Commands, mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    commands.spawn(Camera2dBundle::default());
    settings.limiter = Limiter::from_framerate(10.0);
}

fn spawn_snake(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    let cube = Mesh2dHandle(meshes.add(Rectangle::new(SQUARE_THICKNESS, SQUARE_THICKNESS)));
    commands.spawn((Snake, Direction::Up, MaterialMesh2dBundle{
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
// fn move_cube(keyboard_event: EventReader<KeyboardInput>, mut query: Query<(&mut Transform, &Mesh2dHandle)>) {
//     for (mut transform, _mesh) in query.iter_mut() {
//         for ev in keyboard_event.read() {
//             match ev.key_code {
//                 KeyCode::KeyW => {
//                     transform.translation.y += 10.0;
//                 },
//                 KeyCode::KeyS => {
//                     transform.translation.y -= 10.0;
//                 },
//                 KeyCode::KeyA => {
//                     transform.translation.x -= 10.0;
//                 },
//                 KeyCode::KeyD => {
//                     transform.translation.x += 10.0;
//                 }
//                 _=> {}
//             }
//         }
//     }
// }

fn main() {
    App::new()
          .add_plugins((CustomPlugin, bevy_framepace::FramepacePlugin))
          .add_systems(Startup, setup)
          .add_systems(Startup, generate_borders)
          .add_systems(Startup, spawn_snake)
          .add_systems(Update, handle_input)
          .add_systems(Update, move_snake)
          .run();
}