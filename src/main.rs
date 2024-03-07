mod window;
mod helper;

use bevy::{app::AppExit, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use window::{CustomPlugin, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::input::ButtonInput;
use bevy_framepace::Limiter;
use helper::MathHelper;
use rand::Rng;

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

#[derive(Component)]
struct Food;

fn get_food_position(snake_position : Vec3) -> (f32, f32) {
    let spawnable_area_width = WINDOW_WIDTH - SQUARE_THICKNESS * 2.0;
    let spawnable_area_height = WINDOW_HEIGHT - SQUARE_THICKNESS * 2.0;

    let mut x : f32 = 0.0;
    let mut y : f32 = 0.0;

    while (x == snake_position.x || x == 0.0) && (y == snake_position.y || y == 0.0){
        let mut rng = rand::thread_rng();
        x = rng.gen_range(-spawnable_area_width / 2.0..spawnable_area_width / 2.0);
        y = rng.gen_range(-spawnable_area_height / 2.0..spawnable_area_height / 2.0);
    }
    (x, y)
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

fn handle_collisions( mut snake_param: ParamSet<(
    Query<(Entity, &Transform, &mut Snake)>,
    Query<(Entity, &mut Transform), With<Food>>,
)>){
    let mut snake_query = snake_param.p0();
    let mut snake_position : Vec3 = Vec3::new(0.0, 0.0, 0.0);
    for(_snake_entity, snake_transform, mut snake) in snake_query.iter_mut(){
        snake_position = snake_transform.translation;
        println!("Snake Y:{:?}", snake_position.y);
        println!("Snake X:{:?}", snake_position.x);

        //Check border collision
        let hit_vertical_border = ((snake_position.y + SQUARE_THICKNESS/ 2.0) >= MathHelper::round(WINDOW_HEIGHT / 2.0 - SQUARE_THICKNESS, 1))
                                    || ((snake_position.y - SQUARE_THICKNESS / 2.0) <= MathHelper::round(-(WINDOW_HEIGHT / 2.0 - SQUARE_THICKNESS), 1));
        let hit_horizontal_border = ((snake_position.x + SQUARE_THICKNESS / 2.0) >= MathHelper::round(WINDOW_WIDTH / 2.0 - SQUARE_THICKNESS, 1))
                                        || ((snake_position.x - SQUARE_THICKNESS / 2.0) <= MathHelper::round(-(WINDOW_WIDTH / 2.0 - SQUARE_THICKNESS), 1));

        snake.is_dead = hit_horizontal_border || hit_vertical_border;
        if snake.is_dead {
            println!("Hit border");
            return;
        }
    }
    let mut food_query = snake_param.p1();
    for (_food_entity, mut food_transform,) in food_query.iter_mut(){
        if snake_position.distance(food_transform.translation) < SQUARE_THICKNESS {

            //Change food position
            let position = get_food_position(snake_position);
            food_transform.translation.x = position.0;
            food_transform.translation.y = position.1;
            //Handle snake growth;
        }
    }
}

fn handle_snake_death(mut query: Query<&Snake>, mut exit: EventWriter<AppExit>){
    for snake in query.iter_mut(){
        if snake.is_dead {
            exit.send(AppExit);
        }
    }
}

fn spawn_food(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, mut snake_query : Query<&Transform, With<Snake>>){
    let mut snake_position: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    for transform in snake_query.iter_mut(){
        snake_position = transform.translation;
    }
   
    let position = get_food_position(snake_position);
    
    let food_mesh = Mesh2dHandle(meshes.add(Rectangle::new(SQUARE_THICKNESS, SQUARE_THICKNESS)));

    commands.spawn((Food, MaterialMesh2dBundle{
        mesh: food_mesh,
        material: materials.add(Color::rgb(1.0, 1.0, 1.0)),
        transform: Transform::from_xyz(position.0, position.1, 0.0),
        ..default()
    }));
}

// fn spawn_food(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, mut snake_query : Query<&Transform, With<Snake>>, mut food_query : Query<&Food>){
//     let mut snake_position: Vec3 = Vec3::new(0.0, 0.0, 0.0);
//     for transform in snake_query.iter_mut(){
//         snake_position = transform.translation;
//     }
//     for food in food_query.iter_mut(){
//         if !food.is_eaten{
//             let spawnable_area_width = WINDOW_WIDTH - SQUARE_THICKNESS * 2.0;
//             let spawnable_area_height = WINDOW_HEIGHT - SQUARE_THICKNESS * 2.0;
    
//             let mut x : f32 = 0.0;
//             let mut y : f32 = 0.0;
    
//             while (x == snake_position.x || x == 0.0) && (y == snake_position.y || y == 0.0){
//                 let mut rng = rand::thread_rng();
//                 x = rng.gen_range(-spawnable_area_width / 2.0..spawnable_area_width / 2.0);
//                 y = rng.gen_range(-spawnable_area_height / 2.0..spawnable_area_height / 2.0);
//             }
            
//             let food_mesh = Mesh2dHandle(meshes.add(Rectangle::new(SQUARE_THICKNESS, SQUARE_THICKNESS)));
    
//             commands.spawn((Food{is_eaten: false}, MaterialMesh2dBundle{
//                 mesh: food_mesh,
//                 material: materials.add(Color::rgb(1.0, 1.0, 1.0)),
//                 transform: Transform::from_xyz(x, y, 0.0),
//                 ..default()
//             }));
//         }
//     }
// }

fn main() {
    App::new()
          .add_plugins((CustomPlugin, bevy_framepace::FramepacePlugin))
          .add_systems(Startup, setup)
          .add_systems(Startup, generate_borders)
          .add_systems(Startup, (spawn_snake, spawn_food))
          .add_systems(Update, (handle_collisions,
                                                handle_snake_death,
                                                handle_input,
                                                move_snake,
                                                ))
          .run();
}