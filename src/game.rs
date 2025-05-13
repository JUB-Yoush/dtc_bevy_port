use avian2d::prelude::PhysicsDebugPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_cobweb_ui::prelude::*;

use crate::enemy;
use crate::player;
use crate::resolution;
use crate::ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_plugins(AsepriteUltraPlugin)
            .add_plugins((
                PhysicsPlugins::default(),
                PhysicsDebugPlugin::default(),
                resolution::ResolutionPlugin,
                enemy::EnemyPlugin,
                player::PlayerPlugin,
                ui::UiPlugin,
                CobwebUiPlugin,
            ));
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
