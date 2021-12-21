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

/// Integer Hex Coordinate indicating a Location on a Hex Map
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct HexLocation(i32, i32);

impl HexLocation {
    ///
    #[inline]
    pub fn from_axial(q: i32, r: i32) -> HexLocation {
        Self(q, r)
    }
    #[inline]
    pub fn from_cubic(x: i32, y: i32, z: i32) -> HexLocation {
        assert_eq!(0, x + y + z);
        Self(x, z)
    }
    #[inline]
    pub fn x(&self) -> i32 {
        self.0
    }
    #[inline]
    pub fn y(&self) -> i32 {
        -self.0 - self.1
    }
    #[inline]
    pub fn z(&self) -> i32 {
        self.1
    }
    #[inline]
    pub fn q(&self) -> i32 {
        self.0
    }
    #[inline]
    pub fn r(&self) -> i32 {
        self.1
    }
    #[inline]
    pub fn length(&self) -> i32 {
        (self.0.abs() + (-self.0 - self.1).abs() + self.1.abs()) / 2
    }
    #[inline]
    pub fn distance(&self, rhs: &HexLocation) -> i32 {
        ((self.x() - rhs.x()).abs() + (self.y() - rhs.y()).abs() + (self.z() - rhs.z()).abs()) / 2
    }
    #[inline]
    pub fn neighbor(&self, direction: i32) -> HexLocation {
        match (6 + (direction % 6)) % 6 {
            0 => HexLocation(self.0 + 1, self.1 + 0),
            1 => HexLocation(self.0 + 1, self.1 - 1),
            2 => HexLocation(self.0 + 0, self.1 - 1),
            3 => HexLocation(self.0 - 0, self.1 + 0),
            4 => HexLocation(self.0 - 1, self.1 + 1),
            5 => HexLocation(self.0 + 0, self.1 + 1),
            _ => panic!("This cannot be outside of 0..6"),
        }
    }
    #[inline]
    pub fn diagonal_neighbor(&self, direction: i32) -> HexLocation {
        match (6 + (direction % 6)) % 6 {
            0 => HexLocation(self.0 + 2, self.1 - 1),
            1 => HexLocation(self.0 + 1, self.1 - 2),
            2 => HexLocation(self.0 - 1, self.1 - 1),
            3 => HexLocation(self.0 - 2, self.1 + 1),
            4 => HexLocation(self.0 - 1, self.1 + 2),
            5 => HexLocation(self.0 + 1, self.1 + 1),
            _ => panic!("This cannot be outside of 0..6"),
        }
    }
}

impl std::ops::Add<HexLocation> for HexLocation {
    type Output = HexLocation;

    #[inline]
    fn add(self, rhs: HexLocation) -> Self::Output {
        HexLocation(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign<HexLocation> for HexLocation {
    #[inline]
    fn add_assign(&mut self, rhs: HexLocation) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Sub<HexLocation> for HexLocation {
    type Output = HexLocation;

    #[inline]
    fn sub(self, rhs: HexLocation) -> Self::Output {
        HexLocation(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::SubAssign<HexLocation> for HexLocation {
    #[inline]
    fn sub_assign(&mut self, rhs: HexLocation) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl std::ops::Mul<i32> for HexLocation {
    type Output = HexLocation;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        HexLocation(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::MulAssign<i32> for HexLocation {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl From<HexPosition> for HexLocation {
    /// Implements Hex Rounding
    fn from(pos: HexPosition) -> Self {
        HexLocation::from(&pos)
    }
}

impl From<&HexPosition> for HexLocation {
    /// Implements Hex Rounding
    fn from(pos: &HexPosition) -> Self {
        let x = pos.x().round();
        let y = pos.y().round();
        let z = pos.z().round();

        let x_diff = (x - pos.x()).abs();
        let y_diff = (y - pos.y()).abs();
        let z_diff = (z - pos.z()).abs();

        if x_diff > y_diff && x_diff > z_diff {
            HexLocation((-y - z) as i32, z as i32)
        } else if y_diff > z_diff {
            HexLocation(x as i32, z as i32)
        } else {
            HexLocation(x as i32, (-x - y) as i32)
        }
    }
}