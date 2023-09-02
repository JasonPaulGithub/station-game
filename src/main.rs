use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

// Create a grid.
// Create black and white squares and put on grid.
// Click a square and it turns green

#[derive(Component)]
enum Direction {
    UP,
    DOWN,
    Left,
    Right,
}

struct Tile {
    x: i32,
    y: i32,
    sprite: String,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // let grid : Grid<Tile> = Grid::new(3,3);
    // let tile = Tile(1,1,"bevy_pixel_light.png": String);

    // grid[0] = tile;

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_pixel_light.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Right,
    ));
}

fn sprite_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>
) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::UP => transform.translation.y += 30. * time.delta_seconds(),
            Direction::DOWN => transform.translation.y -= 30. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 30. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 30. * time.delta_seconds(),
        }
        if keyboard_input.pressed(KeyCode::W) {
            *logo = Direction::UP;
        }
        if keyboard_input.pressed(KeyCode::S) {
            *logo = Direction::DOWN;
        }
        if keyboard_input.pressed(KeyCode::A) {
            *logo = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::D) {
            *logo = Direction::Right;
        }
    }
}
