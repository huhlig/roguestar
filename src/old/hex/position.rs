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

use super::HexLocation;

/// Fractional Hex Coordinate indicating a Position on a Hex Map
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct HexPosition(f32, f32);

impl HexPosition {
    #[inline]
    pub fn from_axial(q: f32, r: f32) -> HexPosition {
        Self(q, r)
    }
    #[inline]
    pub fn from_cubic(x: f32, y: f32, z: f32) -> HexPosition {
        assert!(x + y + z < f32::EPSILON);
        Self(x, z)
    }
    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }
    #[inline]
    pub fn y(&self) -> f32 {
        -self.0 - self.1
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.1
    }
    #[inline]
    pub fn q(&self) -> f32 {
        self.0
    }
    #[inline]
    pub fn r(&self) -> f32 {
        self.1
    }
    /// Distance from this position and 0.
    #[inline]
    pub fn length(&self) -> f32 {
        (self.x().abs() + self.y().abs() + self.z().abs()) / 2.0
    }
    /// Distance between this Position and another.
    #[inline]
    pub fn distance(&self, rhs: &HexPosition) -> f32 {
        let dx = (self.x() - rhs.x()).abs();
        let dy = (self.y() - rhs.y()).abs();
        let dz = (self.z() - rhs.z()).abs();
        (dx + dy + dz) / 2.0
    }
}

impl std::ops::Add<HexPosition> for HexPosition {
    type Output = HexPosition;

    #[inline]
    fn add(self, rhs: HexPosition) -> Self::Output {
        HexPosition(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign<HexPosition> for HexPosition {
    #[inline]
    fn add_assign(&mut self, rhs: HexPosition) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Sub<HexPosition> for HexPosition {
    type Output = HexPosition;

    #[inline]
    fn sub(self, rhs: HexPosition) -> Self::Output {
        HexPosition(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::SubAssign<HexPosition> for HexPosition {
    #[inline]
    fn sub_assign(&mut self, rhs: HexPosition) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl std::ops::Mul<f32> for HexPosition {
    type Output = HexPosition;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        HexPosition(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::MulAssign<f32> for HexPosition {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl std::ops::Mul<i32> for HexPosition {
    type Output = HexPosition;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        HexPosition(self.0 * rhs as f32, self.1 * rhs as f32)
    }
}

impl std::ops::MulAssign<i32> for HexPosition {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.0 *= rhs as f32;
        self.1 *= rhs as f32;
    }
}

impl From<HexLocation> for HexPosition {
    #[inline]
    fn from(loc: HexLocation) -> Self {
        HexPosition::from(&loc)
    }
}

impl From<&HexLocation> for HexPosition {
    #[inline]
    fn from(loc: &HexLocation) -> Self {
        HexPosition(loc.q() as f32, loc.r() as f32)
    }
}
