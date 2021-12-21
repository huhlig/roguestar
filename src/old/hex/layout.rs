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

use super::HexPosition;
use bevy::math::{Mat2, Vec2};

/// Layout of Hex Grid
pub enum HexLayout {
    Pointy {
        /// Origin of Hex Grid
        origin: Vec2,
        /// Size of each Hex
        size: Vec2,
    },
    Flat {
        /// Origin of Hex Grid
        origin: Vec2,
        /// Size of each Hex
        size: Vec2,
    },
}

impl HexLayout {
    pub fn flat(origin: Vec2, size: Vec2) -> HexLayout {
        HexLayout::Flat { origin, size }
    }
    pub fn pointy(origin: Vec2, size: Vec2) -> HexLayout {
        HexLayout::Pointy { origin, size }
    }
    pub fn to_cartesian(&self, hex: HexPosition) -> Vec2 {
        // TODO: Convert to const when https://github.com/rust-lang/rust/issues/57241 is complete.
        #![allow(non_snake_case)]
        /* const */ let  FLAT_FORWARD: Mat2 =
            Mat2::from_cols_array_2d(&[[f32::sqrt(3.0) / 3.0, 0.0], [-1.0 / 3.0, 2.0 / 3.0]]);
        match *self {
            HexLayout::Pointy { origin, size } => {
                let x = size[0] * (f32::sqrt(3.0) * hex.q() + f32::sqrt(3.0) / 2.0 * hex.r());
                let y = size[1] * (3.0 / 2.0 * hex.r());
                Vec2::new(origin[0] + x, origin[1] + y)
            }
            HexLayout::Flat { origin, size } => {
                let x = size[0] * (3.0 / 2.0 * hex.q());
                let y = size[1] * (f32::sqrt(3.0) / 2.0 * hex.q() + f32::sqrt(3.0) * hex.r());
                Vec2::new(origin[0] + x, origin[1] + y)
            }
        }
    }
    pub fn from_cartesian(&self, point: Vec2) -> HexPosition {
        match *self {
            HexLayout::Pointy { origin, size } => {
                let q = (f32::sqrt(3.0) / 3.0 * point.x - 1.0 / 3.0 * point.x) / size.x;
                let r = (2.0 / 3.0 * point.y) / size.y;
                HexPosition::from_axial(origin.x + q, origin.y + r)
            }
            HexLayout::Flat { origin, size } => {
                let q = (2.0 / 3.0 * point.x) / size.x;
                let r = (-1.0 / 3.0 * point.x + f32::sqrt(3.0) / 3.0 * point.y) / size.y;
                HexPosition::from_axial(origin.x + q, origin.y + r)
            }
        }
    }
}

impl Default for HexLayout {
    fn default() -> Self {
        HexLayout::Pointy {
            origin: Vec2::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
        }
    }
}
