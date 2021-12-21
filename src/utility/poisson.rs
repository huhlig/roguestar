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

//! Based on the work of [Martin Roberts](https://observablehq.com/@techsparx/an-improvement-on-bridsons-algorithm-for-poisson-disc-samp/2)
//!

use std::collections::VecDeque;

pub struct PoissonDiscSampler {
    k: usize,
    grid_radius: f64,
    grid_size: f64,
    grid: Vec<i32>,
    cell_size: f64,
    cluster_radius: f64,
    cluster_radius2: f64,
    queue: VecDeque<i32>,
}

impl PoissonDiscSampler {
    pub fn new(grid_radius: f64, cluster_radius: f64) -> PoissonDiscSampler {
        let k = 4;
        let cluster_radius2 = cluster_radius * cluster_radius;
        let cell_size = cluster_radius * f64::sqrt(0.5);
        let grid_size = f64::ceil(grid_radius / cell_size);
        let grid = Vec::with_capacity((grid_size * grid_size) as usize);
        let queue = VecDeque::new();
        PoissonDiscSampler {
            grid_radius,
            cluster_radius,
            cluster_radius2,
            cell_size,
            grid_size,
            grid,
            queue,
            k,
        }
    }
}

// function* poissonDiscSampler(width, height, radius) {
//   const k = 4; // maximum number of samples before rejection
//   const radius2 = radius * radius;
//   const cellSize = radius * Math.SQRT1_2;
//   const gridWidth = Math.ceil(width / cellSize);
//   const gridHeight = Math.ceil(height / cellSize);
//   const grid = new Array(gridWidth * gridHeight);
//   const queue = [];
//
//   // Pick the first sample.
//   yield {add: sample(width / 2 , height / 2, null)};
//
//   // Pick a random existing sample from the queue.
//   pick: while (queue.length) {
//     const i = Math.random() * queue.length | 0;
//     const parent = queue[i];
//     const seed = Math.random();
//     const epsilon = 0.0000001;
//
//     // Make a new candidate.
//     for (let j = 0; j < k; ++j) {
//       const a = 2 * Math.PI * (seed + 1.0*j/k);
//       const r = radius + epsilon;
//       const x = parent[0] + r * Math.cos(a);
//       const y = parent[1] + r * Math.sin(a);
//
//       // Accept candidates that are inside the allowed extent
//       // and farther than 2 * radius to all existing samples.
//       if (0 <= x && x < width && 0 <= y && y < height && far(x, y)) {
//         yield {add: sample(x, y), parent};
//         continue pick;
//       }
//     }
//
//     // If none of k candidates were accepted, remove it from the queue.
//     const r = queue.pop();
//     if (i < queue.length) queue[i] = r;
//     yield {remove: parent};
//   }
//
//   function far(x, y) {
//     const i = x / cellSize | 0;
//     const j = y / cellSize | 0;
//     const i0 = Math.max(i - 2, 0);
//     const j0 = Math.max(j - 2, 0);
//     const i1 = Math.min(i + 3, gridWidth);
//     const j1 = Math.min(j + 3, gridHeight);
//     for (let j = j0; j < j1; ++j) {
//       const o = j * gridWidth;
//       for (let i = i0; i < i1; ++i) {
//         const s = grid[o + i];
//         if (s) {
//           const dx = s[0] - x;
//           const dy = s[1] - y;
//           if (dx * dx + dy * dy < radius2) return false;
//         }
//       }
//     }
//     return true;
//   }
//
//   function sample(x, y, parent) {
//     const s = grid[gridWidth * (y / cellSize | 0) + (x / cellSize | 0)] = [x, y];
//     queue.push(s);
//     return s;
//   }
// }
