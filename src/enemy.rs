use crate::resolution;

use bevy::time::common_conditions::*;
use bevy::utils::Duration;
use bevy::{input::keyboard::Key, math::VectorSpace, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use rand::{rng, Rng};
//
pub struct EnemyPlugin;

//#[require(Transform(|| Transform::from_xyz(0.,0.,0.)))]
#[derive(Component)]
struct Enemy {
    velocity: Vec2,
}

enum SpawnArea {
    X(XSpawn),
    Y(YSpawn),
}

enum XSpawn {
    Left,
    Right,
}

enum YSpawn {
    Top,
    Bottom,
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
const X_SPAWNS: [XSpawn; 2] = [XSpawn::Left, XSpawn::Right];
const Y_SPAWNS: [YSpawn; 2] = [YSpawn::Top, YSpawn::Bottom];

fn spawn_enemy(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    let mut rng = rng();
    if let Some(enemy_type) = ENEMY_TYPES.choose(&mut rng) {
        let x_spawn = X_SPAWNS.choose(&mut rng);
        let y_spawn = Y_SPAWNS.choose(&mut rng);
        cmd.spawn((
            AseSpriteAnimation {
                aseprite: asset_server.load(*enemy_type),
                animation: Animation::tag("move"),
            },
            Enemy {
                velocity: Vec2::new(0., 0.), // do the enum pattern matching wrapped whatever the
                                             // heck dude
            },
            Transform::from_xyz(
                rng.random_range(
                    -resolution.screen_dimensions.x / 2. ..resolution.screen_dimensions.x / 2.,
                ),
                rng.random_range(
                    -resolution.screen_dimensions.y / 2. ..resolution.screen_dimensions.y / 2.,
                ),
                0.,
            ),
            //   Transform::from_xyz(0., 0., 0.),
            Sprite {
                ..Default::default()
            },
        ));
    }
}
