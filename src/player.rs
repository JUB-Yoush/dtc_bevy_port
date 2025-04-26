use crate::resolution::*;
use bevy::{input::keyboard::Key, math::VectorSpace, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::*;

const PLAYER_SPEED: f32 = 500.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (update_player, animate_player));
    }
}

#[derive(Debug, PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

pub enum PlayerState {
    Idle,
    Moving,
}

#[derive(Component)]
#[require(Position, Collider(|| Collider::cuboid(4.0,4.0)))]
struct Player {
    direction: PlayerDirection,
    state: PlayerState,
}

fn setup_player(mut cmd: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    cmd.spawn((
        Player {
            direction: PlayerDirection::Right,
            state: PlayerState::Idle,
        },
        AseSpriteAnimation {
            aseprite: asset_server.load("player.aseprite"),
            animation: Animation::tag("idle"),
        },
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(resolution.pixel_ratio)),
        Sprite {
            ..Default::default()
        },
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn update_player(
    mut query: Query<(&mut Player, &mut Position)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<Resolution>,
) {
    for (mut player, mut position) in query.iter_mut() {
        let mut pressed = false;
        if keys.pressed(KeyCode::KeyW) {
            position.0.y -= PLAYER_SPEED * time.delta_secs();
            player.direction = PlayerDirection::Up;
            pressed = true;
        }
        if keys.pressed(KeyCode::KeyA) {
            position.0.x -= PLAYER_SPEED * time.delta_secs();
            player.direction = PlayerDirection::Left;
            pressed = true;
        }
        if keys.pressed(KeyCode::KeyS) {
            position.0.y += PLAYER_SPEED * time.delta_secs();
            player.direction = PlayerDirection::Down;
            pressed = true;
        }
        if keys.pressed(KeyCode::KeyD) {
            position.0.x += PLAYER_SPEED * time.delta_secs();
            player.direction = PlayerDirection::Right;
            pressed = true;
        }
        position.0.x = position.0.x.clamp(0., resolution.screen_dimensions.x);
        position.0.y = position.0.y.clamp(0., resolution.screen_dimensions.y);

        if pressed {
            player.state = PlayerState::Moving;
        } else {
            player.state = PlayerState::Idle;
        }
    }
}

fn animate_player(
    mut query: Query<(&mut Player, &mut AseSpriteAnimation, &mut Sprite)>,
    time: Res<Time>,
    resolution: Res<AssetServer>,
) {
    for (player, mut aseprite, mut sprite) in query.iter_mut() {
        match player.state {
            PlayerState::Idle => aseprite.animation.play_loop("idle"),
            PlayerState::Moving => match player.direction {
                PlayerDirection::Up => {
                    aseprite.animation.play_loop("up");
                    sprite.flip_y = false;
                }
                PlayerDirection::Down => {
                    aseprite.animation.play_loop("up");
                    sprite.flip_y = true;
                }
                PlayerDirection::Right => {
                    aseprite.animation.play_loop("side");
                    sprite.flip_x = false;
                }
                PlayerDirection::Left => {
                    aseprite.animation.play_loop("side");
                    sprite.flip_x = true;
                }
            },
        }
    }
}
