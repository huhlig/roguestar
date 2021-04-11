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

use crate::random::RandomNumberGenerator;

/// An implementation of the alias method implemented using a modified Vose's algorithm.
///
/// The alias method allows for efficient sampling of random values from a discrete probability
/// distribution (i.e. rolling a loaded die) in O(1) time each after O(n) preprocessing time.
///
/// For a complete writeup on the alias method, including the intuition and important proofs,
/// please see the article "Darts, Dice, and Coins: Sampling from a Discrete Distribution" at
/// http://www.keithschwarz.com/darts-dice-coins/
pub struct WeightedTable<V> {
    rng: RandomNumberGenerator,
    values: Vec<V>,
    alias: Vec<usize>,
    probability: Vec<f32>,
}

impl<V> WeightedTable<V> {
    /// Prepare a new Weighted Table
    pub fn prepare() -> WeightedTableBuilder<V> {
        WeightedTableBuilder {
            seed: None,
            total: 0.0,
            count: 0,
            values: Vec::new(),
            weights: Vec::new(),
        }
    }
    /// Query the weighted table.
    pub fn query(&mut self) -> &V {
        let index = self.rng.gen_range(0, self.values.len());
        if self.rng.gen::<f32>() <= self.probability[index] {
            &self.values[index]
        } else {
            &self.values[self.alias[index]]
        }
    }
}

/// Builder for [Weighted Table](sdl::collections::WeightedTable).
pub struct WeightedTableBuilder<V> {
    seed: Option<[u8; 32]>,
    total: f32,
    count: usize,
    values: Vec<V>,
    weights: Vec<f32>,
}

impl<V> WeightedTableBuilder<V> {
    /// Set or remove Seed Value
    pub fn seed(mut self, seed: Option<[u8; 32]>) -> WeightedTableBuilder<V> {
        self.seed = seed;
        self
    }
    /// Add Weighted Entry
    pub fn add(mut self, value: V, weight: f32) -> WeightedTableBuilder<V> {
        self.values.push(value);
        self.weights.push(weight);
        self.total += weight;
        self.count += 1;
        self
    }
    /// Reset Builder to Empty State
    pub fn reset(mut self) -> WeightedTableBuilder<V> {
        self.seed = None;
        self.count = 0;
        self.total = 0.0;
        self.values.clear();
        self.weights.clear();
        self
    }
    /// Build [Weighted Table](sdl::collections::WeightedTable)
    pub fn build(self) -> WeightedTable<V> {
        let mut alias = vec![0usize; self.count];
        let mut probability = vec![0.0; self.count];
        let mut small = Vec::with_capacity(self.count);
        let mut large = Vec::with_capacity(self.count);
        for idx in 0..self.count {
            // Normalize weight into probability, and then scale to average.
            let p = (self.weights[idx] / self.total) * self.count as f32;
            // Push index to queue based on being more or less than average.
            if p < 1.0 {
                small.push(idx)
            } else {
                large.push(idx)
            }
            // Set calculated probability to probability table.
            probability[idx] = p;
        }
        while let (Some(l), Some(g)) = (small.pop(), large.pop()) {
            probability[l] = probability[l];
            alias[l] = g;
            probability[g] = (probability[g] + probability[l]) - 1.0;
            if probability[g] < 1.0 {
                small.push(g)
            } else {
                large.push(g)
            }
        }
        while let Some(g) = large.pop() {
            probability[g] = 1.0;
        }
        while let Some(l) = small.pop() {
            probability[l] = 1.0;
        }

        let rng = if let Some(seed) = self.seed {
            Xoshiro256Plus::from_seed(seed)
        } else {
            Xoshiro256Plus::from_entropy()
        };

        WeightedTable {
            rng,
            probability,
            alias,
            values: self.values,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WeightedTable;

    #[test]
    fn test_balanced() {
        let mut table: WeightedTable<usize> = WeightedTable::prepare()
            .reset()
            .seed(Some([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]))
            .add(0, 1.0 / 6.0)
            .add(1, 1.0 / 6.0)
            .add(2, 1.0 / 6.0)
            .add(3, 1.0 / 6.0)
            .add(4, 1.0 / 6.0)
            .add(5, 1.0 / 6.0)
            .build();
        let total = 1_000_000;
        let mut results: [usize; 6] = [0, 0, 0, 0, 0, 0];
        for _ in 0..total {
            results[*table.query()] += 1;
        }
        println!(
            "[1: {}% 2: {}% 3: {}% 4: {}% 5: {}% 6: {}%]",
            results[0] as f32 / total as f32 * 100.0,
            results[1] as f32 / total as f32 * 100.0,
            results[2] as f32 / total as f32 * 100.0,
            results[3] as f32 / total as f32 * 100.0,
            results[4] as f32 / total as f32 * 100.0,
            results[5] as f32 / total as f32 * 100.0,
        );
    }

    #[test]
    fn test_loaded_six() {
        let mut table: WeightedTable<usize> = WeightedTable::prepare()
            .reset()
            .seed(Some([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]))
            .add(0, 1.0 / 6.0)
            .add(1, 1.0 / 6.0)
            .add(2, 1.0 / 6.0)
            .add(3, 1.0 / 6.0)
            .add(4, 1.0 / 6.0)
            .add(5, 2.0 / 6.0)
            .build();
        let total = 1_000_000;
        let mut results: [usize; 6] = [0, 0, 0, 0, 0, 0];
        for _ in 0..total {
            results[*table.query()] += 1;
        }
        println!(
            "[1: {}% 2: {}% 3: {}% 4: {}% 5: {}% 6: {}%]",
            results[0] as f32 / total as f32 * 100.0,
            results[1] as f32 / total as f32 * 100.0,
            results[2] as f32 / total as f32 * 100.0,
            results[3] as f32 / total as f32 * 100.0,
            results[4] as f32 / total as f32 * 100.0,
            results[5] as f32 / total as f32 * 100.0,
        );
    }
}
