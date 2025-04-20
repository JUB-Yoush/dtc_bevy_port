use bevy::prelude::*;

pub mod enemy;
pub mod game;
pub mod player;
pub mod resolution;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Dodge the Creeps"),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: Vec2::new(480., 720.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            game::GamePlugin,
        ))
        .run();
}
