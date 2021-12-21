//
//  Roguestar - An experimental Roguelike Adventure across the stars.
//  Copyright (C) 2021 Hans W. Uhlig
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use bevy_tilemap::tilemap::TilemapResult;

pub fn generate_city(texture_atlas: Handle<TextureAtlas>) -> TilemapResult<Tilemap> {
    // Build a default Tilemap with 32x32 pixel tiles.
    let mut tilemap = TilemapBuilder::new()
        .chunk_dimensions(256, 256, 4)
        .texture_atlas(texture_atlas)
        .auto_chunk()
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            0,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            1,
        )
        .finish()?;

    TilemapResult::Ok(tilemap)
}

pub fn generate_apartment(texture_atlas: Handle<TextureAtlas>) -> TilemapResult<Tilemap> {
    // Build a default Tilemap with 32x32 pixel tiles.
    let mut tilemap = TilemapBuilder::new()
        .chunk_dimensions(128, 128, 4)
        .texture_atlas(texture_atlas)
        .auto_chunk()
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            0,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            1,
        )
        .finish()?;

    // Insert a single tile
    tilemap
        .insert_tile(Tile {
            // 2D location x,y (units are in tiles)
            point: (16, 16),

            // Which tile from the TextureAtlas
            sprite_index: 0,

            // Which z-layer in the Tilemap (0-up)
            sprite_order: 0,

            // Give the tile an optional green tint
            tint: Default::default(),
        })
        .unwrap();

    TilemapResult::Ok(tilemap)
}
