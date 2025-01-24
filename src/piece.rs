use bevy::{
    app::Plugin,
    prelude::{Commands, Component},
};
use bevy_ecs_tilemap::tiles::TilePos;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rock;

pub trait Piece {
    fn spawn(cmds: &mut Commands, tile_pos: TilePos);
    fn show_moves(&self, cmds: &mut Commands);
    fn kill(&self, cmds: &mut Commands);
    fn die(&self, cmds: &mut Commands);
}

pub enum PieceTeam {
    Black,
    White,
}

#[derive(Component)]
pub struct PieceInfo {
    pos: TilePos,
    team: PieceTeam,
}

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}
