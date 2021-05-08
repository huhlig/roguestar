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
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};
use noise::{OpenSimplex, Seedable, Perlin, NoiseFn};

pub struct CloudTextureGenerator {
    noise1: OpenSimplex,
    noise2: OpenSimplex,
    noise3: OpenSimplex,
    noise4: OpenSimplex,
    noise5: OpenSimplex,
}

impl CloudTextureGenerator {
    pub fn new(seed: u32) -> CloudTextureGenerator {
        CloudTextureGenerator {
            noise1: OpenSimplex::new().set_seed(seed ^ 679125833),
            noise2: OpenSimplex::new().set_seed(seed ^ 3274989671),
            noise3: OpenSimplex::new().set_seed(seed ^ 2776948319),
            noise4: OpenSimplex::new().set_seed(seed ^ 4054792873),
            noise5: OpenSimplex::new().set_seed(seed ^ 2100345001),
        }
    }
    pub fn texture(&mut self, width: usize, height: usize, frame: usize, base: bool) -> Texture {
        let mut data = Vec::with_capacity(width * height * 4);
        for x in 0..width {
            for y in 0..height {
                let nx = x as f64 / (width as f64 * 0.1);
                let ny = y as f64 / (height as f64 * 0.1);
                let nz = frame as f64;
                let e
                    = 16.0 * (self.noise1.get([01.0 * nx, 01.0 * ny, 01.0 * nz]) + 1.0) / 2.0
                    + 08.0 * (self.noise2.get([02.0 * nx, 02.0 * ny, 02.0 * nz]) + 1.0) / 2.0
                    + 04.0 * (self.noise3.get([04.0 * nx, 04.0 * ny, 04.0 * nz]) + 1.0) / 2.0
                    + 02.0 * (self.noise4.get([08.0 * nx, 08.0 * ny, 08.0 * nz]) + 1.0) / 2.0
                    + 01.0 * (self.noise5.get([16.0 * nx, 16.0 * ny, 16.0 * nz]) + 1.0) / 2.0;
                let val = ((e / 31.0) * 256.0) as u8;
                //trace!("{},{} => {} -> {}", x, y, e, val);
                data.push(val);
                data.push(0);
                data.push(0);
                if base {
                    data.push(255);
                } else {
                    data.push(val);
                }
            }
        }
        Texture::new(
            Extent3d::new(width as u32, height as u32, 1),
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
        )
    }
}

/*
pub fn hyperspace_texture(width: usize, height: usize, layer: u8, frame: u8) -> Texture {
    use noise::{Add, NoiseFn, Perlin, OpenSimplex, ScaleBias, ScalePoint};
    let noise = OpenSimplex::new();

    let noise1 = ScaleBias::new(&ScalePoint::new(Perlin::new())
        .set_all_scales(01.0, 01.0, 01.0, 01.0)).set_scale(16.0);
    let noise2 = ScaleBias::new(&ScalePoint::new(Perlin::new())
        .set_all_scales(02.0, 02.0, 02.0, 02.0)).set_scale(08.0);
    let noise3 = ScaleBias::new(&ScalePoint::new(Perlin::new())
        .set_all_scales(04.0, 04.0, 04.0, 04.0)).set_scale(04.0);
    let noise4 = ScaleBias::new(&ScalePoint::new(Perlin::new())
        .set_all_scales(08.0, 08.0, 08.0, 08.0)).set_scale(02.0);
    let noise4 = ScaleBias::new(&ScalePoint::new(Perlin::new())
        .set_all_scales(16.0, 16.0, 16.0, 16.0)).set_scale(01.0);

    let e
        = 16.0 * noise1(x, y)
        + 08.0 * noise2(2 * x, 2 * y)
        + 04.0 * noise3(4 * x, 4 * y)
        + 02.0 * noise4(8 * x, 8 * y)
        + 01.0 * noise5(16 * x, 16 * y);


    // Compile a list of animated textures

    let y_scale = 0.05;
    let x_scale = y_scale * (height as f64 / width as f64);
    let mut data = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let nx = x as f64;
            let ny = y as f64;
            let e// Noise Equation 
                = 1.00 * noise.get([1.0 * nx, 1.0 * ny])
                + 0.50 * noise.get([2.0 * nx, 2.0 * ny])
                + 0.25 * noise.get([4.0 * nx, 4.0 * ny]);
            let val = ((e / (1.0 + 0.5 + 0.25)) * 128.0 + 128.0) as u8;
            //trace!("({},{}) - {}", nx, ny, val);
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
        TextureFormat::Rgba8UnormSrgb,
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
*/