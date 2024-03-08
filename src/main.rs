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
    is_dead: bool,
    length : i32,
    tail_positions : Vec<Vec3>
}

#[derive(Component)]
struct Tail{
    length : i32
}

#[derive(Component)]
struct Food;

fn get_food_position(snake_position : Vec3) -> (f32, f32) {
    let spawnable_area_width = WINDOW_WIDTH - SQUARE_THICKNESS * 3.0;
    let spawnable_area_height = WINDOW_HEIGHT - SQUARE_THICKNESS * 3.0;

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
    commands.spawn((Snake{ is_dead: false, tail_positions: Vec::new(), length: 0}, Direction::Up, MaterialMesh2dBundle{
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

fn move_snake(mut query: Query<(&Direction, &mut Transform, &mut Snake), With<Snake>>){
    for (direction, mut transform, mut snake) in query.iter_mut(){
        let mut previous_x : f32 = transform.translation.x;
        let mut previous_y : f32 = transform.translation.y;
        match *direction{
            Direction::Up => transform.translation.y += 10.0,
            Direction::Down => transform.translation.y -= 10.0,
            Direction::Left => transform.translation.x -= 10.0,
            Direction::Right => transform.translation.x += 10.0,
        }
        //Move tail positions
        for tail_position in snake.tail_positions.iter_mut(){
            let temp = Clone::clone(tail_position);
            tail_position.x = previous_x;
            tail_position.y = previous_y;
            previous_x = temp.x;
            previous_y = temp.y;
        }
    }
}

fn move_tails(mut tail_query: Query<(&mut Transform, &Tail), Without<Snake>>, mut snake_query : Query<&Snake, Without<Tail>>){
    for snake in snake_query.iter_mut(){
        for (mut tail_transform, tail) in tail_query.iter_mut(){
            let tail_position = snake.tail_positions.get((tail.length - 1) as usize).unwrap();
            tail_transform.translation.x = tail_position.x;
            tail_transform.translation.y = tail_position.y;
        }
    }
}

fn handle_snake_collisions( mut snake_query : Query<(&Transform, &mut Snake)>){
    for(snake_transform, mut snake) in snake_query.iter_mut(){
        let snake_position = snake_transform.translation;
        // println!("Snake Y:{:?}", snake_position.y);
        // println!("Snake X:{:?}", snake_position.x);

        //Check border collision
        let hit_vertical_border = ((snake_position.y + SQUARE_THICKNESS/ 2.0) >= MathHelper::round(WINDOW_HEIGHT / 2.0 - SQUARE_THICKNESS, 1))
                                    || ((snake_position.y - SQUARE_THICKNESS / 2.0) <= MathHelper::round(-(WINDOW_HEIGHT / 2.0 - SQUARE_THICKNESS), 1));
        let hit_horizontal_border = ((snake_position.x + SQUARE_THICKNESS / 2.0) >= MathHelper::round(WINDOW_WIDTH / 2.0 - SQUARE_THICKNESS, 1))
                                        || ((snake_position.x - SQUARE_THICKNESS / 2.0) <= MathHelper::round(-(WINDOW_WIDTH / 2.0 - SQUARE_THICKNESS), 1));

        let mut hit_tail = false;
        for tail_positions in snake.tail_positions.iter().skip(1) {
            hit_tail = snake_position.distance(*tail_positions) < 10.0;
            if hit_tail {
                println!("HIT TAIL");
                break;
            }
        }

        snake.is_dead = hit_horizontal_border || hit_vertical_border || hit_tail;
        if snake.is_dead {
            println!("Died");
            return;
        }
    }
}

fn handle_food_collision(mut food_query : Query<(&mut Transform, &Food), Without<Snake>>,
mut snake_query : Query<(&Transform, &mut Snake), Without<Food>>,
mut commands: Commands,
mut meshes: ResMut<Assets<Mesh>>,
mut materials: ResMut<Assets<ColorMaterial>>,
mut settings: ResMut<bevy_framepace::FramepaceSettings>){
    for (snake_transform, mut snake) in snake_query.iter_mut(){
        for (mut food_transform, _food) in food_query.iter_mut(){
            //Eat food
            if snake_transform.translation.distance(food_transform.translation) < SQUARE_THICKNESS {
                println!("FOOD ATE");
                //Change food position
                let position = get_food_position(snake_transform.translation);
                food_transform.translation.x = position.0;
                food_transform.translation.y = position.1;
    
                //Handle snake growth;
                snake.length += 1;

                let tail = Mesh2dHandle(meshes.add(Rectangle::new(SQUARE_THICKNESS, SQUARE_THICKNESS)));
                if snake.tail_positions.len() == 0{
                    commands.spawn((Tail{length: snake.length}, MaterialMesh2dBundle{
                        mesh: tail,
                        material: materials.add(Color::rgb(0.0,1.0,0.0)),
                        transform: Transform::from_translation(snake_transform.translation),
                        ..default()
                    }));
                    snake.tail_positions.push(snake_transform.translation);
                }
                else{
                    commands.spawn((Tail{length: snake.length}, MaterialMesh2dBundle{
                        mesh: tail,
                        material: materials.add(Color::rgb(0.0,1.0,0.0)),
                        transform: Transform::from_translation(Clone::clone(snake.tail_positions.get(snake.tail_positions.len() - 1).unwrap())),
                        ..default()
                    }));
                    let pos = Clone::clone(snake.tail_positions.get(snake.tail_positions.len() - 1).unwrap());
                    snake.tail_positions.push(pos);
                }
                
                //Increase game speed
                settings.limiter = Limiter::from_framerate(10.0 + snake.length as f64);
            }
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

fn main() {
    App::new()
          .add_plugins((CustomPlugin, bevy_framepace::FramepacePlugin))
          .add_systems(Startup, setup)
          .add_systems(Startup, generate_borders)
          .add_systems(Startup, (spawn_snake, spawn_food))
          .add_systems(Update, (
                                                handle_food_collision,
                                                handle_snake_death,
                                                handle_input,
                                                move_snake,
                                                move_tails,
                                                handle_snake_collisions,
                                                ))
          .run();
}