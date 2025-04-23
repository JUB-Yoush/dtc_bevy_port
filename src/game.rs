use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_rapier2d::plugin::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::enemy;
use crate::player;
use crate::resolution;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_plugins(AsepriteUltraPlugin)
            .add_plugins((
                resolution::ResolutionPlugin,
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                RapierDebugRenderPlugin::default(),
                enemy::EnemyPlugin,
                player::PlayerPlugin,
            ));
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
