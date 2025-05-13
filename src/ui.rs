use bevy::prelude::*;
use bevy::time::*;
use bevy_cobweb_ui::prelude::*;
use std::time::Duration;

#[derive(Resource, Default)]
struct Score(u32);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_systems(OnEnter(LoadState::Done), setup_ui);
        app.add_systems(
            Update,
            update_ui.run_if(common_conditions::on_timer(Duration::from_millis(500))),
        );
        app.load("ui.cob");
    }
}

fn setup_ui(mut cmd: Commands, mut s: SceneBuilder, mut score: ResMut<Score>) {
    score.0 = 10;
    cmd.ui_root()
        .spawn_scene(("ui.cob", "main_scene"), &mut s, |scene_handle| {
            scene_handle.update_text(score.0.to_string());
        });
}

fn update_ui(mut score: ResMut<Score>) {
    score.0 += 1;
    println!("{}", score.0);
}
