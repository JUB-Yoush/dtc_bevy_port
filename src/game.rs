use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;

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
                player::PlayerPlugin,
                enemy::EnemyPlugin,
            ));
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
