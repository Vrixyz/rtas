use std::collections::HashMap;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, GeometryBuilder, PathBuilder, StrokeMode};
use bevy_prototype_lyon::shapes;

use crate::core_game::map::{Wall, WallSize};

use super::{super::core_game::components::*, selection::selection_comp::SelectionRectVisual};

use super::components::*;

pub fn create_render_resource(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_goblin = asset_server.load("units/goblin.png");

    let mut render_sprite_visuals = HashMap::new();
    render_sprite_visuals.insert(
        RenderSprite::Goblin,
        RenderSpriteVisual {
            color: Color::rgb(0.1, 0.9, 0.3),
            image: texture_goblin,
        },
    );
    let texture_ogre = asset_server.load("units/ogre.png");
    render_sprite_visuals.insert(
        RenderSprite::Ogre,
        RenderSpriteVisual {
            color: Color::rgb(1.0, 0.5, 0.0),
            image: texture_ogre,
        },
    );
    let texture_bandit = asset_server.load("units/bandit.png");
    render_sprite_visuals.insert(
        RenderSprite::Bandit,
        RenderSpriteVisual {
            color: Color::rgb(0.6, 0.6, 0.6),
            image: texture_bandit,
        },
    );

    let color_selection = Color::rgba(1.0, 1.0, 1.0, 1.0);
    let team_colors = vec![
        Color::rgba(0.0, 0.0, 1.0, 0.8),
        Color::rgba(0.6, 0.6, 0.6, 0.8),
        Color::rgba(1.0, 0.0, 0.0, 0.8),
    ];
    let color_walls = Color::rgba(1.0, 1.0, 1.0, 1.0);

    let render_sprites_resource = RenderResource {
        render_sprite_visuals,
        color_selection,
        team_colors,
        color_walls,
    };
    commands.insert_resource(render_sprites_resource);
}

pub fn create_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    let e = commands.spawn().insert_bundle(camera).id();
    commands.insert_resource(MainCamera { camera_e: e });
    commands.insert_resource(MyCursorState {
        camera_e: e,
        world_position: Position { x: 0f32, y: 0f32 },
        ui_position: Vec2::default(),
    });
}
pub fn create_ui(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut selection_rect_visual: Option<Entity> = None;
    commands.insert_resource(Selection::Hover(None));
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            let visual_entity = parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        position: UiRect {
                            left: Val::Px(600.0),
                            bottom: Val::Px(180.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: Color::rgba(0.15, 0.65, 0.15, 0.5).into(),
                    ..Default::default()
                })
                .id();
            selection_rect_visual = Some(visual_entity);
        });
    if let Some(visual) = selection_rect_visual {
        commands.insert_resource(SelectionRectVisual { visual });
    }
}

pub fn adapt_map_for_client(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    render: Res<RenderResource>,
    query: Query<(Entity, &Transform, &Wall, &WallSize)>,
) {
    let rectShape = shapes::Rectangle {
        extents: Vec2::new(2.0, 2.0),
        ..Default::default()
    };
    for (entity, transform, _, size) in query.iter() {
        commands.spawn().insert_bundle(GeometryBuilder::build_as(
            &rectShape,
            DrawMode::Fill(FillMode::color(render.color_walls.clone())),
            transform
                .with_translation(transform.translation + Vec3::new(0.0, 0.0, 0.05))
                .with_scale(Vec3::new(size.x, size.y, 1.0)),
        ));
        // FIXME: I should parent the drawing to its collider.
        // .insert(Parent(entity));
    }
}

pub fn adapt_units_for_client(
    mut commands: Commands,
    render: Res<RenderResource>,
    query: Query<(Entity, &Team, &RenderSprite, &UnitSize)>,
) {
    let circleShape = shapes::Circle {
        radius: 1.0,
        ..Default::default()
    };
    let triangleShape = shapes::Polygon {
        points: vec![(1.0, 0.5).into(), (1.5, 0.0).into(), (1.0, -0.5).into()],
        closed: true,
    };

    for (entity, team, render_sprite, size) in query.iter() {
        commands
            .entity(entity)
            .insert(Selectable {
                is_selected: false,
                half_size: size.0,
            })
            .insert(Visibility::visible())
            .insert(ComputedVisibility::default());
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn()
                .insert_bundle(GeometryBuilder::build_as(
                    &circleShape,
                    DrawMode::Stroke(StrokeMode::new(render.team_colors[team.id], 3.0 / 20.0)),
                    Transform::from_translation(Vec2::ZERO.extend(0.1))
                        .with_scale(Vec2::splat(size.0).extend(1.0)),
                ))
                .insert(NoRotation);
            parent.spawn().insert_bundle(GeometryBuilder::build_as(
                &triangleShape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::NONE),
                    outline_mode: StrokeMode::new(render.team_colors[team.id], 5.0 / 20.0),
                },
                Transform::from_translation(Vec2::ZERO.extend(0.1))
                    .with_scale(Vec2::splat(size.0).extend(1.0)),
            ));

            parent
                .spawn()
                .insert_bundle(SpriteBundle {
                    texture: render.render_sprite_visuals[render_sprite].image.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(size.0 * 2.0)),
                        color: render.render_sprite_visuals[render_sprite].color,
                        ..default()
                    },
                    ..default()
                })
                .insert(NoRotation);

            parent
                .spawn()
                .insert_bundle(GeometryBuilder::build_as(
                    &circleShape,
                    DrawMode::Stroke(StrokeMode::new(render.color_selection, 2.0 / 20.0)),
                    Transform::from_translation(Vec2::ZERO.extend(1.0))
                        .with_scale(Vec2::splat(size.0 + 2.0).extend(1.0)),
                ))
                .insert(SelectionVisual)
                .insert(NoRotation);
        });
    }
}

