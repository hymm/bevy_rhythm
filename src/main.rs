use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;
mod consts;
use consts::*;
mod types;
mod ui;
use ui::UIPlugin;
mod score;
use score::ScoreResource;
mod audio;
use audio::AudioPlugin;
mod shaders;
use shaders::ShadersPlugin;
mod menu;
use menu::MenuPlugin;
mod time;
use time::TimePlugin;
mod map_maker;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_resource(State::new(AppState::Menu))
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(TimePlugin)
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}
