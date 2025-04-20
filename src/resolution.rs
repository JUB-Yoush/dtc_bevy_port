use bevy::prelude::*;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution {
    pub screen_dimensions: Vec2,
    pub pixel_ratio: f32,
}
#[derive(Component)]
pub struct Position(Vec2);

fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();
    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::new(window.width(), window.height()),
        pixel_ratio: 2.0,
    });
}

fn project_positions(
    mut cmd: Commands,
    mut query: Query<(&Position, &mut Transform)>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    for (position, mut transform) in &mut query {
        transform.translation = position.0.extend(0.);
    }
}