pub fn no_rotation(
    mut q_no_rotation: Query<(Entity, &Parent, &mut Transform), With<NoRotation>>,
    mut q_parent: Query<&mut Transform, Without<NoRotation>>,
) {
    for (e, parent, mut transform) in q_no_rotation.iter_mut() {
        let parent_transform = q_parent.get(parent.get()).unwrap();
        transform.rotation = parent_transform.rotation.inverse();
    }
}

/// Adapted from https://github.com/jamadazi/bevy-cookbook/blob/master/bevy-cookbook.md#convert-screen-coordinates-to-world-coordinates
pub fn mouse_world_position_system(
    mut state: ResMut<MyCursorState>,
    mut ev_cursor: EventReader<CursorMoved>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera components
    q_camera: Query<&Transform>,
) {
    let camera_transform = q_camera.get_component::<Transform>(state.camera_e).unwrap();

    if let Some(ev) = ev_cursor.iter().last() {
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = ev.position - size / 2.0;

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let position = Position {
            x: pos_wld.x,
            y: pos_wld.y,
        };
        state.world_position = position;
        state.ui_position = ev.position;
    }
}

// Mod health

pub struct HealthVisualResource {
    max_health: Color,
    current_health: Color,
}
#[derive(Component)]
pub struct HealthVisual {
    pub max_hp_visual: Entity,
    pub current_hp_visual: Entity,
}

fn create_health_visual(
    health_visual_resource: &mut Res<HealthVisualResource>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    health: &Health,
    size: &UnitSize,
) -> (ShapeBundle, ShapeBundle) {
    let max_health_material = &health_visual_resource.max_health;
    let current_health_material = &health_visual_resource.current_health;
    const WIDTH: f32 = 20f32;
    const HEIGHT: f32 = 5f32;

    let offset = size.0;

    let first_point = (-WIDTH / 2f32, offset);
    let max_point = (WIDTH / 2f32, offset).into();
    let current_point = (
        first_point.0 + (WIDTH * health.current_hp / health.max_hp),
        offset,
    )
        .into();

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(first_point.into());
    path_builder.line_to(max_point);
    let line = path_builder.build();

    let line_max = GeometryBuilder::build_as(
        &line,
        DrawMode::Stroke(StrokeMode::new(*max_health_material, HEIGHT)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    );
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(first_point.into());
    path_builder.line_to(current_point);
    let line = path_builder.build();
    let line_current = GeometryBuilder::build_as(
        &line,
        DrawMode::Stroke(StrokeMode::new(*current_health_material, HEIGHT)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.5)),
    );
    (line_max, line_current)
}

pub fn health_visual_startup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(HealthVisualResource {
        max_health: Color::rgb(1.0, 0.0, 0.0),
        current_health: Color::rgb(0.0, 1.0, 0.0),
    });
}

pub fn health_visual_setup_system(
    mut commands: Commands,
    mut health_visual_resource: Res<HealthVisualResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_health: Query<(Entity, &Health, &UnitSize), Without<HealthVisual>>,
) {
    for (entity, health, size) in q_health.iter_mut() {
        let sprites = create_health_visual(&mut health_visual_resource, &mut meshes, health, size);

        let mut max_hp_entity = None;
        let mut current_hp_entity = None;

        commands.entity(entity).with_children(|parent| {
            max_hp_entity = Some(
                parent
                    .spawn()
                    .insert_bundle(sprites.0)
                    .insert(NoRotation)
                    .id(),
            );
            current_hp_entity = Some(
                parent
                    .spawn()
                    .insert_bundle(sprites.1)
                    .insert(NoRotation)
                    .id(),
            );
        });

        commands.entity(entity).insert(HealthVisual {
            max_hp_visual: max_hp_entity.unwrap(),
            current_hp_visual: current_hp_entity.unwrap(),
        });
    }
}

