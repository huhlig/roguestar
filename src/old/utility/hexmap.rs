//
// Copyright 2021 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use bevy::prelude::*;

/// Create Hex Coordinates & Hex Centers in a spiral pattern
pub fn hex_spiral(i: i32) -> (IVec2, Vec2) {
    let scalar: f32 = 0.75f32.sqrt();
    use std::f32::consts::PI;
    if i != 0 {
        let layer = (i as f32 / 3.0).sqrt().round() as i32;
        let first_idx_in_layer = 3 * layer * (layer - 1) + 1;
        let side = (i - first_idx_in_layer) / layer;
        let idx = (i - first_idx_in_layer) % layer;
        let x = layer as f32 * ((side - 1) as f32 * PI / 3.0).cos()
            + (idx + 1) as f32 * ((side + 1) as f32 * PI / 3.0).cos();
        let y = -(layer as f32) * ((side - 1) as f32 * PI / 3.0).sin()
            - (idx + 1) as f32 * ((side + 1) as f32 * PI / 3.0).sin();
        // Convert from Pixel Coordinates to Rounded Axial Coordinates for pointy hexes
        let q = (3.0f32.sqrt() / 3.0) * x - (1.0 / 3.0 * y) / 1.0;
        let r = (2.0 / 3.0 * y) / 1.0;
        let s = -q - r;
        // Round Axial Coordinates
        let mut rx = q.round();
        let mut ry = s.round();
        let mut rz = r.round();
        let x_diff = (rx - q).abs();
        let y_diff = (ry - s).abs();
        let z_diff = (rz - r).abs();
        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff > z_diff {
            ry = -rx - rz;
        } else {
            rz = -rx - ry;
        }
        // Return both
        (
            IVec2::new(rx as i32, rz as i32),
            Vec2::new(x * f32::sqrt(0.75), y * f32::sqrt(0.75)),
        )
    } else {
        (IVec2::new(0, 0), Vec2::new(0.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::hexmap::hex_spiral;

    #[test]
    fn test_hexspiral() {
        (0..100)
            .map(hex_spiral)
            .enumerate()
            .for_each(|(idx, (axial, center))| {
                println!(
                    "{}, {}, {}, {}, {}",
                    idx, axial.x, axial.y, center.x, center.y
                )
            });
    }
}
