mod window;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, transform};
use window::CustomPlugin;
use bevy::input::ButtonInput;
use bevy_framepace::Limiter;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component)]
struct Snake;

fn setup(mut commands: Commands, mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    commands.spawn(Camera2dBundle::default());
    settings.limiter = Limiter::from_framerate(10.0);
}

fn spawn_snake(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    let cube = Mesh2dHandle(meshes.add(Rectangle::new(25.0, 25.0)));
    commands.spawn((Snake, Direction::Up, MaterialMesh2dBundle{
        mesh: cube,
        material: materials.add(Color::rgb(1.0,0.0,0.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }));
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Direction, With<Snake>>) {
    for mut direction in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            *direction = Direction::Up;
        }
        if keys.pressed(KeyCode::KeyS) {
            *direction = Direction::Down;
            // transform.translation.y -= 10.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            *direction = Direction::Left;
            // transform.translation.x -= 10.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            *direction = Direction::Right;
            // transform.translation.x += 10.0;
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
          .add_systems(Startup, spawn_snake)
          .add_systems(Update, handle_input)
          .add_systems(Update, move_snake)
          .run();
}