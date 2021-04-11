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

use crate::common::WorldCamera;
use crate::mode::hyperspace::{Hyperspace, SectorInfo};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::texture::{Extent3d, ImageType, TextureDimension, TextureFormat};
use bevy::text::Text2dSize;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;

pub fn generate_galaxy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let seed = 50;
    let mut rng = rand_xoshiro::Xoshiro128PlusPlus::seed_from_u64(seed);
    let texture_handle = asset_server.load("decals/hyperwell2.png");
    let material_handle = materials.add(texture_handle.into());

    (0..102)
        .map(crate::utility::hexmap::hex_spiral)
        .for_each(|(sector, center)| {
            if rng.gen_range(0.0, 100.0) <= 100.0 {
                let name = nominae::Totro::generate(2, 9, &mut rng);
                let distance = (sector.x).abs() + (sector.y).abs();
                let info = SectorInfo {
                    location: sector,
                    identifier: format!("{} {}", distance, name),
                };
                trace!("({},{}) @ {}", center.x, center.y, &info.identifier);
                commands
                    .spawn_bundle(SpriteBundle {
                        material: material_handle.clone(),
                        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                        transform: Transform::from_xyz(center.x, center.y, 0.0),
                        ..Default::default()
                    })
                    .insert(info)
                    .insert(Hyperspace);
            }
        });
}
