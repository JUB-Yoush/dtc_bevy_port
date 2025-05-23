use std::time::Duration;

use crate::resolution::Position as ScreenPosition;
use crate::resolution::Resolution;
use avian2d::prelude::*;
use bevy::ecs::entity;

use bevy::time::common_conditions::*;
use bevy::time::*;
use bevy::{input::keyboard::Key, math::VectorSpace, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use rand::distr::{Distribution, StandardUniform};
use rand::seq::IndexedRandom;
use rand::{random_range, rng, Rng};

pub struct EnemyPlugin;

#[derive(Component)]
#[require(ScreenPosition)]
pub struct Enemy {
    direction: Vec2,
    speed: f32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemy.run_if(common_conditions::on_timer(Duration::from_millis(500))),
                update_enemy,
                out_of_bounds.after(update_enemy),
            ),
        )
        .add_event::<OutOfBounds>();
    }
}

const MIN_SPEED: f32 = 100.0;
const MAX_SPEED: f32 = 400.0;

#[derive(Event)]
struct OutOfBounds(Entity);

#[derive(Debug)]
enum SpawnEdge {
    Top,
    Bottom,
    Left,
    Right,
}
impl Distribution<SpawnEdge> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SpawnEdge {
        match rng.random_range(0..4) {
            0 => SpawnEdge::Top,
            1 => SpawnEdge::Bottom,
            2 => SpawnEdge::Left,
            _ => SpawnEdge::Right,
        }
    }
}

const ENEMY_TYPES: [&str; 3] = [
    "enemy_walk.aseprite",
    "enemy_swim.aseprite",
    "enemy_fly.aseprite",
];

fn spawn_enemy(mut cmd: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    let mut rng = rng();
    if let Some(enemy_type) = ENEMY_TYPES.choose(&mut rng) {
        let spawnedge: SpawnEdge = rand::random();
        cmd.spawn((
            CollisionLayers::new(0b10, 0b01),
            CollisionEventsEnabled,
            CollidingEntities::default(),
            Collider::circle(15.0),
            AseAnimation {
                aseprite: asset_server.load(*enemy_type),
                animation: Animation::tag("move"),
            },
            Enemy {
                direction: {
                    match spawnedge {
                        SpawnEdge::Top => {
                            Vec2::new(random_range(-1.0..1.0), random_range(0.0..1.0))
                        }
                        SpawnEdge::Bottom => {
                            Vec2::new(random_range(-1.0..1.0), random_range(-1.0..0.0))
                        }
                        SpawnEdge::Left => {
                            Vec2::new(random_range(0.0..1.0), random_range(-1.0..1.0))
                        }
                        SpawnEdge::Right => {
                            Vec2::new(random_range(-1.0..0.0), random_range(-1.0..1.0))
                        }
                    }
                    .normalize()
                },
                speed: rng.random_range(MIN_SPEED..MAX_SPEED),
            },
            ScreenPosition(match spawnedge {
                SpawnEdge::Top => Vec2::new(random_range(0.0..resolution.screen_dimensions.x), 0.0),
                SpawnEdge::Bottom => Vec2::new(
                    random_range(0.0..resolution.screen_dimensions.x),
                    resolution.screen_dimensions.y,
                ),
                SpawnEdge::Left => {
                    Vec2::new(0.0, random_range(0.0..resolution.screen_dimensions.y))
                }
                SpawnEdge::Right => Vec2::new(
                    resolution.screen_dimensions.x,
                    random_range(0.0..resolution.screen_dimensions.y),
                ),
            }),
            Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(resolution.pixel_ratio)),
            //   Transform::from_xyz(0., 0., 0.),
            Sprite {
                ..Default::default()
            },
        ));
    }
}

fn update_enemy(
    mut query: Query<(Entity, &Enemy, &mut ScreenPosition)>,
    time: Res<Time>,
    resolution: Res<Resolution>,
    mut events: EventWriter<OutOfBounds>,
) {
    for (entity, enemy, mut position) in query.iter_mut() {
        position.0.x += enemy.direction.x * enemy.speed * time.delta_secs();
        position.0.y += enemy.direction.y * enemy.speed * time.delta_secs();

        if position.0.x < 0.0 || position.0.x > resolution.screen_dimensions.x {
            events.write(OutOfBounds(entity));
        }
    }
}

fn out_of_bounds(mut cmd: Commands, mut events: EventReader<OutOfBounds>) {
    for event in events.read() {
        cmd.entity(event.0).despawn();
    }
}
