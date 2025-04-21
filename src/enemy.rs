use crate::resolution::*;
use bevy::time::common_conditions::*;
use bevy::utils::Duration;
use bevy::{input::keyboard::Key, math::VectorSpace, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use rand::distr::{Distribution, StandardUniform};
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use rand::{random_range, rng, Rng};

//
pub struct EnemyPlugin;

const MIN_SPEED: f32 = 100.0;
const MAX_SPEED: f32 = 300.0;

//#[require(Transform(|| Transform::from_xyz(0.,0.,0.)))]
#[derive(Component)]
#[require(Position)]
struct Enemy {
    velocity: Vec2,
    speed: f32,
}

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

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy.run_if(on_timer(Duration::from_secs(1))));
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
        let lmao: SpawnEdge = rand::random();
        cmd.spawn((
            AseSpriteAnimation {
                aseprite: asset_server.load(*enemy_type),
                animation: Animation::tag("move"),
            },
            Enemy {
                velocity: Vec2::new(0., 0.), // do the enum pattern matching wrapped whatever the
                speed: rng.random_range(MIN_SPEED..MAX_SPEED),
            },
            Position(Vec2::new(
                random_range(0. ..resolution.screen_dimensions.x),
                random_range(0. ..resolution.screen_dimensions.y),
            )),
            Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(resolution.pixel_ratio)),
            //   Transform::from_xyz(0., 0., 0.),
            Sprite {
                ..Default::default()
            },
        ));
    }
}
