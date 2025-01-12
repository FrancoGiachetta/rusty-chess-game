use bevy::{app::Plugin, prelude::{Commands, Component, Entity}};

pub enum TileState {
    Ocuppied(Entity),
    Empty
}

#[derive(Component)]
pub struct Tile {
    pub state: TileState
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}

impl BoardPlugin {
    fn create(cmds: &mut Commands) {}
}
