//
// Copyright 2019-2021 Hans W. Uhlig. All Rights Reserved.
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

use super::{ProtoCluster, ProtoFaction, ProtoSector};
use crate::generation::ProtoOrbital;
use bevy::prelude::Commands;
use hexgrid::{Coordinate, Direction, Spin, Spiral};
use nominae::Totro;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoroshiro128PlusPlus;
use std::ops::Range;
use tracing::trace;

/// Current Generation State
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GenerationState {
    Configuration,
    Initialization,
    ClusterGeneration,
    SectorGeneration(Spiral<i32>),
    Finalization,
    Complete,
}

impl Default for GenerationState {
    fn default() -> GenerationState {
        GenerationState::Configuration
    }
}

/// Configuration for Generation
#[derive(Clone, Debug)]
pub struct GenerationConfiguration {
    pub universe_seed: u64,
    pub universe_radius: i32,
    pub cluster_density: f32,
    pub cluster_size: Range<i32>,
}

impl Default for GenerationConfiguration {
    fn default() -> Self {
        GenerationConfiguration {
            universe_seed: 0,
            universe_radius: 100,
            cluster_density: 1.0,
            cluster_size: 5..15,
        }
    }
}

#[derive(Debug)]
pub struct GenerationContext {
    pub state: GenerationState,
    pub random: Xoroshiro128PlusPlus,
    pub config: GenerationConfiguration,
    pub clusters: Vec<ProtoCluster>,
    pub sectors: Vec<ProtoSector>,
    pub orbitals: Vec<ProtoOrbital>,
    pub factions: Vec<ProtoFaction>,
}

impl Default for GenerationContext {
    fn default() -> GenerationContext {
        GenerationContext {
            state: GenerationState::Configuration,
            random: Xoroshiro128PlusPlus::from_seed([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            config: GenerationConfiguration::default(),
            clusters: Vec::default(),
            sectors: Vec::default(),
            orbitals: Vec::default(),
            factions: Vec::default(),
        }
    }
}

impl GenerationContext {
    pub fn clusters(&self) -> &[ProtoCluster] {
        &self.clusters
    }
    pub fn sectors(&self) -> &[ProtoSector] {
        &self.sectors
    }
    pub fn factions(&self) -> &[ProtoFaction] {
        &self.factions
    }
    pub fn step(&mut self, mut commands: Commands) {
        match self.state {
            GenerationState::Configuration => {
                self.state = GenerationState::Initialization;
                trace!("Generation set to Initialization State");
            }
            GenerationState::Initialization => {
                self.random = Xoroshiro128PlusPlus::seed_from_u64(self.config.universe_seed);
                trace!("Generation set to Cluster Generation State");
                self.state = GenerationState::ClusterGeneration;
            }
            GenerationState::ClusterGeneration => {
                if self.step_generate_cluster(commands) {
                    trace!("Generation set to Sector Generation State");
                    self.state = GenerationState::SectorGeneration(
                        Coordinate::from_cubic(0, 0)
                            .spiral_iter(self.config.universe_radius, Spin::CW(Direction::XY)),
                    );
                }
            }
            GenerationState::SectorGeneration(mut iter) => {
                if let Some(location) = iter.next() {
                    trace!(
                        "Generation of Sector {:?} in ring {}",
                        &location,
                        iter.layer()
                    );
                    self.step_generate_sector(commands, location);
                } else {
                    trace!("Generation set to Finalization State");
                    self.state = GenerationState::Finalization;
                }
            }
            GenerationState::Finalization => {
                trace!("Generation set to Complete State");
                self.state = GenerationState::Complete;
            }
            GenerationState::Complete => {
                panic!("Logic Error, Complete State reached in GenerationContext::step")
            }
        }
    }
}
