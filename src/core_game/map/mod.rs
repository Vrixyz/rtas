use bevy::{ecs::Commands, math::Vec2, math::Vec3, prelude::GlobalTransform, prelude::Transform};
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

use mapgen::filter;
use mapgen::{
    geometry::Rect, AreaStartingPosition, CullUnreachable, DistantExit, MapBuilder, NoiseGenerator,
    TileType, XStart, YStart,
};
use rand::prelude::*;

pub struct Wall;
pub struct WallSize {
    pub x: f32,
    pub y: f32,
}

pub struct Map {
    pub map: mapgen::Map,
}

const TILE_SIZE: f32 = 120f32;
const HALF_TILE: f32 = TILE_SIZE / 2f32;
const MAP_SIZE: (usize, usize) = (20, 20);
const offset_x: f32 = MAP_SIZE.0 as f32 * HALF_TILE;
const offset_y: f32 = MAP_SIZE.0 as f32 * HALF_TILE;

impl Map {
    pub fn real_x_at(x: usize) -> f32 {
        let position_x = x as f32 * TILE_SIZE - offset_x;
        position_x
    }
    pub fn real_y_at(y: usize) -> f32 {
        let position_y = y as f32 * TILE_SIZE - offset_y;
        position_y
    }

    pub fn real_position_at(x: usize, y: usize) -> Vec2 {
        Vec2::new(Self::real_x_at(x), Self::real_y_at(y))
    }

    pub fn map_x_at(x: f32) -> usize {
        let position_x = (x + offset_x) / (TILE_SIZE as f32);
        position_x.round() as usize
    }
    pub fn map_y_at(y: f32) -> usize {
        let position_y = (y + offset_y) / (TILE_SIZE as f32);
        position_y.round() as usize
    }
}

fn spawn_wall_at(commands: &mut Commands, position: Vec3, size: f32) {
    let size = Vec2::new(size, size);
    let rigid_body2 = RigidBodyBuilder::new_static().translation(position.x(), position.y());
    let collider2 = ColliderBuilder::cuboid(size.x(), size.y());
    commands.spawn((
        rigid_body2,
        collider2,
        Wall,
        WallSize {
            x: size.x(),
            y: size.y(),
        },
        Transform::from_translation(position),
        GlobalTransform::from_translation(position),
    ));
}

pub fn create_map(mut commands: Commands) {
    let mut rng = StdRng::seed_from_u64(100);
    let mut map = MapBuilder::new(MAP_SIZE.0, MAP_SIZE.1)
        .with(NoiseGenerator::uniform())
        //.with(filter::MazeBuilder::new())
        .with(filter::CellularAutomata::new())
        .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
        .with(CullUnreachable::new())
        .with(DistantExit::new())
        .build();
    if let Some(starting_point) = map.starting_point {
        let new_room = Rect::new(starting_point.x, starting_point.y, 3, 3);
        map.add_room(new_room);
        println!("Start: {:#?}", starting_point);
    } else {
        println!("no start..");
    }
    if let Some(exit_point) = map.exit_point {
        let new_room = Rect::new(exit_point.x, exit_point.y, 3, 3);
        //map.add_room(new_room);
        println!("Exit: {:#?}", exit_point);
    } else {
        println!("no exit..");
    }

    for y in (0..MAP_SIZE.1).rev() {
        let position_y = y as f32 * TILE_SIZE - offset_y;
        for x in 0..MAP_SIZE.0 {
            let position_x = x as f32 * TILE_SIZE - offset_x;
            let tile_type = map.at(x, y);

            if tile_type == TileType::Floor {
                if let Some(start) = map.starting_point {
                    if start.x == x && start.y == y {
                        print!("S");
                        continue;
                    }
                }
                if let Some(exit) = map.exit_point {
                    if exit.x == x && exit.y == y {
                        print!("E");
                        continue;
                    }
                }
                print!(" ");
                continue;
            }
            spawn_wall_at(
                &mut commands,
                Vec3::new(position_x, position_y, 0.0),
                HALF_TILE,
            );
            print!("X");
        }
        println!();
    }
    commands.insert_resource(Map { map });
}
