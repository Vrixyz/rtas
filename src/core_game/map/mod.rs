use bevy::{ecs::Commands, math::Vec3, prelude::GlobalTransform, prelude::Transform, math::Vec2};
use bevy_rapier2d::rapier::{geometry::ColliderBuilder, dynamics::RigidBodyBuilder};

use rand::prelude::*;
use mapgen::{AreaStartingPosition, CullUnreachable, DistantExit, MapBuilder, NoiseGenerator, geometry::Rect, TileType, XStart, YStart};
use mapgen::filter;


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

impl Map {
    pub fn real_position_at(x: usize, y: usize) -> Vec2 {
        let offset_x = MAP_SIZE.0 as f32 * HALF_TILE;
        let offset_y = MAP_SIZE.1 as f32 * HALF_TILE;

        let position_x = x as f32 * TILE_SIZE - offset_x;
        let position_y = y as f32 * TILE_SIZE - offset_y;

        Vec2::new(position_x, position_y)
    }
}

fn spawn_wall_at(commands: &mut Commands, position: Vec3, size: f32) {
    let size = Vec2::new(size, size);
    let rigid_body2 = RigidBodyBuilder::new_static()
        .translation(position.x(), position.y())
    ;
    let collider2 = ColliderBuilder::cuboid(size.x(), size.y());
    commands.spawn((rigid_body2, collider2, Wall, WallSize {x: size.x(), y: size.y()}, Transform::from_translation(position), GlobalTransform::from_translation(position)));
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
    }
    else {
        println!("no start..");
    }
    if let Some(exit_point) = map.exit_point {
        let new_room = Rect::new(exit_point.x, exit_point.y, 3, 3);
        //map.add_room(new_room);
        println!("Exit: {:#?}", exit_point);
    }
    else {
        println!("no exit..");
    }
    /*
    let offset_x = (MAP_SIZE.0) as f32 * HALF_TILE;
    let offset_y = (MAP_SIZE.1) as f32 * HALF_TILE;

    for y in 0..MAP_SIZE.1 as i64 {
        let position_y = y as f32 * TILE_SIZE - offset_y;
        spawn_wall_at(&mut commands, Vec3::new(-offset_x - TILE_SIZE, position_y as f32, 0.0), HALF_TILE);
        spawn_wall_at(&mut commands, Vec3::new(offset_x, position_y as f32, 0.0), HALF_TILE);
    }
    for x in 0..MAP_SIZE.0 {
        let position_x = x as f32 * TILE_SIZE - offset_x;
        spawn_wall_at(&mut commands, Vec3::new(position_x, -offset_y - TILE_SIZE, 0.0), HALF_TILE);
        spawn_wall_at(&mut commands, Vec3::new(position_x, offset_y, 0.0), HALF_TILE);
    }
    spawn_wall_at(&mut commands, Vec3::new(-offset_x - TILE_SIZE, -offset_y - TILE_SIZE, 0.0), HALF_TILE);
    spawn_wall_at(&mut commands, Vec3::new(-offset_x - TILE_SIZE, offset_y, 0.0), HALF_TILE);
    spawn_wall_at(&mut commands, Vec3::new(offset_x, -offset_y - TILE_SIZE, 0.0), HALF_TILE);
    spawn_wall_at(&mut commands, Vec3::new(offset_x, offset_y, 0.0), HALF_TILE);
    */
    let offset_x = MAP_SIZE.0 as f32 * HALF_TILE;
    let offset_y = MAP_SIZE.1 as f32 * HALF_TILE;

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
            spawn_wall_at(&mut commands, Vec3::new(position_x, position_y, 0.0), HALF_TILE);
            print!("X");
        }
        println!();
    }
    commands.insert_resource(Map {map});
}