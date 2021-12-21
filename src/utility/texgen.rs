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
use noise::{NoiseFn, OpenSimplex, Perlin, Seedable};

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
    pub fn texture(&mut self, width: usize, height: usize, frame: usize) -> Texture {
        let mut data = Vec::with_capacity(width * height * 4);
        for x in 0..width {
            for y in 0..height {
                let nx = x as f64 / (width as f64 * 0.1);
                let ny = y as f64 / (width as f64 * 0.1);
                let nz = frame as f64 * 0.1;
                let e = 16.0 * (self.noise1.get([01.0 * nx, 01.0 * ny, 01.0 * nz]) + 1.0)
                    + 08.0 * (self.noise2.get([02.0 * nx, 02.0 * ny, 02.0 * nz]) + 1.0)
                    + 04.0 * (self.noise3.get([04.0 * nx, 04.0 * ny, 04.0 * nz]) + 1.0)
                    + 02.0 * (self.noise4.get([08.0 * nx, 08.0 * ny, 08.0 * nz]) + 1.0)
                    + 01.0 * (self.noise5.get([16.0 * nx, 16.0 * ny, 16.0 * nz]) + 1.0);
                let val = ((e / 64.0) * 255.0) as u8;
                //trace!("{},{} => {} -> {}", x, y, e, val);
                data.push(val);
                data.push(0);
                data.push(0);
                data.push(val);
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

#[cfg(test)]
mod tests {
    use super::CloudTextureGenerator;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    #[test]
    fn test_texture() {
        const WIDTH: usize = 1024;
        const HEIGHT: usize = 768;
        let path = Path::new(r"target/perlin_test.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let texture = CloudTextureGenerator::new(0).texture(WIDTH, HEIGHT, 0);
        writer.write_image_data(&texture.data).unwrap();
    }
}