pub fn health_visual_system(
    mut commands: Commands,
    mut health_visual_resource: Res<HealthVisualResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_health: Query<(&Health, &HealthVisual, &UnitSize), Changed<Health>>,
) {
    for (health, visual, size) in q_health.iter_mut() {
        let sprites = create_health_visual(&mut health_visual_resource, &mut meshes, health, size);

        commands
            .entity(visual.max_hp_visual)
            .insert_bundle(sprites.0);
        commands
            .entity(visual.current_hp_visual)
            .insert_bundle(sprites.1);
    }
}

pub mod ability {
    use bevy::prelude::*;
    use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

    use crate::{client::components::NoRotation, core_game::components::*};

    pub struct AbilityVisualResource {
        background: Color,
        current: Color,
    }
    #[derive(Component)]
    pub struct AbilityVisual {
        pub background: Entity,
        pub current: Entity,
    }

    fn create_ability_visual(
        health_visual_resource: &mut Res<AbilityVisualResource>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        size: &UnitSize,
        ratio: f32,
    ) -> (ShapeBundle, ShapeBundle) {
        let background_material = &health_visual_resource.background;
        let current_material = &health_visual_resource.current;
        const WIDTH: f32 = 20f32;
        const HEIGHT: f32 = 2f32;
        let offset = size.0 - 2.5 - HEIGHT;

        let first_point = (-WIDTH / 2f32, offset);
        let max_point = (WIDTH / 2f32, offset).into();
        let current_point = (first_point.0 + (WIDTH * ratio), offset).into();

        let mut path_builder = PathBuilder::new();
        path_builder.move_to(first_point.into());
        path_builder.line_to(max_point);
        let line = path_builder.build();

        let line_max = GeometryBuilder::build_as(
            &line,
            DrawMode::Stroke(StrokeMode::new(*background_material, HEIGHT)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        );
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(first_point.into());
        path_builder.line_to(current_point);
        let line = path_builder.build();
        let line_current = GeometryBuilder::build_as(
            &line,
            DrawMode::Stroke(StrokeMode::new(*current_material, HEIGHT)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 1.5)),
        );
        (line_max, line_current)
    }

    pub fn ability_visual_startup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.insert_resource(AbilityVisualResource {
            background: Color::rgb(0.25, 0.25, 0.25),
            current: Color::rgb(1.0, 1.0, 1.0),
        });
    }
    pub fn ability_visual_setup(
        mut commands: Commands,
        mut ability_visual_resource: Res<AbilityVisualResource>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut q_orders: Query<
            (Entity, &MeleeAbility, &MeleeAbilityState, &UnitSize),
            Without<AbilityVisual>,
        >,
    ) {
        for (entity, _, _, size) in q_orders.iter_mut() {
            let sprites =
                create_ability_visual(&mut ability_visual_resource, &mut meshes, size, 0f32);

            let mut max_hp_entity = None;
            let mut current_hp_entity = None;
            commands.entity(entity).with_children(|parent| {
                max_hp_entity = Some(
                    parent
                        .spawn()
                        .insert_bundle(sprites.0)
                        .insert(NoRotation)
                        .id(),
                );
                current_hp_entity = Some(
                    parent
                        .spawn()
                        .insert_bundle(sprites.1)
                        .insert(NoRotation)
                        .id(),
                );
            });

            commands.entity(entity).insert(AbilityVisual {
                background: max_hp_entity.unwrap(),
                current: current_hp_entity.unwrap(),
            });
        }
    }

    pub fn ability_visual(
        mut commands: Commands,
        time: Res<Time>,
        mut ability_visual_resource: Res<AbilityVisualResource>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut q_orders: Query<(&MeleeAbility, &MeleeAbilityState, &AbilityVisual, &UnitSize)>,
    ) {
        for (ability, state, visual, size) in q_orders.iter_mut() {
            let sprites = match state {
                MeleeAbilityState::Ready => Some(create_ability_visual(
                    &mut ability_visual_resource,
                    &mut meshes,
                    size,
                    0f32,
                )),
                MeleeAbilityState::WillAttack(will_attack) => {
                    let ratio = (time.seconds_since_startup() as f32 - will_attack.start_time)
                        / ability.time_to_strike;
                    Some(create_ability_visual(
                        &mut ability_visual_resource,
                        &mut meshes,
                        size,
                        ratio,
                    ))
                }
                MeleeAbilityState::AttackCooldown(cooldown) => {
                    let ratio = (time.seconds_since_startup() as f32 - cooldown.start_time)
                        / ability.cooldown;
                    Some(create_ability_visual(
                        &mut ability_visual_resource,
                        &mut meshes,
                        size,
                        1f32 - ratio,
                    ))
                }
                MeleeAbilityState::MotionBufferExceeded => {
                    let ratio = 0.0;
                    Some(create_ability_visual(
                        &mut ability_visual_resource,
                        &mut meshes,
                        size,
                        ratio,
                    ))
                }
            };
            if let Some(sprites) = sprites {
                commands.entity(visual.background).insert_bundle(sprites.0);
                commands.entity(visual.background).insert_bundle(sprites.1);
            }
        }
    }
}
