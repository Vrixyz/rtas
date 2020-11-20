use std::collections::HashMap;

use bevy::prelude::*;
use bevy_prototype_lyon::{TessellationMode, prelude::{FillOptions, ShapeType, StrokeOptions, primitive}};

use super::{super::core_game::components::*, selection::selection_comp::SelectionRectVisual};

use super::components::*;

pub fn create_render_resource(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_goblin = asset_server.load("units/goblin.png");

    let texture_ogre = asset_server.load("units/ogre.png");
    let mut render_sprite_visuals = HashMap::new();
    render_sprite_visuals.insert(
        RenderSprite::Goblin,
        RenderSpriteVisual {
            color: materials.add(Color::rgb(0.1, 0.9, 0.3).into()),
            material: materials.add(texture_goblin.into()),
        },
    );
    render_sprite_visuals.insert(
        RenderSprite::Ogre,
        RenderSpriteVisual {
            color: materials.add(Color::rgb(1.0, 0.5, 0.0).into()),
            material: materials.add(texture_ogre.into()),
        },
    );

    let color_selection = materials.add(Color::rgba(1.0, 1.0, 1.0, 0.2).into());
    let team_colors = vec![
        materials.add(Color::rgba(0.0, 0.0, 1.0, 0.8).into()),
        materials.add(Color::rgba(1.0, 0.0, 0.0, 0.8).into()),
    ];

    let render_sprites_resource = RenderResource {
        render_sprite_visuals,
        color_selection,
        team_colors,
    };
    commands.insert_resource(render_sprites_resource);
}

pub fn create_camera(mut commands: Commands) {
    let camera = Camera2dComponents::default();
    let e = commands.spawn(camera).current_entity().unwrap();
    commands.insert_resource(MyCursorState {
        cursor: Default::default(),
        camera_e: e,
        world_position: Position { x: 0f32, y: 0f32 },
        ui_position: Vec2::default(),
    });
}
pub fn create_ui(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut selection_rect_visual: Option<Entity> = None;
    commands.insert_resource(Selection::Hover(None));
    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        // root node
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        position: Rect {
                            left: Val::Px(600.0),
                            bottom: Val::Px(180.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba(0.15, 0.65, 0.15, 0.5).into()),
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .for_current_entity(|e| {
                    selection_rect_visual = Some(e);
                });
        });
    if let Some(visual) = selection_rect_visual {
        commands.insert_resource(SelectionRectVisual { visual });
    }
}

pub fn adapt_units_for_client(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    render: Res<RenderResource>,
    mut query: Query<(Entity, &Team, &RenderSprite, &UnitSize, &Transform)>,
) {
    for (entity, team, render_sprite, size, transform) in query.iter_mut() {
        commands
        .spawn(primitive(
            render.team_colors[team.id].clone(),
            &mut meshes,
            ShapeType::Circle(size.0),
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(3.0)),
            Vec3::default(),
        ))
        .with(Parent(entity));
        commands.spawn(primitive(
            render.team_colors[team.id].clone(),
            &mut meshes,
            ShapeType::Triangle(
                (size.0, size.0 * 0.5).into(),
                (size.0 + size.0 * 0.5, 0.0).into(),
                (size.0, -size.0 * 0.5).into()),
                TessellationMode::Stroke(&StrokeOptions::default().with_line_width(5.0)),
            Vec3::new(0.0, 0.0, 0.5),
        ))
        .with(Parent(entity));

        commands.spawn(SpriteComponents {
                material: render.render_sprite_visuals[render_sprite].material.clone(),
                sprite: Sprite::new(Vec2::new(size.0 * 2.0, size.0 * 2.0)),
                transform: Default::default(),
                ..Default::default()
            },
        ).with(Parent(entity)).with(NoRotation);
        commands.insert_one(
            entity,
            Selectable {
                is_selected: false,
                half_size: size.0,
            },
        );

        commands
            .spawn(primitive(
                render.color_selection.clone(),
                &mut meshes,
                ShapeType::Circle(size.0 + 2.0),
                TessellationMode::Stroke(&StrokeOptions::default().with_line_width(2.0)),
                Vec3::default(),
            ))
            .with(SelectionVisual)
            .with(Parent(entity));
    }
}

pub fn no_rotation(
    mut q: Query<(&mut GlobalTransform, &NoRotation)>,
) {
    for (mut gt,  _) in q.iter_mut() {
        gt.rotation = Default::default();
    }
}

/// Adapted from https://github.com/jamadazi/bevy-cookbook/blob/master/bevy-cookbook.md#convert-screen-coordinates-to-world-coordinates
pub fn mouse_world_position_system(
    mut state: ResMut<MyCursorState>,
    ev_cursor: Res<Events<CursorMoved>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera components
    q_camera: Query<&Transform>,
) {
    let camera_transform = q_camera.get_component::<Transform>(state.camera_e).unwrap();

    if let Some(ev) = state.cursor.latest(&ev_cursor) {
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = ev.position - size / 2.0;
        
        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let position = Position {
            x: pos_wld.x(),
            y: pos_wld.y(),
        };
        state.world_position = position;
        state.ui_position = ev.position;
    }
}

// Mod health

pub struct HealthVisualResource {
    max_health: Handle<ColorMaterial>,
    current_health: Handle<ColorMaterial>,
}
pub struct HealthVisual {
    pub max_hp_visual: Entity,
    pub current_hp_visual: Entity,
}

