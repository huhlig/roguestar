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

/// Hyperspace Position is the fractional position on a Hyperspace Map.
/// | type     | cubic | axial |
/// |----------|-------|-------|
/// |      [0] |     x |     q |
/// | -[0]-[1] |     y |       |
/// |      [1] |     z |     r |
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct HyperspacePosition(f64, f64);

impl HyperspacePosition {
    /// Create HyperspaceLocation from Axial Coordinates
    pub fn from_axial(q: f64, r: f64) -> HyperspacePosition {
        HyperspacePosition(q, r)
    }
    /// Create HyperspaceLocation from Cubic Coordinates
    pub fn from_cubic(x: f64, y: f64, z: f64) -> HyperspacePosition {
        assert!(x + y + z <= 0.001);
        HyperspacePosition(x, z)
    }
    /// Cubic X Coordinate
    pub fn x(&self) -> f64 {
        self.0
    }
    /// Cubic Y Coordinate
    pub fn y(&self) -> f64 {
        -self.0 - self.1
    }
    /// Cubic Z Coordinate
    pub fn z(&self) -> f64 {
        self.1
    }
    /// Axial Q Coordinate
    pub fn q(&self) -> f64 {
        self.0
    }
    /// Axial R Coordinate
    pub fn r(&self) -> f64 {
        self.1
    }
    /// Distance of a HyperspacePosition from Origin
    pub fn len(&self) -> f64 {
        let ax = self.0.abs();
        let ay = (-self.0 - self.1).abs();
        let az = self.1.abs();
        (ax + ay + az) / 2.0
    }
    /// Distance between two Hyperspace Locations
    pub fn distance(&self, rhs: &HyperspacePosition) -> f64 {
        let ax = (self.0 - rhs.0).abs();
        let ay = (-self.0 - self.1 - rhs.0 - rhs.1).abs();
        let az = (self.1 - self.1).abs();
        (ax + ay + az) / 2.0
    }
    pub fn round(&self) -> (i64, i64) {
        let x = self.0.round() as i64;
        let z = self.1.round() as i64;
        let y = (self.0 - self.1).round() as i64;
        let x_diff = (x as f64 - self.0).abs();
        let z_diff = (z as f64 - self.1).abs();
        let y_diff = (y as f64 - (-self.0 - self.1)).abs();
        if x_diff > z_diff && x_diff > y_diff {
            (-z - y, z)
        } else if z_diff > y_diff {
            (x, -x - y)
        } else {
            (x, y)
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

auto_ops::impl_op_ex!(+ |a: &HyperspacePosition, b: &HyperspacePosition| -> HyperspacePosition {
    HyperspacePosition(a.0 + b.0,  a.1 + b.1)
});
auto_ops::impl_op_ex!(+= |a: &mut HyperspacePosition, b: &HyperspacePosition| {
    a.0 += b.0;
    a.1 += b.1;
});
auto_ops::impl_op_ex!(
    -|a: &HyperspacePosition, b: &HyperspacePosition| -> HyperspacePosition {
        HyperspacePosition(a.0 - b.0, a.1 - b.1)
    }
);
auto_ops::impl_op_ex!(-= |a: &mut HyperspacePosition, b: &HyperspacePosition| {
    a.0 -= b.0;
    a.1 -= b.1;
});
auto_ops::impl_op_ex!(*|a: &HyperspacePosition, b: &f64| -> HyperspacePosition {
    HyperspacePosition(a.0 * b, a.1 * b)
});
auto_ops::impl_op_ex!(*= |a: &mut HyperspacePosition, b: &f64| {
    a.0 *= b;
    a.1 *= b;
});

/*
impl From<HyperspaceLocation> for HyperspacePosition {
    fn from(rhs: HyperspaceLocation) -> Self {
        HyperspacePosition::from(rhs)
    }
}

impl From<&HyperspaceLocation> for HyperspacePosition {
    fn from(rhs: &HyperspaceLocation) -> Self {
        HyperspacePosition(rhs.0 as f64, rhs.1 as f64)
    }
}
*/
