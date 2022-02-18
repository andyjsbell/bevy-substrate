use bevy::prelude::*;
mod map;

#[derive(Component)]
struct Wizard;

#[derive(Component)]
enum Collider {
    Solid,
    Wizard,
}
const START_POS: (usize, usize) = (0, 0);

fn move_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Wizard, &mut Transform)>,
) {
    const SPEED: f32 = 200.0;
    for (_, mut transform) in query.iter_mut() {
        let mut x_direction = 0.0;
        let mut y_direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            x_direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            x_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            y_direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            y_direction -= 1.0;
        }

        let translation = &mut transform.translation;
        // move sensei horizontally
        translation.x += time.delta_seconds() * x_direction * SPEED;
        // move sensei vertically
        translation.y += time.delta_seconds() * y_direction * SPEED;

        // bound sensei within the walls
        translation.x = translation.x.min(380.0).max(-380.0);
        translation.y = translation.y.min(300.0).max(-300.0);
    }
}

#[derive(Component)]
struct Block;

fn initialise_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // setup the world
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("wizard.png"),
            transform: Transform {
                translation: Vec3::new(
                    (START_POS.0 * map::GRID_SIZE + map::GRID_SIZE / 2) as f32,
                    (START_POS.1 * map::GRID_SIZE + map::GRID_SIZE / 2) as f32,
                    0.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wizard {})
        .insert(Collider::Wizard);
}

fn initialise_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    match map::create_map(640usize, 480usize, (320, 240)) {
        Ok(map) => {
            let mut row = 0;
            let mut column = 0;

            for tile in map.tiles {
                if tile == map::TileType::Wall {
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(
                                    ((column * map::GRID_SIZE) + (map::GRID_SIZE / 2)) as f32 - 320.,
                                    ((row * map::GRID_SIZE) + (map::GRID_SIZE / 2)) as f32 - 240.,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                            texture: asset_server.load("wall.png"),
                            ..Default::default()
                        })
                        .insert(Block {})
                        .insert(Collider::Solid);
                }

                column += 1;
                if column == map.columns {
                    column = 0;
                    row += 1;
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Sub".to_string(),
            width: 640.,
            height: 480.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(initialise_player)
        .add_startup_system(initialise_map)
        .add_system(move_system)
        .run();
}
