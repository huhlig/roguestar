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

use bevy::prelude::Texture;
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};

pub fn hyperspace_texture(width: usize, height: usize, layer: u8, frame: u8) -> Texture {
    use noise::{NoiseFn, Perlin};
    let perlin = Perlin::new();

    // Compile a list of animated textures

    let mut data = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let val = (perlin.get([x as f64, y as f64, frame as f64]) * 255.0) as u8;
            // Push In R, G, B, A Channels
            data.push(val);
            data.push(0);
            data.push(0);
            data.push(255);
        }
    }
    Texture::new(
        Extent3d::new(width as u32, height as u32, 1),
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Uint,
    )
}

pub fn hypospace_texture(width: usize, height: usize, frames: u8) -> Vec<Texture> {
    use noise::{NoiseFn, Perlin};
    let perlin = Perlin::new();

    // Compile a list of animated textures
    let mut textures = Vec::with_capacity(frames as usize);
    for t in 0..frames {
        let mut data = Vec::new();
        for x in 0..width {
            for y in 0..height {
                let val =
                    (perlin.get([x as f64, y as f64, t as f64 / frames as f64]) * 255.0) as u8;
                // Push In R, G, B, A Channels
                data.push(0);
                data.push(val);
                data.push(val);
                data.push(val);
            }
        }
        textures.push(Texture::new(
            Extent3d::new(width as u32, height as u32, 1),
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8Uint,
        ));
    }
    textures
}
