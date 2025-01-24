use bevy::{
    app::{Plugin, PostStartup, Startup},
    color::Color,
    core::Name,
    prelude::{Commands, Component, Entity},
};
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapSize, TilemapTileSize, TilemapType},
    prelude::get_tilemap_center_transform,
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};

use crate::piece::{
    bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rock::Rock, Piece,
};

pub const TILE_SIZE: f32 = 64.0;

pub enum TileState {
    Ocuppied(Entity),
    Empty,
}

#[derive(Component)]
pub struct Tile {
    pub state: TileState,
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, Self::create);
        app.add_systems(PostStartup, Self::spawn_pieces);
    }
}

impl BoardPlugin {
    fn create(mut cmds: Commands) {
        let tilemap_entity = cmds.spawn_empty().id();
        let tile_size = TilemapTileSize {
            x: TILE_SIZE,
            y: TILE_SIZE,
        };
        let map_size = TilemapSize { x: 8, y: 8 };
        let grid_size = tile_size.into();
        let map_type = TilemapType::default();
        let mut tilemap_storage = TileStorage::empty(map_size);

        for x in 0..7 {
            for y in 0..7 {
                let tile_pos = TilePos { x, y };
                let color = if (y + x) % 2 == 0 {
                    Color::srgb(255.0, 255.0, 255.0)
                } else {
                    Color::srgb(0.0, 0.0, 0.0)
                };

                let tile_entity = cmds
                    .spawn(TileBundle {
                        position: tile_pos,
                        color: color.into(),
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    })
                    .insert(Tile {
                        state: TileState::Empty,
                    })
                    .insert(Name::new(format!("Tile: {x}, {y}")))
                    .id();

                tilemap_storage.set(&tile_pos, tile_entity);
            }
        }

        cmds.entity(tilemap_entity).insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            tile_size,
            storage: tilemap_storage,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        });
    }

    fn spawn_pieces(mut cmds: Commands) {
        // spawn black rocks
        Rock::spawn(&mut cmds, TilePos { x: 0, y: 0 });
        Rock::spawn(&mut cmds, TilePos { x: 7, y: 0 });

        // spawn black knights
        Knight::spawn(&mut cmds, TilePos { x: 1, y: 0 });
        Knight::spawn(&mut cmds, TilePos { x: 6, y: 0 });

        // spawn black bishops

        Bishop::spawn(&mut cmds, TilePos { x: 2, y: 0 });
        Bishop::spawn(&mut cmds, TilePos { x: 5, y: 0 });

        // spawn black queen
        Queen::spawn(&mut cmds, TilePos { x: 3, y: 0 });

        // spawn black king
        King::spawn(&mut cmds, TilePos { x: 4, y: 0 });

        // spawn black pawns
        for x in 0..8 {
            Pawn::spawn(&mut cmds, TilePos { x, y: 1 });
        }

        // WHITES

        // spawn white rocks
        Rock::spawn(&mut cmds, TilePos { x: 0, y: 7 });
        Rock::spawn(&mut cmds, TilePos { x: 7, y: 7 });

        // spawn white knights
        Knight::spawn(&mut cmds, TilePos { x: 1, y: 7 });
        Knight::spawn(&mut cmds, TilePos { x: 6, y: 7 });

        // spawn white bishops

        Bishop::spawn(&mut cmds, TilePos { x: 2, y: 7 });
        Bishop::spawn(&mut cmds, TilePos { x: 5, y: 7 });

        // spawn white queen
        Queen::spawn(&mut cmds, TilePos { x: 3, y: 7 });

        // spawn white king
        King::spawn(&mut cmds, TilePos { x: 4, y: 7 });

        // spawn white pawns
        for x in 0..8 {
            Pawn::spawn(&mut cmds, TilePos { x, y: 6 });
        }
    }
}
