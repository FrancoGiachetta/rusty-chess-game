use bevy::{
    app::{App, Startup},
    prelude::{Camera2d, Commands, DefaultPickingPlugins, Transform},
    DefaultPlugins,
};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board::TILE_SIZE;

mod board;
mod piece;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TilemapPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(TILE_SIZE * 4.0, TILE_SIZE * 4.0, 999.9),
    ));
}
