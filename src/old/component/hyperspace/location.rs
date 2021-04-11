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

//! Positional Components

use super::HyperspacePosition;
use bevy::ecs::Entity;

/// Hyperspace Location is the Sector Location on a Hyperspace Map.
/// | type     | cubic | axial |
/// |----------|-------|-------|
/// |      [0] |     x |     q |
/// | -[0]-[1] |     y |       |
/// |      [1] |     z |     r |
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HyperspaceLocation(i64, i64);

impl HyperspaceLocation {
    /// Create HyperspaceLocation from Axial Coordinates
    pub fn from_axial(q: i64, r: i64) -> HyperspaceLocation {
        HyperspaceLocation(q, r)
    }
    /// Create HyperspaceLocation from Cubic Coordinates
    pub fn from_cubic(x: i64, y: i64, z: i64) -> HyperspaceLocation {
        assert_eq!(x + y + z, 0);
        HyperspaceLocation(x, z)
    }
    /// Cubic X Coordinate
    pub fn x(&self) -> i64 {
        self.0
    }
    /// Cubic Y Coordinate
    pub fn y(&self) -> i64 {
        -self.0 - self.1
    }
    /// Cubic Z Coordinate
    pub fn z(&self) -> i64 {
        self.1
    }
    /// Axial Q Coordinate
    pub fn q(&self) -> i64 {
        self.0
    }
    /// Axial R Coordinate
    pub fn r(&self) -> i64 {
        self.1
    }
    /// Distance of a HyperspaceLocation from Origin
    pub fn len(&self) -> i64 {
        let ax = self.0.abs();
        let ay = (-self.0 - self.1).abs();
        let az = self.1.abs();
        (ax + ay + az) / 2
    }
    /// Distance between two Hyperspace Locations
    pub fn distance(&self, rhs: &HyperspaceLocation) -> i64 {
        let ax = (self.0 - rhs.0).abs();
        let ay = (-self.0 - self.1 - rhs.0 - rhs.1).abs();
        let az = (self.1 - self.1).abs();
        (ax + ay + az) / 2
    }
    /// Neighboring Hyperspace Sector
    pub fn direction(direction: i32) -> HyperspaceLocation {
        match (6 + (direction % 6)) % 6 {
            0 => HyperspaceLocation::from_cubic(1, 0, -1),
            1 => HyperspaceLocation::from_cubic(1, -1, 0),
            2 => HyperspaceLocation::from_cubic(0, -1, 1),
            3 => HyperspaceLocation::from_cubic(-1, 0, 1),
            4 => HyperspaceLocation::from_cubic(-1, 1, 0),
            5 => HyperspaceLocation::from_cubic(0, 1, -1),
            _ => panic!("This should be impossible"),
        }
    }
    pub fn neighbor(&self, direction: i32) -> HyperspaceLocation {
        match (6 + (direction % 6)) % 6 {
            0 => HyperspaceLocation::from_cubic(self.x() + 1, self.y() + 0, self.z() - 1),
            1 => HyperspaceLocation::from_cubic(self.x() + 1, self.y() - 1, self.z() + 0),
            2 => HyperspaceLocation::from_cubic(self.x() + 0, self.y() - 1, self.z() + 1),
            3 => HyperspaceLocation::from_cubic(self.x() - 1, self.y() + 0, self.z() + 1),
            4 => HyperspaceLocation::from_cubic(self.x() - 1, self.y() + 1, self.z() + 0),
            5 => HyperspaceLocation::from_cubic(self.x() + 0, self.y() + 1, self.z() - 1),
            _ => panic!("This should be impossible"),
        }
    }
    /*
    pub fn into_point(&self, layout: &HexLayout) -> HexPoint {
        HexPoint {
            x: (3.0 / 2.0 * self.q as f32 + 0.0 * self.r as f32) * layout.size.x + layout.origin.x,
            y: (f32::sqrt(3.0) / 2.0 * self.q as f32 + f32::sqrt(3.0) * self.r as f32)
                * layout.size.y
                + layout.origin.y,
        }
    }
    */
}

auto_ops::impl_op_ex!(+ |a: &HyperspaceLocation, b: &HyperspaceLocation| -> HyperspaceLocation {
    HyperspaceLocation(a.0 + b.0,  a.1 + b.1)
});
auto_ops::impl_op_ex!(+= |a: &mut HyperspaceLocation, b: &HyperspaceLocation| {
    a.0 += b.0;
    a.1 += b.1;
});
auto_ops::impl_op_ex!(
    -|a: &HyperspaceLocation, b: &HyperspaceLocation| -> HyperspaceLocation {
        HyperspaceLocation(a.0 - b.0, a.1 - b.1)
    }
);
auto_ops::impl_op_ex!(-= |a: &mut HyperspaceLocation, b: &HyperspaceLocation| {
    a.0 -= b.0;
    a.1 -= b.1;
});
auto_ops::impl_op_ex!(*|a: &HyperspaceLocation, b: &i64| -> HyperspaceLocation {
    HyperspaceLocation(a.0 * b, a.1 * b)
});
auto_ops::impl_op_ex!(*= |a: &mut HyperspaceLocation, b: &i64| {
    a.0 *= b;
    a.1 *= b;
});

impl From<HyperspacePosition> for HyperspaceLocation {
    fn from(rhs: HyperspacePosition) -> Self {
        HyperspaceLocation::from(&rhs)
    }
}

impl From<&HyperspacePosition> for HyperspaceLocation {
    fn from(rhs: &HyperspacePosition) -> Self {
        let x = rhs.x().round() as i64;
        let z = rhs.z().round() as i64;
        let y = rhs.y().round() as i64;
        let x_diff = (x as f64 - rhs.x()).abs();
        let z_diff = (z as f64 - rhs.z()).abs();
        let y_diff = (y as f64 - rhs.y()).abs();
        if x_diff > z_diff && x_diff > y_diff {
            HyperspaceLocation(-z - y, z)
        } else if z_diff > y_diff {
            HyperspaceLocation(x, -x - y)
        } else {
            HyperspaceLocation(x, y)
        }
    }
}
