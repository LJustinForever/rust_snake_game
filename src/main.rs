mod window;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use window::CustomPlugin;
use bevy::input::ButtonInput;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    let cube = Mesh2dHandle(meshes.add(Rectangle::new(25.0, 25.0)));
    commands.spawn(MaterialMesh2dBundle{
        mesh: cube,
        material: materials.add(Color::rgb(1.0,0.0,0.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn move_cube(keys: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Transform, &Mesh2dHandle)>) {
    for (mut transform, _mesh) in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += 1.0;
        }
    }
}

fn main() {
    App::new()
          .add_plugins(CustomPlugin)
          .add_systems(Startup, setup_camera)
          .add_systems(Startup, draw_cube)
          .add_systems(Update, move_cube)
          .run();
}