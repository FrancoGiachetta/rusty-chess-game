use super::Piece;

pub struct Bishop;

impl Piece for Bishop {
    fn spawn(cmds: &mut bevy::prelude::Commands) {}
    fn show_moves(&self, cmds: &mut bevy::prelude::Commands) {}
    fn kill(&self, cmds: &mut bevy::prelude::Commands) {}
    fn die(&self, cmds: &mut bevy::prelude::Commands) {}
}
