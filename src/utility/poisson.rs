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

//! Poisson Disc Generator

const SQRT1_2: f64 = f64::sqrt(0.5);

/// A Simple Poisson Generator based on Bostock's improved Bridson Algorithm.
/// https://observablehq.com/@techsparx/an-improvement-on-bridsons-algorithm-for-poisson-disc-samp/2
///
pub struct Poisson {
    /// Area Poisson Generator will generate within
    area: ((isize, isize), (isize, isize)),
    /// Exclusion Zone radius around each point
    radius: f64,
    /// Maximum number of samples before rejection
    samples: usize,
    grid_width: f64,
}

impl Poisson {
    pub fn new(area: ((isize, isize), (isize, isize)), radius: f64, samples: usize) -> Poisson {
        // function* poissonDiscSampler(width, height, radius) {
        //   const k = 4; // maximum number of samples before rejection
        //   const radius2 = radius * radius;
        //   const cellSize = radius * Math.SQRT1_2;
        //   const gridWidth = Math.ceil(width / cellSize);
        //   const gridHeight = Math.ceil(height / cellSize);
        //   const grid = new Array(gridWidth * gridHeight);
        //   const queue = [];
        let width = (area.0 .0 - area.1 .0).abs();
        let height = (area.1 .0 - area.1 .1).abs();
        let x_offset = 0;
        let y_offset = 0;
        let radius_2 = radius * radius;
        let cell_size = radius * SQRT1_2;
        let grid_width = (width as f64 / cell_size).ceil();
        let grid_height = (height as f64 / cell_size).ceil();
        Poisson {
            area,
            radius,
            samples,
        }
    }
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
    pub fn far(&mut self, x: f64, y: f64) -> bool {
        let i = x / self.cell_size | 0;
        let j = y / self.cell_size | 0;
        let i0 = f64::max(i - 2, 0.0);
        let j0 = f64::max(j - 2, 0.0);
        let i1 = f64::min(i + 3, self.grid_width);
        let j1 = f64::min(j + 3, self.grid_height);
        for j in j0..j1 {
            let o = j * self.grid_width;
            for i in i0..i1 {
                let s = grid[o + i];
                if s {
                    let dx = s[0] - x;
                    let dy = s[1] - y;
                    if dx * dx + dy * dy < radius2 {
                        return false;
                    }
                }
            }
        }
        true
    }
    //   function sample(x, y, parent) {
    //     const s = grid[gridWidth * (y / cellSize | 0) + (x / cellSize | 0)] = [x, y];
    //     queue.push(s);
    //     return s;
    //   }
    fn sample(&mut self, x: f64, y: f64) -> bool {
        let s = (x, y);
        self.grid.set()
    }
    // }
}
