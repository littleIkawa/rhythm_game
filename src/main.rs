mod components;
mod game_constants;
mod resources;
mod systems;

use bevy::prelude::*;
use resources::{game_scene::NextAppState, note::Speed};
use systems::{
    audio::GameAudioPlugin, load::LoadPlugin, note::NotePlugin, target_note::TargetNotePlugin,
    ui::GameUiPlugin,
};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    HomeMenu,
    Loading,
    Game,
}

pub fn global_setup(mut commands: Commands) {
    // カメラのセット
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    let window = WindowDescriptor {
        title: "rhythm".to_string(),
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        resizable: true,
        ..Default::default()
    };

    let mut app = App::new();

    app.insert_resource(window);
    // Set antialiasing to use 4 samples
    app.insert_resource(Msaa { samples: 4 });
    app.add_system(bevy::window::close_on_esc);
    app.add_plugins(DefaultPlugins);
    // ステート初期化
    // 次に向かいたいステートをセットしてからローディングステートで開始する.
    app.insert_resource(NextAppState(AppState::Game));
    app.insert_resource(Speed(1.0));
    app.add_state(AppState::Loading);

    app.add_startup_system(global_setup);
    app.add_plugin(LoadPlugin);
    app.add_plugin(NotePlugin);
    app.add_plugin(TargetNotePlugin);
    app.add_plugin(GameUiPlugin);
    app.add_plugin(GameAudioPlugin);
    // app.add_plugin(ShadersPlugin);
    app.run();
}