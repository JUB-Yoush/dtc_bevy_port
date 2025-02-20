use bevy::prelude::*;
use std::fmt;

#[derive(Component, Default)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component)]
#[require(Sprite, Position, Velocity)]
struct Player;

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn spawn_player_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(
        asset_server.load("../assets/art/playerGrey_walk1.png"),
    ));
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = player.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y += 1.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            velocity.0.x = 1.;
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            velocity.0.x = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn move_player(mut query: Query<(&Player, &mut Position, &mut Velocity)>) {
    //if let Ok(mut velocity,mut position,mut player) = query.get_single_mut() {
    for (player, mut position, mut velocity) in &mut query {
        position.0 += velocity.0;
    }
}

fn move_sprite(mut query: Query<(&Player, &mut Position, &mut Velocity, &mut Sprite)>) {
    //if let Ok(mut velocity,mut position,mut player) = query.get_single_mut() {
    for (player, mut position, mut velocity, mut Sprite) in &mut query {
        position.0 += velocity.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, spawn_player_sprite)
        .run();
}
