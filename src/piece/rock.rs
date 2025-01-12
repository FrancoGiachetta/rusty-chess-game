use super::Piece;

pub struct Rock;

impl Piece for Rock {
    fn spawn(cmds: &mut bevy::prelude::Commands) {}
    fn show_moves(&self, cmds: &mut bevy::prelude::Commands) {}
    fn kill(&self, cmds: &mut bevy::prelude::Commands) {}
    fn die(&self, cmds: &mut bevy::prelude::Commands) {}
}
