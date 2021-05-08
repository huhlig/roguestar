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

/// Iterates through all hexes in a hexagonal shaped pattern.
pub struct HexIterator {
    origin: HexLocation,
    radius: i32,
    q: i32,
    r1: i32,
    r2: i32,
    r: i32,
}

impl HexIterator {
    pub fn new(origin: HexLocation, radius: i32) -> HexIterator {
        let q = -radius;
        let r1 = i32::max(-radius, -q - radius);
        let r2 = i32::min(radius, -q + radius);
        HexIterator {
            origin,
            radius,
            q,
            r1,
            r2,
            r: r1,
        }
    }
}

impl Iterator for HexIterator {
    type Item = HexLocation;

    fn next(&mut self) -> Option<Self::Item> {
        if self.r > self.r2 {
            self.q += 1;
            self.r1 = i32::max(-self.radius, -self.q - self.radius);
            self.r2 = i32::min(self.radius, -self.q + self.radius);
            self.r = self.r1;
        }
        if self.q > self.radius {
            None
        } else {
            let hex = self.origin + HexLocation::from_axial(self.q, self.r);
            self.r += 1;
            Some(hex)
        }
    }
}
