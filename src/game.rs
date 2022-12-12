// Don't touch this piece, needed for Web
#[cfg(target_arch = "wasm32")]
mod web_main;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};

pub fn run(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Game".to_string(),
            ..default()
        },
        ..default()
    }))
    .add_plugin(AudioPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    audio: Res<Audio>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: assets.load("character.png"),
        sprite: Sprite {
            color: Color::WHITE,
            ..default()
        },
        ..default()
    });

    audio.play(assets.load("powerup.wav"))
        .with_volume(0.7);
}