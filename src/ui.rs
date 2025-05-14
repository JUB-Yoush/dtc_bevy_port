use bevy::prelude::*;
use bevy::time::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use std::time::Duration;

use crate::player::Hit;

#[derive(Resource, Default)]
struct Score(u32);

pub struct UiPlugin;

pub struct ScoreUpdate;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_systems(OnEnter(LoadState::Done), setup_ui);
        app.add_systems(
            Update,
            (
                update_ui.run_if(common_conditions::on_timer(Duration::from_millis(500))),
                on_player_hit,
            ),
        );
        app.load("ui.cob");
    }
}

fn setup_ui(mut cmd: Commands, mut s: SceneBuilder, mut score: ResMut<Score>) {
    score.0 = 0;
    cmd.ui_root()
        .spawn_scene(("ui.cob", "main_scene"), &mut s, |scene_handle| {
            scene_handle.update_on(
                broadcast::<ScoreUpdate>(),
                move |id: TargetId, mut editor: TextEditor, score: Res<Score>| {
                    write_text!(editor, *id, "{}", score.0);
                },
            );
            scene_handle.update_text(score.0.to_string());
        });
}

fn update_ui(mut cmd: Commands, mut score: ResMut<Score>) {
    score.0 += 1;
    cmd.react().broadcast(ScoreUpdate);
}
fn on_player_hit(mut cmd: Commands, mut events: EventReader<Hit>, mut score: ResMut<Score>) {
    for event in events.read() {
        score.0 = 0;
        cmd.react().broadcast(ScoreUpdate);
    }
}