fn create_health_visual(
    health_visual_resource: &mut Res<HealthVisualResource>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    health: &Health,
    size: &UnitSize,
) -> (SpriteComponents, SpriteComponents) {
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

    let line_max = primitive(
        max_health_material.clone(),
        &mut meshes,
        ShapeType::Polyline {
            points: vec![first_point.into(), max_point],
            closed: false,
        },
        TessellationMode::Stroke(&StrokeOptions::default().with_line_width(HEIGHT)),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let line_current = primitive(
        current_health_material.clone(),
        &mut meshes,
        ShapeType::Polyline {
            points: vec![first_point.into(), current_point],
            closed: false,
        },
        TessellationMode::Stroke(&StrokeOptions::default().with_line_width(HEIGHT)),
        Vec3::new(0.0, 0.0, 1.5),
    );
    (line_max, line_current)
}

pub fn health_visual_startup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(HealthVisualResource {
        max_health: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        current_health: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
    });
}

pub fn health_visual_setup_system(
    mut commands: Commands,
    mut health_visual_resource: Res<HealthVisualResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_health: Query<Without<HealthVisual, (Entity, &Health, &UnitSize)>>,
) {
    for (entity, health, size) in q_health.iter_mut() {
        let sprites = create_health_visual(&mut health_visual_resource, &mut meshes, health, size);

        let max_hp_entity = commands
            .spawn(sprites.0)
            .with(Parent(entity))
            .with(NoRotation)
            .current_entity()
            .unwrap();
        let current_hp_entity = commands
            .spawn(sprites.1)
            .with(Parent(entity))
            .with(NoRotation)
            .current_entity()
            .unwrap();
        commands.insert_one(
            entity,
            HealthVisual {
                max_hp_visual: max_hp_entity,
                current_hp_visual: current_hp_entity,
            },
        );
    }
}

pub fn health_visual_system(
    mut commands: Commands,
    mut health_visual_resource: Res<HealthVisualResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_health: Query<(&Health, &HealthVisual, &UnitSize)>,
) {
    for (health, visual, size) in q_health.iter_mut() {
        let sprites = create_health_visual(&mut health_visual_resource, &mut meshes, health, size);

        commands.insert(visual.max_hp_visual, sprites.0);
        commands.insert(visual.current_hp_visual, sprites.1);
    }
}

pub mod ability {
    use bevy::prelude::*;
    use bevy_prototype_lyon::prelude::*;

    use crate::{client::components::NoRotation, core_game::components::*};

    pub struct AbilityVisualResource {
        background: Handle<ColorMaterial>,
        current: Handle<ColorMaterial>,
    }
    pub struct AbilityVisual {
        pub background: Entity,
        pub current: Entity,
    }

    fn create_ability_visual(
        health_visual_resource: &mut Res<AbilityVisualResource>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        size: &UnitSize,
        ratio: f32,
    ) -> (SpriteComponents, SpriteComponents) {
        let background_material = &health_visual_resource.background;
        let current_material = &health_visual_resource.current;
        const WIDTH: f32 = 20f32;
        const HEIGHT: f32 = 2f32;
        let offset = size.0 - 2.5 - HEIGHT;

        let first_point = (-WIDTH / 2f32, offset);
        let max_point = (WIDTH / 2f32, offset).into();
        let current_point = (first_point.0 + (WIDTH * ratio), offset).into();

        let line_max = primitive(
            background_material.clone(),
            &mut meshes,
            ShapeType::Polyline {
                points: vec![first_point.into(), max_point],
                closed: false,
            },
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(HEIGHT)),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let line_current = primitive(
            current_material.clone(),
            &mut meshes,
            ShapeType::Polyline {
                points: vec![first_point.into(), current_point],
                closed: false,
            },
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(HEIGHT)),
            Vec3::new(0.0, 0.0, 1.5),
        );
        (line_max, line_current)
    }

    pub fn ability_visual_startup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.insert_resource(AbilityVisualResource {
            background: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            current: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        });
    }
    pub fn ability_visual_setup(
        mut commands: Commands,
        mut ability_visual_resource: Res<AbilityVisualResource>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut q_orders: Query<Without<AbilityVisual, (Entity, &MeleeAbility, &MeleeAbilityState, &UnitSize)>>,
    ) {
        for (entity, _, _, size) in q_orders.iter_mut() {
            let sprites = create_ability_visual(&mut ability_visual_resource, &mut meshes, size, 0f32);

            let max_hp_entity = commands
                .spawn(sprites.0)
                .with(Parent(entity))
                .with(NoRotation)
                .current_entity()
                .unwrap();
            let current_hp_entity = commands
                .spawn(sprites.1)
                .with(Parent(entity))
                .with(NoRotation)
                .current_entity()
                .unwrap();
            commands.insert_one(
                entity,
                AbilityVisual {
                    background: max_hp_entity,
                    current: current_hp_entity,
                },
            );
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
                    let ratio = (time.seconds_since_startup as f32 - will_attack.start_time)
                        / ability.time_to_strike;
                    Some(create_ability_visual(
                        &mut ability_visual_resource,
                        &mut meshes,
                        size,
                        ratio,
                    ))
                }
                MeleeAbilityState::AttackCooldown(cooldown) => {
                    let ratio = (time.seconds_since_startup as f32 - cooldown.start_time)
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
                commands.insert(visual.background, sprites.0);
                commands.insert(visual.current, sprites.1);
            }
        }
    }
}