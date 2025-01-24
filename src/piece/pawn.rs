use bevy_ecs_tilemap::tiles::TilePos;

use super::Piece;

pub struct Pawn;

impl Piece for Pawn {
    fn spawn(mut cmds: &mut bevy::prelude::Commands, tile_pos: TilePos) {}
    fn show_moves(&self, cmds: &mut bevy::prelude::Commands) {}
    fn kill(&self, cmds: &mut bevy::prelude::Commands) {}
    fn die(&self, cmds: &mut bevy::prelude::Commands) {}
}
