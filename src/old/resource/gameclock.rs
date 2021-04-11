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

use bevy::core::Time;
use bevy::ecs::Res;

pub struct GameClock {
    state: State,
    speed: f64,
    time: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    StepUntil(f64),
    Realtime,
    Paused,
}

impl GameClock {
    /// Create a new Game clock
    pub fn new(time: f64, speed: f64) -> GameClock {
        GameClock {
            state: State::Realtime,
            speed: 1.0,
            time: 0.0,
        }
    }
    /// Pause Game Clock
    pub fn pause(&mut self) {
        self.state = State::Paused
    }
    /// Unpause game clock
    pub fn unpause(&mut self) {
        self.state = State::Realtime
    }
    /// Is the clock currently paused.
    pub fn is_paused(&self) -> bool {
        self.state == State::Paused
    }
    /// Get Simulation Time in Seconds
    pub fn get_time(&self) -> f64 {
        self.time
    }
    /// Get current clock speed
    pub fn get_speed(&self) -> f64 {
        self.speed
    }
    /// Advance clock by one turn.
    pub fn update(&mut self, time: Res<Time>) {
        let delta = time.delta_seconds_f64() * self.speed;
        match self.state {
            State::Realtime => self.time += delta,
            State::StepUntil(val) => {
                if delta >= val {
                    self.time += val;
                    self.state = State::Paused;
                } else {
                    self.time += delta;
                    self.state = State::StepUntil(val - time.delta_seconds_f64())
                }
            }
            State::Paused => {}
        }
    }
}

impl Default for GameClock {
    fn default() -> GameClock {
        GameClock {
            state: State::Realtime,
            speed: 1.0,
            time: 0.0,
        }
    }
}
