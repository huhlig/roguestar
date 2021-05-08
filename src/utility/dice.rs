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

use rand::Rng;

/// Dice Exteension to [Rng](rand::Rng)
pub trait Dice: rand::Rng {
    /// Roll `count` dice with `pips` and count the total above `target`.
    fn roll_target(&mut self, count: usize, pips: usize, target: usize) -> usize {
        (0..count)
            .map(|_| self.gen_range(1..pips))
            .filter(|v| *v >= target)
            .count()
    }
    /// Roll `count` dice with `pips` and sum the total pips.
    fn roll_sum(&mut self, count: usize, pips: usize) -> usize {
        (0..count).map(|_| self.gen_range(1..pips)).sum()
    }
    /// Roll percentile dice and return the value between 0 and 100.
    fn roll_pct(&mut self) -> f32 {
        self.gen_range(0.0..100.0)
    }
}
