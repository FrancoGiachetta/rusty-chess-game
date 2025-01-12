use bevy::{app::App, prelude::DefaultPickingPlugins, DefaultPlugins};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod board;
mod piece;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TilemapPlugin)
        .add_plugins(DefaultPickingPlugins)
        .run();
}
