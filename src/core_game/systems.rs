use bevy::{math, prelude::*};
use super::components::*;

pub fn create_units(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Selectable {is_selected: false})
        .with(Mover{target_position:Position{x:0.0, y:0.0}, speed: 50f32})
        .with_children(|parent| {
            parent.spawn(SpriteComponents {
                material: materials.add(Color::rgba(1.0, 0.5, 0.0, 0.2).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(26.0, 26.0)),
                ..Default::default()
            }).with(SelectionVisual);
        })
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Selectable {is_selected: false})
        .with(Mover{target_position:Position{x:0.0, y:-200.0}, speed: 100f32})
        .with_children(|parent| {
            parent.spawn(SpriteComponents {
                material: materials.add(Color::rgba(1.0, 0.5, 0.0, 0.2).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(26.0, 26.0)),
                ..Default::default()
            }).with(SelectionVisual);
        })
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Selectable {is_selected: false})
        .with(Mover{target_position:Position{x:100.0, y:0.0}, speed: 150f32})
        .with_children(|parent| {
            parent.spawn(SpriteComponents {
                material: materials.add(Color::rgba(1.0, 0.5, 0.0, 0.2).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(26.0, 26.0)),
                ..Default::default()
            }).with(SelectionVisual);
        })
        ;
}

pub fn mover_update(time: Res<Time>, mut query: Query<(&Mover, &mut Transform)>) {
    for (mover, mut transform) in &mut query.iter() {
        let position = transform.translation();
        let target = math::vec2(mover.target_position.x, mover.target_position.y);
        let mut offset: Vec2 = target - math::vec2( position[0], position[1]);
        if offset.length() < 0.01 {
            continue;
        }
        offset = offset.normalize();
        offset *= mover.speed * time.delta_seconds_f64 as f32;
        // TODO: cap offset distance to match target position
        transform.set_translation(position + math::vec3(offset[0], offset[1], position[2]));    
    }
}