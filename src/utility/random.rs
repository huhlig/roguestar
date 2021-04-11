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

use rand::distributions::{Distribution, Uniform};
use rand::{Rng, RngCore, SeedableRng};
use rand_xoshiro::{Seed512, Xoshiro512StarStar};

/// Random Number Generator for RogueStar
pub struct RandomNumberGenerator(Xoshiro512StarStar);

impl RandomNumberGenerator {
    /// Create Random Number Generator from Entropy
    pub fn from_entropy() -> RandomNumberGenerator {
        RandomNumberGenerator(Xoshiro512StarStar::from_entropy())
    }
    /// Create Random Number Generator from 64bit Seed
    pub fn from_seed(seed: u64) -> RandomNumberGenerator {
        RandomNumberGenerator(Xoshiro512StarStar::seed_from_u64(seed))
    }
    /// Returns a random value of whatever type you specify
    pub fn rand<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        self.0.gen::<T>()
    }
    /// Returns a random value in the specified range, of type specified at the call site.
    /// This is INCLUSIVE of the first parameter, and EXCLUSIVE of the second.
    /// So range(1,6) will give you numbers from 1 to 5.
    pub fn range<T>(&mut self, min: T, max: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
    {
        self.0.gen_range(min, max)
    }
    /// Roll dice using the classic xDy format and add them together.
    pub fn accumulate(&mut self, count: usize, sides: usize) -> usize {
        (0..count).map(|_| self.0.gen_range(0, sides)).sum()
    }
    /// Roll dice and count successes
    fn successes(&mut self, count: usize, sides: usize, target: usize) -> usize {
        (0..count)
            .map(|_| self.0.gen_range(0, sides))
            .filter(|i| *i > target)
            .count()
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> RandomNumberGenerator {
        RandomNumberGenerator(Xoshiro512StarStar::from_seed(Seed512::default()))
    }
}

pub struct DiceIterator<'a, T: RngCore> {
    rng: &'a mut T,
    die: usize,
}
